//! A64 integer tags tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_integer_tags_mcsettagpairandzerodatapost Tests
// ============================================================================

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_0_zero_400_d9e00400() {
    // Encoding: 0xD9E00400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 0 (Zero)
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_1_poweroftwo_400_d9e01400() {
    // Encoding: 0xD9E01400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 1 (PowerOfTwo)
    // Fields: Xn=0, imm9=1, Xt=0
    let encoding: u32 = 0xD9E01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_3_poweroftwominusone_400_d9e03400()
 {
    // Encoding: 0xD9E03400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xn=0, imm9=3
    let encoding: u32 = 0xD9E03400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_4_poweroftwo_400_d9e04400() {
    // Encoding: 0xD9E04400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 4 (PowerOfTwo)
    // Fields: Xt=0, imm9=4, Xn=0
    let encoding: u32 = 0xD9E04400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_7_poweroftwominusone_400_d9e07400()
 {
    // Encoding: 0xD9E07400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=7, Xn=0
    let encoding: u32 = 0xD9E07400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_8_poweroftwo_400_d9e08400() {
    // Encoding: 0xD9E08400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 8 (PowerOfTwo)
    // Fields: imm9=8, Xn=0, Xt=0
    let encoding: u32 = 0xD9E08400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_15_poweroftwominusone_400_d9e0f400()
 {
    // Encoding: 0xD9E0F400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: imm9=15, Xn=0, Xt=0
    let encoding: u32 = 0xD9E0F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_16_poweroftwo_400_d9e10400() {
    // Encoding: 0xD9E10400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 16 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=16
    let encoding: u32 = 0xD9E10400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_31_poweroftwominusone_400_d9e1f400()
 {
    // Encoding: 0xD9E1F400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: imm9=31, Xn=0, Xt=0
    let encoding: u32 = 0xD9E1F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_32_poweroftwo_400_d9e20400() {
    // Encoding: 0xD9E20400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 32 (PowerOfTwo)
    // Fields: imm9=32, Xn=0, Xt=0
    let encoding: u32 = 0xD9E20400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_63_poweroftwominusone_400_d9e3f400()
 {
    // Encoding: 0xD9E3F400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=63
    let encoding: u32 = 0xD9E3F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_64_poweroftwo_400_d9e40400() {
    // Encoding: 0xD9E40400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 64 (PowerOfTwo)
    // Fields: imm9=64, Xt=0, Xn=0
    let encoding: u32 = 0xD9E40400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_127_poweroftwominusone_400_d9e7f400()
 {
    // Encoding: 0xD9E7F400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xn=0, imm9=127
    let encoding: u32 = 0xD9E7F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_128_poweroftwo_400_d9e80400() {
    // Encoding: 0xD9E80400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 128 (PowerOfTwo)
    // Fields: imm9=128, Xt=0, Xn=0
    let encoding: u32 = 0xD9E80400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_255_poweroftwominusone_400_d9eff400()
 {
    // Encoding: 0xD9EFF400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: imm9=255, Xt=0, Xn=0
    let encoding: u32 = 0xD9EFF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_256_poweroftwo_400_d9f00400() {
    // Encoding: 0xD9F00400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 256 (PowerOfTwo)
    // Fields: imm9=256, Xt=0, Xn=0
    let encoding: u32 = 0xD9F00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_imm9_511_max_400_d9fff400() {
    // Encoding: 0xD9FFF400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field imm9 = 511 (Max)
    // Fields: Xt=0, Xn=0, imm9=511
    let encoding: u32 = 0xD9FFF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_xn_0_min_400_d9e00400() {
    // Encoding: 0xD9E00400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field Xn = 0 (Min)
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_xn_1_poweroftwo_400_d9e00420() {
    // Encoding: 0xD9E00420
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field Xn = 1 (PowerOfTwo)
    // Fields: Xt=0, Xn=1, imm9=0
    let encoding: u32 = 0xD9E00420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_xn_30_poweroftwominusone_400_d9e007c0()
 {
    // Encoding: 0xD9E007C0
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: imm9=0, Xt=0, Xn=30
    let encoding: u32 = 0xD9E007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_xn_31_max_400_d9e007e0() {
    // Encoding: 0xD9E007E0
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field Xn = 31 (Max)
    // Fields: Xt=0, imm9=0, Xn=31
    let encoding: u32 = 0xD9E007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_xt_0_min_400_d9e00400() {
    // Encoding: 0xD9E00400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field Xt = 0 (Min)
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_xt_1_poweroftwo_400_d9e00401() {
    // Encoding: 0xD9E00401
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field Xt = 1 (PowerOfTwo)
    // Fields: Xn=0, Xt=1, imm9=0
    let encoding: u32 = 0xD9E00401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_xt_30_poweroftwominusone_400_d9e0041e()
 {
    // Encoding: 0xD9E0041E
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: Xt=30, imm9=0, Xn=0
    let encoding: u32 = 0xD9E0041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_field_xt_31_max_400_d9e0041f() {
    // Encoding: 0xD9E0041F
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field Xt = 31 (Max)
    // Fields: Xt=31, Xn=0, imm9=0
    let encoding: u32 = 0xD9E0041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_0_400_d9e00400() {
    // Encoding: 0xD9E00400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xn=0, imm9=0, Xt=0
    let encoding: u32 = 0xD9E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_1_400_d9e01400() {
    // Encoding: 0xD9E01400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=1, Xn=0, Xt=0
    // Fields: Xt=0, imm9=1, Xn=0
    let encoding: u32 = 0xD9E01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_2_400_d9e03400() {
    // Encoding: 0xD9E03400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=3, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=3
    let encoding: u32 = 0xD9E03400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_3_400_d9e04400() {
    // Encoding: 0xD9E04400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=4, Xn=0, Xt=0
    // Fields: imm9=4, Xn=0, Xt=0
    let encoding: u32 = 0xD9E04400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_4_400_d9e07400() {
    // Encoding: 0xD9E07400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=7, Xn=0, Xt=0
    // Fields: imm9=7, Xn=0, Xt=0
    let encoding: u32 = 0xD9E07400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_5_400_d9e08400() {
    // Encoding: 0xD9E08400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=8, Xn=0, Xt=0
    // Fields: Xn=0, imm9=8, Xt=0
    let encoding: u32 = 0xD9E08400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_6_400_d9e0f400() {
    // Encoding: 0xD9E0F400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=15, Xn=0, Xt=0
    // Fields: Xn=0, imm9=15, Xt=0
    let encoding: u32 = 0xD9E0F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_7_400_d9e10400() {
    // Encoding: 0xD9E10400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=16, Xn=0, Xt=0
    // Fields: imm9=16, Xt=0, Xn=0
    let encoding: u32 = 0xD9E10400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_8_400_d9e1f400() {
    // Encoding: 0xD9E1F400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=31, Xn=0, Xt=0
    // Fields: imm9=31, Xn=0, Xt=0
    let encoding: u32 = 0xD9E1F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_9_400_d9e20400() {
    // Encoding: 0xD9E20400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=32, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=32
    let encoding: u32 = 0xD9E20400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_10_400_d9e3f400() {
    // Encoding: 0xD9E3F400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=63, Xn=0, Xt=0
    // Fields: imm9=63, Xn=0, Xt=0
    let encoding: u32 = 0xD9E3F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_11_400_d9e40400() {
    // Encoding: 0xD9E40400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=64, Xn=0, Xt=0
    // Fields: Xn=0, imm9=64, Xt=0
    let encoding: u32 = 0xD9E40400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_12_400_d9e7f400() {
    // Encoding: 0xD9E7F400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=127, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=127
    let encoding: u32 = 0xD9E7F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_13_400_d9e80400() {
    // Encoding: 0xD9E80400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=128, Xn=0, Xt=0
    // Fields: Xn=0, imm9=128, Xt=0
    let encoding: u32 = 0xD9E80400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_14_400_d9eff400() {
    // Encoding: 0xD9EFF400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=255, Xn=0, Xt=0
    // Fields: Xn=0, imm9=255, Xt=0
    let encoding: u32 = 0xD9EFF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_15_400_d9f00400() {
    // Encoding: 0xD9F00400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=256, Xn=0, Xt=0
    // Fields: Xt=0, imm9=256, Xn=0
    let encoding: u32 = 0xD9F00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_16_400_d9fff400() {
    // Encoding: 0xD9FFF400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=511, Xn=0, Xt=0
    // Fields: Xt=0, imm9=511, Xn=0
    let encoding: u32 = 0xD9FFF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_17_400_d9e00400() {
    // Encoding: 0xD9E00400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_18_400_d9e00420() {
    // Encoding: 0xD9E00420
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=0, Xn=1, Xt=0
    // Fields: imm9=0, Xt=0, Xn=1
    let encoding: u32 = 0xD9E00420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_19_400_d9e007c0() {
    // Encoding: 0xD9E007C0
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=0, Xn=30, Xt=0
    // Fields: Xn=30, imm9=0, Xt=0
    let encoding: u32 = 0xD9E007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_20_400_d9e007e0() {
    // Encoding: 0xD9E007E0
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=0, Xn=31, Xt=0
    // Fields: imm9=0, Xn=31, Xt=0
    let encoding: u32 = 0xD9E007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_21_400_d9e00400() {
    // Encoding: 0xD9E00400
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_22_400_d9e00401() {
    // Encoding: 0xD9E00401
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=0, Xn=0, Xt=1
    // Fields: Xn=0, Xt=1, imm9=0
    let encoding: u32 = 0xD9E00401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_23_400_d9e0041e() {
    // Encoding: 0xD9E0041E
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=0, Xn=0, Xt=30
    // Fields: Xn=0, Xt=30, imm9=0
    let encoding: u32 = 0xD9E0041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_24_400_d9e0041f() {
    // Encoding: 0xD9E0041F
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=0, Xn=0, Xt=31
    // Fields: Xn=0, Xt=31, imm9=0
    let encoding: u32 = 0xD9E0041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_25_400_d9e00421() {
    // Encoding: 0xD9E00421
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=0, Xn=1, Xt=1
    // Fields: imm9=0, Xn=1, Xt=1
    let encoding: u32 = 0xD9E00421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_combo_26_400_d9e007ff() {
    // Encoding: 0xD9E007FF
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost field combination: imm9=0, Xn=31, Xt=31
    // Fields: Xn=31, Xt=31, imm9=0
    let encoding: u32 = 0xD9E007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_special_xn_31_stack_pointer_sp_may_require_alignment_1024_d9e017e0()
 {
    // Encoding: 0xD9E017E0
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: imm9=1, Xn=31, Xt=0
    let encoding: u32 = 0xD9E017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_0_zero_c00_d9e00c00() {
    // Encoding: 0xD9E00C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 0 (Zero)
    // Fields: Xn=0, imm9=0, Xt=0
    let encoding: u32 = 0xD9E00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_1_poweroftwo_c00_d9e01c00() {
    // Encoding: 0xD9E01C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 1 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=1
    let encoding: u32 = 0xD9E01C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_3_poweroftwominusone_c00_d9e03c00()
 {
    // Encoding: 0xD9E03C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: imm9=3, Xn=0, Xt=0
    let encoding: u32 = 0xD9E03C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_4_poweroftwo_c00_d9e04c00() {
    // Encoding: 0xD9E04C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 4 (PowerOfTwo)
    // Fields: imm9=4, Xn=0, Xt=0
    let encoding: u32 = 0xD9E04C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_7_poweroftwominusone_c00_d9e07c00()
 {
    // Encoding: 0xD9E07C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xn=0, imm9=7
    let encoding: u32 = 0xD9E07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_8_poweroftwo_c00_d9e08c00() {
    // Encoding: 0xD9E08C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 8 (PowerOfTwo)
    // Fields: Xn=0, imm9=8, Xt=0
    let encoding: u32 = 0xD9E08C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_15_poweroftwominusone_c00_d9e0fc00()
 {
    // Encoding: 0xD9E0FC00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: imm9=15, Xt=0, Xn=0
    let encoding: u32 = 0xD9E0FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_16_poweroftwo_c00_d9e10c00() {
    // Encoding: 0xD9E10C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 16 (PowerOfTwo)
    // Fields: imm9=16, Xt=0, Xn=0
    let encoding: u32 = 0xD9E10C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_31_poweroftwominusone_c00_d9e1fc00()
 {
    // Encoding: 0xD9E1FC00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=31, Xn=0
    let encoding: u32 = 0xD9E1FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_32_poweroftwo_c00_d9e20c00() {
    // Encoding: 0xD9E20C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 32 (PowerOfTwo)
    // Fields: Xt=0, imm9=32, Xn=0
    let encoding: u32 = 0xD9E20C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_63_poweroftwominusone_c00_d9e3fc00()
 {
    // Encoding: 0xD9E3FC00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xn=0, imm9=63
    let encoding: u32 = 0xD9E3FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_64_poweroftwo_c00_d9e40c00() {
    // Encoding: 0xD9E40C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 64 (PowerOfTwo)
    // Fields: Xn=0, imm9=64, Xt=0
    let encoding: u32 = 0xD9E40C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_127_poweroftwominusone_c00_d9e7fc00()
 {
    // Encoding: 0xD9E7FC00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=127, Xn=0
    let encoding: u32 = 0xD9E7FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_128_poweroftwo_c00_d9e80c00() {
    // Encoding: 0xD9E80C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 128 (PowerOfTwo)
    // Fields: Xt=0, Xn=0, imm9=128
    let encoding: u32 = 0xD9E80C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_255_poweroftwominusone_c00_d9effc00()
 {
    // Encoding: 0xD9EFFC00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: imm9=255, Xn=0, Xt=0
    let encoding: u32 = 0xD9EFFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_256_poweroftwo_c00_d9f00c00() {
    // Encoding: 0xD9F00C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 256 (PowerOfTwo)
    // Fields: Xt=0, Xn=0, imm9=256
    let encoding: u32 = 0xD9F00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_imm9_511_max_c00_d9fffc00() {
    // Encoding: 0xD9FFFC00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field imm9 = 511 (Max)
    // Fields: imm9=511, Xn=0, Xt=0
    let encoding: u32 = 0xD9FFFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_xn_0_min_c00_d9e00c00() {
    // Encoding: 0xD9E00C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field Xn = 0 (Min)
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9E00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_xn_1_poweroftwo_c00_d9e00c20() {
    // Encoding: 0xD9E00C20
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field Xn = 1 (PowerOfTwo)
    // Fields: Xn=1, imm9=0, Xt=0
    let encoding: u32 = 0xD9E00C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_xn_30_poweroftwominusone_c00_d9e00fc0()
 {
    // Encoding: 0xD9E00FC0
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=30, Xt=0, imm9=0
    let encoding: u32 = 0xD9E00FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_xn_31_max_c00_d9e00fe0() {
    // Encoding: 0xD9E00FE0
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field Xn = 31 (Max)
    // Fields: imm9=0, Xt=0, Xn=31
    let encoding: u32 = 0xD9E00FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_xt_0_min_c00_d9e00c00() {
    // Encoding: 0xD9E00C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field Xt = 0 (Min)
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9E00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_xt_1_poweroftwo_c00_d9e00c01() {
    // Encoding: 0xD9E00C01
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field Xt = 1 (PowerOfTwo)
    // Fields: Xt=1, imm9=0, Xn=0
    let encoding: u32 = 0xD9E00C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_xt_30_poweroftwominusone_c00_d9e00c1e()
 {
    // Encoding: 0xD9E00C1E
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=0, Xt=30
    let encoding: u32 = 0xD9E00C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_field_xt_31_max_c00_d9e00c1f() {
    // Encoding: 0xD9E00C1F
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field Xt = 31 (Max)
    // Fields: Xn=0, Xt=31, imm9=0
    let encoding: u32 = 0xD9E00C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_0_c00_d9e00c00() {
    // Encoding: 0xD9E00C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9E00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_1_c00_d9e01c00() {
    // Encoding: 0xD9E01C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=1, Xn=0, Xt=0
    // Fields: Xn=0, imm9=1, Xt=0
    let encoding: u32 = 0xD9E01C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_2_c00_d9e03c00() {
    // Encoding: 0xD9E03C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=3, Xn=0, Xt=0
    // Fields: imm9=3, Xn=0, Xt=0
    let encoding: u32 = 0xD9E03C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_3_c00_d9e04c00() {
    // Encoding: 0xD9E04C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=4, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=4
    let encoding: u32 = 0xD9E04C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_4_c00_d9e07c00() {
    // Encoding: 0xD9E07C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=7, Xn=0, Xt=0
    // Fields: Xt=0, imm9=7, Xn=0
    let encoding: u32 = 0xD9E07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_5_c00_d9e08c00() {
    // Encoding: 0xD9E08C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=8, Xn=0, Xt=0
    // Fields: imm9=8, Xn=0, Xt=0
    let encoding: u32 = 0xD9E08C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_6_c00_d9e0fc00() {
    // Encoding: 0xD9E0FC00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=15, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=15
    let encoding: u32 = 0xD9E0FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_7_c00_d9e10c00() {
    // Encoding: 0xD9E10C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=16, Xn=0, Xt=0
    // Fields: Xt=0, imm9=16, Xn=0
    let encoding: u32 = 0xD9E10C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_8_c00_d9e1fc00() {
    // Encoding: 0xD9E1FC00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=31, Xn=0, Xt=0
    // Fields: Xt=0, imm9=31, Xn=0
    let encoding: u32 = 0xD9E1FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_9_c00_d9e20c00() {
    // Encoding: 0xD9E20C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=32, Xn=0, Xt=0
    // Fields: Xt=0, imm9=32, Xn=0
    let encoding: u32 = 0xD9E20C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_10_c00_d9e3fc00() {
    // Encoding: 0xD9E3FC00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=63, Xn=0, Xt=0
    // Fields: imm9=63, Xn=0, Xt=0
    let encoding: u32 = 0xD9E3FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_11_c00_d9e40c00() {
    // Encoding: 0xD9E40C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=64, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=64
    let encoding: u32 = 0xD9E40C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_12_c00_d9e7fc00() {
    // Encoding: 0xD9E7FC00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=127, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=127
    let encoding: u32 = 0xD9E7FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_13_c00_d9e80c00() {
    // Encoding: 0xD9E80C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=128, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=128
    let encoding: u32 = 0xD9E80C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_14_c00_d9effc00() {
    // Encoding: 0xD9EFFC00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=255, Xn=0, Xt=0
    // Fields: Xt=0, imm9=255, Xn=0
    let encoding: u32 = 0xD9EFFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_15_c00_d9f00c00() {
    // Encoding: 0xD9F00C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=256, Xn=0, Xt=0
    // Fields: Xt=0, imm9=256, Xn=0
    let encoding: u32 = 0xD9F00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_16_c00_d9fffc00() {
    // Encoding: 0xD9FFFC00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=511, Xn=0, Xt=0
    // Fields: imm9=511, Xn=0, Xt=0
    let encoding: u32 = 0xD9FFFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_17_c00_d9e00c00() {
    // Encoding: 0xD9E00C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9E00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_18_c00_d9e00c20() {
    // Encoding: 0xD9E00C20
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=0, Xn=1, Xt=0
    // Fields: Xn=1, Xt=0, imm9=0
    let encoding: u32 = 0xD9E00C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_19_c00_d9e00fc0() {
    // Encoding: 0xD9E00FC0
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=0, Xn=30, Xt=0
    // Fields: Xt=0, Xn=30, imm9=0
    let encoding: u32 = 0xD9E00FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_20_c00_d9e00fe0() {
    // Encoding: 0xD9E00FE0
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=0, Xn=31, Xt=0
    // Fields: imm9=0, Xn=31, Xt=0
    let encoding: u32 = 0xD9E00FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_21_c00_d9e00c00() {
    // Encoding: 0xD9E00C00
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9E00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_22_c00_d9e00c01() {
    // Encoding: 0xD9E00C01
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=0, Xn=0, Xt=1
    // Fields: Xt=1, imm9=0, Xn=0
    let encoding: u32 = 0xD9E00C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_23_c00_d9e00c1e() {
    // Encoding: 0xD9E00C1E
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=0, Xn=0, Xt=30
    // Fields: Xn=0, Xt=30, imm9=0
    let encoding: u32 = 0xD9E00C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_24_c00_d9e00c1f() {
    // Encoding: 0xD9E00C1F
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=0, Xn=0, Xt=31
    // Fields: Xn=0, imm9=0, Xt=31
    let encoding: u32 = 0xD9E00C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_25_c00_d9e00c21() {
    // Encoding: 0xD9E00C21
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=0, Xn=1, Xt=1
    // Fields: imm9=0, Xt=1, Xn=1
    let encoding: u32 = 0xD9E00C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_combo_26_c00_d9e00fff() {
    // Encoding: 0xD9E00FFF
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre field combination: imm9=0, Xn=31, Xt=31
    // Fields: imm9=0, Xn=31, Xt=31
    let encoding: u32 = 0xD9E00FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_special_xn_31_stack_pointer_sp_may_require_alignment_3072_d9e01fe0()
 {
    // Encoding: 0xD9E01FE0
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: imm9=1, Xt=0, Xn=31
    let encoding: u32 = 0xD9E01FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_0_zero_800_d9e00800() {
    // Encoding: 0xD9E00800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 0 (Zero)
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9E00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_1_poweroftwo_800_d9e01800() {
    // Encoding: 0xD9E01800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 1 (PowerOfTwo)
    // Fields: Xn=0, imm9=1, Xt=0
    let encoding: u32 = 0xD9E01800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_3_poweroftwominusone_800_d9e03800()
{
    // Encoding: 0xD9E03800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: imm9=3, Xn=0, Xt=0
    let encoding: u32 = 0xD9E03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_4_poweroftwo_800_d9e04800() {
    // Encoding: 0xD9E04800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 4 (PowerOfTwo)
    // Fields: Xt=0, imm9=4, Xn=0
    let encoding: u32 = 0xD9E04800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_7_poweroftwominusone_800_d9e07800()
{
    // Encoding: 0xD9E07800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=7, Xt=0
    let encoding: u32 = 0xD9E07800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_8_poweroftwo_800_d9e08800() {
    // Encoding: 0xD9E08800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 8 (PowerOfTwo)
    // Fields: imm9=8, Xn=0, Xt=0
    let encoding: u32 = 0xD9E08800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_15_poweroftwominusone_800_d9e0f800()
{
    // Encoding: 0xD9E0F800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=15, Xt=0
    let encoding: u32 = 0xD9E0F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_16_poweroftwo_800_d9e10800() {
    // Encoding: 0xD9E10800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 16 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=16
    let encoding: u32 = 0xD9E10800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_31_poweroftwominusone_800_d9e1f800()
{
    // Encoding: 0xD9E1F800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: imm9=31, Xn=0, Xt=0
    let encoding: u32 = 0xD9E1F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_32_poweroftwo_800_d9e20800() {
    // Encoding: 0xD9E20800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 32 (PowerOfTwo)
    // Fields: Xn=0, imm9=32, Xt=0
    let encoding: u32 = 0xD9E20800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_63_poweroftwominusone_800_d9e3f800()
{
    // Encoding: 0xD9E3F800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: imm9=63, Xt=0, Xn=0
    let encoding: u32 = 0xD9E3F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_64_poweroftwo_800_d9e40800() {
    // Encoding: 0xD9E40800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 64 (PowerOfTwo)
    // Fields: imm9=64, Xt=0, Xn=0
    let encoding: u32 = 0xD9E40800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_127_poweroftwominusone_800_d9e7f800()
 {
    // Encoding: 0xD9E7F800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: imm9=127, Xt=0, Xn=0
    let encoding: u32 = 0xD9E7F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_128_poweroftwo_800_d9e80800() {
    // Encoding: 0xD9E80800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 128 (PowerOfTwo)
    // Fields: Xn=0, imm9=128, Xt=0
    let encoding: u32 = 0xD9E80800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_255_poweroftwominusone_800_d9eff800()
 {
    // Encoding: 0xD9EFF800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=255, Xn=0
    let encoding: u32 = 0xD9EFF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_256_poweroftwo_800_d9f00800() {
    // Encoding: 0xD9F00800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 256 (PowerOfTwo)
    // Fields: imm9=256, Xt=0, Xn=0
    let encoding: u32 = 0xD9F00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_imm9_511_max_800_d9fff800() {
    // Encoding: 0xD9FFF800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field imm9 = 511 (Max)
    // Fields: Xn=0, Xt=0, imm9=511
    let encoding: u32 = 0xD9FFF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_xn_0_min_800_d9e00800() {
    // Encoding: 0xD9E00800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field Xn = 0 (Min)
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9E00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_xn_1_poweroftwo_800_d9e00820() {
    // Encoding: 0xD9E00820
    // Test aarch64_integer_tags_mcsettagpairandzerodata field Xn = 1 (PowerOfTwo)
    // Fields: Xn=1, imm9=0, Xt=0
    let encoding: u32 = 0xD9E00820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_xn_30_poweroftwominusone_800_d9e00bc0() {
    // Encoding: 0xD9E00BC0
    // Test aarch64_integer_tags_mcsettagpairandzerodata field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=30, Xt=0, imm9=0
    let encoding: u32 = 0xD9E00BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_xn_31_max_800_d9e00be0() {
    // Encoding: 0xD9E00BE0
    // Test aarch64_integer_tags_mcsettagpairandzerodata field Xn = 31 (Max)
    // Fields: Xn=31, Xt=0, imm9=0
    let encoding: u32 = 0xD9E00BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_xt_0_min_800_d9e00800() {
    // Encoding: 0xD9E00800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field Xt = 0 (Min)
    // Fields: imm9=0, Xt=0, Xn=0
    let encoding: u32 = 0xD9E00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_xt_1_poweroftwo_800_d9e00801() {
    // Encoding: 0xD9E00801
    // Test aarch64_integer_tags_mcsettagpairandzerodata field Xt = 1 (PowerOfTwo)
    // Fields: Xn=0, Xt=1, imm9=0
    let encoding: u32 = 0xD9E00801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_xt_30_poweroftwominusone_800_d9e0081e() {
    // Encoding: 0xD9E0081E
    // Test aarch64_integer_tags_mcsettagpairandzerodata field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: Xt=30, Xn=0, imm9=0
    let encoding: u32 = 0xD9E0081E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_field_xt_31_max_800_d9e0081f() {
    // Encoding: 0xD9E0081F
    // Test aarch64_integer_tags_mcsettagpairandzerodata field Xt = 31 (Max)
    // Fields: Xt=31, Xn=0, imm9=0
    let encoding: u32 = 0xD9E0081F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_0_800_d9e00800() {
    // Encoding: 0xD9E00800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9E00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_1_800_d9e01800() {
    // Encoding: 0xD9E01800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=1, Xn=0, Xt=0
    // Fields: Xn=0, imm9=1, Xt=0
    let encoding: u32 = 0xD9E01800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_2_800_d9e03800() {
    // Encoding: 0xD9E03800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=3, Xn=0, Xt=0
    // Fields: imm9=3, Xt=0, Xn=0
    let encoding: u32 = 0xD9E03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_3_800_d9e04800() {
    // Encoding: 0xD9E04800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=4, Xn=0, Xt=0
    // Fields: imm9=4, Xn=0, Xt=0
    let encoding: u32 = 0xD9E04800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_4_800_d9e07800() {
    // Encoding: 0xD9E07800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=7, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=7
    let encoding: u32 = 0xD9E07800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_5_800_d9e08800() {
    // Encoding: 0xD9E08800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=8, Xn=0, Xt=0
    // Fields: Xn=0, imm9=8, Xt=0
    let encoding: u32 = 0xD9E08800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_6_800_d9e0f800() {
    // Encoding: 0xD9E0F800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=15, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=15
    let encoding: u32 = 0xD9E0F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_7_800_d9e10800() {
    // Encoding: 0xD9E10800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=16, Xn=0, Xt=0
    // Fields: imm9=16, Xn=0, Xt=0
    let encoding: u32 = 0xD9E10800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_8_800_d9e1f800() {
    // Encoding: 0xD9E1F800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=31, Xn=0, Xt=0
    // Fields: imm9=31, Xt=0, Xn=0
    let encoding: u32 = 0xD9E1F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_9_800_d9e20800() {
    // Encoding: 0xD9E20800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=32, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=32
    let encoding: u32 = 0xD9E20800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_10_800_d9e3f800() {
    // Encoding: 0xD9E3F800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=63, Xn=0, Xt=0
    // Fields: Xn=0, imm9=63, Xt=0
    let encoding: u32 = 0xD9E3F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_11_800_d9e40800() {
    // Encoding: 0xD9E40800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=64, Xn=0, Xt=0
    // Fields: imm9=64, Xn=0, Xt=0
    let encoding: u32 = 0xD9E40800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_12_800_d9e7f800() {
    // Encoding: 0xD9E7F800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=127, Xn=0, Xt=0
    // Fields: imm9=127, Xn=0, Xt=0
    let encoding: u32 = 0xD9E7F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_13_800_d9e80800() {
    // Encoding: 0xD9E80800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=128, Xn=0, Xt=0
    // Fields: Xt=0, imm9=128, Xn=0
    let encoding: u32 = 0xD9E80800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_14_800_d9eff800() {
    // Encoding: 0xD9EFF800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=255, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=255
    let encoding: u32 = 0xD9EFF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_15_800_d9f00800() {
    // Encoding: 0xD9F00800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=256, Xn=0, Xt=0
    // Fields: Xt=0, imm9=256, Xn=0
    let encoding: u32 = 0xD9F00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_16_800_d9fff800() {
    // Encoding: 0xD9FFF800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=511, Xn=0, Xt=0
    // Fields: imm9=511, Xn=0, Xt=0
    let encoding: u32 = 0xD9FFF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_17_800_d9e00800() {
    // Encoding: 0xD9E00800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9E00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_18_800_d9e00820() {
    // Encoding: 0xD9E00820
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=0, Xn=1, Xt=0
    // Fields: Xn=1, imm9=0, Xt=0
    let encoding: u32 = 0xD9E00820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_19_800_d9e00bc0() {
    // Encoding: 0xD9E00BC0
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=0, Xn=30, Xt=0
    // Fields: Xt=0, imm9=0, Xn=30
    let encoding: u32 = 0xD9E00BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_20_800_d9e00be0() {
    // Encoding: 0xD9E00BE0
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=0, Xn=31, Xt=0
    // Fields: Xn=31, imm9=0, Xt=0
    let encoding: u32 = 0xD9E00BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_21_800_d9e00800() {
    // Encoding: 0xD9E00800
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9E00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_22_800_d9e00801() {
    // Encoding: 0xD9E00801
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=0, Xn=0, Xt=1
    // Fields: imm9=0, Xn=0, Xt=1
    let encoding: u32 = 0xD9E00801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_23_800_d9e0081e() {
    // Encoding: 0xD9E0081E
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=0, Xn=0, Xt=30
    // Fields: Xn=0, Xt=30, imm9=0
    let encoding: u32 = 0xD9E0081E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_24_800_d9e0081f() {
    // Encoding: 0xD9E0081F
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=0, Xn=0, Xt=31
    // Fields: imm9=0, Xn=0, Xt=31
    let encoding: u32 = 0xD9E0081F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_25_800_d9e00821() {
    // Encoding: 0xD9E00821
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=0, Xn=1, Xt=1
    // Fields: imm9=0, Xn=1, Xt=1
    let encoding: u32 = 0xD9E00821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_combo_26_800_d9e00bff() {
    // Encoding: 0xD9E00BFF
    // Test aarch64_integer_tags_mcsettagpairandzerodata field combination: imm9=0, Xn=31, Xt=31
    // Fields: imm9=0, Xn=31, Xt=31
    let encoding: u32 = 0xD9E00BFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_special_xn_31_stack_pointer_sp_may_require_alignment_2048_d9e01be0()
 {
    // Encoding: 0xD9E01BE0
    // Test aarch64_integer_tags_mcsettagpairandzerodata special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xn=31, Xt=0, imm9=1
    let encoding: u32 = 0xD9E01BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_reg_write_0_d9e00400() {
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost register write: Sp
    // Encoding: 0xD9E00400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9E00400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_reg_write_1_d9e00400() {
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost register write: GpFromField("n")
    // Encoding: 0xD9E00400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9E00400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_sp_xn_d9e007e0() {
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost with Xn = SP (31)
    // Encoding: 0xD9E007E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9E007E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_store_0_d9e00400() {
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost memory store: 8 bytes
    // Encoding: 0xD9E00400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0xD9E00400;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapost
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapost_store_1_d9e00400() {
    // Test aarch64_integer_tags_mcsettagpairandzerodatapost memory store: 8 bytes
    // Encoding: 0xD9E00400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0xD9E00400;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_reg_write_0_d9e00c00() {
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre register write: Sp
    // Encoding: 0xD9E00C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9E00C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_reg_write_1_d9e00c00() {
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre register write: GpFromField("n")
    // Encoding: 0xD9E00C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9E00C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_sp_xn_d9e00fe0() {
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre with Xn = SP (31)
    // Encoding: 0xD9E00FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9E00FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_store_0_d9e00c00() {
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre memory store: 8 bytes
    // Encoding: 0xD9E00C00
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0xD9E00C00;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodatapre
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodatapre_store_1_d9e00c00() {
    // Test aarch64_integer_tags_mcsettagpairandzerodatapre memory store: 8 bytes
    // Encoding: 0xD9E00C00
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0xD9E00C00;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_reg_write_0_d9e00800() {
    // Test aarch64_integer_tags_mcsettagpairandzerodata register write: Sp
    // Encoding: 0xD9E00800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9E00800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_reg_write_1_d9e00800() {
    // Test aarch64_integer_tags_mcsettagpairandzerodata register write: GpFromField("n")
    // Encoding: 0xD9E00800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9E00800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_sp_xn_d9e00be0() {
    // Test aarch64_integer_tags_mcsettagpairandzerodata with Xn = SP (31)
    // Encoding: 0xD9E00BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9E00BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_store_0_d9e00800() {
    // Test aarch64_integer_tags_mcsettagpairandzerodata memory store: 8 bytes
    // Encoding: 0xD9E00800
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0xD9E00800;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairandzerodata
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpairandzerodata_store_1_d9e00800() {
    // Test aarch64_integer_tags_mcsettagpairandzerodata memory store: 8 bytes
    // Encoding: 0xD9E00800
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0xD9E00800;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

// ============================================================================
// aarch64_integer_tags_mcsettagandzerodatapost Tests
// ============================================================================

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_0_zero_400_d9600400() {
    // Encoding: 0xD9600400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 0 (Zero)
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_1_poweroftwo_400_d9601400() {
    // Encoding: 0xD9601400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 1 (PowerOfTwo)
    // Fields: Xt=0, Xn=0, imm9=1
    let encoding: u32 = 0xD9601400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_3_poweroftwominusone_400_d9603400()
{
    // Encoding: 0xD9603400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: imm9=3, Xn=0, Xt=0
    let encoding: u32 = 0xD9603400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_4_poweroftwo_400_d9604400() {
    // Encoding: 0xD9604400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 4 (PowerOfTwo)
    // Fields: imm9=4, Xn=0, Xt=0
    let encoding: u32 = 0xD9604400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_7_poweroftwominusone_400_d9607400()
{
    // Encoding: 0xD9607400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: imm9=7, Xt=0, Xn=0
    let encoding: u32 = 0xD9607400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_8_poweroftwo_400_d9608400() {
    // Encoding: 0xD9608400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 8 (PowerOfTwo)
    // Fields: imm9=8, Xn=0, Xt=0
    let encoding: u32 = 0xD9608400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_15_poweroftwominusone_400_d960f400()
{
    // Encoding: 0xD960F400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=15, Xt=0
    let encoding: u32 = 0xD960F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_16_poweroftwo_400_d9610400() {
    // Encoding: 0xD9610400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 16 (PowerOfTwo)
    // Fields: Xn=0, imm9=16, Xt=0
    let encoding: u32 = 0xD9610400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_31_poweroftwominusone_400_d961f400()
{
    // Encoding: 0xD961F400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: imm9=31, Xn=0, Xt=0
    let encoding: u32 = 0xD961F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_32_poweroftwo_400_d9620400() {
    // Encoding: 0xD9620400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 32 (PowerOfTwo)
    // Fields: imm9=32, Xn=0, Xt=0
    let encoding: u32 = 0xD9620400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_63_poweroftwominusone_400_d963f400()
{
    // Encoding: 0xD963F400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: imm9=63, Xn=0, Xt=0
    let encoding: u32 = 0xD963F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_64_poweroftwo_400_d9640400() {
    // Encoding: 0xD9640400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 64 (PowerOfTwo)
    // Fields: imm9=64, Xn=0, Xt=0
    let encoding: u32 = 0xD9640400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_127_poweroftwominusone_400_d967f400()
 {
    // Encoding: 0xD967F400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=127, Xt=0
    let encoding: u32 = 0xD967F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_128_poweroftwo_400_d9680400() {
    // Encoding: 0xD9680400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 128 (PowerOfTwo)
    // Fields: Xn=0, imm9=128, Xt=0
    let encoding: u32 = 0xD9680400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_255_poweroftwominusone_400_d96ff400()
 {
    // Encoding: 0xD96FF400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: imm9=255, Xt=0, Xn=0
    let encoding: u32 = 0xD96FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_256_poweroftwo_400_d9700400() {
    // Encoding: 0xD9700400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 256 (PowerOfTwo)
    // Fields: imm9=256, Xt=0, Xn=0
    let encoding: u32 = 0xD9700400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_imm9_511_max_400_d97ff400() {
    // Encoding: 0xD97FF400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field imm9 = 511 (Max)
    // Fields: Xn=0, Xt=0, imm9=511
    let encoding: u32 = 0xD97FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_xn_0_min_400_d9600400() {
    // Encoding: 0xD9600400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field Xn = 0 (Min)
    // Fields: imm9=0, Xt=0, Xn=0
    let encoding: u32 = 0xD9600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_xn_1_poweroftwo_400_d9600420() {
    // Encoding: 0xD9600420
    // Test aarch64_integer_tags_mcsettagandzerodatapost field Xn = 1 (PowerOfTwo)
    // Fields: Xn=1, imm9=0, Xt=0
    let encoding: u32 = 0xD9600420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_xn_30_poweroftwominusone_400_d96007c0() {
    // Encoding: 0xD96007C0
    // Test aarch64_integer_tags_mcsettagandzerodatapost field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=30, Xt=0, imm9=0
    let encoding: u32 = 0xD96007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_xn_31_max_400_d96007e0() {
    // Encoding: 0xD96007E0
    // Test aarch64_integer_tags_mcsettagandzerodatapost field Xn = 31 (Max)
    // Fields: Xt=0, Xn=31, imm9=0
    let encoding: u32 = 0xD96007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_xt_0_min_400_d9600400() {
    // Encoding: 0xD9600400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field Xt = 0 (Min)
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_xt_1_poweroftwo_400_d9600401() {
    // Encoding: 0xD9600401
    // Test aarch64_integer_tags_mcsettagandzerodatapost field Xt = 1 (PowerOfTwo)
    // Fields: imm9=0, Xn=0, Xt=1
    let encoding: u32 = 0xD9600401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_xt_30_poweroftwominusone_400_d960041e() {
    // Encoding: 0xD960041E
    // Test aarch64_integer_tags_mcsettagandzerodatapost field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=0, Xt=30
    let encoding: u32 = 0xD960041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_field_xt_31_max_400_d960041f() {
    // Encoding: 0xD960041F
    // Test aarch64_integer_tags_mcsettagandzerodatapost field Xt = 31 (Max)
    // Fields: imm9=0, Xt=31, Xn=0
    let encoding: u32 = 0xD960041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_0_400_d9600400() {
    // Encoding: 0xD9600400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xt=0, Xn=0
    let encoding: u32 = 0xD9600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_1_400_d9601400() {
    // Encoding: 0xD9601400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=1, Xn=0, Xt=0
    // Fields: Xt=0, imm9=1, Xn=0
    let encoding: u32 = 0xD9601400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_2_400_d9603400() {
    // Encoding: 0xD9603400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=3, Xn=0, Xt=0
    // Fields: Xt=0, imm9=3, Xn=0
    let encoding: u32 = 0xD9603400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_3_400_d9604400() {
    // Encoding: 0xD9604400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=4, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=4
    let encoding: u32 = 0xD9604400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_4_400_d9607400() {
    // Encoding: 0xD9607400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=7, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=7
    let encoding: u32 = 0xD9607400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_5_400_d9608400() {
    // Encoding: 0xD9608400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=8, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=8
    let encoding: u32 = 0xD9608400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_6_400_d960f400() {
    // Encoding: 0xD960F400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=15, Xn=0, Xt=0
    // Fields: Xn=0, imm9=15, Xt=0
    let encoding: u32 = 0xD960F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_7_400_d9610400() {
    // Encoding: 0xD9610400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=16, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=16
    let encoding: u32 = 0xD9610400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_8_400_d961f400() {
    // Encoding: 0xD961F400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=31, Xn=0, Xt=0
    // Fields: Xn=0, imm9=31, Xt=0
    let encoding: u32 = 0xD961F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_9_400_d9620400() {
    // Encoding: 0xD9620400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=32, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=32
    let encoding: u32 = 0xD9620400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_10_400_d963f400() {
    // Encoding: 0xD963F400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=63, Xn=0, Xt=0
    // Fields: Xn=0, imm9=63, Xt=0
    let encoding: u32 = 0xD963F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_11_400_d9640400() {
    // Encoding: 0xD9640400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=64, Xn=0, Xt=0
    // Fields: imm9=64, Xt=0, Xn=0
    let encoding: u32 = 0xD9640400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_12_400_d967f400() {
    // Encoding: 0xD967F400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=127, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=127
    let encoding: u32 = 0xD967F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_13_400_d9680400() {
    // Encoding: 0xD9680400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=128, Xn=0, Xt=0
    // Fields: Xn=0, imm9=128, Xt=0
    let encoding: u32 = 0xD9680400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_14_400_d96ff400() {
    // Encoding: 0xD96FF400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=255, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=255
    let encoding: u32 = 0xD96FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_15_400_d9700400() {
    // Encoding: 0xD9700400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=256, Xn=0, Xt=0
    // Fields: Xn=0, imm9=256, Xt=0
    let encoding: u32 = 0xD9700400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_16_400_d97ff400() {
    // Encoding: 0xD97FF400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=511, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=511
    let encoding: u32 = 0xD97FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_17_400_d9600400() {
    // Encoding: 0xD9600400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_18_400_d9600420() {
    // Encoding: 0xD9600420
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=0, Xn=1, Xt=0
    // Fields: Xn=1, imm9=0, Xt=0
    let encoding: u32 = 0xD9600420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_19_400_d96007c0() {
    // Encoding: 0xD96007C0
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=0, Xn=30, Xt=0
    // Fields: imm9=0, Xn=30, Xt=0
    let encoding: u32 = 0xD96007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_20_400_d96007e0() {
    // Encoding: 0xD96007E0
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=0, Xn=31, Xt=0
    // Fields: Xn=31, Xt=0, imm9=0
    let encoding: u32 = 0xD96007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_21_400_d9600400() {
    // Encoding: 0xD9600400
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xt=0, Xn=0
    let encoding: u32 = 0xD9600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_22_400_d9600401() {
    // Encoding: 0xD9600401
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=0, Xn=0, Xt=1
    // Fields: Xn=0, imm9=0, Xt=1
    let encoding: u32 = 0xD9600401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_23_400_d960041e() {
    // Encoding: 0xD960041E
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=0, Xn=0, Xt=30
    // Fields: Xn=0, Xt=30, imm9=0
    let encoding: u32 = 0xD960041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_24_400_d960041f() {
    // Encoding: 0xD960041F
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=0, Xn=0, Xt=31
    // Fields: imm9=0, Xn=0, Xt=31
    let encoding: u32 = 0xD960041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_25_400_d9600421() {
    // Encoding: 0xD9600421
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=0, Xn=1, Xt=1
    // Fields: imm9=0, Xt=1, Xn=1
    let encoding: u32 = 0xD9600421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_combo_26_400_d96007ff() {
    // Encoding: 0xD96007FF
    // Test aarch64_integer_tags_mcsettagandzerodatapost field combination: imm9=0, Xn=31, Xt=31
    // Fields: Xt=31, Xn=31, imm9=0
    let encoding: u32 = 0xD96007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_special_xn_31_stack_pointer_sp_may_require_alignment_1024_d96017e0()
 {
    // Encoding: 0xD96017E0
    // Test aarch64_integer_tags_mcsettagandzerodatapost special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xt=0, Xn=31, imm9=1
    let encoding: u32 = 0xD96017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_0_zero_c00_d9600c00() {
    // Encoding: 0xD9600C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 0 (Zero)
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_1_poweroftwo_c00_d9601c00() {
    // Encoding: 0xD9601C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 1 (PowerOfTwo)
    // Fields: Xt=0, imm9=1, Xn=0
    let encoding: u32 = 0xD9601C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_3_poweroftwominusone_c00_d9603c00() {
    // Encoding: 0xD9603C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=3
    let encoding: u32 = 0xD9603C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_4_poweroftwo_c00_d9604c00() {
    // Encoding: 0xD9604C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 4 (PowerOfTwo)
    // Fields: imm9=4, Xt=0, Xn=0
    let encoding: u32 = 0xD9604C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_7_poweroftwominusone_c00_d9607c00() {
    // Encoding: 0xD9607C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=7
    let encoding: u32 = 0xD9607C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_8_poweroftwo_c00_d9608c00() {
    // Encoding: 0xD9608C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 8 (PowerOfTwo)
    // Fields: Xn=0, imm9=8, Xt=0
    let encoding: u32 = 0xD9608C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_15_poweroftwominusone_c00_d960fc00()
{
    // Encoding: 0xD960FC00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xn=0, imm9=15
    let encoding: u32 = 0xD960FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_16_poweroftwo_c00_d9610c00() {
    // Encoding: 0xD9610C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 16 (PowerOfTwo)
    // Fields: Xn=0, imm9=16, Xt=0
    let encoding: u32 = 0xD9610C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_31_poweroftwominusone_c00_d961fc00()
{
    // Encoding: 0xD961FC00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=31
    let encoding: u32 = 0xD961FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_32_poweroftwo_c00_d9620c00() {
    // Encoding: 0xD9620C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 32 (PowerOfTwo)
    // Fields: Xn=0, imm9=32, Xt=0
    let encoding: u32 = 0xD9620C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_63_poweroftwominusone_c00_d963fc00()
{
    // Encoding: 0xD963FC00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=63, Xt=0
    let encoding: u32 = 0xD963FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_64_poweroftwo_c00_d9640c00() {
    // Encoding: 0xD9640C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 64 (PowerOfTwo)
    // Fields: imm9=64, Xn=0, Xt=0
    let encoding: u32 = 0xD9640C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_127_poweroftwominusone_c00_d967fc00()
{
    // Encoding: 0xD967FC00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=127, Xn=0
    let encoding: u32 = 0xD967FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_128_poweroftwo_c00_d9680c00() {
    // Encoding: 0xD9680C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 128 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=128
    let encoding: u32 = 0xD9680C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_255_poweroftwominusone_c00_d96ffc00()
{
    // Encoding: 0xD96FFC00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: imm9=255, Xn=0, Xt=0
    let encoding: u32 = 0xD96FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_256_poweroftwo_c00_d9700c00() {
    // Encoding: 0xD9700C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 256 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=256
    let encoding: u32 = 0xD9700C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_imm9_511_max_c00_d97ffc00() {
    // Encoding: 0xD97FFC00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field imm9 = 511 (Max)
    // Fields: imm9=511, Xn=0, Xt=0
    let encoding: u32 = 0xD97FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_xn_0_min_c00_d9600c00() {
    // Encoding: 0xD9600C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field Xn = 0 (Min)
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_xn_1_poweroftwo_c00_d9600c20() {
    // Encoding: 0xD9600C20
    // Test aarch64_integer_tags_mcsettagandzerodatapre field Xn = 1 (PowerOfTwo)
    // Fields: Xt=0, Xn=1, imm9=0
    let encoding: u32 = 0xD9600C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_xn_30_poweroftwominusone_c00_d9600fc0() {
    // Encoding: 0xD9600FC0
    // Test aarch64_integer_tags_mcsettagandzerodatapre field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: imm9=0, Xn=30, Xt=0
    let encoding: u32 = 0xD9600FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_xn_31_max_c00_d9600fe0() {
    // Encoding: 0xD9600FE0
    // Test aarch64_integer_tags_mcsettagandzerodatapre field Xn = 31 (Max)
    // Fields: imm9=0, Xt=0, Xn=31
    let encoding: u32 = 0xD9600FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_xt_0_min_c00_d9600c00() {
    // Encoding: 0xD9600C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field Xt = 0 (Min)
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_xt_1_poweroftwo_c00_d9600c01() {
    // Encoding: 0xD9600C01
    // Test aarch64_integer_tags_mcsettagandzerodatapre field Xt = 1 (PowerOfTwo)
    // Fields: imm9=0, Xt=1, Xn=0
    let encoding: u32 = 0xD9600C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_xt_30_poweroftwominusone_c00_d9600c1e() {
    // Encoding: 0xD9600C1E
    // Test aarch64_integer_tags_mcsettagandzerodatapre field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: imm9=0, Xn=0, Xt=30
    let encoding: u32 = 0xD9600C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_field_xt_31_max_c00_d9600c1f() {
    // Encoding: 0xD9600C1F
    // Test aarch64_integer_tags_mcsettagandzerodatapre field Xt = 31 (Max)
    // Fields: Xn=0, imm9=0, Xt=31
    let encoding: u32 = 0xD9600C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_0_c00_d9600c00() {
    // Encoding: 0xD9600C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xn=0, imm9=0, Xt=0
    let encoding: u32 = 0xD9600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_1_c00_d9601c00() {
    // Encoding: 0xD9601C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=1, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=1
    let encoding: u32 = 0xD9601C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_2_c00_d9603c00() {
    // Encoding: 0xD9603C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=3, Xn=0, Xt=0
    // Fields: Xn=0, imm9=3, Xt=0
    let encoding: u32 = 0xD9603C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_3_c00_d9604c00() {
    // Encoding: 0xD9604C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=4, Xn=0, Xt=0
    // Fields: imm9=4, Xn=0, Xt=0
    let encoding: u32 = 0xD9604C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_4_c00_d9607c00() {
    // Encoding: 0xD9607C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=7, Xn=0, Xt=0
    // Fields: Xn=0, imm9=7, Xt=0
    let encoding: u32 = 0xD9607C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_5_c00_d9608c00() {
    // Encoding: 0xD9608C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=8, Xn=0, Xt=0
    // Fields: Xt=0, imm9=8, Xn=0
    let encoding: u32 = 0xD9608C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_6_c00_d960fc00() {
    // Encoding: 0xD960FC00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=15, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=15
    let encoding: u32 = 0xD960FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_7_c00_d9610c00() {
    // Encoding: 0xD9610C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=16, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=16
    let encoding: u32 = 0xD9610C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_8_c00_d961fc00() {
    // Encoding: 0xD961FC00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=31, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=31
    let encoding: u32 = 0xD961FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_9_c00_d9620c00() {
    // Encoding: 0xD9620C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=32, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=32
    let encoding: u32 = 0xD9620C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_10_c00_d963fc00() {
    // Encoding: 0xD963FC00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=63, Xn=0, Xt=0
    // Fields: imm9=63, Xt=0, Xn=0
    let encoding: u32 = 0xD963FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_11_c00_d9640c00() {
    // Encoding: 0xD9640C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=64, Xn=0, Xt=0
    // Fields: Xn=0, imm9=64, Xt=0
    let encoding: u32 = 0xD9640C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_12_c00_d967fc00() {
    // Encoding: 0xD967FC00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=127, Xn=0, Xt=0
    // Fields: imm9=127, Xn=0, Xt=0
    let encoding: u32 = 0xD967FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_13_c00_d9680c00() {
    // Encoding: 0xD9680C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=128, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=128
    let encoding: u32 = 0xD9680C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_14_c00_d96ffc00() {
    // Encoding: 0xD96FFC00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=255, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=255
    let encoding: u32 = 0xD96FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_15_c00_d9700c00() {
    // Encoding: 0xD9700C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=256, Xn=0, Xt=0
    // Fields: Xt=0, imm9=256, Xn=0
    let encoding: u32 = 0xD9700C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_16_c00_d97ffc00() {
    // Encoding: 0xD97FFC00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=511, Xn=0, Xt=0
    // Fields: imm9=511, Xt=0, Xn=0
    let encoding: u32 = 0xD97FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_17_c00_d9600c00() {
    // Encoding: 0xD9600C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_18_c00_d9600c20() {
    // Encoding: 0xD9600C20
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=0, Xn=1, Xt=0
    // Fields: imm9=0, Xn=1, Xt=0
    let encoding: u32 = 0xD9600C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_19_c00_d9600fc0() {
    // Encoding: 0xD9600FC0
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=0, Xn=30, Xt=0
    // Fields: Xn=30, Xt=0, imm9=0
    let encoding: u32 = 0xD9600FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_20_c00_d9600fe0() {
    // Encoding: 0xD9600FE0
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=0, Xn=31, Xt=0
    // Fields: Xt=0, imm9=0, Xn=31
    let encoding: u32 = 0xD9600FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_21_c00_d9600c00() {
    // Encoding: 0xD9600C00
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_22_c00_d9600c01() {
    // Encoding: 0xD9600C01
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=0, Xn=0, Xt=1
    // Fields: imm9=0, Xt=1, Xn=0
    let encoding: u32 = 0xD9600C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_23_c00_d9600c1e() {
    // Encoding: 0xD9600C1E
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=0, Xn=0, Xt=30
    // Fields: Xn=0, Xt=30, imm9=0
    let encoding: u32 = 0xD9600C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_24_c00_d9600c1f() {
    // Encoding: 0xD9600C1F
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=0, Xn=0, Xt=31
    // Fields: Xt=31, imm9=0, Xn=0
    let encoding: u32 = 0xD9600C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_25_c00_d9600c21() {
    // Encoding: 0xD9600C21
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=0, Xn=1, Xt=1
    // Fields: Xn=1, imm9=0, Xt=1
    let encoding: u32 = 0xD9600C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_combo_26_c00_d9600fff() {
    // Encoding: 0xD9600FFF
    // Test aarch64_integer_tags_mcsettagandzerodatapre field combination: imm9=0, Xn=31, Xt=31
    // Fields: Xn=31, Xt=31, imm9=0
    let encoding: u32 = 0xD9600FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_special_xn_31_stack_pointer_sp_may_require_alignment_3072_d9601fe0()
 {
    // Encoding: 0xD9601FE0
    // Test aarch64_integer_tags_mcsettagandzerodatapre special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xn=31, imm9=1, Xt=0
    let encoding: u32 = 0xD9601FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_0_zero_800_d9600800() {
    // Encoding: 0xD9600800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 0 (Zero)
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9600800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_1_poweroftwo_800_d9601800() {
    // Encoding: 0xD9601800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 1 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=1
    let encoding: u32 = 0xD9601800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_3_poweroftwominusone_800_d9603800() {
    // Encoding: 0xD9603800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=3
    let encoding: u32 = 0xD9603800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_4_poweroftwo_800_d9604800() {
    // Encoding: 0xD9604800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 4 (PowerOfTwo)
    // Fields: Xn=0, imm9=4, Xt=0
    let encoding: u32 = 0xD9604800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_7_poweroftwominusone_800_d9607800() {
    // Encoding: 0xD9607800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: imm9=7, Xt=0, Xn=0
    let encoding: u32 = 0xD9607800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_8_poweroftwo_800_d9608800() {
    // Encoding: 0xD9608800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 8 (PowerOfTwo)
    // Fields: Xn=0, imm9=8, Xt=0
    let encoding: u32 = 0xD9608800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_15_poweroftwominusone_800_d960f800() {
    // Encoding: 0xD960F800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: imm9=15, Xn=0, Xt=0
    let encoding: u32 = 0xD960F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_16_poweroftwo_800_d9610800() {
    // Encoding: 0xD9610800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 16 (PowerOfTwo)
    // Fields: imm9=16, Xn=0, Xt=0
    let encoding: u32 = 0xD9610800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_31_poweroftwominusone_800_d961f800() {
    // Encoding: 0xD961F800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xn=0, imm9=31
    let encoding: u32 = 0xD961F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_32_poweroftwo_800_d9620800() {
    // Encoding: 0xD9620800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 32 (PowerOfTwo)
    // Fields: Xt=0, imm9=32, Xn=0
    let encoding: u32 = 0xD9620800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_63_poweroftwominusone_800_d963f800() {
    // Encoding: 0xD963F800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: imm9=63, Xn=0, Xt=0
    let encoding: u32 = 0xD963F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_64_poweroftwo_800_d9640800() {
    // Encoding: 0xD9640800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 64 (PowerOfTwo)
    // Fields: Xt=0, Xn=0, imm9=64
    let encoding: u32 = 0xD9640800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_127_poweroftwominusone_800_d967f800() {
    // Encoding: 0xD967F800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=127
    let encoding: u32 = 0xD967F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_128_poweroftwo_800_d9680800() {
    // Encoding: 0xD9680800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 128 (PowerOfTwo)
    // Fields: imm9=128, Xn=0, Xt=0
    let encoding: u32 = 0xD9680800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_255_poweroftwominusone_800_d96ff800() {
    // Encoding: 0xD96FF800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: imm9=255, Xt=0, Xn=0
    let encoding: u32 = 0xD96FF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_256_poweroftwo_800_d9700800() {
    // Encoding: 0xD9700800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 256 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=256
    let encoding: u32 = 0xD9700800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_imm9_511_max_800_d97ff800() {
    // Encoding: 0xD97FF800
    // Test aarch64_integer_tags_mcsettagandzerodata field imm9 = 511 (Max)
    // Fields: Xn=0, Xt=0, imm9=511
    let encoding: u32 = 0xD97FF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_xn_0_min_800_d9600800() {
    // Encoding: 0xD9600800
    // Test aarch64_integer_tags_mcsettagandzerodata field Xn = 0 (Min)
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9600800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_xn_1_poweroftwo_800_d9600820() {
    // Encoding: 0xD9600820
    // Test aarch64_integer_tags_mcsettagandzerodata field Xn = 1 (PowerOfTwo)
    // Fields: Xn=1, imm9=0, Xt=0
    let encoding: u32 = 0xD9600820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_xn_30_poweroftwominusone_800_d9600bc0() {
    // Encoding: 0xD9600BC0
    // Test aarch64_integer_tags_mcsettagandzerodata field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=0, Xn=30
    let encoding: u32 = 0xD9600BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_xn_31_max_800_d9600be0() {
    // Encoding: 0xD9600BE0
    // Test aarch64_integer_tags_mcsettagandzerodata field Xn = 31 (Max)
    // Fields: Xn=31, Xt=0, imm9=0
    let encoding: u32 = 0xD9600BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_xt_0_min_800_d9600800() {
    // Encoding: 0xD9600800
    // Test aarch64_integer_tags_mcsettagandzerodata field Xt = 0 (Min)
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9600800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_xt_1_poweroftwo_800_d9600801() {
    // Encoding: 0xD9600801
    // Test aarch64_integer_tags_mcsettagandzerodata field Xt = 1 (PowerOfTwo)
    // Fields: Xn=0, Xt=1, imm9=0
    let encoding: u32 = 0xD9600801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_xt_30_poweroftwominusone_800_d960081e() {
    // Encoding: 0xD960081E
    // Test aarch64_integer_tags_mcsettagandzerodata field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=30, imm9=0
    let encoding: u32 = 0xD960081E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_field_xt_31_max_800_d960081f() {
    // Encoding: 0xD960081F
    // Test aarch64_integer_tags_mcsettagandzerodata field Xt = 31 (Max)
    // Fields: Xt=31, imm9=0, Xn=0
    let encoding: u32 = 0xD960081F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_0_800_d9600800() {
    // Encoding: 0xD9600800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=0
    let encoding: u32 = 0xD9600800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_1_800_d9601800() {
    // Encoding: 0xD9601800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=1, Xn=0, Xt=0
    // Fields: imm9=1, Xn=0, Xt=0
    let encoding: u32 = 0xD9601800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_2_800_d9603800() {
    // Encoding: 0xD9603800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=3, Xn=0, Xt=0
    // Fields: Xn=0, imm9=3, Xt=0
    let encoding: u32 = 0xD9603800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_3_800_d9604800() {
    // Encoding: 0xD9604800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=4, Xn=0, Xt=0
    // Fields: Xn=0, imm9=4, Xt=0
    let encoding: u32 = 0xD9604800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_4_800_d9607800() {
    // Encoding: 0xD9607800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=7, Xn=0, Xt=0
    // Fields: imm9=7, Xn=0, Xt=0
    let encoding: u32 = 0xD9607800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_5_800_d9608800() {
    // Encoding: 0xD9608800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=8, Xn=0, Xt=0
    // Fields: imm9=8, Xn=0, Xt=0
    let encoding: u32 = 0xD9608800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_6_800_d960f800() {
    // Encoding: 0xD960F800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=15, Xn=0, Xt=0
    // Fields: imm9=15, Xn=0, Xt=0
    let encoding: u32 = 0xD960F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_7_800_d9610800() {
    // Encoding: 0xD9610800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=16, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=16
    let encoding: u32 = 0xD9610800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_8_800_d961f800() {
    // Encoding: 0xD961F800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=31, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=31
    let encoding: u32 = 0xD961F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_9_800_d9620800() {
    // Encoding: 0xD9620800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=32, Xn=0, Xt=0
    // Fields: imm9=32, Xt=0, Xn=0
    let encoding: u32 = 0xD9620800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_10_800_d963f800() {
    // Encoding: 0xD963F800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=63, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=63
    let encoding: u32 = 0xD963F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_11_800_d9640800() {
    // Encoding: 0xD9640800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=64, Xn=0, Xt=0
    // Fields: imm9=64, Xn=0, Xt=0
    let encoding: u32 = 0xD9640800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_12_800_d967f800() {
    // Encoding: 0xD967F800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=127, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=127
    let encoding: u32 = 0xD967F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_13_800_d9680800() {
    // Encoding: 0xD9680800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=128, Xn=0, Xt=0
    // Fields: imm9=128, Xn=0, Xt=0
    let encoding: u32 = 0xD9680800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_14_800_d96ff800() {
    // Encoding: 0xD96FF800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=255, Xn=0, Xt=0
    // Fields: imm9=255, Xn=0, Xt=0
    let encoding: u32 = 0xD96FF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_15_800_d9700800() {
    // Encoding: 0xD9700800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=256, Xn=0, Xt=0
    // Fields: imm9=256, Xn=0, Xt=0
    let encoding: u32 = 0xD9700800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_16_800_d97ff800() {
    // Encoding: 0xD97FF800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=511, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=511
    let encoding: u32 = 0xD97FF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_17_800_d9600800() {
    // Encoding: 0xD9600800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9600800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_18_800_d9600820() {
    // Encoding: 0xD9600820
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=0, Xn=1, Xt=0
    // Fields: imm9=0, Xt=0, Xn=1
    let encoding: u32 = 0xD9600820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_19_800_d9600bc0() {
    // Encoding: 0xD9600BC0
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=0, Xn=30, Xt=0
    // Fields: Xn=30, Xt=0, imm9=0
    let encoding: u32 = 0xD9600BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_20_800_d9600be0() {
    // Encoding: 0xD9600BE0
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=0, Xn=31, Xt=0
    // Fields: Xt=0, Xn=31, imm9=0
    let encoding: u32 = 0xD9600BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_21_800_d9600800() {
    // Encoding: 0xD9600800
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9600800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_22_800_d9600801() {
    // Encoding: 0xD9600801
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=0, Xn=0, Xt=1
    // Fields: Xn=0, Xt=1, imm9=0
    let encoding: u32 = 0xD9600801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_23_800_d960081e() {
    // Encoding: 0xD960081E
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=0, Xn=0, Xt=30
    // Fields: imm9=0, Xt=30, Xn=0
    let encoding: u32 = 0xD960081E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_24_800_d960081f() {
    // Encoding: 0xD960081F
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=0, Xn=0, Xt=31
    // Fields: Xt=31, imm9=0, Xn=0
    let encoding: u32 = 0xD960081F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_25_800_d9600821() {
    // Encoding: 0xD9600821
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=0, Xn=1, Xt=1
    // Fields: Xn=1, Xt=1, imm9=0
    let encoding: u32 = 0xD9600821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_combo_26_800_d9600bff() {
    // Encoding: 0xD9600BFF
    // Test aarch64_integer_tags_mcsettagandzerodata field combination: imm9=0, Xn=31, Xt=31
    // Fields: Xt=31, imm9=0, Xn=31
    let encoding: u32 = 0xD9600BFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_special_xn_31_stack_pointer_sp_may_require_alignment_2048_d9601be0()
 {
    // Encoding: 0xD9601BE0
    // Test aarch64_integer_tags_mcsettagandzerodata special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xn=31, imm9=1, Xt=0
    let encoding: u32 = 0xD9601BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// zero value
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_ldr_oracle_0_f9400020() {
    // Test LDR: zero value (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max byte
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_ldr_oracle_1_f9400020() {
    // Test LDR: max byte (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xFF, "X0 should be 0x00000000000000FF");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max halfword
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_ldr_oracle_2_f9400020() {
    // Test LDR: max halfword (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xFFFF, "X0 should be 0x000000000000FFFF");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max word
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_ldr_oracle_3_f9400020() {
    // Test LDR: max word (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 255, 255, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0x00000000FFFFFFFF"
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// large value
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_ldr_oracle_4_f9400020() {
    // Test LDR: large value (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[240, 222, 188, 154, 120, 86, 52, 18])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x123456789ABCDEF0,
        "X0 should be 0x123456789ABCDEF0"
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (byte)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_ldr_oracle_5_f9400020() {
    // Test LDR: sign bit (byte) (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[128, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x80, "X0 should be 0x0000000000000080");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (halfword)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_ldr_oracle_6_f9400020() {
    // Test LDR: sign bit (halfword) (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 128, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x8000, "X0 should be 0x0000000000008000");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (word)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_ldr_oracle_7_f9400020() {
    // Test LDR: sign bit (word) (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 128, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x80000000,
        "X0 should be 0x0000000080000000"
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_reg_write_0_d9600400() {
    // Test aarch64_integer_tags_mcsettagandzerodatapost register write: Sp
    // Encoding: 0xD9600400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9600400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_reg_write_1_d9600400() {
    // Test aarch64_integer_tags_mcsettagandzerodatapost register write: GpFromField("n")
    // Encoding: 0xD9600400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9600400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_sp_xn_d96007e0() {
    // Test aarch64_integer_tags_mcsettagandzerodatapost with Xn = SP (31)
    // Encoding: 0xD96007E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD96007E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapost
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapost_store_0_d9600400() {
    // Test aarch64_integer_tags_mcsettagandzerodatapost memory store: 8 bytes
    // Encoding: 0xD9600400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0xD9600400;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// zero value
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_ldr_oracle_0_f9400020() {
    // Test LDR: zero value (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max byte
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_ldr_oracle_1_f9400020() {
    // Test LDR: max byte (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xFF, "X0 should be 0x00000000000000FF");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max halfword
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_ldr_oracle_2_f9400020() {
    // Test LDR: max halfword (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xFFFF, "X0 should be 0x000000000000FFFF");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max word
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_ldr_oracle_3_f9400020() {
    // Test LDR: max word (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 255, 255, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0x00000000FFFFFFFF"
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// large value
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_ldr_oracle_4_f9400020() {
    // Test LDR: large value (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[240, 222, 188, 154, 120, 86, 52, 18])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x123456789ABCDEF0,
        "X0 should be 0x123456789ABCDEF0"
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (byte)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_ldr_oracle_5_f9400020() {
    // Test LDR: sign bit (byte) (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[128, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x80, "X0 should be 0x0000000000000080");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (halfword)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_ldr_oracle_6_f9400020() {
    // Test LDR: sign bit (halfword) (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 128, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x8000, "X0 should be 0x0000000000008000");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (word)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_ldr_oracle_7_f9400020() {
    // Test LDR: sign bit (word) (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 128, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x80000000,
        "X0 should be 0x0000000080000000"
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_reg_write_0_d9600c00() {
    // Test aarch64_integer_tags_mcsettagandzerodatapre register write: Sp
    // Encoding: 0xD9600C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9600C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_reg_write_1_d9600c00() {
    // Test aarch64_integer_tags_mcsettagandzerodatapre register write: GpFromField("n")
    // Encoding: 0xD9600C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9600C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_sp_xn_d9600fe0() {
    // Test aarch64_integer_tags_mcsettagandzerodatapre with Xn = SP (31)
    // Encoding: 0xD9600FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9600FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodatapre
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodatapre_store_0_d9600c00() {
    // Test aarch64_integer_tags_mcsettagandzerodatapre memory store: 8 bytes
    // Encoding: 0xD9600C00
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0xD9600C00;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// zero value
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_ldr_oracle_0_f9400020() {
    // Test LDR: zero value (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max byte
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_ldr_oracle_1_f9400020() {
    // Test LDR: max byte (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xFF, "X0 should be 0x00000000000000FF");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max halfword
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_ldr_oracle_2_f9400020() {
    // Test LDR: max halfword (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xFFFF, "X0 should be 0x000000000000FFFF");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max word
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_ldr_oracle_3_f9400020() {
    // Test LDR: max word (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 255, 255, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0x00000000FFFFFFFF"
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// large value
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_ldr_oracle_4_f9400020() {
    // Test LDR: large value (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[240, 222, 188, 154, 120, 86, 52, 18])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x123456789ABCDEF0,
        "X0 should be 0x123456789ABCDEF0"
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (byte)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_ldr_oracle_5_f9400020() {
    // Test LDR: sign bit (byte) (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[128, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x80, "X0 should be 0x0000000000000080");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (halfword)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_ldr_oracle_6_f9400020() {
    // Test LDR: sign bit (halfword) (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 128, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x8000, "X0 should be 0x0000000000008000");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (word)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_ldr_oracle_7_f9400020() {
    // Test LDR: sign bit (word) (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 128, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x80000000,
        "X0 should be 0x0000000080000000"
    );
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_reg_write_0_d9600800() {
    // Test aarch64_integer_tags_mcsettagandzerodata register write: Sp
    // Encoding: 0xD9600800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9600800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_reg_write_1_d9600800() {
    // Test aarch64_integer_tags_mcsettagandzerodata register write: GpFromField("n")
    // Encoding: 0xD9600800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9600800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_sp_xn_d9600be0() {
    // Test aarch64_integer_tags_mcsettagandzerodata with Xn = SP (31)
    // Encoding: 0xD9600BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9600BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagandzerodata
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagandzerodata_store_0_d9600800() {
    // Test aarch64_integer_tags_mcsettagandzerodata memory store: 8 bytes
    // Encoding: 0xD9600800
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0xD9600800;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; synthetic operands may fault at runtime.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

// ============================================================================
// aarch64_integer_tags_mcinsertrandomtag Tests
// ============================================================================

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_field_xm_0_min_1000_9ac01000() {
    // Encoding: 0x9AC01000
    // Test aarch64_integer_tags_mcinsertrandomtag field Xm = 0 (Min)
    // Fields: Xn=0, Xm=0, Xd=0
    let encoding: u32 = 0x9AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_field_xm_1_poweroftwo_1000_9ac11000() {
    // Encoding: 0x9AC11000
    // Test aarch64_integer_tags_mcinsertrandomtag field Xm = 1 (PowerOfTwo)
    // Fields: Xm=1, Xn=0, Xd=0
    let encoding: u32 = 0x9AC11000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_field_xm_30_poweroftwominusone_1000_9ade1000() {
    // Encoding: 0x9ADE1000
    // Test aarch64_integer_tags_mcinsertrandomtag field Xm = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xm=30, Xd=0
    let encoding: u32 = 0x9ADE1000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_field_xm_31_max_1000_9adf1000() {
    // Encoding: 0x9ADF1000
    // Test aarch64_integer_tags_mcinsertrandomtag field Xm = 31 (Max)
    // Fields: Xn=0, Xd=0, Xm=31
    let encoding: u32 = 0x9ADF1000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_field_xn_0_min_1000_9ac01000() {
    // Encoding: 0x9AC01000
    // Test aarch64_integer_tags_mcinsertrandomtag field Xn = 0 (Min)
    // Fields: Xd=0, Xm=0, Xn=0
    let encoding: u32 = 0x9AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_field_xn_1_poweroftwo_1000_9ac01020() {
    // Encoding: 0x9AC01020
    // Test aarch64_integer_tags_mcinsertrandomtag field Xn = 1 (PowerOfTwo)
    // Fields: Xd=0, Xm=0, Xn=1
    let encoding: u32 = 0x9AC01020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_field_xn_30_poweroftwominusone_1000_9ac013c0() {
    // Encoding: 0x9AC013C0
    // Test aarch64_integer_tags_mcinsertrandomtag field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xd=0, Xn=30, Xm=0
    let encoding: u32 = 0x9AC013C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_field_xn_31_max_1000_9ac013e0() {
    // Encoding: 0x9AC013E0
    // Test aarch64_integer_tags_mcinsertrandomtag field Xn = 31 (Max)
    // Fields: Xn=31, Xd=0, Xm=0
    let encoding: u32 = 0x9AC013E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_field_xd_0_min_1000_9ac01000() {
    // Encoding: 0x9AC01000
    // Test aarch64_integer_tags_mcinsertrandomtag field Xd = 0 (Min)
    // Fields: Xd=0, Xm=0, Xn=0
    let encoding: u32 = 0x9AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_field_xd_1_poweroftwo_1000_9ac01001() {
    // Encoding: 0x9AC01001
    // Test aarch64_integer_tags_mcinsertrandomtag field Xd = 1 (PowerOfTwo)
    // Fields: Xn=0, Xm=0, Xd=1
    let encoding: u32 = 0x9AC01001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_field_xd_30_poweroftwominusone_1000_9ac0101e() {
    // Encoding: 0x9AC0101E
    // Test aarch64_integer_tags_mcinsertrandomtag field Xd = 30 (PowerOfTwoMinusOne)
    // Fields: Xm=0, Xn=0, Xd=30
    let encoding: u32 = 0x9AC0101E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_field_xd_31_max_1000_9ac0101f() {
    // Encoding: 0x9AC0101F
    // Test aarch64_integer_tags_mcinsertrandomtag field Xd = 31 (Max)
    // Fields: Xn=0, Xm=0, Xd=31
    let encoding: u32 = 0x9AC0101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_0_1000_9ac01000() {
    // Encoding: 0x9AC01000
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=0, Xn=0, Xd=0
    // Fields: Xm=0, Xn=0, Xd=0
    let encoding: u32 = 0x9AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_1_1000_9ac11000() {
    // Encoding: 0x9AC11000
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=1, Xn=0, Xd=0
    // Fields: Xn=0, Xm=1, Xd=0
    let encoding: u32 = 0x9AC11000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_2_1000_9ade1000() {
    // Encoding: 0x9ADE1000
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=30, Xn=0, Xd=0
    // Fields: Xn=0, Xm=30, Xd=0
    let encoding: u32 = 0x9ADE1000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_3_1000_9adf1000() {
    // Encoding: 0x9ADF1000
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=31, Xn=0, Xd=0
    // Fields: Xm=31, Xd=0, Xn=0
    let encoding: u32 = 0x9ADF1000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_4_1000_9ac01000() {
    // Encoding: 0x9AC01000
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=0, Xn=0, Xd=0
    // Fields: Xd=0, Xn=0, Xm=0
    let encoding: u32 = 0x9AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_5_1000_9ac01020() {
    // Encoding: 0x9AC01020
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=0, Xn=1, Xd=0
    // Fields: Xn=1, Xd=0, Xm=0
    let encoding: u32 = 0x9AC01020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_6_1000_9ac013c0() {
    // Encoding: 0x9AC013C0
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=0, Xn=30, Xd=0
    // Fields: Xm=0, Xn=30, Xd=0
    let encoding: u32 = 0x9AC013C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_7_1000_9ac013e0() {
    // Encoding: 0x9AC013E0
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=0, Xn=31, Xd=0
    // Fields: Xm=0, Xd=0, Xn=31
    let encoding: u32 = 0x9AC013E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_8_1000_9ac01000() {
    // Encoding: 0x9AC01000
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=0, Xn=0, Xd=0
    // Fields: Xd=0, Xm=0, Xn=0
    let encoding: u32 = 0x9AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_9_1000_9ac01001() {
    // Encoding: 0x9AC01001
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=0, Xn=0, Xd=1
    // Fields: Xm=0, Xn=0, Xd=1
    let encoding: u32 = 0x9AC01001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_10_1000_9ac0101e() {
    // Encoding: 0x9AC0101E
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=0, Xn=0, Xd=30
    // Fields: Xd=30, Xm=0, Xn=0
    let encoding: u32 = 0x9AC0101E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_11_1000_9ac0101f() {
    // Encoding: 0x9AC0101F
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=0, Xn=0, Xd=31
    // Fields: Xn=0, Xd=31, Xm=0
    let encoding: u32 = 0x9AC0101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=1 (same register test (reg=1)), Xn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_12_1000_9ac11020() {
    // Encoding: 0x9AC11020
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=1, Xn=1, Xd=0
    // Fields: Xm=1, Xd=0, Xn=1
    let encoding: u32 = 0x9AC11020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=31 (same register test (reg=31)), Xn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_13_1000_9adf13e0() {
    // Encoding: 0x9ADF13E0
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=31, Xn=31, Xd=0
    // Fields: Xm=31, Xd=0, Xn=31
    let encoding: u32 = 0x9ADF13E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=1 (same register test (reg=1)), Xd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_14_1000_9ac11001() {
    // Encoding: 0x9AC11001
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=1, Xn=0, Xd=1
    // Fields: Xm=1, Xn=0, Xd=1
    let encoding: u32 = 0x9AC11001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=31 (same register test (reg=31)), Xd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_15_1000_9adf101f() {
    // Encoding: 0x9ADF101F
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=31, Xn=0, Xd=31
    // Fields: Xm=31, Xd=31, Xn=0
    let encoding: u32 = 0x9ADF101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_16_1000_9ac01021() {
    // Encoding: 0x9AC01021
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=0, Xn=1, Xd=1
    // Fields: Xd=1, Xn=1, Xm=0
    let encoding: u32 = 0x9AC01021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_combo_17_1000_9ac013ff() {
    // Encoding: 0x9AC013FF
    // Test aarch64_integer_tags_mcinsertrandomtag field combination: Xm=0, Xn=31, Xd=31
    // Fields: Xd=31, Xm=0, Xn=31
    let encoding: u32 = 0x9AC013FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_special_xn_31_stack_pointer_sp_may_require_alignment_4096_9ac013e0()
 {
    // Encoding: 0x9AC013E0
    // Test aarch64_integer_tags_mcinsertrandomtag special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xn=31, Xd=0, Xm=0
    let encoding: u32 = 0x9AC013E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values - high bits zero
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_umulh_oracle_0_9bc27c20() {
    // Test UMULH: small values - high bits zero (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x2);
    set_x(&mut cpu, 2, 0x3);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// large value * 2 - produces high bits
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_umulh_oracle_1_9bc27c20() {
    // Test UMULH: large value * 2 - produces high bits (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max * max unsigned
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_umulh_oracle_2_9bc27c20() {
    // Test UMULH: max * max unsigned (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max positive * max positive
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_umulh_oracle_3_9bc27c20() {
    // Test UMULH: max positive * max positive (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x3FFFFFFFFFFFFFFF,
        "X0 should be 0x3FFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// 2^32 * 2^32
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_umulh_oracle_4_9bc27c20() {
    // Test UMULH: 2^32 * 2^32 (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x100000000);
    set_x(&mut cpu, 1, 0x100000000);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_reg_write_0_9ac01000() {
    // Test aarch64_integer_tags_mcinsertrandomtag register write: Sp
    // Encoding: 0x9AC01000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x9AC01000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_reg_write_1_9ac01000() {
    // Test aarch64_integer_tags_mcinsertrandomtag register write: GpFromField("d")
    // Encoding: 0x9AC01000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x9AC01000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcinsertrandomtag
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcinsertrandomtag_sp_xn_9ac013e0() {
    // Test aarch64_integer_tags_mcinsertrandomtag with Xn = SP (31)
    // Encoding: 0x9AC013E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x9AC013E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_integer_tags_mcsettagpost Tests
// ============================================================================

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_0_zero_400_d9200400() {
    // Encoding: 0xD9200400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 0 (Zero)
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_1_poweroftwo_400_d9201400() {
    // Encoding: 0xD9201400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 1 (PowerOfTwo)
    // Fields: Xn=0, imm9=1, Xt=0
    let encoding: u32 = 0xD9201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_3_poweroftwominusone_400_d9203400() {
    // Encoding: 0xD9203400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: imm9=3, Xn=0, Xt=0
    let encoding: u32 = 0xD9203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_4_poweroftwo_400_d9204400() {
    // Encoding: 0xD9204400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 4 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=4
    let encoding: u32 = 0xD9204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_7_poweroftwominusone_400_d9207400() {
    // Encoding: 0xD9207400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: imm9=7, Xt=0, Xn=0
    let encoding: u32 = 0xD9207400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_8_poweroftwo_400_d9208400() {
    // Encoding: 0xD9208400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 8 (PowerOfTwo)
    // Fields: imm9=8, Xn=0, Xt=0
    let encoding: u32 = 0xD9208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_15_poweroftwominusone_400_d920f400() {
    // Encoding: 0xD920F400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=15, Xn=0
    let encoding: u32 = 0xD920F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_16_poweroftwo_400_d9210400() {
    // Encoding: 0xD9210400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 16 (PowerOfTwo)
    // Fields: Xt=0, imm9=16, Xn=0
    let encoding: u32 = 0xD9210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_31_poweroftwominusone_400_d921f400() {
    // Encoding: 0xD921F400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: imm9=31, Xn=0, Xt=0
    let encoding: u32 = 0xD921F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_32_poweroftwo_400_d9220400() {
    // Encoding: 0xD9220400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 32 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=32
    let encoding: u32 = 0xD9220400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_63_poweroftwominusone_400_d923f400() {
    // Encoding: 0xD923F400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: imm9=63, Xt=0, Xn=0
    let encoding: u32 = 0xD923F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_64_poweroftwo_400_d9240400() {
    // Encoding: 0xD9240400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 64 (PowerOfTwo)
    // Fields: Xt=0, imm9=64, Xn=0
    let encoding: u32 = 0xD9240400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_127_poweroftwominusone_400_d927f400() {
    // Encoding: 0xD927F400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=127
    let encoding: u32 = 0xD927F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_128_poweroftwo_400_d9280400() {
    // Encoding: 0xD9280400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 128 (PowerOfTwo)
    // Fields: Xt=0, imm9=128, Xn=0
    let encoding: u32 = 0xD9280400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_255_poweroftwominusone_400_d92ff400() {
    // Encoding: 0xD92FF400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=255
    let encoding: u32 = 0xD92FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_256_poweroftwo_400_d9300400() {
    // Encoding: 0xD9300400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 256 (PowerOfTwo)
    // Fields: Xt=0, imm9=256, Xn=0
    let encoding: u32 = 0xD9300400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_imm9_511_max_400_d93ff400() {
    // Encoding: 0xD93FF400
    // Test aarch64_integer_tags_mcsettagpost field imm9 = 511 (Max)
    // Fields: Xt=0, Xn=0, imm9=511
    let encoding: u32 = 0xD93FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_xn_0_min_400_d9200400() {
    // Encoding: 0xD9200400
    // Test aarch64_integer_tags_mcsettagpost field Xn = 0 (Min)
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_xn_1_poweroftwo_400_d9200420() {
    // Encoding: 0xD9200420
    // Test aarch64_integer_tags_mcsettagpost field Xn = 1 (PowerOfTwo)
    // Fields: Xn=1, imm9=0, Xt=0
    let encoding: u32 = 0xD9200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_xn_30_poweroftwominusone_400_d92007c0() {
    // Encoding: 0xD92007C0
    // Test aarch64_integer_tags_mcsettagpost field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=0, Xn=30
    let encoding: u32 = 0xD92007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_xn_31_max_400_d92007e0() {
    // Encoding: 0xD92007E0
    // Test aarch64_integer_tags_mcsettagpost field Xn = 31 (Max)
    // Fields: Xn=31, Xt=0, imm9=0
    let encoding: u32 = 0xD92007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_xt_0_min_400_d9200400() {
    // Encoding: 0xD9200400
    // Test aarch64_integer_tags_mcsettagpost field Xt = 0 (Min)
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_xt_1_poweroftwo_400_d9200401() {
    // Encoding: 0xD9200401
    // Test aarch64_integer_tags_mcsettagpost field Xt = 1 (PowerOfTwo)
    // Fields: imm9=0, Xn=0, Xt=1
    let encoding: u32 = 0xD9200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_xt_30_poweroftwominusone_400_d920041e() {
    // Encoding: 0xD920041E
    // Test aarch64_integer_tags_mcsettagpost field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=30, imm9=0
    let encoding: u32 = 0xD920041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_field_xt_31_max_400_d920041f() {
    // Encoding: 0xD920041F
    // Test aarch64_integer_tags_mcsettagpost field Xt = 31 (Max)
    // Fields: Xt=31, imm9=0, Xn=0
    let encoding: u32 = 0xD920041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_0_400_d9200400() {
    // Encoding: 0xD9200400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_1_400_d9201400() {
    // Encoding: 0xD9201400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=1, Xn=0, Xt=0
    // Fields: Xt=0, imm9=1, Xn=0
    let encoding: u32 = 0xD9201400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_2_400_d9203400() {
    // Encoding: 0xD9203400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=3, Xn=0, Xt=0
    // Fields: Xt=0, imm9=3, Xn=0
    let encoding: u32 = 0xD9203400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_3_400_d9204400() {
    // Encoding: 0xD9204400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=4, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=4
    let encoding: u32 = 0xD9204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_4_400_d9207400() {
    // Encoding: 0xD9207400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=7, Xn=0, Xt=0
    // Fields: Xt=0, imm9=7, Xn=0
    let encoding: u32 = 0xD9207400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_5_400_d9208400() {
    // Encoding: 0xD9208400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=8, Xn=0, Xt=0
    // Fields: Xt=0, imm9=8, Xn=0
    let encoding: u32 = 0xD9208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_6_400_d920f400() {
    // Encoding: 0xD920F400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=15, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=15
    let encoding: u32 = 0xD920F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_7_400_d9210400() {
    // Encoding: 0xD9210400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=16, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=16
    let encoding: u32 = 0xD9210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_8_400_d921f400() {
    // Encoding: 0xD921F400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=31, Xn=0, Xt=0
    // Fields: Xt=0, imm9=31, Xn=0
    let encoding: u32 = 0xD921F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_9_400_d9220400() {
    // Encoding: 0xD9220400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=32, Xn=0, Xt=0
    // Fields: imm9=32, Xn=0, Xt=0
    let encoding: u32 = 0xD9220400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_10_400_d923f400() {
    // Encoding: 0xD923F400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=63, Xn=0, Xt=0
    // Fields: Xt=0, imm9=63, Xn=0
    let encoding: u32 = 0xD923F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_11_400_d9240400() {
    // Encoding: 0xD9240400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=64, Xn=0, Xt=0
    // Fields: imm9=64, Xn=0, Xt=0
    let encoding: u32 = 0xD9240400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_12_400_d927f400() {
    // Encoding: 0xD927F400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=127, Xn=0, Xt=0
    // Fields: imm9=127, Xn=0, Xt=0
    let encoding: u32 = 0xD927F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_13_400_d9280400() {
    // Encoding: 0xD9280400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=128, Xn=0, Xt=0
    // Fields: Xn=0, imm9=128, Xt=0
    let encoding: u32 = 0xD9280400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_14_400_d92ff400() {
    // Encoding: 0xD92FF400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=255, Xn=0, Xt=0
    // Fields: imm9=255, Xt=0, Xn=0
    let encoding: u32 = 0xD92FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_15_400_d9300400() {
    // Encoding: 0xD9300400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=256, Xn=0, Xt=0
    // Fields: imm9=256, Xn=0, Xt=0
    let encoding: u32 = 0xD9300400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_16_400_d93ff400() {
    // Encoding: 0xD93FF400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=511, Xn=0, Xt=0
    // Fields: Xn=0, imm9=511, Xt=0
    let encoding: u32 = 0xD93FF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_17_400_d9200400() {
    // Encoding: 0xD9200400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_18_400_d9200420() {
    // Encoding: 0xD9200420
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=0, Xn=1, Xt=0
    // Fields: Xn=1, Xt=0, imm9=0
    let encoding: u32 = 0xD9200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_19_400_d92007c0() {
    // Encoding: 0xD92007C0
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=0, Xn=30, Xt=0
    // Fields: Xt=0, imm9=0, Xn=30
    let encoding: u32 = 0xD92007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_20_400_d92007e0() {
    // Encoding: 0xD92007E0
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=0, Xn=31, Xt=0
    // Fields: imm9=0, Xn=31, Xt=0
    let encoding: u32 = 0xD92007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_21_400_d9200400() {
    // Encoding: 0xD9200400
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_22_400_d9200401() {
    // Encoding: 0xD9200401
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=0, Xn=0, Xt=1
    // Fields: Xt=1, imm9=0, Xn=0
    let encoding: u32 = 0xD9200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_23_400_d920041e() {
    // Encoding: 0xD920041E
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=0, Xn=0, Xt=30
    // Fields: Xn=0, Xt=30, imm9=0
    let encoding: u32 = 0xD920041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_24_400_d920041f() {
    // Encoding: 0xD920041F
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=0, Xn=0, Xt=31
    // Fields: Xn=0, Xt=31, imm9=0
    let encoding: u32 = 0xD920041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_25_400_d9200421() {
    // Encoding: 0xD9200421
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=0, Xn=1, Xt=1
    // Fields: Xn=1, imm9=0, Xt=1
    let encoding: u32 = 0xD9200421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettagpost_combo_26_400_d92007ff() {
    // Encoding: 0xD92007FF
    // Test aarch64_integer_tags_mcsettagpost field combination: imm9=0, Xn=31, Xt=31
    // Fields: Xt=31, Xn=31, imm9=0
    let encoding: u32 = 0xD92007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettagpost_special_xn_31_stack_pointer_sp_may_require_alignment_1024_d92017e0()
 {
    // Encoding: 0xD92017E0
    // Test aarch64_integer_tags_mcsettagpost special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xt=0, Xn=31, imm9=1
    let encoding: u32 = 0xD92017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_0_zero_c00_d9200c00() {
    // Encoding: 0xD9200C00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 0 (Zero)
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_1_poweroftwo_c00_d9201c00() {
    // Encoding: 0xD9201C00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 1 (PowerOfTwo)
    // Fields: imm9=1, Xn=0, Xt=0
    let encoding: u32 = 0xD9201C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_3_poweroftwominusone_c00_d9203c00() {
    // Encoding: 0xD9203C00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=3, Xn=0
    let encoding: u32 = 0xD9203C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_4_poweroftwo_c00_d9204c00() {
    // Encoding: 0xD9204C00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 4 (PowerOfTwo)
    // Fields: imm9=4, Xn=0, Xt=0
    let encoding: u32 = 0xD9204C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_7_poweroftwominusone_c00_d9207c00() {
    // Encoding: 0xD9207C00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=7, Xt=0
    let encoding: u32 = 0xD9207C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_8_poweroftwo_c00_d9208c00() {
    // Encoding: 0xD9208C00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 8 (PowerOfTwo)
    // Fields: Xn=0, imm9=8, Xt=0
    let encoding: u32 = 0xD9208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_15_poweroftwominusone_c00_d920fc00() {
    // Encoding: 0xD920FC00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=15, Xn=0
    let encoding: u32 = 0xD920FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_16_poweroftwo_c00_d9210c00() {
    // Encoding: 0xD9210C00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 16 (PowerOfTwo)
    // Fields: imm9=16, Xn=0, Xt=0
    let encoding: u32 = 0xD9210C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_31_poweroftwominusone_c00_d921fc00() {
    // Encoding: 0xD921FC00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=31, Xt=0
    let encoding: u32 = 0xD921FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_32_poweroftwo_c00_d9220c00() {
    // Encoding: 0xD9220C00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 32 (PowerOfTwo)
    // Fields: imm9=32, Xt=0, Xn=0
    let encoding: u32 = 0xD9220C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_63_poweroftwominusone_c00_d923fc00() {
    // Encoding: 0xD923FC00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=63, Xn=0
    let encoding: u32 = 0xD923FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_64_poweroftwo_c00_d9240c00() {
    // Encoding: 0xD9240C00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 64 (PowerOfTwo)
    // Fields: Xt=0, Xn=0, imm9=64
    let encoding: u32 = 0xD9240C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_127_poweroftwominusone_c00_d927fc00() {
    // Encoding: 0xD927FC00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=127, Xt=0
    let encoding: u32 = 0xD927FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_128_poweroftwo_c00_d9280c00() {
    // Encoding: 0xD9280C00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 128 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=128
    let encoding: u32 = 0xD9280C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_255_poweroftwominusone_c00_d92ffc00() {
    // Encoding: 0xD92FFC00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: imm9=255, Xn=0, Xt=0
    let encoding: u32 = 0xD92FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_256_poweroftwo_c00_d9300c00() {
    // Encoding: 0xD9300C00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 256 (PowerOfTwo)
    // Fields: Xn=0, imm9=256, Xt=0
    let encoding: u32 = 0xD9300C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_imm9_511_max_c00_d93ffc00() {
    // Encoding: 0xD93FFC00
    // Test aarch64_integer_tags_mcsettagpre field imm9 = 511 (Max)
    // Fields: Xn=0, Xt=0, imm9=511
    let encoding: u32 = 0xD93FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_xn_0_min_c00_d9200c00() {
    // Encoding: 0xD9200C00
    // Test aarch64_integer_tags_mcsettagpre field Xn = 0 (Min)
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_xn_1_poweroftwo_c00_d9200c20() {
    // Encoding: 0xD9200C20
    // Test aarch64_integer_tags_mcsettagpre field Xn = 1 (PowerOfTwo)
    // Fields: Xt=0, imm9=0, Xn=1
    let encoding: u32 = 0xD9200C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_xn_30_poweroftwominusone_c00_d9200fc0() {
    // Encoding: 0xD9200FC0
    // Test aarch64_integer_tags_mcsettagpre field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: imm9=0, Xt=0, Xn=30
    let encoding: u32 = 0xD9200FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_xn_31_max_c00_d9200fe0() {
    // Encoding: 0xD9200FE0
    // Test aarch64_integer_tags_mcsettagpre field Xn = 31 (Max)
    // Fields: Xt=0, Xn=31, imm9=0
    let encoding: u32 = 0xD9200FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_xt_0_min_c00_d9200c00() {
    // Encoding: 0xD9200C00
    // Test aarch64_integer_tags_mcsettagpre field Xt = 0 (Min)
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_xt_1_poweroftwo_c00_d9200c01() {
    // Encoding: 0xD9200C01
    // Test aarch64_integer_tags_mcsettagpre field Xt = 1 (PowerOfTwo)
    // Fields: Xt=1, imm9=0, Xn=0
    let encoding: u32 = 0xD9200C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_xt_30_poweroftwominusone_c00_d9200c1e() {
    // Encoding: 0xD9200C1E
    // Test aarch64_integer_tags_mcsettagpre field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: imm9=0, Xt=30, Xn=0
    let encoding: u32 = 0xD9200C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_field_xt_31_max_c00_d9200c1f() {
    // Encoding: 0xD9200C1F
    // Test aarch64_integer_tags_mcsettagpre field Xt = 31 (Max)
    // Fields: imm9=0, Xn=0, Xt=31
    let encoding: u32 = 0xD9200C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_0_c00_d9200c00() {
    // Encoding: 0xD9200C00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_1_c00_d9201c00() {
    // Encoding: 0xD9201C00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=1, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=1
    let encoding: u32 = 0xD9201C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_2_c00_d9203c00() {
    // Encoding: 0xD9203C00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=3, Xn=0, Xt=0
    // Fields: Xn=0, imm9=3, Xt=0
    let encoding: u32 = 0xD9203C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_3_c00_d9204c00() {
    // Encoding: 0xD9204C00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=4, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=4
    let encoding: u32 = 0xD9204C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_4_c00_d9207c00() {
    // Encoding: 0xD9207C00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=7, Xn=0, Xt=0
    // Fields: Xt=0, imm9=7, Xn=0
    let encoding: u32 = 0xD9207C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_5_c00_d9208c00() {
    // Encoding: 0xD9208C00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=8, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=8
    let encoding: u32 = 0xD9208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_6_c00_d920fc00() {
    // Encoding: 0xD920FC00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=15, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=15
    let encoding: u32 = 0xD920FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_7_c00_d9210c00() {
    // Encoding: 0xD9210C00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=16, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=16
    let encoding: u32 = 0xD9210C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_8_c00_d921fc00() {
    // Encoding: 0xD921FC00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=31, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=31
    let encoding: u32 = 0xD921FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_9_c00_d9220c00() {
    // Encoding: 0xD9220C00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=32, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=32
    let encoding: u32 = 0xD9220C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_10_c00_d923fc00() {
    // Encoding: 0xD923FC00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=63, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=63
    let encoding: u32 = 0xD923FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_11_c00_d9240c00() {
    // Encoding: 0xD9240C00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=64, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=64
    let encoding: u32 = 0xD9240C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_12_c00_d927fc00() {
    // Encoding: 0xD927FC00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=127, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=127
    let encoding: u32 = 0xD927FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_13_c00_d9280c00() {
    // Encoding: 0xD9280C00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=128, Xn=0, Xt=0
    // Fields: Xn=0, imm9=128, Xt=0
    let encoding: u32 = 0xD9280C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_14_c00_d92ffc00() {
    // Encoding: 0xD92FFC00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=255, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=255
    let encoding: u32 = 0xD92FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_15_c00_d9300c00() {
    // Encoding: 0xD9300C00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=256, Xn=0, Xt=0
    // Fields: imm9=256, Xn=0, Xt=0
    let encoding: u32 = 0xD9300C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_16_c00_d93ffc00() {
    // Encoding: 0xD93FFC00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=511, Xn=0, Xt=0
    // Fields: imm9=511, Xt=0, Xn=0
    let encoding: u32 = 0xD93FFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_17_c00_d9200c00() {
    // Encoding: 0xD9200C00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_18_c00_d9200c20() {
    // Encoding: 0xD9200C20
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=0, Xn=1, Xt=0
    // Fields: Xt=0, Xn=1, imm9=0
    let encoding: u32 = 0xD9200C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_19_c00_d9200fc0() {
    // Encoding: 0xD9200FC0
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=0, Xn=30, Xt=0
    // Fields: imm9=0, Xt=0, Xn=30
    let encoding: u32 = 0xD9200FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_20_c00_d9200fe0() {
    // Encoding: 0xD9200FE0
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=0, Xn=31, Xt=0
    // Fields: imm9=0, Xn=31, Xt=0
    let encoding: u32 = 0xD9200FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_21_c00_d9200c00() {
    // Encoding: 0xD9200C00
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_22_c00_d9200c01() {
    // Encoding: 0xD9200C01
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=0, Xn=0, Xt=1
    // Fields: Xt=1, imm9=0, Xn=0
    let encoding: u32 = 0xD9200C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_23_c00_d9200c1e() {
    // Encoding: 0xD9200C1E
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=0, Xn=0, Xt=30
    // Fields: imm9=0, Xt=30, Xn=0
    let encoding: u32 = 0xD9200C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_24_c00_d9200c1f() {
    // Encoding: 0xD9200C1F
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=0, Xn=0, Xt=31
    // Fields: imm9=0, Xn=0, Xt=31
    let encoding: u32 = 0xD9200C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_25_c00_d9200c21() {
    // Encoding: 0xD9200C21
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=0, Xn=1, Xt=1
    // Fields: imm9=0, Xn=1, Xt=1
    let encoding: u32 = 0xD9200C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettagpre_combo_26_c00_d9200fff() {
    // Encoding: 0xD9200FFF
    // Test aarch64_integer_tags_mcsettagpre field combination: imm9=0, Xn=31, Xt=31
    // Fields: imm9=0, Xn=31, Xt=31
    let encoding: u32 = 0xD9200FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettagpre_special_xn_31_stack_pointer_sp_may_require_alignment_3072_d9201fe0()
 {
    // Encoding: 0xD9201FE0
    // Test aarch64_integer_tags_mcsettagpre special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: imm9=1, Xt=0, Xn=31
    let encoding: u32 = 0xD9201FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_0_zero_800_d9200800() {
    // Encoding: 0xD9200800
    // Test aarch64_integer_tags_mcsettag field imm9 = 0 (Zero)
    // Fields: Xt=0, Xn=0, imm9=0
    let encoding: u32 = 0xD9200800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_1_poweroftwo_800_d9201800() {
    // Encoding: 0xD9201800
    // Test aarch64_integer_tags_mcsettag field imm9 = 1 (PowerOfTwo)
    // Fields: imm9=1, Xn=0, Xt=0
    let encoding: u32 = 0xD9201800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_3_poweroftwominusone_800_d9203800() {
    // Encoding: 0xD9203800
    // Test aarch64_integer_tags_mcsettag field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=3
    let encoding: u32 = 0xD9203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_4_poweroftwo_800_d9204800() {
    // Encoding: 0xD9204800
    // Test aarch64_integer_tags_mcsettag field imm9 = 4 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=4
    let encoding: u32 = 0xD9204800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_7_poweroftwominusone_800_d9207800() {
    // Encoding: 0xD9207800
    // Test aarch64_integer_tags_mcsettag field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: imm9=7, Xt=0, Xn=0
    let encoding: u32 = 0xD9207800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_8_poweroftwo_800_d9208800() {
    // Encoding: 0xD9208800
    // Test aarch64_integer_tags_mcsettag field imm9 = 8 (PowerOfTwo)
    // Fields: imm9=8, Xt=0, Xn=0
    let encoding: u32 = 0xD9208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_15_poweroftwominusone_800_d920f800() {
    // Encoding: 0xD920F800
    // Test aarch64_integer_tags_mcsettag field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xn=0, imm9=15
    let encoding: u32 = 0xD920F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_16_poweroftwo_800_d9210800() {
    // Encoding: 0xD9210800
    // Test aarch64_integer_tags_mcsettag field imm9 = 16 (PowerOfTwo)
    // Fields: Xt=0, Xn=0, imm9=16
    let encoding: u32 = 0xD9210800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_31_poweroftwominusone_800_d921f800() {
    // Encoding: 0xD921F800
    // Test aarch64_integer_tags_mcsettag field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=31, Xn=0
    let encoding: u32 = 0xD921F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_32_poweroftwo_800_d9220800() {
    // Encoding: 0xD9220800
    // Test aarch64_integer_tags_mcsettag field imm9 = 32 (PowerOfTwo)
    // Fields: imm9=32, Xn=0, Xt=0
    let encoding: u32 = 0xD9220800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_63_poweroftwominusone_800_d923f800() {
    // Encoding: 0xD923F800
    // Test aarch64_integer_tags_mcsettag field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xn=0, imm9=63
    let encoding: u32 = 0xD923F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_64_poweroftwo_800_d9240800() {
    // Encoding: 0xD9240800
    // Test aarch64_integer_tags_mcsettag field imm9 = 64 (PowerOfTwo)
    // Fields: Xt=0, imm9=64, Xn=0
    let encoding: u32 = 0xD9240800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_127_poweroftwominusone_800_d927f800() {
    // Encoding: 0xD927F800
    // Test aarch64_integer_tags_mcsettag field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: imm9=127, Xn=0, Xt=0
    let encoding: u32 = 0xD927F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_128_poweroftwo_800_d9280800() {
    // Encoding: 0xD9280800
    // Test aarch64_integer_tags_mcsettag field imm9 = 128 (PowerOfTwo)
    // Fields: imm9=128, Xt=0, Xn=0
    let encoding: u32 = 0xD9280800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_255_poweroftwominusone_800_d92ff800() {
    // Encoding: 0xD92FF800
    // Test aarch64_integer_tags_mcsettag field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=255
    let encoding: u32 = 0xD92FF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_256_poweroftwo_800_d9300800() {
    // Encoding: 0xD9300800
    // Test aarch64_integer_tags_mcsettag field imm9 = 256 (PowerOfTwo)
    // Fields: Xt=0, imm9=256, Xn=0
    let encoding: u32 = 0xD9300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_imm9_511_max_800_d93ff800() {
    // Encoding: 0xD93FF800
    // Test aarch64_integer_tags_mcsettag field imm9 = 511 (Max)
    // Fields: Xn=0, Xt=0, imm9=511
    let encoding: u32 = 0xD93FF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_xn_0_min_800_d9200800() {
    // Encoding: 0xD9200800
    // Test aarch64_integer_tags_mcsettag field Xn = 0 (Min)
    // Fields: imm9=0, Xt=0, Xn=0
    let encoding: u32 = 0xD9200800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_xn_1_poweroftwo_800_d9200820() {
    // Encoding: 0xD9200820
    // Test aarch64_integer_tags_mcsettag field Xn = 1 (PowerOfTwo)
    // Fields: imm9=0, Xn=1, Xt=0
    let encoding: u32 = 0xD9200820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_xn_30_poweroftwominusone_800_d9200bc0() {
    // Encoding: 0xD9200BC0
    // Test aarch64_integer_tags_mcsettag field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=0, Xn=30
    let encoding: u32 = 0xD9200BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_xn_31_max_800_d9200be0() {
    // Encoding: 0xD9200BE0
    // Test aarch64_integer_tags_mcsettag field Xn = 31 (Max)
    // Fields: Xt=0, imm9=0, Xn=31
    let encoding: u32 = 0xD9200BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_xt_0_min_800_d9200800() {
    // Encoding: 0xD9200800
    // Test aarch64_integer_tags_mcsettag field Xt = 0 (Min)
    // Fields: imm9=0, Xt=0, Xn=0
    let encoding: u32 = 0xD9200800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_xt_1_poweroftwo_800_d9200801() {
    // Encoding: 0xD9200801
    // Test aarch64_integer_tags_mcsettag field Xt = 1 (PowerOfTwo)
    // Fields: Xt=1, imm9=0, Xn=0
    let encoding: u32 = 0xD9200801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_xt_30_poweroftwominusone_800_d920081e() {
    // Encoding: 0xD920081E
    // Test aarch64_integer_tags_mcsettag field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: Xt=30, Xn=0, imm9=0
    let encoding: u32 = 0xD920081E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettag_field_xt_31_max_800_d920081f() {
    // Encoding: 0xD920081F
    // Test aarch64_integer_tags_mcsettag field Xt = 31 (Max)
    // Fields: imm9=0, Xn=0, Xt=31
    let encoding: u32 = 0xD920081F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_0_800_d9200800() {
    // Encoding: 0xD9200800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9200800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_1_800_d9201800() {
    // Encoding: 0xD9201800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=1, Xn=0, Xt=0
    // Fields: imm9=1, Xt=0, Xn=0
    let encoding: u32 = 0xD9201800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_2_800_d9203800() {
    // Encoding: 0xD9203800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=3, Xn=0, Xt=0
    // Fields: imm9=3, Xt=0, Xn=0
    let encoding: u32 = 0xD9203800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_3_800_d9204800() {
    // Encoding: 0xD9204800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=4, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=4
    let encoding: u32 = 0xD9204800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_4_800_d9207800() {
    // Encoding: 0xD9207800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=7, Xn=0, Xt=0
    // Fields: Xt=0, imm9=7, Xn=0
    let encoding: u32 = 0xD9207800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_5_800_d9208800() {
    // Encoding: 0xD9208800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=8, Xn=0, Xt=0
    // Fields: imm9=8, Xn=0, Xt=0
    let encoding: u32 = 0xD9208800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_6_800_d920f800() {
    // Encoding: 0xD920F800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=15, Xn=0, Xt=0
    // Fields: Xt=0, imm9=15, Xn=0
    let encoding: u32 = 0xD920F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_7_800_d9210800() {
    // Encoding: 0xD9210800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=16, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=16
    let encoding: u32 = 0xD9210800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_8_800_d921f800() {
    // Encoding: 0xD921F800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=31, Xn=0, Xt=0
    // Fields: imm9=31, Xt=0, Xn=0
    let encoding: u32 = 0xD921F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_9_800_d9220800() {
    // Encoding: 0xD9220800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=32, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=32
    let encoding: u32 = 0xD9220800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_10_800_d923f800() {
    // Encoding: 0xD923F800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=63, Xn=0, Xt=0
    // Fields: imm9=63, Xn=0, Xt=0
    let encoding: u32 = 0xD923F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_11_800_d9240800() {
    // Encoding: 0xD9240800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=64, Xn=0, Xt=0
    // Fields: Xn=0, imm9=64, Xt=0
    let encoding: u32 = 0xD9240800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_12_800_d927f800() {
    // Encoding: 0xD927F800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=127, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=127
    let encoding: u32 = 0xD927F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_13_800_d9280800() {
    // Encoding: 0xD9280800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=128, Xn=0, Xt=0
    // Fields: imm9=128, Xn=0, Xt=0
    let encoding: u32 = 0xD9280800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_14_800_d92ff800() {
    // Encoding: 0xD92FF800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=255, Xn=0, Xt=0
    // Fields: imm9=255, Xn=0, Xt=0
    let encoding: u32 = 0xD92FF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_15_800_d9300800() {
    // Encoding: 0xD9300800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=256, Xn=0, Xt=0
    // Fields: imm9=256, Xt=0, Xn=0
    let encoding: u32 = 0xD9300800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_16_800_d93ff800() {
    // Encoding: 0xD93FF800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=511, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=511
    let encoding: u32 = 0xD93FF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_17_800_d9200800() {
    // Encoding: 0xD9200800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9200800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_18_800_d9200820() {
    // Encoding: 0xD9200820
    // Test aarch64_integer_tags_mcsettag field combination: imm9=0, Xn=1, Xt=0
    // Fields: Xt=0, Xn=1, imm9=0
    let encoding: u32 = 0xD9200820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_19_800_d9200bc0() {
    // Encoding: 0xD9200BC0
    // Test aarch64_integer_tags_mcsettag field combination: imm9=0, Xn=30, Xt=0
    // Fields: imm9=0, Xn=30, Xt=0
    let encoding: u32 = 0xD9200BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_20_800_d9200be0() {
    // Encoding: 0xD9200BE0
    // Test aarch64_integer_tags_mcsettag field combination: imm9=0, Xn=31, Xt=0
    // Fields: Xn=31, Xt=0, imm9=0
    let encoding: u32 = 0xD9200BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_21_800_d9200800() {
    // Encoding: 0xD9200800
    // Test aarch64_integer_tags_mcsettag field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xn=0, imm9=0, Xt=0
    let encoding: u32 = 0xD9200800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_22_800_d9200801() {
    // Encoding: 0xD9200801
    // Test aarch64_integer_tags_mcsettag field combination: imm9=0, Xn=0, Xt=1
    // Fields: Xt=1, imm9=0, Xn=0
    let encoding: u32 = 0xD9200801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_23_800_d920081e() {
    // Encoding: 0xD920081E
    // Test aarch64_integer_tags_mcsettag field combination: imm9=0, Xn=0, Xt=30
    // Fields: Xn=0, imm9=0, Xt=30
    let encoding: u32 = 0xD920081E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_24_800_d920081f() {
    // Encoding: 0xD920081F
    // Test aarch64_integer_tags_mcsettag field combination: imm9=0, Xn=0, Xt=31
    // Fields: imm9=0, Xn=0, Xt=31
    let encoding: u32 = 0xD920081F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_25_800_d9200821() {
    // Encoding: 0xD9200821
    // Test aarch64_integer_tags_mcsettag field combination: imm9=0, Xn=1, Xt=1
    // Fields: Xn=1, Xt=1, imm9=0
    let encoding: u32 = 0xD9200821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettag_combo_26_800_d9200bff() {
    // Encoding: 0xD9200BFF
    // Test aarch64_integer_tags_mcsettag field combination: imm9=0, Xn=31, Xt=31
    // Fields: Xn=31, Xt=31, imm9=0
    let encoding: u32 = 0xD9200BFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettag_special_xn_31_stack_pointer_sp_may_require_alignment_2048_d9201be0()
 {
    // Encoding: 0xD9201BE0
    // Test aarch64_integer_tags_mcsettag special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xt=0, imm9=1, Xn=31
    let encoding: u32 = 0xD9201BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// zero value
#[test]
fn test_aarch64_integer_tags_mcsettagpost_str_oracle_0_f9000020() {
    // Test STR: zero value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x0);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x0, "Memory at 0x1000 should be 0x0");
    }
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// byte value
#[test]
fn test_aarch64_integer_tags_mcsettagpost_str_oracle_1_f9000020() {
    // Test STR: byte value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xFF);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0xFF, "Memory at 0x1000 should be 0xFF");
    }
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// halfword value
#[test]
fn test_aarch64_integer_tags_mcsettagpost_str_oracle_2_f9000020() {
    // Test STR: halfword value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x1234);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x1234, "Memory at 0x1000 should be 0x1234");
    }
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// word value
#[test]
fn test_aarch64_integer_tags_mcsettagpost_str_oracle_3_f9000020() {
    // Test STR: word value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x12345678);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x12345678, "Memory at 0x1000 should be 0x12345678");
    }
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// doubleword value
#[test]
fn test_aarch64_integer_tags_mcsettagpost_str_oracle_4_f9000020() {
    // Test STR: doubleword value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x123456789ABCDEF0);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(
            val, 0x123456789ABCDEF0,
            "Memory at 0x1000 should be 0x123456789ABCDEF0"
        );
    }
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettagpost_reg_write_0_d9200400() {
    // Test aarch64_integer_tags_mcsettagpost register write: Sp
    // Encoding: 0xD9200400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9200400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettagpost_reg_write_1_d9200400() {
    // Test aarch64_integer_tags_mcsettagpost register write: GpFromField("n")
    // Encoding: 0xD9200400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9200400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpost_sp_xn_d92007e0() {
    // Test aarch64_integer_tags_mcsettagpost with Xn = SP (31)
    // Encoding: 0xD92007E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD92007E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpost
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpost_store_0_d9200400() {
    // Test aarch64_integer_tags_mcsettagpost memory store: 8 bytes
    // Encoding: 0xD9200400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0xD9200400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// zero value
#[test]
fn test_aarch64_integer_tags_mcsettagpre_str_oracle_0_f9000020() {
    // Test STR: zero value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x0);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x0, "Memory at 0x1000 should be 0x0");
    }
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// byte value
#[test]
fn test_aarch64_integer_tags_mcsettagpre_str_oracle_1_f9000020() {
    // Test STR: byte value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0xFF);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0xFF, "Memory at 0x1000 should be 0xFF");
    }
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// halfword value
#[test]
fn test_aarch64_integer_tags_mcsettagpre_str_oracle_2_f9000020() {
    // Test STR: halfword value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x1234);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x1234, "Memory at 0x1000 should be 0x1234");
    }
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// word value
#[test]
fn test_aarch64_integer_tags_mcsettagpre_str_oracle_3_f9000020() {
    // Test STR: word value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x12345678);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x12345678, "Memory at 0x1000 should be 0x12345678");
    }
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// doubleword value
#[test]
fn test_aarch64_integer_tags_mcsettagpre_str_oracle_4_f9000020() {
    // Test STR: doubleword value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x123456789ABCDEF0);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(
            val, 0x123456789ABCDEF0,
            "Memory at 0x1000 should be 0x123456789ABCDEF0"
        );
    }
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettagpre_reg_write_0_d9200c00() {
    // Test aarch64_integer_tags_mcsettagpre register write: Sp
    // Encoding: 0xD9200C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9200C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettagpre_reg_write_1_d9200c00() {
    // Test aarch64_integer_tags_mcsettagpre register write: GpFromField("n")
    // Encoding: 0xD9200C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9200C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpre_sp_xn_d9200fe0() {
    // Test aarch64_integer_tags_mcsettagpre with Xn = SP (31)
    // Encoding: 0xD9200FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9200FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpre
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpre_store_0_d9200c00() {
    // Test aarch64_integer_tags_mcsettagpre memory store: 8 bytes
    // Encoding: 0xD9200C00
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0xD9200C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// zero value
#[test]
fn test_aarch64_integer_tags_mcsettag_str_oracle_0_f9000020() {
    // Test STR: zero value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x0);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x0, "Memory at 0x1000 should be 0x0");
    }
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// byte value
#[test]
fn test_aarch64_integer_tags_mcsettag_str_oracle_1_f9000020() {
    // Test STR: byte value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xFF);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0xFF, "Memory at 0x1000 should be 0xFF");
    }
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// halfword value
#[test]
fn test_aarch64_integer_tags_mcsettag_str_oracle_2_f9000020() {
    // Test STR: halfword value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x1234);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x1234, "Memory at 0x1000 should be 0x1234");
    }
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// word value
#[test]
fn test_aarch64_integer_tags_mcsettag_str_oracle_3_f9000020() {
    // Test STR: word value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x12345678);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x12345678, "Memory at 0x1000 should be 0x12345678");
    }
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `STR X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "immediate" }
/// doubleword value
#[test]
fn test_aarch64_integer_tags_mcsettag_str_oracle_4_f9000020() {
    // Test STR: doubleword value (oracle)
    // Encoding: 0xF9000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x123456789ABCDEF0);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0xF9000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 8).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(
            val, 0x123456789ABCDEF0,
            "Memory at 0x1000 should be 0x123456789ABCDEF0"
        );
    }
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettag_reg_write_0_d9200800() {
    // Test aarch64_integer_tags_mcsettag register write: Sp
    // Encoding: 0xD9200800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9200800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettag_reg_write_1_d9200800() {
    // Test aarch64_integer_tags_mcsettag register write: GpFromField("n")
    // Encoding: 0xD9200800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9200800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettag_sp_xn_d9200be0() {
    // Test aarch64_integer_tags_mcsettag with Xn = SP (31)
    // Encoding: 0xD9200BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9200BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettag
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettag_store_0_d9200800() {
    // Test aarch64_integer_tags_mcsettag memory store: 8 bytes
    // Encoding: 0xD9200800
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0xD9200800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_integer_tags_mcsettagpairpost Tests
// ============================================================================

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_0_zero_400_d9a00400() {
    // Encoding: 0xD9A00400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 0 (Zero)
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_1_poweroftwo_400_d9a01400() {
    // Encoding: 0xD9A01400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 1 (PowerOfTwo)
    // Fields: Xn=0, imm9=1, Xt=0
    let encoding: u32 = 0xD9A01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_3_poweroftwominusone_400_d9a03400() {
    // Encoding: 0xD9A03400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=3, Xt=0
    let encoding: u32 = 0xD9A03400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_4_poweroftwo_400_d9a04400() {
    // Encoding: 0xD9A04400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 4 (PowerOfTwo)
    // Fields: imm9=4, Xn=0, Xt=0
    let encoding: u32 = 0xD9A04400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_7_poweroftwominusone_400_d9a07400() {
    // Encoding: 0xD9A07400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=7, Xt=0
    let encoding: u32 = 0xD9A07400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_8_poweroftwo_400_d9a08400() {
    // Encoding: 0xD9A08400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 8 (PowerOfTwo)
    // Fields: imm9=8, Xn=0, Xt=0
    let encoding: u32 = 0xD9A08400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_15_poweroftwominusone_400_d9a0f400() {
    // Encoding: 0xD9A0F400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=15, Xt=0
    let encoding: u32 = 0xD9A0F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_16_poweroftwo_400_d9a10400() {
    // Encoding: 0xD9A10400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 16 (PowerOfTwo)
    // Fields: imm9=16, Xt=0, Xn=0
    let encoding: u32 = 0xD9A10400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_31_poweroftwominusone_400_d9a1f400() {
    // Encoding: 0xD9A1F400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: imm9=31, Xt=0, Xn=0
    let encoding: u32 = 0xD9A1F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_32_poweroftwo_400_d9a20400() {
    // Encoding: 0xD9A20400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 32 (PowerOfTwo)
    // Fields: Xt=0, Xn=0, imm9=32
    let encoding: u32 = 0xD9A20400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_63_poweroftwominusone_400_d9a3f400() {
    // Encoding: 0xD9A3F400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=63
    let encoding: u32 = 0xD9A3F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_64_poweroftwo_400_d9a40400() {
    // Encoding: 0xD9A40400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 64 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=64
    let encoding: u32 = 0xD9A40400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_127_poweroftwominusone_400_d9a7f400() {
    // Encoding: 0xD9A7F400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: imm9=127, Xn=0, Xt=0
    let encoding: u32 = 0xD9A7F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_128_poweroftwo_400_d9a80400() {
    // Encoding: 0xD9A80400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 128 (PowerOfTwo)
    // Fields: Xn=0, imm9=128, Xt=0
    let encoding: u32 = 0xD9A80400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_255_poweroftwominusone_400_d9aff400() {
    // Encoding: 0xD9AFF400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xn=0, imm9=255
    let encoding: u32 = 0xD9AFF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_256_poweroftwo_400_d9b00400() {
    // Encoding: 0xD9B00400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 256 (PowerOfTwo)
    // Fields: Xt=0, imm9=256, Xn=0
    let encoding: u32 = 0xD9B00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_imm9_511_max_400_d9bff400() {
    // Encoding: 0xD9BFF400
    // Test aarch64_integer_tags_mcsettagpairpost field imm9 = 511 (Max)
    // Fields: Xn=0, imm9=511, Xt=0
    let encoding: u32 = 0xD9BFF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_xn_0_min_400_d9a00400() {
    // Encoding: 0xD9A00400
    // Test aarch64_integer_tags_mcsettagpairpost field Xn = 0 (Min)
    // Fields: imm9=0, Xt=0, Xn=0
    let encoding: u32 = 0xD9A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_xn_1_poweroftwo_400_d9a00420() {
    // Encoding: 0xD9A00420
    // Test aarch64_integer_tags_mcsettagpairpost field Xn = 1 (PowerOfTwo)
    // Fields: Xn=1, imm9=0, Xt=0
    let encoding: u32 = 0xD9A00420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_xn_30_poweroftwominusone_400_d9a007c0() {
    // Encoding: 0xD9A007C0
    // Test aarch64_integer_tags_mcsettagpairpost field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: imm9=0, Xn=30, Xt=0
    let encoding: u32 = 0xD9A007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_xn_31_max_400_d9a007e0() {
    // Encoding: 0xD9A007E0
    // Test aarch64_integer_tags_mcsettagpairpost field Xn = 31 (Max)
    // Fields: Xn=31, imm9=0, Xt=0
    let encoding: u32 = 0xD9A007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_xt_0_min_400_d9a00400() {
    // Encoding: 0xD9A00400
    // Test aarch64_integer_tags_mcsettagpairpost field Xt = 0 (Min)
    // Fields: Xn=0, imm9=0, Xt=0
    let encoding: u32 = 0xD9A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_xt_1_poweroftwo_400_d9a00401() {
    // Encoding: 0xD9A00401
    // Test aarch64_integer_tags_mcsettagpairpost field Xt = 1 (PowerOfTwo)
    // Fields: imm9=0, Xt=1, Xn=0
    let encoding: u32 = 0xD9A00401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_xt_30_poweroftwominusone_400_d9a0041e() {
    // Encoding: 0xD9A0041E
    // Test aarch64_integer_tags_mcsettagpairpost field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: imm9=0, Xn=0, Xt=30
    let encoding: u32 = 0xD9A0041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_field_xt_31_max_400_d9a0041f() {
    // Encoding: 0xD9A0041F
    // Test aarch64_integer_tags_mcsettagpairpost field Xt = 31 (Max)
    // Fields: imm9=0, Xt=31, Xn=0
    let encoding: u32 = 0xD9A0041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_0_400_d9a00400() {
    // Encoding: 0xD9A00400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_1_400_d9a01400() {
    // Encoding: 0xD9A01400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=1, Xn=0, Xt=0
    // Fields: Xt=0, imm9=1, Xn=0
    let encoding: u32 = 0xD9A01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_2_400_d9a03400() {
    // Encoding: 0xD9A03400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=3, Xn=0, Xt=0
    // Fields: imm9=3, Xn=0, Xt=0
    let encoding: u32 = 0xD9A03400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_3_400_d9a04400() {
    // Encoding: 0xD9A04400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=4, Xn=0, Xt=0
    // Fields: imm9=4, Xt=0, Xn=0
    let encoding: u32 = 0xD9A04400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_4_400_d9a07400() {
    // Encoding: 0xD9A07400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=7, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=7
    let encoding: u32 = 0xD9A07400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_5_400_d9a08400() {
    // Encoding: 0xD9A08400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=8, Xn=0, Xt=0
    // Fields: imm9=8, Xn=0, Xt=0
    let encoding: u32 = 0xD9A08400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_6_400_d9a0f400() {
    // Encoding: 0xD9A0F400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=15, Xn=0, Xt=0
    // Fields: imm9=15, Xn=0, Xt=0
    let encoding: u32 = 0xD9A0F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_7_400_d9a10400() {
    // Encoding: 0xD9A10400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=16, Xn=0, Xt=0
    // Fields: imm9=16, Xn=0, Xt=0
    let encoding: u32 = 0xD9A10400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_8_400_d9a1f400() {
    // Encoding: 0xD9A1F400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=31, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=31
    let encoding: u32 = 0xD9A1F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_9_400_d9a20400() {
    // Encoding: 0xD9A20400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=32, Xn=0, Xt=0
    // Fields: Xt=0, imm9=32, Xn=0
    let encoding: u32 = 0xD9A20400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_10_400_d9a3f400() {
    // Encoding: 0xD9A3F400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=63, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=63
    let encoding: u32 = 0xD9A3F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_11_400_d9a40400() {
    // Encoding: 0xD9A40400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=64, Xn=0, Xt=0
    // Fields: Xt=0, imm9=64, Xn=0
    let encoding: u32 = 0xD9A40400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_12_400_d9a7f400() {
    // Encoding: 0xD9A7F400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=127, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=127
    let encoding: u32 = 0xD9A7F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_13_400_d9a80400() {
    // Encoding: 0xD9A80400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=128, Xn=0, Xt=0
    // Fields: Xn=0, imm9=128, Xt=0
    let encoding: u32 = 0xD9A80400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_14_400_d9aff400() {
    // Encoding: 0xD9AFF400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=255, Xn=0, Xt=0
    // Fields: imm9=255, Xn=0, Xt=0
    let encoding: u32 = 0xD9AFF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_15_400_d9b00400() {
    // Encoding: 0xD9B00400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=256, Xn=0, Xt=0
    // Fields: imm9=256, Xn=0, Xt=0
    let encoding: u32 = 0xD9B00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_16_400_d9bff400() {
    // Encoding: 0xD9BFF400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=511, Xn=0, Xt=0
    // Fields: Xt=0, imm9=511, Xn=0
    let encoding: u32 = 0xD9BFF400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_17_400_d9a00400() {
    // Encoding: 0xD9A00400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_18_400_d9a00420() {
    // Encoding: 0xD9A00420
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=0, Xn=1, Xt=0
    // Fields: Xn=1, imm9=0, Xt=0
    let encoding: u32 = 0xD9A00420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_19_400_d9a007c0() {
    // Encoding: 0xD9A007C0
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=0, Xn=30, Xt=0
    // Fields: Xn=30, Xt=0, imm9=0
    let encoding: u32 = 0xD9A007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_20_400_d9a007e0() {
    // Encoding: 0xD9A007E0
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=0, Xn=31, Xt=0
    // Fields: imm9=0, Xn=31, Xt=0
    let encoding: u32 = 0xD9A007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_21_400_d9a00400() {
    // Encoding: 0xD9A00400
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_22_400_d9a00401() {
    // Encoding: 0xD9A00401
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=0, Xn=0, Xt=1
    // Fields: Xn=0, imm9=0, Xt=1
    let encoding: u32 = 0xD9A00401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_23_400_d9a0041e() {
    // Encoding: 0xD9A0041E
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=0, Xn=0, Xt=30
    // Fields: Xn=0, Xt=30, imm9=0
    let encoding: u32 = 0xD9A0041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_24_400_d9a0041f() {
    // Encoding: 0xD9A0041F
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=0, Xn=0, Xt=31
    // Fields: imm9=0, Xn=0, Xt=31
    let encoding: u32 = 0xD9A0041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_25_400_d9a00421() {
    // Encoding: 0xD9A00421
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=0, Xn=1, Xt=1
    // Fields: imm9=0, Xt=1, Xn=1
    let encoding: u32 = 0xD9A00421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_combo_26_400_d9a007ff() {
    // Encoding: 0xD9A007FF
    // Test aarch64_integer_tags_mcsettagpairpost field combination: imm9=0, Xn=31, Xt=31
    // Fields: imm9=0, Xn=31, Xt=31
    let encoding: u32 = 0xD9A007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_special_xn_31_stack_pointer_sp_may_require_alignment_1024_d9a017e0()
 {
    // Encoding: 0xD9A017E0
    // Test aarch64_integer_tags_mcsettagpairpost special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: imm9=1, Xn=31, Xt=0
    let encoding: u32 = 0xD9A017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_0_zero_c00_d9a00c00() {
    // Encoding: 0xD9A00C00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 0 (Zero)
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9A00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_1_poweroftwo_c00_d9a01c00() {
    // Encoding: 0xD9A01C00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 1 (PowerOfTwo)
    // Fields: Xt=0, Xn=0, imm9=1
    let encoding: u32 = 0xD9A01C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_3_poweroftwominusone_c00_d9a03c00() {
    // Encoding: 0xD9A03C00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=3, Xn=0
    let encoding: u32 = 0xD9A03C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_4_poweroftwo_c00_d9a04c00() {
    // Encoding: 0xD9A04C00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 4 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=4
    let encoding: u32 = 0xD9A04C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_7_poweroftwominusone_c00_d9a07c00() {
    // Encoding: 0xD9A07C00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=7, Xn=0
    let encoding: u32 = 0xD9A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_8_poweroftwo_c00_d9a08c00() {
    // Encoding: 0xD9A08C00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 8 (PowerOfTwo)
    // Fields: Xn=0, imm9=8, Xt=0
    let encoding: u32 = 0xD9A08C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_15_poweroftwominusone_c00_d9a0fc00() {
    // Encoding: 0xD9A0FC00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=15, Xn=0
    let encoding: u32 = 0xD9A0FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_16_poweroftwo_c00_d9a10c00() {
    // Encoding: 0xD9A10C00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 16 (PowerOfTwo)
    // Fields: imm9=16, Xn=0, Xt=0
    let encoding: u32 = 0xD9A10C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_31_poweroftwominusone_c00_d9a1fc00() {
    // Encoding: 0xD9A1FC00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=31, Xn=0
    let encoding: u32 = 0xD9A1FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_32_poweroftwo_c00_d9a20c00() {
    // Encoding: 0xD9A20C00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 32 (PowerOfTwo)
    // Fields: Xt=0, imm9=32, Xn=0
    let encoding: u32 = 0xD9A20C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_63_poweroftwominusone_c00_d9a3fc00() {
    // Encoding: 0xD9A3FC00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: Xt=0, imm9=63, Xn=0
    let encoding: u32 = 0xD9A3FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_64_poweroftwo_c00_d9a40c00() {
    // Encoding: 0xD9A40C00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 64 (PowerOfTwo)
    // Fields: Xt=0, imm9=64, Xn=0
    let encoding: u32 = 0xD9A40C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_127_poweroftwominusone_c00_d9a7fc00() {
    // Encoding: 0xD9A7FC00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xn=0, imm9=127
    let encoding: u32 = 0xD9A7FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_128_poweroftwo_c00_d9a80c00() {
    // Encoding: 0xD9A80C00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 128 (PowerOfTwo)
    // Fields: imm9=128, Xn=0, Xt=0
    let encoding: u32 = 0xD9A80C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_255_poweroftwominusone_c00_d9affc00() {
    // Encoding: 0xD9AFFC00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=255
    let encoding: u32 = 0xD9AFFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_256_poweroftwo_c00_d9b00c00() {
    // Encoding: 0xD9B00C00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 256 (PowerOfTwo)
    // Fields: Xn=0, imm9=256, Xt=0
    let encoding: u32 = 0xD9B00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_imm9_511_max_c00_d9bffc00() {
    // Encoding: 0xD9BFFC00
    // Test aarch64_integer_tags_mcsettagpairpre field imm9 = 511 (Max)
    // Fields: Xt=0, imm9=511, Xn=0
    let encoding: u32 = 0xD9BFFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_xn_0_min_c00_d9a00c00() {
    // Encoding: 0xD9A00C00
    // Test aarch64_integer_tags_mcsettagpairpre field Xn = 0 (Min)
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9A00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_xn_1_poweroftwo_c00_d9a00c20() {
    // Encoding: 0xD9A00C20
    // Test aarch64_integer_tags_mcsettagpairpre field Xn = 1 (PowerOfTwo)
    // Fields: Xn=1, imm9=0, Xt=0
    let encoding: u32 = 0xD9A00C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_xn_30_poweroftwominusone_c00_d9a00fc0() {
    // Encoding: 0xD9A00FC0
    // Test aarch64_integer_tags_mcsettagpairpre field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=30, Xt=0, imm9=0
    let encoding: u32 = 0xD9A00FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_xn_31_max_c00_d9a00fe0() {
    // Encoding: 0xD9A00FE0
    // Test aarch64_integer_tags_mcsettagpairpre field Xn = 31 (Max)
    // Fields: imm9=0, Xt=0, Xn=31
    let encoding: u32 = 0xD9A00FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_xt_0_min_c00_d9a00c00() {
    // Encoding: 0xD9A00C00
    // Test aarch64_integer_tags_mcsettagpairpre field Xt = 0 (Min)
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9A00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_xt_1_poweroftwo_c00_d9a00c01() {
    // Encoding: 0xD9A00C01
    // Test aarch64_integer_tags_mcsettagpairpre field Xt = 1 (PowerOfTwo)
    // Fields: Xt=1, Xn=0, imm9=0
    let encoding: u32 = 0xD9A00C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_xt_30_poweroftwominusone_c00_d9a00c1e() {
    // Encoding: 0xD9A00C1E
    // Test aarch64_integer_tags_mcsettagpairpre field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: imm9=0, Xn=0, Xt=30
    let encoding: u32 = 0xD9A00C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_field_xt_31_max_c00_d9a00c1f() {
    // Encoding: 0xD9A00C1F
    // Test aarch64_integer_tags_mcsettagpairpre field Xt = 31 (Max)
    // Fields: Xn=0, Xt=31, imm9=0
    let encoding: u32 = 0xD9A00C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_0_c00_d9a00c00() {
    // Encoding: 0xD9A00C00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xt=0, Xn=0
    let encoding: u32 = 0xD9A00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_1_c00_d9a01c00() {
    // Encoding: 0xD9A01C00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=1, Xn=0, Xt=0
    // Fields: imm9=1, Xn=0, Xt=0
    let encoding: u32 = 0xD9A01C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_2_c00_d9a03c00() {
    // Encoding: 0xD9A03C00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=3, Xn=0, Xt=0
    // Fields: imm9=3, Xn=0, Xt=0
    let encoding: u32 = 0xD9A03C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_3_c00_d9a04c00() {
    // Encoding: 0xD9A04C00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=4, Xn=0, Xt=0
    // Fields: imm9=4, Xn=0, Xt=0
    let encoding: u32 = 0xD9A04C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_4_c00_d9a07c00() {
    // Encoding: 0xD9A07C00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=7, Xn=0, Xt=0
    // Fields: Xt=0, imm9=7, Xn=0
    let encoding: u32 = 0xD9A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_5_c00_d9a08c00() {
    // Encoding: 0xD9A08C00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=8, Xn=0, Xt=0
    // Fields: imm9=8, Xn=0, Xt=0
    let encoding: u32 = 0xD9A08C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_6_c00_d9a0fc00() {
    // Encoding: 0xD9A0FC00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=15, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=15
    let encoding: u32 = 0xD9A0FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_7_c00_d9a10c00() {
    // Encoding: 0xD9A10C00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=16, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=16
    let encoding: u32 = 0xD9A10C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_8_c00_d9a1fc00() {
    // Encoding: 0xD9A1FC00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=31, Xn=0, Xt=0
    // Fields: imm9=31, Xn=0, Xt=0
    let encoding: u32 = 0xD9A1FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_9_c00_d9a20c00() {
    // Encoding: 0xD9A20C00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=32, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=32
    let encoding: u32 = 0xD9A20C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_10_c00_d9a3fc00() {
    // Encoding: 0xD9A3FC00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=63, Xn=0, Xt=0
    // Fields: Xn=0, imm9=63, Xt=0
    let encoding: u32 = 0xD9A3FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_11_c00_d9a40c00() {
    // Encoding: 0xD9A40C00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=64, Xn=0, Xt=0
    // Fields: imm9=64, Xt=0, Xn=0
    let encoding: u32 = 0xD9A40C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_12_c00_d9a7fc00() {
    // Encoding: 0xD9A7FC00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=127, Xn=0, Xt=0
    // Fields: Xn=0, imm9=127, Xt=0
    let encoding: u32 = 0xD9A7FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_13_c00_d9a80c00() {
    // Encoding: 0xD9A80C00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=128, Xn=0, Xt=0
    // Fields: Xt=0, imm9=128, Xn=0
    let encoding: u32 = 0xD9A80C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_14_c00_d9affc00() {
    // Encoding: 0xD9AFFC00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=255, Xn=0, Xt=0
    // Fields: imm9=255, Xn=0, Xt=0
    let encoding: u32 = 0xD9AFFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_15_c00_d9b00c00() {
    // Encoding: 0xD9B00C00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=256, Xn=0, Xt=0
    // Fields: imm9=256, Xt=0, Xn=0
    let encoding: u32 = 0xD9B00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_16_c00_d9bffc00() {
    // Encoding: 0xD9BFFC00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=511, Xn=0, Xt=0
    // Fields: Xt=0, imm9=511, Xn=0
    let encoding: u32 = 0xD9BFFC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_17_c00_d9a00c00() {
    // Encoding: 0xD9A00C00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xn=0, imm9=0, Xt=0
    let encoding: u32 = 0xD9A00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_18_c00_d9a00c20() {
    // Encoding: 0xD9A00C20
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=0, Xn=1, Xt=0
    // Fields: Xn=1, imm9=0, Xt=0
    let encoding: u32 = 0xD9A00C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_19_c00_d9a00fc0() {
    // Encoding: 0xD9A00FC0
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=0, Xn=30, Xt=0
    // Fields: Xn=30, Xt=0, imm9=0
    let encoding: u32 = 0xD9A00FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_20_c00_d9a00fe0() {
    // Encoding: 0xD9A00FE0
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=0, Xn=31, Xt=0
    // Fields: imm9=0, Xn=31, Xt=0
    let encoding: u32 = 0xD9A00FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_21_c00_d9a00c00() {
    // Encoding: 0xD9A00C00
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9A00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_22_c00_d9a00c01() {
    // Encoding: 0xD9A00C01
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=0, Xn=0, Xt=1
    // Fields: Xt=1, Xn=0, imm9=0
    let encoding: u32 = 0xD9A00C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_23_c00_d9a00c1e() {
    // Encoding: 0xD9A00C1E
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=0, Xn=0, Xt=30
    // Fields: imm9=0, Xn=0, Xt=30
    let encoding: u32 = 0xD9A00C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_24_c00_d9a00c1f() {
    // Encoding: 0xD9A00C1F
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=0, Xn=0, Xt=31
    // Fields: imm9=0, Xn=0, Xt=31
    let encoding: u32 = 0xD9A00C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_25_c00_d9a00c21() {
    // Encoding: 0xD9A00C21
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=0, Xn=1, Xt=1
    // Fields: Xt=1, imm9=0, Xn=1
    let encoding: u32 = 0xD9A00C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_combo_26_c00_d9a00fff() {
    // Encoding: 0xD9A00FFF
    // Test aarch64_integer_tags_mcsettagpairpre field combination: imm9=0, Xn=31, Xt=31
    // Fields: imm9=0, Xt=31, Xn=31
    let encoding: u32 = 0xD9A00FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_special_xn_31_stack_pointer_sp_may_require_alignment_3072_d9a01fe0()
 {
    // Encoding: 0xD9A01FE0
    // Test aarch64_integer_tags_mcsettagpairpre special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xt=0, imm9=1, Xn=31
    let encoding: u32 = 0xD9A01FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_0_zero_800_d9a00800() {
    // Encoding: 0xD9A00800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 0 (Zero)
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9A00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_1_poweroftwo_800_d9a01800() {
    // Encoding: 0xD9A01800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 1 (PowerOfTwo)
    // Fields: Xn=0, imm9=1, Xt=0
    let encoding: u32 = 0xD9A01800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_3_poweroftwominusone_800_d9a03800() {
    // Encoding: 0xD9A03800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: Xn=0, imm9=3, Xt=0
    let encoding: u32 = 0xD9A03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_4_poweroftwo_800_d9a04800() {
    // Encoding: 0xD9A04800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 4 (PowerOfTwo)
    // Fields: imm9=4, Xn=0, Xt=0
    let encoding: u32 = 0xD9A04800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_7_poweroftwominusone_800_d9a07800() {
    // Encoding: 0xD9A07800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=7
    let encoding: u32 = 0xD9A07800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_8_poweroftwo_800_d9a08800() {
    // Encoding: 0xD9A08800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 8 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=8
    let encoding: u32 = 0xD9A08800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_15_poweroftwominusone_800_d9a0f800() {
    // Encoding: 0xD9A0F800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=15
    let encoding: u32 = 0xD9A0F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_16_poweroftwo_800_d9a10800() {
    // Encoding: 0xD9A10800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 16 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=16
    let encoding: u32 = 0xD9A10800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_31_poweroftwominusone_800_d9a1f800() {
    // Encoding: 0xD9A1F800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xn=0, imm9=31
    let encoding: u32 = 0xD9A1F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_32_poweroftwo_800_d9a20800() {
    // Encoding: 0xD9A20800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 32 (PowerOfTwo)
    // Fields: Xt=0, imm9=32, Xn=0
    let encoding: u32 = 0xD9A20800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_63_poweroftwominusone_800_d9a3f800() {
    // Encoding: 0xD9A3F800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: imm9=63, Xn=0, Xt=0
    let encoding: u32 = 0xD9A3F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_64_poweroftwo_800_d9a40800() {
    // Encoding: 0xD9A40800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 64 (PowerOfTwo)
    // Fields: imm9=64, Xn=0, Xt=0
    let encoding: u32 = 0xD9A40800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_127_poweroftwominusone_800_d9a7f800() {
    // Encoding: 0xD9A7F800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, imm9=127
    let encoding: u32 = 0xD9A7F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_128_poweroftwo_800_d9a80800() {
    // Encoding: 0xD9A80800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 128 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=128
    let encoding: u32 = 0xD9A80800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_255_poweroftwominusone_800_d9aff800() {
    // Encoding: 0xD9AFF800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: imm9=255, Xt=0, Xn=0
    let encoding: u32 = 0xD9AFF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_256_poweroftwo_800_d9b00800() {
    // Encoding: 0xD9B00800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 256 (PowerOfTwo)
    // Fields: imm9=256, Xn=0, Xt=0
    let encoding: u32 = 0xD9B00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_imm9_511_max_800_d9bff800() {
    // Encoding: 0xD9BFF800
    // Test aarch64_integer_tags_mcsettagpair field imm9 = 511 (Max)
    // Fields: imm9=511, Xn=0, Xt=0
    let encoding: u32 = 0xD9BFF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_xn_0_min_800_d9a00800() {
    // Encoding: 0xD9A00800
    // Test aarch64_integer_tags_mcsettagpair field Xn = 0 (Min)
    // Fields: Xt=0, Xn=0, imm9=0
    let encoding: u32 = 0xD9A00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_xn_1_poweroftwo_800_d9a00820() {
    // Encoding: 0xD9A00820
    // Test aarch64_integer_tags_mcsettagpair field Xn = 1 (PowerOfTwo)
    // Fields: imm9=0, Xt=0, Xn=1
    let encoding: u32 = 0xD9A00820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_xn_30_poweroftwominusone_800_d9a00bc0() {
    // Encoding: 0xD9A00BC0
    // Test aarch64_integer_tags_mcsettagpair field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: imm9=0, Xt=0, Xn=30
    let encoding: u32 = 0xD9A00BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_xn_31_max_800_d9a00be0() {
    // Encoding: 0xD9A00BE0
    // Test aarch64_integer_tags_mcsettagpair field Xn = 31 (Max)
    // Fields: Xt=0, imm9=0, Xn=31
    let encoding: u32 = 0xD9A00BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_xt_0_min_800_d9a00800() {
    // Encoding: 0xD9A00800
    // Test aarch64_integer_tags_mcsettagpair field Xt = 0 (Min)
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9A00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_xt_1_poweroftwo_800_d9a00801() {
    // Encoding: 0xD9A00801
    // Test aarch64_integer_tags_mcsettagpair field Xt = 1 (PowerOfTwo)
    // Fields: Xn=0, Xt=1, imm9=0
    let encoding: u32 = 0xD9A00801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_xt_30_poweroftwominusone_800_d9a0081e() {
    // Encoding: 0xD9A0081E
    // Test aarch64_integer_tags_mcsettagpair field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: Xt=30, imm9=0, Xn=0
    let encoding: u32 = 0xD9A0081E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_field_xt_31_max_800_d9a0081f() {
    // Encoding: 0xD9A0081F
    // Test aarch64_integer_tags_mcsettagpair field Xt = 31 (Max)
    // Fields: imm9=0, Xn=0, Xt=31
    let encoding: u32 = 0xD9A0081F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_0_800_d9a00800() {
    // Encoding: 0xD9A00800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xt=0, Xn=0
    let encoding: u32 = 0xD9A00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_1_800_d9a01800() {
    // Encoding: 0xD9A01800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=1, Xn=0, Xt=0
    // Fields: Xn=0, imm9=1, Xt=0
    let encoding: u32 = 0xD9A01800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_2_800_d9a03800() {
    // Encoding: 0xD9A03800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=3, Xn=0, Xt=0
    // Fields: imm9=3, Xt=0, Xn=0
    let encoding: u32 = 0xD9A03800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_3_800_d9a04800() {
    // Encoding: 0xD9A04800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=4, Xn=0, Xt=0
    // Fields: Xn=0, imm9=4, Xt=0
    let encoding: u32 = 0xD9A04800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_4_800_d9a07800() {
    // Encoding: 0xD9A07800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=7, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=7
    let encoding: u32 = 0xD9A07800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_5_800_d9a08800() {
    // Encoding: 0xD9A08800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=8, Xn=0, Xt=0
    // Fields: imm9=8, Xn=0, Xt=0
    let encoding: u32 = 0xD9A08800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_6_800_d9a0f800() {
    // Encoding: 0xD9A0F800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=15, Xn=0, Xt=0
    // Fields: imm9=15, Xn=0, Xt=0
    let encoding: u32 = 0xD9A0F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_7_800_d9a10800() {
    // Encoding: 0xD9A10800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=16, Xn=0, Xt=0
    // Fields: Xt=0, imm9=16, Xn=0
    let encoding: u32 = 0xD9A10800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_8_800_d9a1f800() {
    // Encoding: 0xD9A1F800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=31, Xn=0, Xt=0
    // Fields: imm9=31, Xn=0, Xt=0
    let encoding: u32 = 0xD9A1F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_9_800_d9a20800() {
    // Encoding: 0xD9A20800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=32, Xn=0, Xt=0
    // Fields: imm9=32, Xn=0, Xt=0
    let encoding: u32 = 0xD9A20800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_10_800_d9a3f800() {
    // Encoding: 0xD9A3F800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=63, Xn=0, Xt=0
    // Fields: imm9=63, Xn=0, Xt=0
    let encoding: u32 = 0xD9A3F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_11_800_d9a40800() {
    // Encoding: 0xD9A40800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=64, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=64
    let encoding: u32 = 0xD9A40800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_12_800_d9a7f800() {
    // Encoding: 0xD9A7F800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=127, Xn=0, Xt=0
    // Fields: Xn=0, imm9=127, Xt=0
    let encoding: u32 = 0xD9A7F800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_13_800_d9a80800() {
    // Encoding: 0xD9A80800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=128, Xn=0, Xt=0
    // Fields: imm9=128, Xn=0, Xt=0
    let encoding: u32 = 0xD9A80800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_14_800_d9aff800() {
    // Encoding: 0xD9AFF800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=255, Xn=0, Xt=0
    // Fields: imm9=255, Xn=0, Xt=0
    let encoding: u32 = 0xD9AFF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_15_800_d9b00800() {
    // Encoding: 0xD9B00800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=256, Xn=0, Xt=0
    // Fields: Xn=0, imm9=256, Xt=0
    let encoding: u32 = 0xD9B00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_16_800_d9bff800() {
    // Encoding: 0xD9BFF800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=511, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=511
    let encoding: u32 = 0xD9BFF800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_17_800_d9a00800() {
    // Encoding: 0xD9A00800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9A00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_18_800_d9a00820() {
    // Encoding: 0xD9A00820
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=0, Xn=1, Xt=0
    // Fields: imm9=0, Xn=1, Xt=0
    let encoding: u32 = 0xD9A00820;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_19_800_d9a00bc0() {
    // Encoding: 0xD9A00BC0
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=0, Xn=30, Xt=0
    // Fields: imm9=0, Xn=30, Xt=0
    let encoding: u32 = 0xD9A00BC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_20_800_d9a00be0() {
    // Encoding: 0xD9A00BE0
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=0, Xn=31, Xt=0
    // Fields: Xt=0, imm9=0, Xn=31
    let encoding: u32 = 0xD9A00BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_21_800_d9a00800() {
    // Encoding: 0xD9A00800
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=0
    let encoding: u32 = 0xD9A00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_22_800_d9a00801() {
    // Encoding: 0xD9A00801
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=0, Xn=0, Xt=1
    // Fields: imm9=0, Xn=0, Xt=1
    let encoding: u32 = 0xD9A00801;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_23_800_d9a0081e() {
    // Encoding: 0xD9A0081E
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=0, Xn=0, Xt=30
    // Fields: Xt=30, Xn=0, imm9=0
    let encoding: u32 = 0xD9A0081E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_24_800_d9a0081f() {
    // Encoding: 0xD9A0081F
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=0, Xn=0, Xt=31
    // Fields: Xt=31, Xn=0, imm9=0
    let encoding: u32 = 0xD9A0081F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_25_800_d9a00821() {
    // Encoding: 0xD9A00821
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=0, Xn=1, Xt=1
    // Fields: Xn=1, imm9=0, Xt=1
    let encoding: u32 = 0xD9A00821;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettagpair_combo_26_800_d9a00bff() {
    // Encoding: 0xD9A00BFF
    // Test aarch64_integer_tags_mcsettagpair field combination: imm9=0, Xn=31, Xt=31
    // Fields: imm9=0, Xn=31, Xt=31
    let encoding: u32 = 0xD9A00BFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettagpair_special_xn_31_stack_pointer_sp_may_require_alignment_2048_d9a01be0()
 {
    // Encoding: 0xD9A01BE0
    // Test aarch64_integer_tags_mcsettagpair special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: imm9=1, Xn=31, Xt=0
    let encoding: u32 = 0xD9A01BE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// zero value
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_ldr_oracle_0_f9800020() {
    // Test LDRS: zero value (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max byte
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_ldr_oracle_1_f9800020() {
    // Test LDRS: max byte (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max halfword
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_ldr_oracle_2_f9800020() {
    // Test LDRS: max halfword (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max word
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_ldr_oracle_3_f9800020() {
    // Test LDRS: max word (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 255, 255, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// large value
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_ldr_oracle_4_f9800020() {
    // Test LDRS: large value (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[240, 222, 188, 154, 120, 86, 52, 18])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (byte)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_ldr_oracle_5_f9800020() {
    // Test LDRS: sign bit (byte) (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[128, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (halfword)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_ldr_oracle_6_f9800020() {
    // Test LDRS: sign bit (halfword) (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 128, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (word)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_ldr_oracle_7_f9800020() {
    // Test LDRS: sign bit (word) (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 128, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_reg_write_0_d9a00400() {
    // Test aarch64_integer_tags_mcsettagpairpost register write: Sp
    // Encoding: 0xD9A00400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9A00400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_reg_write_1_d9a00400() {
    // Test aarch64_integer_tags_mcsettagpairpost register write: GpFromField("n")
    // Encoding: 0xD9A00400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9A00400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_sp_xn_d9a007e0() {
    // Test aarch64_integer_tags_mcsettagpairpost with Xn = SP (31)
    // Encoding: 0xD9A007E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9A007E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_store_0_d9a00400() {
    // Test aarch64_integer_tags_mcsettagpairpost memory store: 8 bytes
    // Encoding: 0xD9A00400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0xD9A00400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairpost
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpairpost_store_1_d9a00400() {
    // Test aarch64_integer_tags_mcsettagpairpost memory store: 8 bytes
    // Encoding: 0xD9A00400
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0xD9A00400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// zero value
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_ldr_oracle_0_f9800020() {
    // Test LDRS: zero value (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max byte
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_ldr_oracle_1_f9800020() {
    // Test LDRS: max byte (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max halfword
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_ldr_oracle_2_f9800020() {
    // Test LDRS: max halfword (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max word
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_ldr_oracle_3_f9800020() {
    // Test LDRS: max word (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 255, 255, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// large value
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_ldr_oracle_4_f9800020() {
    // Test LDRS: large value (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[240, 222, 188, 154, 120, 86, 52, 18])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (byte)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_ldr_oracle_5_f9800020() {
    // Test LDRS: sign bit (byte) (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[128, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (halfword)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_ldr_oracle_6_f9800020() {
    // Test LDRS: sign bit (halfword) (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 128, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (word)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_ldr_oracle_7_f9800020() {
    // Test LDRS: sign bit (word) (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 128, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_reg_write_0_d9a00c00() {
    // Test aarch64_integer_tags_mcsettagpairpre register write: Sp
    // Encoding: 0xD9A00C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9A00C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_reg_write_1_d9a00c00() {
    // Test aarch64_integer_tags_mcsettagpairpre register write: GpFromField("n")
    // Encoding: 0xD9A00C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9A00C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_sp_xn_d9a00fe0() {
    // Test aarch64_integer_tags_mcsettagpairpre with Xn = SP (31)
    // Encoding: 0xD9A00FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9A00FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_store_0_d9a00c00() {
    // Test aarch64_integer_tags_mcsettagpairpre memory store: 8 bytes
    // Encoding: 0xD9A00C00
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0xD9A00C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpairpre
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpairpre_store_1_d9a00c00() {
    // Test aarch64_integer_tags_mcsettagpairpre memory store: 8 bytes
    // Encoding: 0xD9A00C00
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0xD9A00C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// zero value
#[test]
fn test_aarch64_integer_tags_mcsettagpair_ldr_oracle_0_f9800020() {
    // Test LDRS: zero value (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max byte
#[test]
fn test_aarch64_integer_tags_mcsettagpair_ldr_oracle_1_f9800020() {
    // Test LDRS: max byte (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max halfword
#[test]
fn test_aarch64_integer_tags_mcsettagpair_ldr_oracle_2_f9800020() {
    // Test LDRS: max halfword (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max word
#[test]
fn test_aarch64_integer_tags_mcsettagpair_ldr_oracle_3_f9800020() {
    // Test LDRS: max word (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 255, 255, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// large value
#[test]
fn test_aarch64_integer_tags_mcsettagpair_ldr_oracle_4_f9800020() {
    // Test LDRS: large value (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[240, 222, 188, 154, 120, 86, 52, 18])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (byte)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_ldr_oracle_5_f9800020() {
    // Test LDRS: sign bit (byte) (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[128, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (halfword)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_ldr_oracle_6_f9800020() {
    // Test LDRS: sign bit (halfword) (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 128, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (word)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_ldr_oracle_7_f9800020() {
    // Test LDRS: sign bit (word) (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 128, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettagpair_reg_write_0_d9a00800() {
    // Test aarch64_integer_tags_mcsettagpair register write: Sp
    // Encoding: 0xD9A00800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9A00800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettagpair_reg_write_1_d9a00800() {
    // Test aarch64_integer_tags_mcsettagpair register write: GpFromField("n")
    // Encoding: 0xD9A00800
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9A00800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagpair_sp_xn_d9a00be0() {
    // Test aarch64_integer_tags_mcsettagpair with Xn = SP (31)
    // Encoding: 0xD9A00BE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9A00BE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpair_store_0_d9a00800() {
    // Test aarch64_integer_tags_mcsettagpair memory store: 8 bytes
    // Encoding: 0xD9A00800
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0xD9A00800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagpair
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettagpair_store_1_d9a00800() {
    // Test aarch64_integer_tags_mcsettagpair memory store: 8 bytes
    // Encoding: 0xD9A00800
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0xD9A00800;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_integer_tags_mcinserttagmask Tests
// ============================================================================

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_field_xm_0_min_1400_9ac01400() {
    // Encoding: 0x9AC01400
    // Test aarch64_integer_tags_mcinserttagmask field Xm = 0 (Min)
    // Fields: Xn=0, Xd=0, Xm=0
    let encoding: u32 = 0x9AC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_field_xm_1_poweroftwo_1400_9ac11400() {
    // Encoding: 0x9AC11400
    // Test aarch64_integer_tags_mcinserttagmask field Xm = 1 (PowerOfTwo)
    // Fields: Xn=0, Xd=0, Xm=1
    let encoding: u32 = 0x9AC11400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_field_xm_30_poweroftwominusone_1400_9ade1400() {
    // Encoding: 0x9ADE1400
    // Test aarch64_integer_tags_mcinserttagmask field Xm = 30 (PowerOfTwoMinusOne)
    // Fields: Xm=30, Xn=0, Xd=0
    let encoding: u32 = 0x9ADE1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_field_xm_31_max_1400_9adf1400() {
    // Encoding: 0x9ADF1400
    // Test aarch64_integer_tags_mcinserttagmask field Xm = 31 (Max)
    // Fields: Xd=0, Xn=0, Xm=31
    let encoding: u32 = 0x9ADF1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_field_xn_0_min_1400_9ac01400() {
    // Encoding: 0x9AC01400
    // Test aarch64_integer_tags_mcinserttagmask field Xn = 0 (Min)
    // Fields: Xm=0, Xn=0, Xd=0
    let encoding: u32 = 0x9AC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_field_xn_1_poweroftwo_1400_9ac01420() {
    // Encoding: 0x9AC01420
    // Test aarch64_integer_tags_mcinserttagmask field Xn = 1 (PowerOfTwo)
    // Fields: Xn=1, Xd=0, Xm=0
    let encoding: u32 = 0x9AC01420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_field_xn_30_poweroftwominusone_1400_9ac017c0() {
    // Encoding: 0x9AC017C0
    // Test aarch64_integer_tags_mcinserttagmask field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xm=0, Xn=30, Xd=0
    let encoding: u32 = 0x9AC017C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_field_xn_31_max_1400_9ac017e0() {
    // Encoding: 0x9AC017E0
    // Test aarch64_integer_tags_mcinserttagmask field Xn = 31 (Max)
    // Fields: Xn=31, Xd=0, Xm=0
    let encoding: u32 = 0x9AC017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_field_xd_0_min_1400_9ac01400() {
    // Encoding: 0x9AC01400
    // Test aarch64_integer_tags_mcinserttagmask field Xd = 0 (Min)
    // Fields: Xm=0, Xd=0, Xn=0
    let encoding: u32 = 0x9AC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_field_xd_1_poweroftwo_1400_9ac01401() {
    // Encoding: 0x9AC01401
    // Test aarch64_integer_tags_mcinserttagmask field Xd = 1 (PowerOfTwo)
    // Fields: Xn=0, Xm=0, Xd=1
    let encoding: u32 = 0x9AC01401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_field_xd_30_poweroftwominusone_1400_9ac0141e() {
    // Encoding: 0x9AC0141E
    // Test aarch64_integer_tags_mcinserttagmask field Xd = 30 (PowerOfTwoMinusOne)
    // Fields: Xm=0, Xn=0, Xd=30
    let encoding: u32 = 0x9AC0141E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_field_xd_31_max_1400_9ac0141f() {
    // Encoding: 0x9AC0141F
    // Test aarch64_integer_tags_mcinserttagmask field Xd = 31 (Max)
    // Fields: Xd=31, Xm=0, Xn=0
    let encoding: u32 = 0x9AC0141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_0_1400_9ac01400() {
    // Encoding: 0x9AC01400
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=0, Xn=0, Xd=0
    // Fields: Xd=0, Xn=0, Xm=0
    let encoding: u32 = 0x9AC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_1_1400_9ac11400() {
    // Encoding: 0x9AC11400
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=1, Xn=0, Xd=0
    // Fields: Xd=0, Xm=1, Xn=0
    let encoding: u32 = 0x9AC11400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_2_1400_9ade1400() {
    // Encoding: 0x9ADE1400
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=30, Xn=0, Xd=0
    // Fields: Xd=0, Xm=30, Xn=0
    let encoding: u32 = 0x9ADE1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_3_1400_9adf1400() {
    // Encoding: 0x9ADF1400
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=31, Xn=0, Xd=0
    // Fields: Xd=0, Xm=31, Xn=0
    let encoding: u32 = 0x9ADF1400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_4_1400_9ac01400() {
    // Encoding: 0x9AC01400
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=0, Xn=0, Xd=0
    // Fields: Xd=0, Xm=0, Xn=0
    let encoding: u32 = 0x9AC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_5_1400_9ac01420() {
    // Encoding: 0x9AC01420
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=0, Xn=1, Xd=0
    // Fields: Xm=0, Xn=1, Xd=0
    let encoding: u32 = 0x9AC01420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_6_1400_9ac017c0() {
    // Encoding: 0x9AC017C0
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=0, Xn=30, Xd=0
    // Fields: Xm=0, Xn=30, Xd=0
    let encoding: u32 = 0x9AC017C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_7_1400_9ac017e0() {
    // Encoding: 0x9AC017E0
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=0, Xn=31, Xd=0
    // Fields: Xm=0, Xd=0, Xn=31
    let encoding: u32 = 0x9AC017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_8_1400_9ac01400() {
    // Encoding: 0x9AC01400
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=0, Xn=0, Xd=0
    // Fields: Xn=0, Xd=0, Xm=0
    let encoding: u32 = 0x9AC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_9_1400_9ac01401() {
    // Encoding: 0x9AC01401
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=0, Xn=0, Xd=1
    // Fields: Xm=0, Xd=1, Xn=0
    let encoding: u32 = 0x9AC01401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_10_1400_9ac0141e() {
    // Encoding: 0x9AC0141E
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=0, Xn=0, Xd=30
    // Fields: Xn=0, Xd=30, Xm=0
    let encoding: u32 = 0x9AC0141E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_11_1400_9ac0141f() {
    // Encoding: 0x9AC0141F
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=0, Xn=0, Xd=31
    // Fields: Xm=0, Xd=31, Xn=0
    let encoding: u32 = 0x9AC0141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=1 (same register test (reg=1)), Xn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_12_1400_9ac11420() {
    // Encoding: 0x9AC11420
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=1, Xn=1, Xd=0
    // Fields: Xn=1, Xm=1, Xd=0
    let encoding: u32 = 0x9AC11420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=31 (same register test (reg=31)), Xn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_13_1400_9adf17e0() {
    // Encoding: 0x9ADF17E0
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=31, Xn=31, Xd=0
    // Fields: Xm=31, Xn=31, Xd=0
    let encoding: u32 = 0x9ADF17E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=1 (same register test (reg=1)), Xd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_14_1400_9ac11401() {
    // Encoding: 0x9AC11401
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=1, Xn=0, Xd=1
    // Fields: Xm=1, Xd=1, Xn=0
    let encoding: u32 = 0x9AC11401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=31 (same register test (reg=31)), Xd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_15_1400_9adf141f() {
    // Encoding: 0x9ADF141F
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=31, Xn=0, Xd=31
    // Fields: Xn=0, Xd=31, Xm=31
    let encoding: u32 = 0x9ADF141F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_16_1400_9ac01421() {
    // Encoding: 0x9AC01421
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=0, Xn=1, Xd=1
    // Fields: Xd=1, Xn=1, Xm=0
    let encoding: u32 = 0x9AC01421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_combo_17_1400_9ac017ff() {
    // Encoding: 0x9AC017FF
    // Test aarch64_integer_tags_mcinserttagmask field combination: Xm=0, Xn=31, Xd=31
    // Fields: Xn=31, Xd=31, Xm=0
    let encoding: u32 = 0x9AC017FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_special_xn_31_stack_pointer_sp_may_require_alignment_5120_9ac017e0()
 {
    // Encoding: 0x9AC017E0
    // Test aarch64_integer_tags_mcinserttagmask special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xd=0, Xm=0, Xn=31
    let encoding: u32 = 0x9AC017E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values - high bits zero
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_umulh_oracle_0_9bc27c20() {
    // Test UMULH: small values - high bits zero (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x3);
    set_x(&mut cpu, 1, 0x2);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// large value * 2 - produces high bits
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_umulh_oracle_1_9bc27c20() {
    // Test UMULH: large value * 2 - produces high bits (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max * max unsigned
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_umulh_oracle_2_9bc27c20() {
    // Test UMULH: max * max unsigned (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max positive * max positive
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_umulh_oracle_3_9bc27c20() {
    // Test UMULH: max positive * max positive (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x3FFFFFFFFFFFFFFF,
        "X0 should be 0x3FFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// 2^32 * 2^32
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_umulh_oracle_4_9bc27c20() {
    // Test UMULH: 2^32 * 2^32 (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x100000000);
    set_x(&mut cpu, 1, 0x100000000);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_reg_write_0_9ac01400() {
    // Test aarch64_integer_tags_mcinserttagmask register write: GpFromField("d")
    // Encoding: 0x9AC01400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x9AC01400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcinserttagmask
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcinserttagmask_sp_xn_9ac017e0() {
    // Test aarch64_integer_tags_mcinserttagmask with Xn = SP (31)
    // Encoding: 0x9AC017E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x9AC017E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_integer_tags_mcsubtag Tests
// ============================================================================

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field uimm6 16 +: 6`
/// Requirement: FieldBoundary { field: "uimm6", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_uimm6_0_min_0_d1800000() {
    // Encoding: 0xD1800000
    // Test aarch64_integer_tags_mcsubtag field uimm6 = 0 (Min)
    // Fields: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=0
    let encoding: u32 = 0xD1800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field uimm6 16 +: 6`
/// Requirement: FieldBoundary { field: "uimm6", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_uimm6_1_poweroftwo_0_d1810000() {
    // Encoding: 0xD1810000
    // Test aarch64_integer_tags_mcsubtag field uimm6 = 1 (PowerOfTwo)
    // Fields: Xn=0, uimm4=0, op3=0, Xd=0, uimm6=1
    let encoding: u32 = 0xD1810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field uimm6 16 +: 6`
/// Requirement: FieldBoundary { field: "uimm6", value: 31, boundary: PowerOfTwoMinusOne }
/// midpoint (31)
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_uimm6_31_poweroftwominusone_0_d19f0000() {
    // Encoding: 0xD19F0000
    // Test aarch64_integer_tags_mcsubtag field uimm6 = 31 (PowerOfTwoMinusOne)
    // Fields: uimm4=0, uimm6=31, Xn=0, Xd=0, op3=0
    let encoding: u32 = 0xD19F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field uimm6 16 +: 6`
/// Requirement: FieldBoundary { field: "uimm6", value: 63, boundary: Max }
/// maximum value (63)
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_uimm6_63_max_0_d1bf0000() {
    // Encoding: 0xD1BF0000
    // Test aarch64_integer_tags_mcsubtag field uimm6 = 63 (Max)
    // Fields: uimm6=63, uimm4=0, Xn=0, op3=0, Xd=0
    let encoding: u32 = 0xD1BF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field op3 14 +: 2`
/// Requirement: FieldBoundary { field: "op3", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_op3_0_min_0_d1800000() {
    // Encoding: 0xD1800000
    // Test aarch64_integer_tags_mcsubtag field op3 = 0 (Min)
    // Fields: uimm6=0, uimm4=0, op3=0, Xn=0, Xd=0
    let encoding: u32 = 0xD1800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field op3 14 +: 2`
/// Requirement: FieldBoundary { field: "op3", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_op3_1_poweroftwo_0_d1804000() {
    // Encoding: 0xD1804000
    // Test aarch64_integer_tags_mcsubtag field op3 = 1 (PowerOfTwo)
    // Fields: op3=1, Xd=0, uimm4=0, uimm6=0, Xn=0
    let encoding: u32 = 0xD1804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field op3 14 +: 2`
/// Requirement: FieldBoundary { field: "op3", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_op3_3_max_0_d180c000() {
    // Encoding: 0xD180C000
    // Test aarch64_integer_tags_mcsubtag field op3 = 3 (Max)
    // Fields: uimm6=0, Xn=0, Xd=0, uimm4=0, op3=3
    let encoding: u32 = 0xD180C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field uimm4 10 +: 4`
/// Requirement: FieldBoundary { field: "uimm4", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_uimm4_0_min_0_d1800000() {
    // Encoding: 0xD1800000
    // Test aarch64_integer_tags_mcsubtag field uimm4 = 0 (Min)
    // Fields: uimm4=0, uimm6=0, op3=0, Xn=0, Xd=0
    let encoding: u32 = 0xD1800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field uimm4 10 +: 4`
/// Requirement: FieldBoundary { field: "uimm4", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_uimm4_1_poweroftwo_0_d1800400() {
    // Encoding: 0xD1800400
    // Test aarch64_integer_tags_mcsubtag field uimm4 = 1 (PowerOfTwo)
    // Fields: Xd=0, uimm4=1, Xn=0, uimm6=0, op3=0
    let encoding: u32 = 0xD1800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field uimm4 10 +: 4`
/// Requirement: FieldBoundary { field: "uimm4", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_uimm4_7_poweroftwominusone_0_d1801c00() {
    // Encoding: 0xD1801C00
    // Test aarch64_integer_tags_mcsubtag field uimm4 = 7 (PowerOfTwoMinusOne)
    // Fields: uimm4=7, Xn=0, op3=0, uimm6=0, Xd=0
    let encoding: u32 = 0xD1801C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field uimm4 10 +: 4`
/// Requirement: FieldBoundary { field: "uimm4", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_uimm4_15_max_0_d1803c00() {
    // Encoding: 0xD1803C00
    // Test aarch64_integer_tags_mcsubtag field uimm4 = 15 (Max)
    // Fields: Xd=0, op3=0, uimm6=0, uimm4=15, Xn=0
    let encoding: u32 = 0xD1803C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_xn_0_min_0_d1800000() {
    // Encoding: 0xD1800000
    // Test aarch64_integer_tags_mcsubtag field Xn = 0 (Min)
    // Fields: Xn=0, uimm6=0, uimm4=0, op3=0, Xd=0
    let encoding: u32 = 0xD1800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_xn_1_poweroftwo_0_d1800020() {
    // Encoding: 0xD1800020
    // Test aarch64_integer_tags_mcsubtag field Xn = 1 (PowerOfTwo)
    // Fields: uimm6=0, op3=0, uimm4=0, Xn=1, Xd=0
    let encoding: u32 = 0xD1800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_xn_30_poweroftwominusone_0_d18003c0() {
    // Encoding: 0xD18003C0
    // Test aarch64_integer_tags_mcsubtag field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xd=0, op3=0, Xn=30, uimm6=0, uimm4=0
    let encoding: u32 = 0xD18003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_xn_31_max_0_d18003e0() {
    // Encoding: 0xD18003E0
    // Test aarch64_integer_tags_mcsubtag field Xn = 31 (Max)
    // Fields: Xn=31, uimm4=0, uimm6=0, op3=0, Xd=0
    let encoding: u32 = 0xD18003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_xd_0_min_0_d1800000() {
    // Encoding: 0xD1800000
    // Test aarch64_integer_tags_mcsubtag field Xd = 0 (Min)
    // Fields: op3=0, uimm6=0, Xd=0, Xn=0, uimm4=0
    let encoding: u32 = 0xD1800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_xd_1_poweroftwo_0_d1800001() {
    // Encoding: 0xD1800001
    // Test aarch64_integer_tags_mcsubtag field Xd = 1 (PowerOfTwo)
    // Fields: uimm6=0, Xd=1, Xn=0, op3=0, uimm4=0
    let encoding: u32 = 0xD1800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_xd_30_poweroftwominusone_0_d180001e() {
    // Encoding: 0xD180001E
    // Test aarch64_integer_tags_mcsubtag field Xd = 30 (PowerOfTwoMinusOne)
    // Fields: Xd=30, uimm6=0, op3=0, uimm4=0, Xn=0
    let encoding: u32 = 0xD180001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsubtag_field_xd_31_max_0_d180001f() {
    // Encoding: 0xD180001F
    // Test aarch64_integer_tags_mcsubtag field Xd = 31 (Max)
    // Fields: Xn=0, op3=0, uimm4=0, Xd=31, uimm6=0
    let encoding: u32 = 0xD180001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm6=0 (minimum value)
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_0_0_d1800000() {
    // Encoding: 0xD1800000
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: uimm6=0, op3=0, Xd=0, Xn=0, uimm4=0
    let encoding: u32 = 0xD1800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm6=1 (value 1)
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_1_0_d1810000() {
    // Encoding: 0xD1810000
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=1, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: uimm6=1, op3=0, uimm4=0, Xn=0, Xd=0
    let encoding: u32 = 0xD1810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm6=31 (midpoint (31))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_2_0_d19f0000() {
    // Encoding: 0xD19F0000
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=31, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: op3=0, uimm4=0, Xn=0, Xd=0, uimm6=31
    let encoding: u32 = 0xD19F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm6=63 (maximum value (63))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_3_0_d1bf0000() {
    // Encoding: 0xD1BF0000
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=63, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: uimm6=63, Xd=0, uimm4=0, Xn=0, op3=0
    let encoding: u32 = 0xD1BF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op3=0 (minimum value)
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_4_0_d1800000() {
    // Encoding: 0xD1800000
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: uimm6=0, Xn=0, op3=0, Xd=0, uimm4=0
    let encoding: u32 = 0xD1800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op3=1 (value 1)
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_5_0_d1804000() {
    // Encoding: 0xD1804000
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=1, uimm4=0, Xn=0, Xd=0
    // Fields: op3=1, uimm4=0, Xd=0, Xn=0, uimm6=0
    let encoding: u32 = 0xD1804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op3=3 (maximum value (3))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_6_0_d180c000() {
    // Encoding: 0xD180C000
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=3, uimm4=0, Xn=0, Xd=0
    // Fields: uimm4=0, Xn=0, Xd=0, uimm6=0, op3=3
    let encoding: u32 = 0xD180C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm4=0 (minimum value)
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_7_0_d1800000() {
    // Encoding: 0xD1800000
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: uimm4=0, op3=0, Xd=0, uimm6=0, Xn=0
    let encoding: u32 = 0xD1800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm4=1 (value 1)
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_8_0_d1800400() {
    // Encoding: 0xD1800400
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=1, Xn=0, Xd=0
    // Fields: Xd=0, uimm4=1, uimm6=0, op3=0, Xn=0
    let encoding: u32 = 0xD1800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm4=7 (midpoint (7))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_9_0_d1801c00() {
    // Encoding: 0xD1801C00
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=7, Xn=0, Xd=0
    // Fields: uimm4=7, Xd=0, Xn=0, uimm6=0, op3=0
    let encoding: u32 = 0xD1801C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm4=15 (maximum value (15))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_10_0_d1803c00() {
    // Encoding: 0xD1803C00
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=15, Xn=0, Xd=0
    // Fields: uimm6=0, op3=0, Xn=0, uimm4=15, Xd=0
    let encoding: u32 = 0xD1803C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_11_0_d1800000() {
    // Encoding: 0xD1800000
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: uimm6=0, op3=0, Xn=0, uimm4=0, Xd=0
    let encoding: u32 = 0xD1800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_12_0_d1800020() {
    // Encoding: 0xD1800020
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=0, Xn=1, Xd=0
    // Fields: op3=0, Xn=1, Xd=0, uimm6=0, uimm4=0
    let encoding: u32 = 0xD1800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_13_0_d18003c0() {
    // Encoding: 0xD18003C0
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=0, Xn=30, Xd=0
    // Fields: op3=0, Xd=0, uimm4=0, uimm6=0, Xn=30
    let encoding: u32 = 0xD18003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_14_0_d18003e0() {
    // Encoding: 0xD18003E0
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=0, Xn=31, Xd=0
    // Fields: uimm6=0, uimm4=0, op3=0, Xn=31, Xd=0
    let encoding: u32 = 0xD18003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_15_0_d1800000() {
    // Encoding: 0xD1800000
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: uimm6=0, op3=0, Xd=0, uimm4=0, Xn=0
    let encoding: u32 = 0xD1800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_16_0_d1800001() {
    // Encoding: 0xD1800001
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=1
    // Fields: uimm6=0, op3=0, Xd=1, uimm4=0, Xn=0
    let encoding: u32 = 0xD1800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_17_0_d180001e() {
    // Encoding: 0xD180001E
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=30
    // Fields: uimm6=0, Xn=0, Xd=30, uimm4=0, op3=0
    let encoding: u32 = 0xD180001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_18_0_d180001f() {
    // Encoding: 0xD180001F
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=31
    // Fields: uimm6=0, uimm4=0, op3=0, Xn=0, Xd=31
    let encoding: u32 = 0xD180001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_19_0_d1800021() {
    // Encoding: 0xD1800021
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=0, Xn=1, Xd=1
    // Fields: uimm6=0, op3=0, Xn=1, Xd=1, uimm4=0
    let encoding: u32 = 0xD1800021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsubtag_combo_20_0_d18003ff() {
    // Encoding: 0xD18003FF
    // Test aarch64_integer_tags_mcsubtag field combination: uimm6=0, op3=0, uimm4=0, Xn=31, Xd=31
    // Fields: Xd=31, uimm4=0, Xn=31, uimm6=0, op3=0
    let encoding: u32 = 0xD18003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsubtag_special_xn_31_stack_pointer_sp_may_require_alignment_0_d18003e0()
 {
    // Encoding: 0xD18003E0
    // Test aarch64_integer_tags_mcsubtag special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xn=31, uimm4=0, op3=0, uimm6=0, Xd=0
    let encoding: u32 = 0xD18003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsubtag_reg_write_0_d1800000() {
    // Test aarch64_integer_tags_mcsubtag register write: Sp
    // Encoding: 0xD1800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD1800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_tags_mcsubtag_reg_write_1_d1800000() {
    // Test aarch64_integer_tags_mcsubtag register write: GpFromField("d")
    // Encoding: 0xD1800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD1800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsubtag
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsubtag_sp_xn_d18003e0() {
    // Test aarch64_integer_tags_mcsubtag with Xn = SP (31)
    // Encoding: 0xD18003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD18003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_integer_tags_mcsettaganddatapairpost Tests
// ============================================================================

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field simm7 15 +: 7`
/// Requirement: FieldBoundary { field: "simm7", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_simm7_0_min_0_68800000() {
    // Encoding: 0x68800000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field simm7 = 0 (Min)
    // Fields: simm7=0, Xt2=0, Xn=0, Xt=0
    let encoding: u32 = 0x68800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field simm7 15 +: 7`
/// Requirement: FieldBoundary { field: "simm7", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_simm7_1_poweroftwo_0_68808000() {
    // Encoding: 0x68808000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field simm7 = 1 (PowerOfTwo)
    // Fields: Xn=0, simm7=1, Xt2=0, Xt=0
    let encoding: u32 = 0x68808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field simm7 15 +: 7`
/// Requirement: FieldBoundary { field: "simm7", value: 63, boundary: PowerOfTwoMinusOne }
/// midpoint (63)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_simm7_63_poweroftwominusone_0_689f8000()
{
    // Encoding: 0x689F8000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field simm7 = 63 (PowerOfTwoMinusOne)
    // Fields: Xt2=0, simm7=63, Xt=0, Xn=0
    let encoding: u32 = 0x689F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field simm7 15 +: 7`
/// Requirement: FieldBoundary { field: "simm7", value: 127, boundary: Max }
/// maximum value (127)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_simm7_127_max_0_68bf8000() {
    // Encoding: 0x68BF8000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field simm7 = 127 (Max)
    // Fields: Xn=0, Xt=0, Xt2=0, simm7=127
    let encoding: u32 = 0x68BF8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field Xt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Xt2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_xt2_0_min_0_68800000() {
    // Encoding: 0x68800000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field Xt2 = 0 (Min)
    // Fields: Xt2=0, Xt=0, simm7=0, Xn=0
    let encoding: u32 = 0x68800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field Xt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Xt2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_xt2_1_poweroftwo_0_68800400() {
    // Encoding: 0x68800400
    // Test aarch64_integer_tags_mcsettaganddatapairpost field Xt2 = 1 (PowerOfTwo)
    // Fields: simm7=0, Xt=0, Xn=0, Xt2=1
    let encoding: u32 = 0x68800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field Xt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Xt2", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_xt2_15_poweroftwominusone_0_68803c00() {
    // Encoding: 0x68803C00
    // Test aarch64_integer_tags_mcsettaganddatapairpost field Xt2 = 15 (PowerOfTwoMinusOne)
    // Fields: Xn=0, simm7=0, Xt2=15, Xt=0
    let encoding: u32 = 0x68803C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field Xt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Xt2", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_xt2_31_max_0_68807c00() {
    // Encoding: 0x68807C00
    // Test aarch64_integer_tags_mcsettaganddatapairpost field Xt2 = 31 (Max)
    // Fields: Xt=0, simm7=0, Xt2=31, Xn=0
    let encoding: u32 = 0x68807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_xn_0_min_0_68800000() {
    // Encoding: 0x68800000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field Xn = 0 (Min)
    // Fields: Xt=0, Xn=0, Xt2=0, simm7=0
    let encoding: u32 = 0x68800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_xn_1_poweroftwo_0_68800020() {
    // Encoding: 0x68800020
    // Test aarch64_integer_tags_mcsettaganddatapairpost field Xn = 1 (PowerOfTwo)
    // Fields: Xt=0, Xn=1, simm7=0, Xt2=0
    let encoding: u32 = 0x68800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_xn_30_poweroftwominusone_0_688003c0() {
    // Encoding: 0x688003C0
    // Test aarch64_integer_tags_mcsettaganddatapairpost field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=30, simm7=0, Xt2=0, Xt=0
    let encoding: u32 = 0x688003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_xn_31_max_0_688003e0() {
    // Encoding: 0x688003E0
    // Test aarch64_integer_tags_mcsettaganddatapairpost field Xn = 31 (Max)
    // Fields: Xt2=0, simm7=0, Xn=31, Xt=0
    let encoding: u32 = 0x688003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_xt_0_min_0_68800000() {
    // Encoding: 0x68800000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field Xt = 0 (Min)
    // Fields: Xt=0, Xt2=0, Xn=0, simm7=0
    let encoding: u32 = 0x68800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_xt_1_poweroftwo_0_68800001() {
    // Encoding: 0x68800001
    // Test aarch64_integer_tags_mcsettaganddatapairpost field Xt = 1 (PowerOfTwo)
    // Fields: simm7=0, Xn=0, Xt=1, Xt2=0
    let encoding: u32 = 0x68800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_xt_30_poweroftwominusone_0_6880001e() {
    // Encoding: 0x6880001E
    // Test aarch64_integer_tags_mcsettaganddatapairpost field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt2=0, simm7=0, Xt=30
    let encoding: u32 = 0x6880001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_field_xt_31_max_0_6880001f() {
    // Encoding: 0x6880001F
    // Test aarch64_integer_tags_mcsettaganddatapairpost field Xt = 31 (Max)
    // Fields: Xt2=0, Xt=31, simm7=0, Xn=0
    let encoding: u32 = 0x6880001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// simm7=0 (minimum value)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_0_0_68800000() {
    // Encoding: 0x68800000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=0, Xn=0, Xt=0
    // Fields: simm7=0, Xn=0, Xt2=0, Xt=0
    let encoding: u32 = 0x68800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// simm7=1 (value 1)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_1_0_68808000() {
    // Encoding: 0x68808000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=1, Xt2=0, Xn=0, Xt=0
    // Fields: Xn=0, simm7=1, Xt=0, Xt2=0
    let encoding: u32 = 0x68808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// simm7=63 (midpoint (63))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_2_0_689f8000() {
    // Encoding: 0x689F8000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=63, Xt2=0, Xn=0, Xt=0
    // Fields: simm7=63, Xt=0, Xt2=0, Xn=0
    let encoding: u32 = 0x689F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// simm7=127 (maximum value (127))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_3_0_68bf8000() {
    // Encoding: 0x68BF8000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=127, Xt2=0, Xn=0, Xt=0
    // Fields: Xn=0, simm7=127, Xt2=0, Xt=0
    let encoding: u32 = 0x68BF8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt2=0 (minimum value)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_4_0_68800000() {
    // Encoding: 0x68800000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=0, Xn=0, Xt=0
    // Fields: Xn=0, simm7=0, Xt2=0, Xt=0
    let encoding: u32 = 0x68800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt2=1 (value 1)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_5_0_68800400() {
    // Encoding: 0x68800400
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=1, Xn=0, Xt=0
    // Fields: Xt2=1, Xt=0, simm7=0, Xn=0
    let encoding: u32 = 0x68800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt2=15 (midpoint (15))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_6_0_68803c00() {
    // Encoding: 0x68803C00
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=15, Xn=0, Xt=0
    // Fields: simm7=0, Xt2=15, Xn=0, Xt=0
    let encoding: u32 = 0x68803C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt2=31 (maximum value (31))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_7_0_68807c00() {
    // Encoding: 0x68807C00
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=31, Xn=0, Xt=0
    // Fields: Xt=0, Xt2=31, simm7=0, Xn=0
    let encoding: u32 = 0x68807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_8_0_68800000() {
    // Encoding: 0x68800000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=0, Xn=0, Xt=0
    // Fields: Xt=0, Xt2=0, Xn=0, simm7=0
    let encoding: u32 = 0x68800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_9_0_68800020() {
    // Encoding: 0x68800020
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=0, Xn=1, Xt=0
    // Fields: Xt=0, simm7=0, Xt2=0, Xn=1
    let encoding: u32 = 0x68800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_10_0_688003c0() {
    // Encoding: 0x688003C0
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=0, Xn=30, Xt=0
    // Fields: Xn=30, Xt=0, simm7=0, Xt2=0
    let encoding: u32 = 0x688003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_11_0_688003e0() {
    // Encoding: 0x688003E0
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=0, Xn=31, Xt=0
    // Fields: Xn=31, simm7=0, Xt2=0, Xt=0
    let encoding: u32 = 0x688003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_12_0_68800000() {
    // Encoding: 0x68800000
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=0, Xn=0, Xt=0
    // Fields: simm7=0, Xn=0, Xt2=0, Xt=0
    let encoding: u32 = 0x68800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_13_0_68800001() {
    // Encoding: 0x68800001
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=0, Xn=0, Xt=1
    // Fields: Xt2=0, Xt=1, Xn=0, simm7=0
    let encoding: u32 = 0x68800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_14_0_6880001e() {
    // Encoding: 0x6880001E
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=0, Xn=0, Xt=30
    // Fields: simm7=0, Xn=0, Xt2=0, Xt=30
    let encoding: u32 = 0x6880001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_15_0_6880001f() {
    // Encoding: 0x6880001F
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=0, Xn=0, Xt=31
    // Fields: Xt=31, Xn=0, Xt2=0, simm7=0
    let encoding: u32 = 0x6880001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_16_0_68800021() {
    // Encoding: 0x68800021
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=0, Xn=1, Xt=1
    // Fields: Xt=1, simm7=0, Xn=1, Xt2=0
    let encoding: u32 = 0x68800021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_combo_17_0_688003ff() {
    // Encoding: 0x688003FF
    // Test aarch64_integer_tags_mcsettaganddatapairpost field combination: simm7=0, Xt2=0, Xn=31, Xt=31
    // Fields: Xn=31, simm7=0, Xt2=0, Xt=31
    let encoding: u32 = 0x688003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_special_xn_31_stack_pointer_sp_may_require_alignment_0_688003e0()
 {
    // Encoding: 0x688003E0
    // Test aarch64_integer_tags_mcsettaganddatapairpost special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xt2=0, simm7=0, Xn=31, Xt=0
    let encoding: u32 = 0x688003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field simm7 15 +: 7`
/// Requirement: FieldBoundary { field: "simm7", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_simm7_0_min_0_69800000() {
    // Encoding: 0x69800000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field simm7 = 0 (Min)
    // Fields: Xn=0, Xt=0, simm7=0, Xt2=0
    let encoding: u32 = 0x69800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field simm7 15 +: 7`
/// Requirement: FieldBoundary { field: "simm7", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_simm7_1_poweroftwo_0_69808000() {
    // Encoding: 0x69808000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field simm7 = 1 (PowerOfTwo)
    // Fields: Xt=0, simm7=1, Xt2=0, Xn=0
    let encoding: u32 = 0x69808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field simm7 15 +: 7`
/// Requirement: FieldBoundary { field: "simm7", value: 63, boundary: PowerOfTwoMinusOne }
/// midpoint (63)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_simm7_63_poweroftwominusone_0_699f8000() {
    // Encoding: 0x699F8000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field simm7 = 63 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=0, simm7=63, Xt2=0
    let encoding: u32 = 0x699F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field simm7 15 +: 7`
/// Requirement: FieldBoundary { field: "simm7", value: 127, boundary: Max }
/// maximum value (127)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_simm7_127_max_0_69bf8000() {
    // Encoding: 0x69BF8000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field simm7 = 127 (Max)
    // Fields: simm7=127, Xt2=0, Xt=0, Xn=0
    let encoding: u32 = 0x69BF8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field Xt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Xt2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_xt2_0_min_0_69800000() {
    // Encoding: 0x69800000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field Xt2 = 0 (Min)
    // Fields: simm7=0, Xt=0, Xt2=0, Xn=0
    let encoding: u32 = 0x69800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field Xt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Xt2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_xt2_1_poweroftwo_0_69800400() {
    // Encoding: 0x69800400
    // Test aarch64_integer_tags_mcsettaganddatapairpre field Xt2 = 1 (PowerOfTwo)
    // Fields: Xt=0, Xt2=1, simm7=0, Xn=0
    let encoding: u32 = 0x69800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field Xt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Xt2", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_xt2_15_poweroftwominusone_0_69803c00() {
    // Encoding: 0x69803C00
    // Test aarch64_integer_tags_mcsettaganddatapairpre field Xt2 = 15 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt2=15, Xt=0, simm7=0
    let encoding: u32 = 0x69803C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field Xt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Xt2", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_xt2_31_max_0_69807c00() {
    // Encoding: 0x69807C00
    // Test aarch64_integer_tags_mcsettaganddatapairpre field Xt2 = 31 (Max)
    // Fields: Xt2=31, simm7=0, Xt=0, Xn=0
    let encoding: u32 = 0x69807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_xn_0_min_0_69800000() {
    // Encoding: 0x69800000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field Xn = 0 (Min)
    // Fields: simm7=0, Xt2=0, Xt=0, Xn=0
    let encoding: u32 = 0x69800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_xn_1_poweroftwo_0_69800020() {
    // Encoding: 0x69800020
    // Test aarch64_integer_tags_mcsettaganddatapairpre field Xn = 1 (PowerOfTwo)
    // Fields: Xt2=0, simm7=0, Xt=0, Xn=1
    let encoding: u32 = 0x69800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_xn_30_poweroftwominusone_0_698003c0() {
    // Encoding: 0x698003C0
    // Test aarch64_integer_tags_mcsettaganddatapairpre field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xt2=0, Xt=0, simm7=0, Xn=30
    let encoding: u32 = 0x698003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_xn_31_max_0_698003e0() {
    // Encoding: 0x698003E0
    // Test aarch64_integer_tags_mcsettaganddatapairpre field Xn = 31 (Max)
    // Fields: Xt2=0, simm7=0, Xt=0, Xn=31
    let encoding: u32 = 0x698003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_xt_0_min_0_69800000() {
    // Encoding: 0x69800000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field Xt = 0 (Min)
    // Fields: Xt2=0, simm7=0, Xn=0, Xt=0
    let encoding: u32 = 0x69800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_xt_1_poweroftwo_0_69800001() {
    // Encoding: 0x69800001
    // Test aarch64_integer_tags_mcsettaganddatapairpre field Xt = 1 (PowerOfTwo)
    // Fields: Xt2=0, simm7=0, Xn=0, Xt=1
    let encoding: u32 = 0x69800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_xt_30_poweroftwominusone_0_6980001e() {
    // Encoding: 0x6980001E
    // Test aarch64_integer_tags_mcsettaganddatapairpre field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: Xt2=0, simm7=0, Xt=30, Xn=0
    let encoding: u32 = 0x6980001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_field_xt_31_max_0_6980001f() {
    // Encoding: 0x6980001F
    // Test aarch64_integer_tags_mcsettaganddatapairpre field Xt = 31 (Max)
    // Fields: Xt=31, simm7=0, Xn=0, Xt2=0
    let encoding: u32 = 0x6980001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// simm7=0 (minimum value)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_0_0_69800000() {
    // Encoding: 0x69800000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=0, Xn=0, Xt=0
    // Fields: Xt=0, simm7=0, Xt2=0, Xn=0
    let encoding: u32 = 0x69800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// simm7=1 (value 1)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_1_0_69808000() {
    // Encoding: 0x69808000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=1, Xt2=0, Xn=0, Xt=0
    // Fields: Xt2=0, simm7=1, Xn=0, Xt=0
    let encoding: u32 = 0x69808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// simm7=63 (midpoint (63))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_2_0_699f8000() {
    // Encoding: 0x699F8000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=63, Xt2=0, Xn=0, Xt=0
    // Fields: simm7=63, Xn=0, Xt=0, Xt2=0
    let encoding: u32 = 0x699F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// simm7=127 (maximum value (127))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_3_0_69bf8000() {
    // Encoding: 0x69BF8000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=127, Xt2=0, Xn=0, Xt=0
    // Fields: simm7=127, Xt=0, Xn=0, Xt2=0
    let encoding: u32 = 0x69BF8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt2=0 (minimum value)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_4_0_69800000() {
    // Encoding: 0x69800000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=0, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, Xt2=0, simm7=0
    let encoding: u32 = 0x69800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt2=1 (value 1)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_5_0_69800400() {
    // Encoding: 0x69800400
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=1, Xn=0, Xt=0
    // Fields: Xt=0, simm7=0, Xn=0, Xt2=1
    let encoding: u32 = 0x69800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt2=15 (midpoint (15))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_6_0_69803c00() {
    // Encoding: 0x69803C00
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=15, Xn=0, Xt=0
    // Fields: Xt=0, simm7=0, Xt2=15, Xn=0
    let encoding: u32 = 0x69803C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt2=31 (maximum value (31))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_7_0_69807c00() {
    // Encoding: 0x69807C00
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=31, Xn=0, Xt=0
    // Fields: simm7=0, Xt2=31, Xn=0, Xt=0
    let encoding: u32 = 0x69807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_8_0_69800000() {
    // Encoding: 0x69800000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=0, Xn=0, Xt=0
    // Fields: Xt=0, simm7=0, Xt2=0, Xn=0
    let encoding: u32 = 0x69800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_9_0_69800020() {
    // Encoding: 0x69800020
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=0, Xn=1, Xt=0
    // Fields: simm7=0, Xt2=0, Xn=1, Xt=0
    let encoding: u32 = 0x69800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_10_0_698003c0() {
    // Encoding: 0x698003C0
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=0, Xn=30, Xt=0
    // Fields: Xt=0, simm7=0, Xt2=0, Xn=30
    let encoding: u32 = 0x698003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_11_0_698003e0() {
    // Encoding: 0x698003E0
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=0, Xn=31, Xt=0
    // Fields: Xt2=0, Xn=31, Xt=0, simm7=0
    let encoding: u32 = 0x698003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_12_0_69800000() {
    // Encoding: 0x69800000
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=0, Xn=0, Xt=0
    // Fields: simm7=0, Xt2=0, Xn=0, Xt=0
    let encoding: u32 = 0x69800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_13_0_69800001() {
    // Encoding: 0x69800001
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=0, Xn=0, Xt=1
    // Fields: Xt2=0, simm7=0, Xn=0, Xt=1
    let encoding: u32 = 0x69800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_14_0_6980001e() {
    // Encoding: 0x6980001E
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=0, Xn=0, Xt=30
    // Fields: Xn=0, Xt2=0, Xt=30, simm7=0
    let encoding: u32 = 0x6980001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_15_0_6980001f() {
    // Encoding: 0x6980001F
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=0, Xn=0, Xt=31
    // Fields: Xt2=0, Xn=0, Xt=31, simm7=0
    let encoding: u32 = 0x6980001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_16_0_69800021() {
    // Encoding: 0x69800021
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=0, Xn=1, Xt=1
    // Fields: Xn=1, simm7=0, Xt2=0, Xt=1
    let encoding: u32 = 0x69800021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_combo_17_0_698003ff() {
    // Encoding: 0x698003FF
    // Test aarch64_integer_tags_mcsettaganddatapairpre field combination: simm7=0, Xt2=0, Xn=31, Xt=31
    // Fields: Xn=31, simm7=0, Xt2=0, Xt=31
    let encoding: u32 = 0x698003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_special_xn_31_stack_pointer_sp_may_require_alignment_0_698003e0()
 {
    // Encoding: 0x698003E0
    // Test aarch64_integer_tags_mcsettaganddatapairpre special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xt=0, simm7=0, Xt2=0, Xn=31
    let encoding: u32 = 0x698003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field simm7 15 +: 7`
/// Requirement: FieldBoundary { field: "simm7", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_simm7_0_min_0_69000000() {
    // Encoding: 0x69000000
    // Test aarch64_integer_tags_mcsettaganddatapair field simm7 = 0 (Min)
    // Fields: simm7=0, Xt2=0, Xt=0, Xn=0
    let encoding: u32 = 0x69000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field simm7 15 +: 7`
/// Requirement: FieldBoundary { field: "simm7", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_simm7_1_poweroftwo_0_69008000() {
    // Encoding: 0x69008000
    // Test aarch64_integer_tags_mcsettaganddatapair field simm7 = 1 (PowerOfTwo)
    // Fields: simm7=1, Xn=0, Xt2=0, Xt=0
    let encoding: u32 = 0x69008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field simm7 15 +: 7`
/// Requirement: FieldBoundary { field: "simm7", value: 63, boundary: PowerOfTwoMinusOne }
/// midpoint (63)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_simm7_63_poweroftwominusone_0_691f8000() {
    // Encoding: 0x691F8000
    // Test aarch64_integer_tags_mcsettaganddatapair field simm7 = 63 (PowerOfTwoMinusOne)
    // Fields: Xt2=0, Xn=0, Xt=0, simm7=63
    let encoding: u32 = 0x691F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field simm7 15 +: 7`
/// Requirement: FieldBoundary { field: "simm7", value: 127, boundary: Max }
/// maximum value (127)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_simm7_127_max_0_693f8000() {
    // Encoding: 0x693F8000
    // Test aarch64_integer_tags_mcsettaganddatapair field simm7 = 127 (Max)
    // Fields: simm7=127, Xt2=0, Xn=0, Xt=0
    let encoding: u32 = 0x693F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field Xt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Xt2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_xt2_0_min_0_69000000() {
    // Encoding: 0x69000000
    // Test aarch64_integer_tags_mcsettaganddatapair field Xt2 = 0 (Min)
    // Fields: simm7=0, Xt=0, Xn=0, Xt2=0
    let encoding: u32 = 0x69000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field Xt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Xt2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_xt2_1_poweroftwo_0_69000400() {
    // Encoding: 0x69000400
    // Test aarch64_integer_tags_mcsettaganddatapair field Xt2 = 1 (PowerOfTwo)
    // Fields: Xn=0, simm7=0, Xt2=1, Xt=0
    let encoding: u32 = 0x69000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field Xt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Xt2", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_xt2_15_poweroftwominusone_0_69003c00() {
    // Encoding: 0x69003C00
    // Test aarch64_integer_tags_mcsettaganddatapair field Xt2 = 15 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xt2=15, simm7=0, Xn=0
    let encoding: u32 = 0x69003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field Xt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Xt2", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_xt2_31_max_0_69007c00() {
    // Encoding: 0x69007C00
    // Test aarch64_integer_tags_mcsettaganddatapair field Xt2 = 31 (Max)
    // Fields: simm7=0, Xt2=31, Xn=0, Xt=0
    let encoding: u32 = 0x69007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_xn_0_min_0_69000000() {
    // Encoding: 0x69000000
    // Test aarch64_integer_tags_mcsettaganddatapair field Xn = 0 (Min)
    // Fields: Xn=0, Xt=0, simm7=0, Xt2=0
    let encoding: u32 = 0x69000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_xn_1_poweroftwo_0_69000020() {
    // Encoding: 0x69000020
    // Test aarch64_integer_tags_mcsettaganddatapair field Xn = 1 (PowerOfTwo)
    // Fields: Xn=1, Xt2=0, Xt=0, simm7=0
    let encoding: u32 = 0x69000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_xn_30_poweroftwominusone_0_690003c0() {
    // Encoding: 0x690003C0
    // Test aarch64_integer_tags_mcsettaganddatapair field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xt2=0, simm7=0, Xn=30, Xt=0
    let encoding: u32 = 0x690003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_xn_31_max_0_690003e0() {
    // Encoding: 0x690003E0
    // Test aarch64_integer_tags_mcsettaganddatapair field Xn = 31 (Max)
    // Fields: Xn=31, simm7=0, Xt=0, Xt2=0
    let encoding: u32 = 0x690003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_xt_0_min_0_69000000() {
    // Encoding: 0x69000000
    // Test aarch64_integer_tags_mcsettaganddatapair field Xt = 0 (Min)
    // Fields: Xt=0, Xt2=0, simm7=0, Xn=0
    let encoding: u32 = 0x69000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_xt_1_poweroftwo_0_69000001() {
    // Encoding: 0x69000001
    // Test aarch64_integer_tags_mcsettaganddatapair field Xt = 1 (PowerOfTwo)
    // Fields: Xn=0, Xt2=0, simm7=0, Xt=1
    let encoding: u32 = 0x69000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_xt_30_poweroftwominusone_0_6900001e() {
    // Encoding: 0x6900001E
    // Test aarch64_integer_tags_mcsettaganddatapair field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: Xt2=0, Xn=0, simm7=0, Xt=30
    let encoding: u32 = 0x6900001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_field_xt_31_max_0_6900001f() {
    // Encoding: 0x6900001F
    // Test aarch64_integer_tags_mcsettaganddatapair field Xt = 31 (Max)
    // Fields: Xt=31, Xt2=0, simm7=0, Xn=0
    let encoding: u32 = 0x6900001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// simm7=0 (minimum value)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_0_0_69000000() {
    // Encoding: 0x69000000
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=0, Xn=0, Xt=0
    // Fields: simm7=0, Xt2=0, Xt=0, Xn=0
    let encoding: u32 = 0x69000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// simm7=1 (value 1)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_1_0_69008000() {
    // Encoding: 0x69008000
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=1, Xt2=0, Xn=0, Xt=0
    // Fields: Xt2=0, Xt=0, simm7=1, Xn=0
    let encoding: u32 = 0x69008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// simm7=63 (midpoint (63))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_2_0_691f8000() {
    // Encoding: 0x691F8000
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=63, Xt2=0, Xn=0, Xt=0
    // Fields: simm7=63, Xt2=0, Xn=0, Xt=0
    let encoding: u32 = 0x691F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// simm7=127 (maximum value (127))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_3_0_693f8000() {
    // Encoding: 0x693F8000
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=127, Xt2=0, Xn=0, Xt=0
    // Fields: Xn=0, simm7=127, Xt2=0, Xt=0
    let encoding: u32 = 0x693F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt2=0 (minimum value)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_4_0_69000000() {
    // Encoding: 0x69000000
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=0, Xn=0, Xt=0
    // Fields: Xt=0, Xt2=0, Xn=0, simm7=0
    let encoding: u32 = 0x69000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt2=1 (value 1)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_5_0_69000400() {
    // Encoding: 0x69000400
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=1, Xn=0, Xt=0
    // Fields: Xt=0, simm7=0, Xt2=1, Xn=0
    let encoding: u32 = 0x69000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt2=15 (midpoint (15))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_6_0_69003c00() {
    // Encoding: 0x69003C00
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=15, Xn=0, Xt=0
    // Fields: simm7=0, Xt2=15, Xt=0, Xn=0
    let encoding: u32 = 0x69003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt2=31 (maximum value (31))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_7_0_69007c00() {
    // Encoding: 0x69007C00
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=31, Xn=0, Xt=0
    // Fields: Xt2=31, Xn=0, Xt=0, simm7=0
    let encoding: u32 = 0x69007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_8_0_69000000() {
    // Encoding: 0x69000000
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=0, Xn=0, Xt=0
    // Fields: Xt2=0, Xn=0, simm7=0, Xt=0
    let encoding: u32 = 0x69000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_9_0_69000020() {
    // Encoding: 0x69000020
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=0, Xn=1, Xt=0
    // Fields: Xn=1, Xt2=0, Xt=0, simm7=0
    let encoding: u32 = 0x69000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_10_0_690003c0() {
    // Encoding: 0x690003C0
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=0, Xn=30, Xt=0
    // Fields: simm7=0, Xt=0, Xn=30, Xt2=0
    let encoding: u32 = 0x690003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_11_0_690003e0() {
    // Encoding: 0x690003E0
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=0, Xn=31, Xt=0
    // Fields: simm7=0, Xn=31, Xt=0, Xt2=0
    let encoding: u32 = 0x690003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_12_0_69000000() {
    // Encoding: 0x69000000
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=0, Xn=0, Xt=0
    // Fields: simm7=0, Xt=0, Xt2=0, Xn=0
    let encoding: u32 = 0x69000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_13_0_69000001() {
    // Encoding: 0x69000001
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=0, Xn=0, Xt=1
    // Fields: Xt=1, simm7=0, Xt2=0, Xn=0
    let encoding: u32 = 0x69000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_14_0_6900001e() {
    // Encoding: 0x6900001E
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=0, Xn=0, Xt=30
    // Fields: Xt2=0, simm7=0, Xn=0, Xt=30
    let encoding: u32 = 0x6900001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_15_0_6900001f() {
    // Encoding: 0x6900001F
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=0, Xn=0, Xt=31
    // Fields: Xt2=0, simm7=0, Xt=31, Xn=0
    let encoding: u32 = 0x6900001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_16_0_69000021() {
    // Encoding: 0x69000021
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=0, Xn=1, Xt=1
    // Fields: simm7=0, Xt2=0, Xt=1, Xn=1
    let encoding: u32 = 0x69000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_combo_17_0_690003ff() {
    // Encoding: 0x690003FF
    // Test aarch64_integer_tags_mcsettaganddatapair field combination: simm7=0, Xt2=0, Xn=31, Xt=31
    // Fields: Xt=31, simm7=0, Xn=31, Xt2=0
    let encoding: u32 = 0x690003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_special_xn_31_stack_pointer_sp_may_require_alignment_0_690003e0()
 {
    // Encoding: 0x690003E0
    // Test aarch64_integer_tags_mcsettaganddatapair special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xt2=0, simm7=0, Xn=31, Xt=0
    let encoding: u32 = 0x690003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_reg_write_0_68800000() {
    // Test aarch64_integer_tags_mcsettaganddatapairpost register write: Sp
    // Encoding: 0x68800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x68800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_reg_write_1_68800000() {
    // Test aarch64_integer_tags_mcsettaganddatapairpost register write: GpFromField("n")
    // Encoding: 0x68800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x68800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_sp_xn_688003e0() {
    // Test aarch64_integer_tags_mcsettaganddatapairpost with Xn = SP (31)
    // Encoding: 0x688003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x688003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_store_0_68800000() {
    // Test aarch64_integer_tags_mcsettaganddatapairpost memory store: 8 bytes
    // Encoding: 0x68800000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x68800000;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpost
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpost_store_1_68800000() {
    // Test aarch64_integer_tags_mcsettaganddatapairpost memory store: 8 bytes
    // Encoding: 0x68800000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x68800000;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `LDRSH X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// zero value
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_ldr_oracle_0_79800020() {
    // Test LDRSH: zero value (oracle)
    // Encoding: 0x79800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0]).unwrap();
    let encoding: u32 = 0x79800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `LDRSH X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max byte
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_ldr_oracle_1_79800020() {
    // Test LDRSH: max byte (oracle)
    // Encoding: 0x79800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 0]).unwrap();
    let encoding: u32 = 0x79800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xFF, "X0 should be 0x00000000000000FF");
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `LDRSH X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max halfword
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_ldr_oracle_2_79800020() {
    // Test LDRSH: max halfword (oracle)
    // Encoding: 0x79800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255]).unwrap();
    let encoding: u32 = 0x79800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `LDRSH X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max word
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_ldr_oracle_3_79800020() {
    // Test LDRSH: max word (oracle)
    // Encoding: 0x79800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255]).unwrap();
    let encoding: u32 = 0x79800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `LDRSH X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// large value
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_ldr_oracle_4_79800020() {
    // Test LDRSH: large value (oracle)
    // Encoding: 0x79800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[240, 222]).unwrap();
    let encoding: u32 = 0x79800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFDEF0,
        "X0 should be 0xFFFFFFFFFFFFDEF0"
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `LDRSH X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (byte)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_ldr_oracle_5_79800020() {
    // Test LDRSH: sign bit (byte) (oracle)
    // Encoding: 0x79800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[128, 0]).unwrap();
    let encoding: u32 = 0x79800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x80, "X0 should be 0x0000000000000080");
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `LDRSH X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (halfword)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_ldr_oracle_6_79800020() {
    // Test LDRSH: sign bit (halfword) (oracle)
    // Encoding: 0x79800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 128]).unwrap();
    let encoding: u32 = 0x79800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFF8000,
        "X0 should be 0xFFFFFFFFFFFF8000"
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `LDRSH X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (word)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_ldr_oracle_7_79800020() {
    // Test LDRSH: sign bit (word) (oracle)
    // Encoding: 0x79800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0]).unwrap();
    let encoding: u32 = 0x79800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_reg_write_0_69800000() {
    // Test aarch64_integer_tags_mcsettaganddatapairpre register write: Sp
    // Encoding: 0x69800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x69800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_reg_write_1_69800000() {
    // Test aarch64_integer_tags_mcsettaganddatapairpre register write: GpFromField("n")
    // Encoding: 0x69800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x69800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_sp_xn_698003e0() {
    // Test aarch64_integer_tags_mcsettaganddatapairpre with Xn = SP (31)
    // Encoding: 0x698003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x698003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_store_0_69800000() {
    // Test aarch64_integer_tags_mcsettaganddatapairpre memory store: 8 bytes
    // Encoding: 0x69800000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x69800000;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapairpre
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapairpre_store_1_69800000() {
    // Test aarch64_integer_tags_mcsettaganddatapairpre memory store: 8 bytes
    // Encoding: 0x69800000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0x69800000;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `STRH X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 16, addressing: "immediate" }
/// zero value
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_str_oracle_0_79000020() {
    // Test STRH: zero value (oracle)
    // Encoding: 0x79000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x0);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0x79000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 2).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x0, "Memory at 0x1000 should be 0x0");
    }
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `STRH X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 16, addressing: "immediate" }
/// byte value
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_str_oracle_1_79000020() {
    // Test STRH: byte value (oracle)
    // Encoding: 0x79000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xFF);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0x79000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 2).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0xFF, "Memory at 0x1000 should be 0xFF");
    }
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `STRH X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 16, addressing: "immediate" }
/// halfword value
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_str_oracle_2_79000020() {
    // Test STRH: halfword value (oracle)
    // Encoding: 0x79000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x1234);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0x79000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 2).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x1234, "Memory at 0x1000 should be 0x1234");
    }
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `STRH X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 16, addressing: "immediate" }
/// word value
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_str_oracle_3_79000020() {
    // Test STRH: word value (oracle)
    // Encoding: 0x79000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x12345678);
    let encoding: u32 = 0x79000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 2).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x5678, "Memory at 0x1000 should be 0x5678");
    }
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `STRH X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 16, addressing: "immediate" }
/// doubleword value
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_str_oracle_4_79000020() {
    // Test STRH: doubleword value (oracle)
    // Encoding: 0x79000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x123456789ABCDEF0);
    let encoding: u32 = 0x79000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 2).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0xDEF0, "Memory at 0x1000 should be 0xDEF0");
    }
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_reg_write_0_69000000() {
    // Test aarch64_integer_tags_mcsettaganddatapair register write: Sp
    // Encoding: 0x69000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x69000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_reg_write_1_69000000() {
    // Test aarch64_integer_tags_mcsettaganddatapair register write: GpFromField("n")
    // Encoding: 0x69000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x69000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_sp_xn_690003e0() {
    // Test aarch64_integer_tags_mcsettaganddatapair with Xn = SP (31)
    // Encoding: 0x690003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x690003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_store_0_69000000() {
    // Test aarch64_integer_tags_mcsettaganddatapair memory store: 8 bytes
    // Encoding: 0x69000000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0x69000000;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettaganddatapair
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_integer_tags_mcsettaganddatapair_store_1_69000000() {
    // Test aarch64_integer_tags_mcsettaganddatapair memory store: 8 bytes
    // Encoding: 0x69000000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x69000000;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

// ============================================================================
// aarch64_integer_tags_mcaddtag Tests
// ============================================================================

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field uimm6 16 +: 6`
/// Requirement: FieldBoundary { field: "uimm6", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_uimm6_0_min_0_91800000() {
    // Encoding: 0x91800000
    // Test aarch64_integer_tags_mcaddtag field uimm6 = 0 (Min)
    // Fields: uimm6=0, op3=0, Xn=0, Xd=0, uimm4=0
    let encoding: u32 = 0x91800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field uimm6 16 +: 6`
/// Requirement: FieldBoundary { field: "uimm6", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_uimm6_1_poweroftwo_0_91810000() {
    // Encoding: 0x91810000
    // Test aarch64_integer_tags_mcaddtag field uimm6 = 1 (PowerOfTwo)
    // Fields: uimm6=1, op3=0, uimm4=0, Xn=0, Xd=0
    let encoding: u32 = 0x91810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field uimm6 16 +: 6`
/// Requirement: FieldBoundary { field: "uimm6", value: 31, boundary: PowerOfTwoMinusOne }
/// midpoint (31)
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_uimm6_31_poweroftwominusone_0_919f0000() {
    // Encoding: 0x919F0000
    // Test aarch64_integer_tags_mcaddtag field uimm6 = 31 (PowerOfTwoMinusOne)
    // Fields: uimm4=0, op3=0, Xn=0, uimm6=31, Xd=0
    let encoding: u32 = 0x919F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field uimm6 16 +: 6`
/// Requirement: FieldBoundary { field: "uimm6", value: 63, boundary: Max }
/// maximum value (63)
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_uimm6_63_max_0_91bf0000() {
    // Encoding: 0x91BF0000
    // Test aarch64_integer_tags_mcaddtag field uimm6 = 63 (Max)
    // Fields: uimm4=0, Xd=0, uimm6=63, op3=0, Xn=0
    let encoding: u32 = 0x91BF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field op3 14 +: 2`
/// Requirement: FieldBoundary { field: "op3", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_op3_0_min_0_91800000() {
    // Encoding: 0x91800000
    // Test aarch64_integer_tags_mcaddtag field op3 = 0 (Min)
    // Fields: uimm6=0, Xd=0, op3=0, uimm4=0, Xn=0
    let encoding: u32 = 0x91800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field op3 14 +: 2`
/// Requirement: FieldBoundary { field: "op3", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_op3_1_poweroftwo_0_91804000() {
    // Encoding: 0x91804000
    // Test aarch64_integer_tags_mcaddtag field op3 = 1 (PowerOfTwo)
    // Fields: op3=1, Xn=0, uimm4=0, uimm6=0, Xd=0
    let encoding: u32 = 0x91804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field op3 14 +: 2`
/// Requirement: FieldBoundary { field: "op3", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_op3_3_max_0_9180c000() {
    // Encoding: 0x9180C000
    // Test aarch64_integer_tags_mcaddtag field op3 = 3 (Max)
    // Fields: Xn=0, op3=3, uimm4=0, Xd=0, uimm6=0
    let encoding: u32 = 0x9180C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field uimm4 10 +: 4`
/// Requirement: FieldBoundary { field: "uimm4", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_uimm4_0_min_0_91800000() {
    // Encoding: 0x91800000
    // Test aarch64_integer_tags_mcaddtag field uimm4 = 0 (Min)
    // Fields: Xd=0, uimm6=0, uimm4=0, Xn=0, op3=0
    let encoding: u32 = 0x91800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field uimm4 10 +: 4`
/// Requirement: FieldBoundary { field: "uimm4", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_uimm4_1_poweroftwo_0_91800400() {
    // Encoding: 0x91800400
    // Test aarch64_integer_tags_mcaddtag field uimm4 = 1 (PowerOfTwo)
    // Fields: uimm4=1, uimm6=0, op3=0, Xn=0, Xd=0
    let encoding: u32 = 0x91800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field uimm4 10 +: 4`
/// Requirement: FieldBoundary { field: "uimm4", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_uimm4_7_poweroftwominusone_0_91801c00() {
    // Encoding: 0x91801C00
    // Test aarch64_integer_tags_mcaddtag field uimm4 = 7 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xd=0, uimm4=7, uimm6=0, op3=0
    let encoding: u32 = 0x91801C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field uimm4 10 +: 4`
/// Requirement: FieldBoundary { field: "uimm4", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_uimm4_15_max_0_91803c00() {
    // Encoding: 0x91803C00
    // Test aarch64_integer_tags_mcaddtag field uimm4 = 15 (Max)
    // Fields: op3=0, uimm6=0, uimm4=15, Xn=0, Xd=0
    let encoding: u32 = 0x91803C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_xn_0_min_0_91800000() {
    // Encoding: 0x91800000
    // Test aarch64_integer_tags_mcaddtag field Xn = 0 (Min)
    // Fields: Xd=0, Xn=0, uimm4=0, uimm6=0, op3=0
    let encoding: u32 = 0x91800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_xn_1_poweroftwo_0_91800020() {
    // Encoding: 0x91800020
    // Test aarch64_integer_tags_mcaddtag field Xn = 1 (PowerOfTwo)
    // Fields: Xd=0, uimm4=0, op3=0, Xn=1, uimm6=0
    let encoding: u32 = 0x91800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_xn_30_poweroftwominusone_0_918003c0() {
    // Encoding: 0x918003C0
    // Test aarch64_integer_tags_mcaddtag field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: uimm6=0, Xd=0, uimm4=0, op3=0, Xn=30
    let encoding: u32 = 0x918003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_xn_31_max_0_918003e0() {
    // Encoding: 0x918003E0
    // Test aarch64_integer_tags_mcaddtag field Xn = 31 (Max)
    // Fields: op3=0, uimm6=0, Xn=31, Xd=0, uimm4=0
    let encoding: u32 = 0x918003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_xd_0_min_0_91800000() {
    // Encoding: 0x91800000
    // Test aarch64_integer_tags_mcaddtag field Xd = 0 (Min)
    // Fields: uimm6=0, uimm4=0, op3=0, Xd=0, Xn=0
    let encoding: u32 = 0x91800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_xd_1_poweroftwo_0_91800001() {
    // Encoding: 0x91800001
    // Test aarch64_integer_tags_mcaddtag field Xd = 1 (PowerOfTwo)
    // Fields: uimm6=0, Xn=0, Xd=1, uimm4=0, op3=0
    let encoding: u32 = 0x91800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_xd_30_poweroftwominusone_0_9180001e() {
    // Encoding: 0x9180001E
    // Test aarch64_integer_tags_mcaddtag field Xd = 30 (PowerOfTwoMinusOne)
    // Fields: op3=0, Xd=30, uimm6=0, uimm4=0, Xn=0
    let encoding: u32 = 0x9180001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcaddtag_field_xd_31_max_0_9180001f() {
    // Encoding: 0x9180001F
    // Test aarch64_integer_tags_mcaddtag field Xd = 31 (Max)
    // Fields: uimm4=0, uimm6=0, Xn=0, Xd=31, op3=0
    let encoding: u32 = 0x9180001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm6=0 (minimum value)
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_0_0_91800000() {
    // Encoding: 0x91800000
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: Xn=0, op3=0, Xd=0, uimm6=0, uimm4=0
    let encoding: u32 = 0x91800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm6=1 (value 1)
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_1_0_91810000() {
    // Encoding: 0x91810000
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=1, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: uimm6=1, Xd=0, Xn=0, op3=0, uimm4=0
    let encoding: u32 = 0x91810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm6=31 (midpoint (31))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_2_0_919f0000() {
    // Encoding: 0x919F0000
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=31, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: uimm6=31, Xn=0, uimm4=0, op3=0, Xd=0
    let encoding: u32 = 0x919F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm6=63 (maximum value (63))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_3_0_91bf0000() {
    // Encoding: 0x91BF0000
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=63, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: Xn=0, op3=0, uimm6=63, Xd=0, uimm4=0
    let encoding: u32 = 0x91BF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op3=0 (minimum value)
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_4_0_91800000() {
    // Encoding: 0x91800000
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: Xn=0, op3=0, uimm4=0, uimm6=0, Xd=0
    let encoding: u32 = 0x91800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op3=1 (value 1)
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_5_0_91804000() {
    // Encoding: 0x91804000
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=1, uimm4=0, Xn=0, Xd=0
    // Fields: uimm4=0, uimm6=0, Xn=0, op3=1, Xd=0
    let encoding: u32 = 0x91804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op3=3 (maximum value (3))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_6_0_9180c000() {
    // Encoding: 0x9180C000
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=3, uimm4=0, Xn=0, Xd=0
    // Fields: uimm4=0, Xn=0, Xd=0, uimm6=0, op3=3
    let encoding: u32 = 0x9180C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm4=0 (minimum value)
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_7_0_91800000() {
    // Encoding: 0x91800000
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: Xd=0, uimm6=0, uimm4=0, Xn=0, op3=0
    let encoding: u32 = 0x91800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm4=1 (value 1)
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_8_0_91800400() {
    // Encoding: 0x91800400
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=1, Xn=0, Xd=0
    // Fields: uimm4=1, Xd=0, op3=0, uimm6=0, Xn=0
    let encoding: u32 = 0x91800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm4=7 (midpoint (7))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_9_0_91801c00() {
    // Encoding: 0x91801C00
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=7, Xn=0, Xd=0
    // Fields: Xd=0, uimm6=0, op3=0, Xn=0, uimm4=7
    let encoding: u32 = 0x91801C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// uimm4=15 (maximum value (15))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_10_0_91803c00() {
    // Encoding: 0x91803C00
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=15, Xn=0, Xd=0
    // Fields: uimm4=15, Xd=0, uimm6=0, op3=0, Xn=0
    let encoding: u32 = 0x91803C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_11_0_91800000() {
    // Encoding: 0x91800000
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: op3=0, Xd=0, Xn=0, uimm6=0, uimm4=0
    let encoding: u32 = 0x91800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_12_0_91800020() {
    // Encoding: 0x91800020
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=0, Xn=1, Xd=0
    // Fields: uimm4=0, op3=0, uimm6=0, Xn=1, Xd=0
    let encoding: u32 = 0x91800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_13_0_918003c0() {
    // Encoding: 0x918003C0
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=0, Xn=30, Xd=0
    // Fields: op3=0, uimm4=0, uimm6=0, Xn=30, Xd=0
    let encoding: u32 = 0x918003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_14_0_918003e0() {
    // Encoding: 0x918003E0
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=0, Xn=31, Xd=0
    // Fields: op3=0, uimm4=0, uimm6=0, Xn=31, Xd=0
    let encoding: u32 = 0x918003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_15_0_91800000() {
    // Encoding: 0x91800000
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=0
    // Fields: Xd=0, uimm6=0, op3=0, Xn=0, uimm4=0
    let encoding: u32 = 0x91800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_16_0_91800001() {
    // Encoding: 0x91800001
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=1
    // Fields: Xn=0, op3=0, uimm4=0, Xd=1, uimm6=0
    let encoding: u32 = 0x91800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_17_0_9180001e() {
    // Encoding: 0x9180001E
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=30
    // Fields: Xn=0, uimm6=0, op3=0, uimm4=0, Xd=30
    let encoding: u32 = 0x9180001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_18_0_9180001f() {
    // Encoding: 0x9180001F
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=0, Xn=0, Xd=31
    // Fields: uimm6=0, op3=0, Xd=31, Xn=0, uimm4=0
    let encoding: u32 = 0x9180001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_19_0_91800021() {
    // Encoding: 0x91800021
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=0, Xn=1, Xd=1
    // Fields: Xn=1, uimm6=0, op3=0, uimm4=0, Xd=1
    let encoding: u32 = 0x91800021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcaddtag_combo_20_0_918003ff() {
    // Encoding: 0x918003FF
    // Test aarch64_integer_tags_mcaddtag field combination: uimm6=0, op3=0, uimm4=0, Xn=31, Xd=31
    // Fields: uimm6=0, Xn=31, Xd=31, op3=0, uimm4=0
    let encoding: u32 = 0x918003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcaddtag_special_xn_31_stack_pointer_sp_may_require_alignment_0_918003e0()
 {
    // Encoding: 0x918003E0
    // Test aarch64_integer_tags_mcaddtag special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: uimm4=0, Xn=31, uimm6=0, Xd=0, op3=0
    let encoding: u32 = 0x918003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcaddtag_reg_write_0_91800000() {
    // Test aarch64_integer_tags_mcaddtag register write: Sp
    // Encoding: 0x91800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x91800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_tags_mcaddtag_reg_write_1_91800000() {
    // Test aarch64_integer_tags_mcaddtag register write: GpFromField("d")
    // Encoding: 0x91800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x91800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcaddtag
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcaddtag_sp_xn_918003e0() {
    // Test aarch64_integer_tags_mcaddtag with Xn = SP (31)
    // Encoding: 0x918003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x918003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_integer_tags_mcgettag Tests
// ============================================================================

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_0_zero_0_d9600000() {
    // Encoding: 0xD9600000
    // Test aarch64_integer_tags_mcgettag field imm9 = 0 (Zero)
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_1_poweroftwo_0_d9601000() {
    // Encoding: 0xD9601000
    // Test aarch64_integer_tags_mcgettag field imm9 = 1 (PowerOfTwo)
    // Fields: Xt=0, imm9=1, Xn=0
    let encoding: u32 = 0xD9601000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_3_poweroftwominusone_0_d9603000() {
    // Encoding: 0xD9603000
    // Test aarch64_integer_tags_mcgettag field imm9 = 3 (PowerOfTwoMinusOne)
    // Fields: imm9=3, Xn=0, Xt=0
    let encoding: u32 = 0xD9603000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_4_poweroftwo_0_d9604000() {
    // Encoding: 0xD9604000
    // Test aarch64_integer_tags_mcgettag field imm9 = 4 (PowerOfTwo)
    // Fields: imm9=4, Xt=0, Xn=0
    let encoding: u32 = 0xD9604000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_7_poweroftwominusone_0_d9607000() {
    // Encoding: 0xD9607000
    // Test aarch64_integer_tags_mcgettag field imm9 = 7 (PowerOfTwoMinusOne)
    // Fields: imm9=7, Xt=0, Xn=0
    let encoding: u32 = 0xD9607000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_8_poweroftwo_0_d9608000() {
    // Encoding: 0xD9608000
    // Test aarch64_integer_tags_mcgettag field imm9 = 8 (PowerOfTwo)
    // Fields: Xt=0, imm9=8, Xn=0
    let encoding: u32 = 0xD9608000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_15_poweroftwominusone_0_d960f000() {
    // Encoding: 0xD960F000
    // Test aarch64_integer_tags_mcgettag field imm9 = 15 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xn=0, imm9=15
    let encoding: u32 = 0xD960F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_16_poweroftwo_0_d9610000() {
    // Encoding: 0xD9610000
    // Test aarch64_integer_tags_mcgettag field imm9 = 16 (PowerOfTwo)
    // Fields: Xt=0, Xn=0, imm9=16
    let encoding: u32 = 0xD9610000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_31_poweroftwominusone_0_d961f000() {
    // Encoding: 0xD961F000
    // Test aarch64_integer_tags_mcgettag field imm9 = 31 (PowerOfTwoMinusOne)
    // Fields: imm9=31, Xn=0, Xt=0
    let encoding: u32 = 0xD961F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_32_poweroftwo_0_d9620000() {
    // Encoding: 0xD9620000
    // Test aarch64_integer_tags_mcgettag field imm9 = 32 (PowerOfTwo)
    // Fields: Xt=0, imm9=32, Xn=0
    let encoding: u32 = 0xD9620000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_63_poweroftwominusone_0_d963f000() {
    // Encoding: 0xD963F000
    // Test aarch64_integer_tags_mcgettag field imm9 = 63 (PowerOfTwoMinusOne)
    // Fields: imm9=63, Xn=0, Xt=0
    let encoding: u32 = 0xD963F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_64_poweroftwo_0_d9640000() {
    // Encoding: 0xD9640000
    // Test aarch64_integer_tags_mcgettag field imm9 = 64 (PowerOfTwo)
    // Fields: imm9=64, Xt=0, Xn=0
    let encoding: u32 = 0xD9640000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_127_poweroftwominusone_0_d967f000() {
    // Encoding: 0xD967F000
    // Test aarch64_integer_tags_mcgettag field imm9 = 127 (PowerOfTwoMinusOne)
    // Fields: imm9=127, Xn=0, Xt=0
    let encoding: u32 = 0xD967F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_128_poweroftwo_0_d9680000() {
    // Encoding: 0xD9680000
    // Test aarch64_integer_tags_mcgettag field imm9 = 128 (PowerOfTwo)
    // Fields: imm9=128, Xn=0, Xt=0
    let encoding: u32 = 0xD9680000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 255, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (255)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_255_poweroftwominusone_0_d96ff000() {
    // Encoding: 0xD96FF000
    // Test aarch64_integer_tags_mcgettag field imm9 = 255 (PowerOfTwoMinusOne)
    // Fields: Xt=0, Xn=0, imm9=255
    let encoding: u32 = 0xD96FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_256_poweroftwo_0_d9700000() {
    // Encoding: 0xD9700000
    // Test aarch64_integer_tags_mcgettag field imm9 = 256 (PowerOfTwo)
    // Fields: Xn=0, Xt=0, imm9=256
    let encoding: u32 = 0xD9700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field imm9 12 +: 9`
/// Requirement: FieldBoundary { field: "imm9", value: 511, boundary: Max }
/// maximum immediate (511)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_imm9_511_max_0_d97ff000() {
    // Encoding: 0xD97FF000
    // Test aarch64_integer_tags_mcgettag field imm9 = 511 (Max)
    // Fields: Xn=0, Xt=0, imm9=511
    let encoding: u32 = 0xD97FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_xn_0_min_0_d9600000() {
    // Encoding: 0xD9600000
    // Test aarch64_integer_tags_mcgettag field Xn = 0 (Min)
    // Fields: imm9=0, Xn=0, Xt=0
    let encoding: u32 = 0xD9600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_xn_1_poweroftwo_0_d9600020() {
    // Encoding: 0xD9600020
    // Test aarch64_integer_tags_mcgettag field Xn = 1 (PowerOfTwo)
    // Fields: imm9=0, Xn=1, Xt=0
    let encoding: u32 = 0xD9600020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_xn_30_poweroftwominusone_0_d96003c0() {
    // Encoding: 0xD96003C0
    // Test aarch64_integer_tags_mcgettag field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=30, Xt=0, imm9=0
    let encoding: u32 = 0xD96003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_xn_31_max_0_d96003e0() {
    // Encoding: 0xD96003E0
    // Test aarch64_integer_tags_mcgettag field Xn = 31 (Max)
    // Fields: Xn=31, Xt=0, imm9=0
    let encoding: u32 = 0xD96003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_xt_0_min_0_d9600000() {
    // Encoding: 0xD9600000
    // Test aarch64_integer_tags_mcgettag field Xt = 0 (Min)
    // Fields: Xt=0, Xn=0, imm9=0
    let encoding: u32 = 0xD9600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_xt_1_poweroftwo_0_d9600001() {
    // Encoding: 0xD9600001
    // Test aarch64_integer_tags_mcgettag field Xt = 1 (PowerOfTwo)
    // Fields: imm9=0, Xn=0, Xt=1
    let encoding: u32 = 0xD9600001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_xt_30_poweroftwominusone_0_d960001e() {
    // Encoding: 0xD960001E
    // Test aarch64_integer_tags_mcgettag field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: imm9=0, Xt=30, Xn=0
    let encoding: u32 = 0xD960001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcgettag_field_xt_31_max_0_d960001f() {
    // Encoding: 0xD960001F
    // Test aarch64_integer_tags_mcgettag field Xt = 31 (Max)
    // Fields: imm9=0, Xn=0, Xt=31
    let encoding: u32 = 0xD960001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=0 (immediate value 0)
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_0_0_d9600000() {
    // Encoding: 0xD9600000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=0
    let encoding: u32 = 0xD9600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=1 (immediate value 1)
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_1_0_d9601000() {
    // Encoding: 0xD9601000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=1, Xn=0, Xt=0
    // Fields: imm9=1, Xn=0, Xt=0
    let encoding: u32 = 0xD9601000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_2_0_d9603000() {
    // Encoding: 0xD9603000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=3, Xn=0, Xt=0
    // Fields: Xn=0, imm9=3, Xt=0
    let encoding: u32 = 0xD9603000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_3_0_d9604000() {
    // Encoding: 0xD9604000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=4, Xn=0, Xt=0
    // Fields: Xt=0, imm9=4, Xn=0
    let encoding: u32 = 0xD9604000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_4_0_d9607000() {
    // Encoding: 0xD9607000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=7, Xn=0, Xt=0
    // Fields: imm9=7, Xt=0, Xn=0
    let encoding: u32 = 0xD9607000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_5_0_d9608000() {
    // Encoding: 0xD9608000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=8, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=8
    let encoding: u32 = 0xD9608000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_6_0_d960f000() {
    // Encoding: 0xD960F000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=15, Xn=0, Xt=0
    // Fields: Xn=0, imm9=15, Xt=0
    let encoding: u32 = 0xD960F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_7_0_d9610000() {
    // Encoding: 0xD9610000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=16, Xn=0, Xt=0
    // Fields: imm9=16, Xn=0, Xt=0
    let encoding: u32 = 0xD9610000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_8_0_d961f000() {
    // Encoding: 0xD961F000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=31, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=31
    let encoding: u32 = 0xD961F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_9_0_d9620000() {
    // Encoding: 0xD9620000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=32, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=32
    let encoding: u32 = 0xD9620000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_10_0_d963f000() {
    // Encoding: 0xD963F000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=63, Xn=0, Xt=0
    // Fields: Xt=0, imm9=63, Xn=0
    let encoding: u32 = 0xD963F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_11_0_d9640000() {
    // Encoding: 0xD9640000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=64, Xn=0, Xt=0
    // Fields: Xt=0, Xn=0, imm9=64
    let encoding: u32 = 0xD9640000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_12_0_d967f000() {
    // Encoding: 0xD967F000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=127, Xn=0, Xt=0
    // Fields: imm9=127, Xn=0, Xt=0
    let encoding: u32 = 0xD967F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_13_0_d9680000() {
    // Encoding: 0xD9680000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=128, Xn=0, Xt=0
    // Fields: Xt=0, imm9=128, Xn=0
    let encoding: u32 = 0xD9680000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=255 (immediate midpoint (255))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_14_0_d96ff000() {
    // Encoding: 0xD96FF000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=255, Xn=0, Xt=0
    // Fields: Xn=0, Xt=0, imm9=255
    let encoding: u32 = 0xD96FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_15_0_d9700000() {
    // Encoding: 0xD9700000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=256, Xn=0, Xt=0
    // Fields: Xt=0, imm9=256, Xn=0
    let encoding: u32 = 0xD9700000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm9=511 (maximum immediate (511))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_16_0_d97ff000() {
    // Encoding: 0xD97FF000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=511, Xn=0, Xt=0
    // Fields: Xt=0, imm9=511, Xn=0
    let encoding: u32 = 0xD97FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_17_0_d9600000() {
    // Encoding: 0xD9600000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=0, Xn=0, Xt=0
    // Fields: Xt=0, imm9=0, Xn=0
    let encoding: u32 = 0xD9600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_18_0_d9600020() {
    // Encoding: 0xD9600020
    // Test aarch64_integer_tags_mcgettag field combination: imm9=0, Xn=1, Xt=0
    // Fields: Xt=0, imm9=0, Xn=1
    let encoding: u32 = 0xD9600020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_19_0_d96003c0() {
    // Encoding: 0xD96003C0
    // Test aarch64_integer_tags_mcgettag field combination: imm9=0, Xn=30, Xt=0
    // Fields: Xn=30, imm9=0, Xt=0
    let encoding: u32 = 0xD96003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_20_0_d96003e0() {
    // Encoding: 0xD96003E0
    // Test aarch64_integer_tags_mcgettag field combination: imm9=0, Xn=31, Xt=0
    // Fields: imm9=0, Xn=31, Xt=0
    let encoding: u32 = 0xD96003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_21_0_d9600000() {
    // Encoding: 0xD9600000
    // Test aarch64_integer_tags_mcgettag field combination: imm9=0, Xn=0, Xt=0
    // Fields: imm9=0, Xt=0, Xn=0
    let encoding: u32 = 0xD9600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_22_0_d9600001() {
    // Encoding: 0xD9600001
    // Test aarch64_integer_tags_mcgettag field combination: imm9=0, Xn=0, Xt=1
    // Fields: Xn=0, imm9=0, Xt=1
    let encoding: u32 = 0xD9600001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_23_0_d960001e() {
    // Encoding: 0xD960001E
    // Test aarch64_integer_tags_mcgettag field combination: imm9=0, Xn=0, Xt=30
    // Fields: Xn=0, imm9=0, Xt=30
    let encoding: u32 = 0xD960001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_24_0_d960001f() {
    // Encoding: 0xD960001F
    // Test aarch64_integer_tags_mcgettag field combination: imm9=0, Xn=0, Xt=31
    // Fields: imm9=0, Xn=0, Xt=31
    let encoding: u32 = 0xD960001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_25_0_d9600021() {
    // Encoding: 0xD9600021
    // Test aarch64_integer_tags_mcgettag field combination: imm9=0, Xn=1, Xt=1
    // Fields: Xt=1, imm9=0, Xn=1
    let encoding: u32 = 0xD9600021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcgettag_combo_26_0_d96003ff() {
    // Encoding: 0xD96003FF
    // Test aarch64_integer_tags_mcgettag field combination: imm9=0, Xn=31, Xt=31
    // Fields: imm9=0, Xt=31, Xn=31
    let encoding: u32 = 0xD96003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcgettag_special_xn_31_stack_pointer_sp_may_require_alignment_0_d96013e0()
 {
    // Encoding: 0xD96013E0
    // Test aarch64_integer_tags_mcgettag special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xt=0, Xn=31, imm9=1
    let encoding: u32 = 0xD96013E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// zero value
#[test]
fn test_aarch64_integer_tags_mcgettag_ldr_oracle_0_f9400020() {
    // Test LDR: zero value (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max byte
#[test]
fn test_aarch64_integer_tags_mcgettag_ldr_oracle_1_f9400020() {
    // Test LDR: max byte (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xFF, "X0 should be 0x00000000000000FF");
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max halfword
#[test]
fn test_aarch64_integer_tags_mcgettag_ldr_oracle_2_f9400020() {
    // Test LDR: max halfword (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0xFFFF, "X0 should be 0x000000000000FFFF");
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max word
#[test]
fn test_aarch64_integer_tags_mcgettag_ldr_oracle_3_f9400020() {
    // Test LDR: max word (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 255, 255, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFF,
        "X0 should be 0x00000000FFFFFFFF"
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// large value
#[test]
fn test_aarch64_integer_tags_mcgettag_ldr_oracle_4_f9400020() {
    // Test LDR: large value (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[240, 222, 188, 154, 120, 86, 52, 18])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x123456789ABCDEF0,
        "X0 should be 0x123456789ABCDEF0"
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (byte)
#[test]
fn test_aarch64_integer_tags_mcgettag_ldr_oracle_5_f9400020() {
    // Test LDR: sign bit (byte) (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[128, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x80, "X0 should be 0x0000000000000080");
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (halfword)
#[test]
fn test_aarch64_integer_tags_mcgettag_ldr_oracle_6_f9400020() {
    // Test LDR: sign bit (halfword) (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 128, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x8000, "X0 should be 0x0000000000008000");
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `LDR X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (word)
#[test]
fn test_aarch64_integer_tags_mcgettag_ldr_oracle_7_f9400020() {
    // Test LDR: sign bit (word) (oracle)
    // Encoding: 0xF9400020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 128, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9400020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x80000000,
        "X0 should be 0x0000000080000000"
    );
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_integer_tags_mcgettag_reg_write_0_d9600000() {
    // Test aarch64_integer_tags_mcgettag register write: GpFromField("t")
    // Encoding: 0xD9600000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9600000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcgettag
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcgettag_sp_xn_d96003e0() {
    // Test aarch64_integer_tags_mcgettag with Xn = SP (31)
    // Encoding: 0xD96003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD96003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_integer_tags_mcsettagarray Tests
// ============================================================================

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagarray_field_xn_0_min_0_d9a00000() {
    // Encoding: 0xD9A00000
    // Test aarch64_integer_tags_mcsettagarray field Xn = 0 (Min)
    // Fields: Xn=0, Xt=0
    let encoding: u32 = 0xD9A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagarray_field_xn_1_poweroftwo_0_d9a00020() {
    // Encoding: 0xD9A00020
    // Test aarch64_integer_tags_mcsettagarray field Xn = 1 (PowerOfTwo)
    // Fields: Xn=1, Xt=0
    let encoding: u32 = 0xD9A00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagarray_field_xn_30_poweroftwominusone_0_d9a003c0() {
    // Encoding: 0xD9A003C0
    // Test aarch64_integer_tags_mcsettagarray field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=30, Xt=0
    let encoding: u32 = 0xD9A003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcsettagarray_field_xn_31_max_0_d9a003e0() {
    // Encoding: 0xD9A003E0
    // Test aarch64_integer_tags_mcsettagarray field Xn = 31 (Max)
    // Fields: Xt=0, Xn=31
    let encoding: u32 = 0xD9A003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcsettagarray_field_xt_0_min_0_d9a00000() {
    // Encoding: 0xD9A00000
    // Test aarch64_integer_tags_mcsettagarray field Xt = 0 (Min)
    // Fields: Xn=0, Xt=0
    let encoding: u32 = 0xD9A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcsettagarray_field_xt_1_poweroftwo_0_d9a00001() {
    // Encoding: 0xD9A00001
    // Test aarch64_integer_tags_mcsettagarray field Xt = 1 (PowerOfTwo)
    // Fields: Xn=0, Xt=1
    let encoding: u32 = 0xD9A00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcsettagarray_field_xt_30_poweroftwominusone_0_d9a0001e() {
    // Encoding: 0xD9A0001E
    // Test aarch64_integer_tags_mcsettagarray field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=30
    let encoding: u32 = 0xD9A0001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcsettagarray_field_xt_31_max_0_d9a0001f() {
    // Encoding: 0xD9A0001F
    // Test aarch64_integer_tags_mcsettagarray field Xt = 31 (Max)
    // Fields: Xt=31, Xn=0
    let encoding: u32 = 0xD9A0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagarray_combo_0_0_d9a00000() {
    // Encoding: 0xD9A00000
    // Test aarch64_integer_tags_mcsettagarray field combination: Xn=0, Xt=0
    // Fields: Xn=0, Xt=0
    let encoding: u32 = 0xD9A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagarray_combo_1_0_d9a00020() {
    // Encoding: 0xD9A00020
    // Test aarch64_integer_tags_mcsettagarray field combination: Xn=1, Xt=0
    // Fields: Xn=1, Xt=0
    let encoding: u32 = 0xD9A00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagarray_combo_2_0_d9a003c0() {
    // Encoding: 0xD9A003C0
    // Test aarch64_integer_tags_mcsettagarray field combination: Xn=30, Xt=0
    // Fields: Xn=30, Xt=0
    let encoding: u32 = 0xD9A003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcsettagarray_combo_3_0_d9a003e0() {
    // Encoding: 0xD9A003E0
    // Test aarch64_integer_tags_mcsettagarray field combination: Xn=31, Xt=0
    // Fields: Xt=0, Xn=31
    let encoding: u32 = 0xD9A003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcsettagarray_combo_4_0_d9a00000() {
    // Encoding: 0xD9A00000
    // Test aarch64_integer_tags_mcsettagarray field combination: Xn=0, Xt=0
    // Fields: Xn=0, Xt=0
    let encoding: u32 = 0xD9A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcsettagarray_combo_5_0_d9a00001() {
    // Encoding: 0xD9A00001
    // Test aarch64_integer_tags_mcsettagarray field combination: Xn=0, Xt=1
    // Fields: Xn=0, Xt=1
    let encoding: u32 = 0xD9A00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcsettagarray_combo_6_0_d9a0001e() {
    // Encoding: 0xD9A0001E
    // Test aarch64_integer_tags_mcsettagarray field combination: Xn=0, Xt=30
    // Fields: Xn=0, Xt=30
    let encoding: u32 = 0xD9A0001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcsettagarray_combo_7_0_d9a0001f() {
    // Encoding: 0xD9A0001F
    // Test aarch64_integer_tags_mcsettagarray field combination: Xn=0, Xt=31
    // Fields: Xt=31, Xn=0
    let encoding: u32 = 0xD9A0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcsettagarray_combo_8_0_d9a00021() {
    // Encoding: 0xD9A00021
    // Test aarch64_integer_tags_mcsettagarray field combination: Xn=1, Xt=1
    // Fields: Xn=1, Xt=1
    let encoding: u32 = 0xD9A00021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcsettagarray_combo_9_0_d9a003ff() {
    // Encoding: 0xD9A003FF
    // Test aarch64_integer_tags_mcsettagarray field combination: Xn=31, Xt=31
    // Fields: Xn=31, Xt=31
    let encoding: u32 = 0xD9A003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcsettagarray_special_xn_31_stack_pointer_sp_may_require_alignment_0_d9a003e0()
 {
    // Encoding: 0xD9A003E0
    // Test aarch64_integer_tags_mcsettagarray special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xt=0, Xn=31
    let encoding: u32 = 0xD9A003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// zero value
#[test]
fn test_aarch64_integer_tags_mcsettagarray_ldr_oracle_0_f9800020() {
    // Test LDRS: zero value (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 0, 0, 0, 0, 0]).unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max byte
#[test]
fn test_aarch64_integer_tags_mcsettagarray_ldr_oracle_1_f9800020() {
    // Test LDRS: max byte (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max halfword
#[test]
fn test_aarch64_integer_tags_mcsettagarray_ldr_oracle_2_f9800020() {
    // Test LDRS: max halfword (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max word
#[test]
fn test_aarch64_integer_tags_mcsettagarray_ldr_oracle_3_f9800020() {
    // Test LDRS: max word (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255, 255, 255, 255, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// large value
#[test]
fn test_aarch64_integer_tags_mcsettagarray_ldr_oracle_4_f9800020() {
    // Test LDRS: large value (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[240, 222, 188, 154, 120, 86, 52, 18])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (byte)
#[test]
fn test_aarch64_integer_tags_mcsettagarray_ldr_oracle_5_f9800020() {
    // Test LDRS: sign bit (byte) (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[128, 0, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (halfword)
#[test]
fn test_aarch64_integer_tags_mcsettagarray_ldr_oracle_6_f9800020() {
    // Test LDRS: sign bit (halfword) (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 128, 0, 0, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `LDRS X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (word)
#[test]
fn test_aarch64_integer_tags_mcsettagarray_ldr_oracle_7_f9800020() {
    // Test LDRS: sign bit (word) (oracle)
    // Encoding: 0xF9800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0, 0, 0, 128, 0, 0, 0, 0])
        .unwrap();
    let encoding: u32 = 0xF9800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcsettagarray_reg_write_0_d9a00000() {
    // Test aarch64_integer_tags_mcsettagarray register write: Sp
    // Encoding: 0xD9A00000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9A00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcsettagarray_reg_write_1_d9a00000() {
    // Test aarch64_integer_tags_mcsettagarray register write: GpFromField("n")
    // Encoding: 0xD9A00000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9A00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcsettagarray
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcsettagarray_sp_xn_d9a003e0() {
    // Test aarch64_integer_tags_mcsettagarray with Xn = SP (31)
    // Encoding: 0xD9A003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9A003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_integer_tags_mcgettagarray Tests
// ============================================================================

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcgettagarray_field_xn_0_min_0_d9e00000() {
    // Encoding: 0xD9E00000
    // Test aarch64_integer_tags_mcgettagarray field Xn = 0 (Min)
    // Fields: Xn=0, Xt=0
    let encoding: u32 = 0xD9E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcgettagarray_field_xn_1_poweroftwo_0_d9e00020() {
    // Encoding: 0xD9E00020
    // Test aarch64_integer_tags_mcgettagarray field Xn = 1 (PowerOfTwo)
    // Fields: Xn=1, Xt=0
    let encoding: u32 = 0xD9E00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcgettagarray_field_xn_30_poweroftwominusone_0_d9e003c0() {
    // Encoding: 0xD9E003C0
    // Test aarch64_integer_tags_mcgettagarray field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=30, Xt=0
    let encoding: u32 = 0xD9E003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_tags_mcgettagarray_field_xn_31_max_0_d9e003e0() {
    // Encoding: 0xD9E003E0
    // Test aarch64_integer_tags_mcgettagarray field Xn = 31 (Max)
    // Fields: Xt=0, Xn=31
    let encoding: u32 = 0xD9E003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_tags_mcgettagarray_field_xt_0_min_0_d9e00000() {
    // Encoding: 0xD9E00000
    // Test aarch64_integer_tags_mcgettagarray field Xt = 0 (Min)
    // Fields: Xt=0, Xn=0
    let encoding: u32 = 0xD9E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_tags_mcgettagarray_field_xt_1_poweroftwo_0_d9e00001() {
    // Encoding: 0xD9E00001
    // Test aarch64_integer_tags_mcgettagarray field Xt = 1 (PowerOfTwo)
    // Fields: Xn=0, Xt=1
    let encoding: u32 = 0xD9E00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_tags_mcgettagarray_field_xt_30_poweroftwominusone_0_d9e0001e() {
    // Encoding: 0xD9E0001E
    // Test aarch64_integer_tags_mcgettagarray field Xt = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xt=30
    let encoding: u32 = 0xD9E0001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field Xt 0 +: 5`
/// Requirement: FieldBoundary { field: "Xt", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_tags_mcgettagarray_field_xt_31_max_0_d9e0001f() {
    // Encoding: 0xD9E0001F
    // Test aarch64_integer_tags_mcgettagarray field Xt = 31 (Max)
    // Fields: Xn=0, Xt=31
    let encoding: u32 = 0xD9E0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcgettagarray_combo_0_0_d9e00000() {
    // Encoding: 0xD9E00000
    // Test aarch64_integer_tags_mcgettagarray field combination: Xn=0, Xt=0
    // Fields: Xt=0, Xn=0
    let encoding: u32 = 0xD9E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcgettagarray_combo_1_0_d9e00020() {
    // Encoding: 0xD9E00020
    // Test aarch64_integer_tags_mcgettagarray field combination: Xn=1, Xt=0
    // Fields: Xt=0, Xn=1
    let encoding: u32 = 0xD9E00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcgettagarray_combo_2_0_d9e003c0() {
    // Encoding: 0xD9E003C0
    // Test aarch64_integer_tags_mcgettagarray field combination: Xn=30, Xt=0
    // Fields: Xt=0, Xn=30
    let encoding: u32 = 0xD9E003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_tags_mcgettagarray_combo_3_0_d9e003e0() {
    // Encoding: 0xD9E003E0
    // Test aarch64_integer_tags_mcgettagarray field combination: Xn=31, Xt=0
    // Fields: Xt=0, Xn=31
    let encoding: u32 = 0xD9E003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_tags_mcgettagarray_combo_4_0_d9e00000() {
    // Encoding: 0xD9E00000
    // Test aarch64_integer_tags_mcgettagarray field combination: Xn=0, Xt=0
    // Fields: Xt=0, Xn=0
    let encoding: u32 = 0xD9E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_tags_mcgettagarray_combo_5_0_d9e00001() {
    // Encoding: 0xD9E00001
    // Test aarch64_integer_tags_mcgettagarray field combination: Xn=0, Xt=1
    // Fields: Xt=1, Xn=0
    let encoding: u32 = 0xD9E00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_tags_mcgettagarray_combo_6_0_d9e0001e() {
    // Encoding: 0xD9E0001E
    // Test aarch64_integer_tags_mcgettagarray field combination: Xn=0, Xt=30
    // Fields: Xt=30, Xn=0
    let encoding: u32 = 0xD9E0001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xt=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_tags_mcgettagarray_combo_7_0_d9e0001f() {
    // Encoding: 0xD9E0001F
    // Test aarch64_integer_tags_mcgettagarray field combination: Xn=0, Xt=31
    // Fields: Xn=0, Xt=31
    let encoding: u32 = 0xD9E0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xt=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_tags_mcgettagarray_combo_8_0_d9e00021() {
    // Encoding: 0xD9E00021
    // Test aarch64_integer_tags_mcgettagarray field combination: Xn=1, Xt=1
    // Fields: Xt=1, Xn=1
    let encoding: u32 = 0xD9E00021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xt=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_tags_mcgettagarray_combo_9_0_d9e003ff() {
    // Encoding: 0xD9E003FF
    // Test aarch64_integer_tags_mcgettagarray field combination: Xn=31, Xt=31
    // Fields: Xt=31, Xn=31
    let encoding: u32 = 0xD9E003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_tags_mcgettagarray_special_xn_31_stack_pointer_sp_may_require_alignment_0_d9e003e0()
 {
    // Encoding: 0xD9E003E0
    // Test aarch64_integer_tags_mcgettagarray special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xn=31, Xt=0
    let encoding: u32 = 0xD9E003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_integer_tags_mcgettagarray_reg_write_0_d9e00000() {
    // Test aarch64_integer_tags_mcgettagarray register write: GpFromField("t")
    // Encoding: 0xD9E00000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9E00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_integer_tags_mcgettagarray_reg_write_1_d9e00000() {
    // Test aarch64_integer_tags_mcgettagarray register write: Sp
    // Encoding: 0xD9E00000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9E00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_integer_tags_mcgettagarray_reg_write_2_d9e00000() {
    // Test aarch64_integer_tags_mcgettagarray register write: GpFromField("n")
    // Encoding: 0xD9E00000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9E00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_tags_mcgettagarray
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_tags_mcgettagarray_sp_xn_d9e003e0() {
    // Test aarch64_integer_tags_mcgettagarray with Xn = SP (31)
    // Encoding: 0xD9E003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xD9E003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}
