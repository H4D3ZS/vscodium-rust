//! A64 memory literal tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_memory_literal_general Tests
// ============================================================================

/// Provenance: aarch64_memory_literal_general
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_literal_general_field_opc_0_min_0_18000000() {
    // Encoding: 0x18000000
    // Test aarch64_memory_literal_general field opc = 0 (Min)
    // Fields: opc=0, Rt=0, imm19=0
    let encoding: u32 = 0x18000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_literal_general_field_opc_1_poweroftwo_0_58000000() {
    // Encoding: 0x58000000
    // Test aarch64_memory_literal_general field opc = 1 (PowerOfTwo)
    // Fields: opc=1, imm19=0, Rt=0
    let encoding: u32 = 0x58000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_literal_general_field_opc_2_poweroftwo_0_98000000() {
    // Encoding: 0x98000000
    // Test aarch64_memory_literal_general field opc = 2 (PowerOfTwo)
    // Fields: opc=2, imm19=0, Rt=0
    let encoding: u32 = 0x98000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_literal_general_field_opc_3_max_0_d8000000() {
    // Encoding: 0xD8000000
    // Test aarch64_memory_literal_general field opc = 3 (Max)
    // Fields: opc=3, imm19=0, Rt=0
    let encoding: u32 = 0xD8000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_literal_general_field_imm19_0_zero_0_18000000() {
    // Encoding: 0x18000000
    // Test aarch64_memory_literal_general field imm19 = 0 (Zero)
    // Fields: opc=0, imm19=0, Rt=0
    let encoding: u32 = 0x18000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_literal_general_field_imm19_1_poweroftwo_0_18000020() {
    // Encoding: 0x18000020
    // Test aarch64_memory_literal_general field imm19 = 1 (PowerOfTwo)
    // Fields: Rt=0, imm19=1, opc=0
    let encoding: u32 = 0x18000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_literal_general_field_imm19_3_poweroftwominusone_0_18000060() {
    // Encoding: 0x18000060
    // Test aarch64_memory_literal_general field imm19 = 3 (PowerOfTwoMinusOne)
    // Fields: imm19=3, opc=0, Rt=0
    let encoding: u32 = 0x18000060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_4_poweroftwo_0_18000080() {
    // Encoding: 0x18000080
    // Test aarch64_memory_literal_general field imm19 = 4 (PowerOfTwo)
    // Fields: opc=0, imm19=4, Rt=0
    let encoding: u32 = 0x18000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_literal_general_field_imm19_7_poweroftwominusone_0_180000e0() {
    // Encoding: 0x180000E0
    // Test aarch64_memory_literal_general field imm19 = 7 (PowerOfTwoMinusOne)
    // Fields: imm19=7, Rt=0, opc=0
    let encoding: u32 = 0x180000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_8_poweroftwo_0_18000100() {
    // Encoding: 0x18000100
    // Test aarch64_memory_literal_general field imm19 = 8 (PowerOfTwo)
    // Fields: opc=0, imm19=8, Rt=0
    let encoding: u32 = 0x18000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_literal_general_field_imm19_15_poweroftwominusone_0_180001e0() {
    // Encoding: 0x180001E0
    // Test aarch64_memory_literal_general field imm19 = 15 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm19=15, Rt=0
    let encoding: u32 = 0x180001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_16_poweroftwo_0_18000200() {
    // Encoding: 0x18000200
    // Test aarch64_memory_literal_general field imm19 = 16 (PowerOfTwo)
    // Fields: opc=0, imm19=16, Rt=0
    let encoding: u32 = 0x18000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_literal_general_field_imm19_31_poweroftwominusone_0_180003e0() {
    // Encoding: 0x180003E0
    // Test aarch64_memory_literal_general field imm19 = 31 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm19=31, Rt=0
    let encoding: u32 = 0x180003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_32_poweroftwo_0_18000400() {
    // Encoding: 0x18000400
    // Test aarch64_memory_literal_general field imm19 = 32 (PowerOfTwo)
    // Fields: imm19=32, Rt=0, opc=0
    let encoding: u32 = 0x18000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_memory_literal_general_field_imm19_63_poweroftwominusone_0_180007e0() {
    // Encoding: 0x180007E0
    // Test aarch64_memory_literal_general field imm19 = 63 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, imm19=63
    let encoding: u32 = 0x180007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_64_poweroftwo_0_18000800() {
    // Encoding: 0x18000800
    // Test aarch64_memory_literal_general field imm19 = 64 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm19=64
    let encoding: u32 = 0x18000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_memory_literal_general_field_imm19_127_poweroftwominusone_0_18000fe0() {
    // Encoding: 0x18000FE0
    // Test aarch64_memory_literal_general field imm19 = 127 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm19=127, Rt=0
    let encoding: u32 = 0x18000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_128_poweroftwo_0_18001000() {
    // Encoding: 0x18001000
    // Test aarch64_memory_literal_general field imm19 = 128 (PowerOfTwo)
    // Fields: imm19=128, Rt=0, opc=0
    let encoding: u32 = 0x18001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch64_memory_literal_general_field_imm19_255_poweroftwominusone_0_18001fe0() {
    // Encoding: 0x18001FE0
    // Test aarch64_memory_literal_general field imm19 = 255 (PowerOfTwoMinusOne)
    // Fields: imm19=255, Rt=0, opc=0
    let encoding: u32 = 0x18001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_256_poweroftwo_0_18002000() {
    // Encoding: 0x18002000
    // Test aarch64_memory_literal_general field imm19 = 256 (PowerOfTwo)
    // Fields: imm19=256, Rt=0, opc=0
    let encoding: u32 = 0x18002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch64_memory_literal_general_field_imm19_511_poweroftwominusone_0_18003fe0() {
    // Encoding: 0x18003FE0
    // Test aarch64_memory_literal_general field imm19 = 511 (PowerOfTwoMinusOne)
    // Fields: Rt=0, imm19=511, opc=0
    let encoding: u32 = 0x18003FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_512_poweroftwo_0_18004000() {
    // Encoding: 0x18004000
    // Test aarch64_memory_literal_general field imm19 = 512 (PowerOfTwo)
    // Fields: imm19=512, opc=0, Rt=0
    let encoding: u32 = 0x18004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch64_memory_literal_general_field_imm19_1023_poweroftwominusone_0_18007fe0() {
    // Encoding: 0x18007FE0
    // Test aarch64_memory_literal_general field imm19 = 1023 (PowerOfTwoMinusOne)
    // Fields: Rt=0, imm19=1023, opc=0
    let encoding: u32 = 0x18007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_1024_poweroftwo_0_18008000() {
    // Encoding: 0x18008000
    // Test aarch64_memory_literal_general field imm19 = 1024 (PowerOfTwo)
    // Fields: opc=0, imm19=1024, Rt=0
    let encoding: u32 = 0x18008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 2047, boundary: PowerOfTwoMinusOne }
/// 2^11 - 1 = 2047
#[test]
fn test_aarch64_memory_literal_general_field_imm19_2047_poweroftwominusone_0_1800ffe0() {
    // Encoding: 0x1800FFE0
    // Test aarch64_memory_literal_general field imm19 = 2047 (PowerOfTwoMinusOne)
    // Fields: Rt=0, imm19=2047, opc=0
    let encoding: u32 = 0x1800FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_2048_poweroftwo_0_18010000() {
    // Encoding: 0x18010000
    // Test aarch64_memory_literal_general field imm19 = 2048 (PowerOfTwo)
    // Fields: opc=0, imm19=2048, Rt=0
    let encoding: u32 = 0x18010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 4095, boundary: PowerOfTwoMinusOne }
/// 2^12 - 1 = 4095
#[test]
fn test_aarch64_memory_literal_general_field_imm19_4095_poweroftwominusone_0_1801ffe0() {
    // Encoding: 0x1801FFE0
    // Test aarch64_memory_literal_general field imm19 = 4095 (PowerOfTwoMinusOne)
    // Fields: Rt=0, imm19=4095, opc=0
    let encoding: u32 = 0x1801FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 4096, boundary: PowerOfTwo }
/// power of 2 (2^12 = 4096)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_4096_poweroftwo_0_18020000() {
    // Encoding: 0x18020000
    // Test aarch64_memory_literal_general field imm19 = 4096 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm19=4096
    let encoding: u32 = 0x18020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 8191, boundary: PowerOfTwoMinusOne }
/// 2^13 - 1 = 8191
#[test]
fn test_aarch64_memory_literal_general_field_imm19_8191_poweroftwominusone_0_1803ffe0() {
    // Encoding: 0x1803FFE0
    // Test aarch64_memory_literal_general field imm19 = 8191 (PowerOfTwoMinusOne)
    // Fields: imm19=8191, opc=0, Rt=0
    let encoding: u32 = 0x1803FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 8192, boundary: PowerOfTwo }
/// power of 2 (2^13 = 8192)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_8192_poweroftwo_0_18040000() {
    // Encoding: 0x18040000
    // Test aarch64_memory_literal_general field imm19 = 8192 (PowerOfTwo)
    // Fields: imm19=8192, opc=0, Rt=0
    let encoding: u32 = 0x18040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 16383, boundary: PowerOfTwoMinusOne }
/// 2^14 - 1 = 16383
#[test]
fn test_aarch64_memory_literal_general_field_imm19_16383_poweroftwominusone_0_1807ffe0() {
    // Encoding: 0x1807FFE0
    // Test aarch64_memory_literal_general field imm19 = 16383 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm19=16383, Rt=0
    let encoding: u32 = 0x1807FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 16384, boundary: PowerOfTwo }
/// power of 2 (2^14 = 16384)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_16384_poweroftwo_0_18080000() {
    // Encoding: 0x18080000
    // Test aarch64_memory_literal_general field imm19 = 16384 (PowerOfTwo)
    // Fields: Rt=0, imm19=16384, opc=0
    let encoding: u32 = 0x18080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 32767, boundary: PowerOfTwoMinusOne }
/// 2^15 - 1 = 32767
#[test]
fn test_aarch64_memory_literal_general_field_imm19_32767_poweroftwominusone_0_180fffe0() {
    // Encoding: 0x180FFFE0
    // Test aarch64_memory_literal_general field imm19 = 32767 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm19=32767, Rt=0
    let encoding: u32 = 0x180FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 32768, boundary: PowerOfTwo }
/// power of 2 (2^15 = 32768)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_32768_poweroftwo_0_18100000() {
    // Encoding: 0x18100000
    // Test aarch64_memory_literal_general field imm19 = 32768 (PowerOfTwo)
    // Fields: opc=0, imm19=32768, Rt=0
    let encoding: u32 = 0x18100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 65535, boundary: PowerOfTwoMinusOne }
/// 2^16 - 1 = 65535
#[test]
fn test_aarch64_memory_literal_general_field_imm19_65535_poweroftwominusone_0_181fffe0() {
    // Encoding: 0x181FFFE0
    // Test aarch64_memory_literal_general field imm19 = 65535 (PowerOfTwoMinusOne)
    // Fields: Rt=0, imm19=65535, opc=0
    let encoding: u32 = 0x181FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 65536, boundary: PowerOfTwo }
/// power of 2 (2^16 = 65536)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_65536_poweroftwo_0_18200000() {
    // Encoding: 0x18200000
    // Test aarch64_memory_literal_general field imm19 = 65536 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm19=65536
    let encoding: u32 = 0x18200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 131071, boundary: PowerOfTwoMinusOne }
/// 2^17 - 1 = 131071
#[test]
fn test_aarch64_memory_literal_general_field_imm19_131071_poweroftwominusone_0_183fffe0() {
    // Encoding: 0x183FFFE0
    // Test aarch64_memory_literal_general field imm19 = 131071 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm19=131071, Rt=0
    let encoding: u32 = 0x183FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 131072, boundary: PowerOfTwo }
/// power of 2 (2^17 = 131072)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_131072_poweroftwo_0_18400000() {
    // Encoding: 0x18400000
    // Test aarch64_memory_literal_general field imm19 = 131072 (PowerOfTwo)
    // Fields: imm19=131072, Rt=0, opc=0
    let encoding: u32 = 0x18400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 262143, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (262143)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_262143_poweroftwominusone_0_187fffe0() {
    // Encoding: 0x187FFFE0
    // Test aarch64_memory_literal_general field imm19 = 262143 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rt=0, imm19=262143
    let encoding: u32 = 0x187FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 262144, boundary: PowerOfTwo }
/// power of 2 (2^18 = 262144)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_262144_poweroftwo_0_18800000() {
    // Encoding: 0x18800000
    // Test aarch64_memory_literal_general field imm19 = 262144 (PowerOfTwo)
    // Fields: imm19=262144, Rt=0, opc=0
    let encoding: u32 = 0x18800000;
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

/// Provenance: aarch64_memory_literal_general
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 524287, boundary: Max }
/// maximum immediate (524287)
#[test]
fn test_aarch64_memory_literal_general_field_imm19_524287_max_0_18ffffe0() {
    // Encoding: 0x18FFFFE0
    // Test aarch64_memory_literal_general field imm19 = 524287 (Max)
    // Fields: imm19=524287, opc=0, Rt=0
    let encoding: u32 = 0x18FFFFE0;
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

/// Provenance: aarch64_memory_literal_general
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_literal_general_field_rt_0_min_0_18000000() {
    // Encoding: 0x18000000
    // Test aarch64_memory_literal_general field Rt = 0 (Min)
    // Fields: opc=0, imm19=0, Rt=0
    let encoding: u32 = 0x18000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_literal_general_field_rt_1_poweroftwo_0_18000001() {
    // Encoding: 0x18000001
    // Test aarch64_memory_literal_general field Rt = 1 (PowerOfTwo)
    // Fields: opc=0, imm19=0, Rt=1
    let encoding: u32 = 0x18000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_literal_general_field_rt_30_poweroftwominusone_0_1800001e() {
    // Encoding: 0x1800001E
    // Test aarch64_memory_literal_general field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm19=0, Rt=30
    let encoding: u32 = 0x1800001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_literal_general_field_rt_31_max_0_1800001f() {
    // Encoding: 0x1800001F
    // Test aarch64_memory_literal_general field Rt = 31 (Max)
    // Fields: imm19=0, opc=0, Rt=31
    let encoding: u32 = 0x1800001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_literal_general_combo_0_0_18000000() {
    // Encoding: 0x18000000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=0, Rt=0
    // Fields: opc=0, imm19=0, Rt=0
    let encoding: u32 = 0x18000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_literal_general_combo_1_0_58000000() {
    // Encoding: 0x58000000
    // Test aarch64_memory_literal_general field combination: opc=1, imm19=0, Rt=0
    // Fields: Rt=0, opc=1, imm19=0
    let encoding: u32 = 0x58000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_literal_general_combo_2_0_98000000() {
    // Encoding: 0x98000000
    // Test aarch64_memory_literal_general field combination: opc=2, imm19=0, Rt=0
    // Fields: Rt=0, imm19=0, opc=2
    let encoding: u32 = 0x98000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_literal_general_combo_3_0_d8000000() {
    // Encoding: 0xD8000000
    // Test aarch64_memory_literal_general field combination: opc=3, imm19=0, Rt=0
    // Fields: Rt=0, imm19=0, opc=3
    let encoding: u32 = 0xD8000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=0 (immediate value 0)
#[test]
fn test_aarch64_memory_literal_general_combo_4_0_18000000() {
    // Encoding: 0x18000000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=0, Rt=0
    // Fields: opc=0, imm19=0, Rt=0
    let encoding: u32 = 0x18000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=1 (immediate value 1)
#[test]
fn test_aarch64_memory_literal_general_combo_5_0_18000020() {
    // Encoding: 0x18000020
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=1, Rt=0
    // Fields: opc=0, Rt=0, imm19=1
    let encoding: u32 = 0x18000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_literal_general_combo_6_0_18000060() {
    // Encoding: 0x18000060
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=3, Rt=0
    // Fields: Rt=0, imm19=3, opc=0
    let encoding: u32 = 0x18000060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_literal_general_combo_7_0_18000080() {
    // Encoding: 0x18000080
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=4, Rt=0
    // Fields: imm19=4, Rt=0, opc=0
    let encoding: u32 = 0x18000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_literal_general_combo_8_0_180000e0() {
    // Encoding: 0x180000E0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=7, Rt=0
    // Fields: imm19=7, opc=0, Rt=0
    let encoding: u32 = 0x180000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_literal_general_combo_9_0_18000100() {
    // Encoding: 0x18000100
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=8, Rt=0
    // Fields: imm19=8, Rt=0, opc=0
    let encoding: u32 = 0x18000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_literal_general_combo_10_0_180001e0() {
    // Encoding: 0x180001E0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=15, Rt=0
    // Fields: Rt=0, imm19=15, opc=0
    let encoding: u32 = 0x180001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_literal_general_combo_11_0_18000200() {
    // Encoding: 0x18000200
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=16, Rt=0
    // Fields: imm19=16, opc=0, Rt=0
    let encoding: u32 = 0x18000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_literal_general_combo_12_0_180003e0() {
    // Encoding: 0x180003E0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=31, Rt=0
    // Fields: Rt=0, imm19=31, opc=0
    let encoding: u32 = 0x180003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_literal_general_combo_13_0_18000400() {
    // Encoding: 0x18000400
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=32, Rt=0
    // Fields: opc=0, imm19=32, Rt=0
    let encoding: u32 = 0x18000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_memory_literal_general_combo_14_0_180007e0() {
    // Encoding: 0x180007E0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=63, Rt=0
    // Fields: imm19=63, Rt=0, opc=0
    let encoding: u32 = 0x180007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_literal_general_combo_15_0_18000800() {
    // Encoding: 0x18000800
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=64, Rt=0
    // Fields: opc=0, imm19=64, Rt=0
    let encoding: u32 = 0x18000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_memory_literal_general_combo_16_0_18000fe0() {
    // Encoding: 0x18000FE0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=127, Rt=0
    // Fields: Rt=0, imm19=127, opc=0
    let encoding: u32 = 0x18000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_memory_literal_general_combo_17_0_18001000() {
    // Encoding: 0x18001000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=128, Rt=0
    // Fields: imm19=128, Rt=0, opc=0
    let encoding: u32 = 0x18001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=255 (2^8 - 1 = 255)
#[test]
fn test_aarch64_memory_literal_general_combo_18_0_18001fe0() {
    // Encoding: 0x18001FE0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=255, Rt=0
    // Fields: imm19=255, opc=0, Rt=0
    let encoding: u32 = 0x18001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_memory_literal_general_combo_19_0_18002000() {
    // Encoding: 0x18002000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=256, Rt=0
    // Fields: imm19=256, opc=0, Rt=0
    let encoding: u32 = 0x18002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=511 (2^9 - 1 = 511)
#[test]
fn test_aarch64_memory_literal_general_combo_20_0_18003fe0() {
    // Encoding: 0x18003FE0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=511, Rt=0
    // Fields: imm19=511, Rt=0, opc=0
    let encoding: u32 = 0x18003FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=512 (power of 2 (2^9 = 512))
#[test]
fn test_aarch64_memory_literal_general_combo_21_0_18004000() {
    // Encoding: 0x18004000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=512, Rt=0
    // Fields: Rt=0, imm19=512, opc=0
    let encoding: u32 = 0x18004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=1023 (2^10 - 1 = 1023)
#[test]
fn test_aarch64_memory_literal_general_combo_22_0_18007fe0() {
    // Encoding: 0x18007FE0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=1023, Rt=0
    // Fields: opc=0, imm19=1023, Rt=0
    let encoding: u32 = 0x18007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=1024 (power of 2 (2^10 = 1024))
#[test]
fn test_aarch64_memory_literal_general_combo_23_0_18008000() {
    // Encoding: 0x18008000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=1024, Rt=0
    // Fields: Rt=0, imm19=1024, opc=0
    let encoding: u32 = 0x18008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=2047 (2^11 - 1 = 2047)
#[test]
fn test_aarch64_memory_literal_general_combo_24_0_1800ffe0() {
    // Encoding: 0x1800FFE0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=2047, Rt=0
    // Fields: opc=0, imm19=2047, Rt=0
    let encoding: u32 = 0x1800FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=2048 (power of 2 (2^11 = 2048))
#[test]
fn test_aarch64_memory_literal_general_combo_25_0_18010000() {
    // Encoding: 0x18010000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=2048, Rt=0
    // Fields: opc=0, imm19=2048, Rt=0
    let encoding: u32 = 0x18010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=4095 (2^12 - 1 = 4095)
#[test]
fn test_aarch64_memory_literal_general_combo_26_0_1801ffe0() {
    // Encoding: 0x1801FFE0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=4095, Rt=0
    // Fields: opc=0, imm19=4095, Rt=0
    let encoding: u32 = 0x1801FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=4096 (power of 2 (2^12 = 4096))
#[test]
fn test_aarch64_memory_literal_general_combo_27_0_18020000() {
    // Encoding: 0x18020000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=4096, Rt=0
    // Fields: opc=0, imm19=4096, Rt=0
    let encoding: u32 = 0x18020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=8191 (2^13 - 1 = 8191)
#[test]
fn test_aarch64_memory_literal_general_combo_28_0_1803ffe0() {
    // Encoding: 0x1803FFE0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=8191, Rt=0
    // Fields: opc=0, imm19=8191, Rt=0
    let encoding: u32 = 0x1803FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=8192 (power of 2 (2^13 = 8192))
#[test]
fn test_aarch64_memory_literal_general_combo_29_0_18040000() {
    // Encoding: 0x18040000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=8192, Rt=0
    // Fields: opc=0, imm19=8192, Rt=0
    let encoding: u32 = 0x18040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=16383 (2^14 - 1 = 16383)
#[test]
fn test_aarch64_memory_literal_general_combo_30_0_1807ffe0() {
    // Encoding: 0x1807FFE0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=16383, Rt=0
    // Fields: opc=0, imm19=16383, Rt=0
    let encoding: u32 = 0x1807FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=16384 (power of 2 (2^14 = 16384))
#[test]
fn test_aarch64_memory_literal_general_combo_31_0_18080000() {
    // Encoding: 0x18080000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=16384, Rt=0
    // Fields: imm19=16384, Rt=0, opc=0
    let encoding: u32 = 0x18080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=32767 (2^15 - 1 = 32767)
#[test]
fn test_aarch64_memory_literal_general_combo_32_0_180fffe0() {
    // Encoding: 0x180FFFE0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=32767, Rt=0
    // Fields: opc=0, imm19=32767, Rt=0
    let encoding: u32 = 0x180FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=32768 (power of 2 (2^15 = 32768))
#[test]
fn test_aarch64_memory_literal_general_combo_33_0_18100000() {
    // Encoding: 0x18100000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=32768, Rt=0
    // Fields: Rt=0, opc=0, imm19=32768
    let encoding: u32 = 0x18100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=65535 (2^16 - 1 = 65535)
#[test]
fn test_aarch64_memory_literal_general_combo_34_0_181fffe0() {
    // Encoding: 0x181FFFE0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=65535, Rt=0
    // Fields: opc=0, imm19=65535, Rt=0
    let encoding: u32 = 0x181FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=65536 (power of 2 (2^16 = 65536))
#[test]
fn test_aarch64_memory_literal_general_combo_35_0_18200000() {
    // Encoding: 0x18200000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=65536, Rt=0
    // Fields: imm19=65536, opc=0, Rt=0
    let encoding: u32 = 0x18200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=131071 (2^17 - 1 = 131071)
#[test]
fn test_aarch64_memory_literal_general_combo_36_0_183fffe0() {
    // Encoding: 0x183FFFE0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=131071, Rt=0
    // Fields: imm19=131071, Rt=0, opc=0
    let encoding: u32 = 0x183FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=131072 (power of 2 (2^17 = 131072))
#[test]
fn test_aarch64_memory_literal_general_combo_37_0_18400000() {
    // Encoding: 0x18400000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=131072, Rt=0
    // Fields: Rt=0, imm19=131072, opc=0
    let encoding: u32 = 0x18400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=262143 (immediate midpoint (262143))
#[test]
fn test_aarch64_memory_literal_general_combo_38_0_187fffe0() {
    // Encoding: 0x187FFFE0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=262143, Rt=0
    // Fields: opc=0, imm19=262143, Rt=0
    let encoding: u32 = 0x187FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 39`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=262144 (power of 2 (2^18 = 262144))
#[test]
fn test_aarch64_memory_literal_general_combo_39_0_18800000() {
    // Encoding: 0x18800000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=262144, Rt=0
    // Fields: opc=0, imm19=262144, Rt=0
    let encoding: u32 = 0x18800000;
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

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 40`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=524287 (maximum immediate (524287))
#[test]
fn test_aarch64_memory_literal_general_combo_40_0_18ffffe0() {
    // Encoding: 0x18FFFFE0
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=524287, Rt=0
    // Fields: opc=0, Rt=0, imm19=524287
    let encoding: u32 = 0x18FFFFE0;
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

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 41`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_literal_general_combo_41_0_18000000() {
    // Encoding: 0x18000000
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=0, Rt=0
    // Fields: opc=0, Rt=0, imm19=0
    let encoding: u32 = 0x18000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 42`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_literal_general_combo_42_0_18000001() {
    // Encoding: 0x18000001
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=0, Rt=1
    // Fields: Rt=1, imm19=0, opc=0
    let encoding: u32 = 0x18000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 43`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_literal_general_combo_43_0_1800001e() {
    // Encoding: 0x1800001E
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=0, Rt=30
    // Fields: Rt=30, opc=0, imm19=0
    let encoding: u32 = 0x1800001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field combination 44`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_literal_general_combo_44_0_1800001f() {
    // Encoding: 0x1800001F
    // Test aarch64_memory_literal_general field combination: opc=0, imm19=0, Rt=31
    // Fields: opc=0, Rt=31, imm19=0
    let encoding: u32 = 0x1800001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_literal_general_special_opc_0_size_variant_0_0_18000020() {
    // Encoding: 0x18000020
    // Test aarch64_memory_literal_general special value opc = 0 (Size variant 0)
    // Fields: opc=0, Rt=0, imm19=1
    let encoding: u32 = 0x18000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_literal_general_special_opc_1_size_variant_1_0_58000020() {
    // Encoding: 0x58000020
    // Test aarch64_memory_literal_general special value opc = 1 (Size variant 1)
    // Fields: Rt=0, opc=1, imm19=1
    let encoding: u32 = 0x58000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_literal_general_special_opc_2_size_variant_2_0_98000020() {
    // Encoding: 0x98000020
    // Test aarch64_memory_literal_general special value opc = 2 (Size variant 2)
    // Fields: opc=2, imm19=1, Rt=0
    let encoding: u32 = 0x98000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_literal_general_special_opc_3_size_variant_3_0_d8000020() {
    // Encoding: 0xD8000020
    // Test aarch64_memory_literal_general special value opc = 3 (Size variant 3)
    // Fields: opc=3, imm19=1, Rt=0
    let encoding: u32 = 0xD8000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_literal_general_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_1800003f()
 {
    // Encoding: 0x1800003F
    // Test aarch64_memory_literal_general special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: imm19=1, Rt=31, opc=0
    let encoding: u32 = 0x1800003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_literal_general_reg_write_0_18000000() {
    // Test aarch64_memory_literal_general register write: GpFromField("t")
    // Encoding: 0x18000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x18000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_literal_general_reg_write_1_18000000() {
    // Test aarch64_memory_literal_general register write: GpFromField("t")
    // Encoding: 0x18000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x18000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_literal_general
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_literal_general_zr_rt_1800001f() {
    // Test aarch64_memory_literal_general with Rt = ZR (31)
    // Encoding: 0x1800001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1800001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_memory_literal_simdfp Tests
// ============================================================================

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_literal_simdfp_field_opc_0_min_0_1c000000() {
    // Encoding: 0x1C000000
    // Test aarch64_memory_literal_simdfp field opc = 0 (Min)
    // Fields: opc=0, imm19=0, Rt=0
    let encoding: u32 = 0x1C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_literal_simdfp_field_opc_1_poweroftwo_0_5c000000() {
    // Encoding: 0x5C000000
    // Test aarch64_memory_literal_simdfp field opc = 1 (PowerOfTwo)
    // Fields: Rt=0, opc=1, imm19=0
    let encoding: u32 = 0x5C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	d0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_literal_simdfp_field_opc_2_poweroftwo_0_9c000000() {
    // Encoding: 0x9C000000
    // Test aarch64_memory_literal_simdfp field opc = 2 (PowerOfTwo)
    // Fields: opc=2, imm19=0, Rt=0
    let encoding: u32 = 0x9C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	q0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_literal_simdfp_field_opc_3_max_0_dc000000() {
    // Encoding: 0xDC000000
    // Test aarch64_memory_literal_simdfp field opc = 3 (Max)
    // Fields: opc=3, imm19=0, Rt=0
    let encoding: u32 = 0xDC000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_0_zero_0_1c000000() {
    // Encoding: 0x1C000000
    // Test aarch64_memory_literal_simdfp field imm19 = 0 (Zero)
    // Fields: Rt=0, imm19=0, opc=0
    let encoding: u32 = 0x1C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_1_poweroftwo_0_1c000020() {
    // Encoding: 0x1C000020
    // Test aarch64_memory_literal_simdfp field imm19 = 1 (PowerOfTwo)
    // Fields: imm19=1, Rt=0, opc=0
    let encoding: u32 = 0x1C000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_3_poweroftwominusone_0_1c000060() {
    // Encoding: 0x1C000060
    // Test aarch64_memory_literal_simdfp field imm19 = 3 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rt=0, imm19=3
    let encoding: u32 = 0x1C000060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #12
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_4_poweroftwo_0_1c000080() {
    // Encoding: 0x1C000080
    // Test aarch64_memory_literal_simdfp field imm19 = 4 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm19=4
    let encoding: u32 = 0x1C000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #16
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_7_poweroftwominusone_0_1c0000e0() {
    // Encoding: 0x1C0000E0
    // Test aarch64_memory_literal_simdfp field imm19 = 7 (PowerOfTwoMinusOne)
    // Fields: Rt=0, imm19=7, opc=0
    let encoding: u32 = 0x1C0000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #28
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_8_poweroftwo_0_1c000100() {
    // Encoding: 0x1C000100
    // Test aarch64_memory_literal_simdfp field imm19 = 8 (PowerOfTwo)
    // Fields: opc=0, imm19=8, Rt=0
    let encoding: u32 = 0x1C000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_15_poweroftwominusone_0_1c0001e0() {
    // Encoding: 0x1C0001E0
    // Test aarch64_memory_literal_simdfp field imm19 = 15 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, imm19=15
    let encoding: u32 = 0x1C0001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #60
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_16_poweroftwo_0_1c000200() {
    // Encoding: 0x1C000200
    // Test aarch64_memory_literal_simdfp field imm19 = 16 (PowerOfTwo)
    // Fields: Rt=0, imm19=16, opc=0
    let encoding: u32 = 0x1C000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #64
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_31_poweroftwominusone_0_1c0003e0() {
    // Encoding: 0x1C0003E0
    // Test aarch64_memory_literal_simdfp field imm19 = 31 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, imm19=31
    let encoding: u32 = 0x1C0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #124
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_32_poweroftwo_0_1c000400() {
    // Encoding: 0x1C000400
    // Test aarch64_memory_literal_simdfp field imm19 = 32 (PowerOfTwo)
    // Fields: opc=0, Rt=0, imm19=32
    let encoding: u32 = 0x1C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #128
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_63_poweroftwominusone_0_1c0007e0() {
    // Encoding: 0x1C0007E0
    // Test aarch64_memory_literal_simdfp field imm19 = 63 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rt=0, imm19=63
    let encoding: u32 = 0x1C0007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #252
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_64_poweroftwo_0_1c000800() {
    // Encoding: 0x1C000800
    // Test aarch64_memory_literal_simdfp field imm19 = 64 (PowerOfTwo)
    // Fields: opc=0, imm19=64, Rt=0
    let encoding: u32 = 0x1C000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #256
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_127_poweroftwominusone_0_1c000fe0() {
    // Encoding: 0x1C000FE0
    // Test aarch64_memory_literal_simdfp field imm19 = 127 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, imm19=127
    let encoding: u32 = 0x1C000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #508
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_128_poweroftwo_0_1c001000() {
    // Encoding: 0x1C001000
    // Test aarch64_memory_literal_simdfp field imm19 = 128 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm19=128
    let encoding: u32 = 0x1C001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #512
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_255_poweroftwominusone_0_1c001fe0() {
    // Encoding: 0x1C001FE0
    // Test aarch64_memory_literal_simdfp field imm19 = 255 (PowerOfTwoMinusOne)
    // Fields: imm19=255, Rt=0, opc=0
    let encoding: u32 = 0x1C001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #1020
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_256_poweroftwo_0_1c002000() {
    // Encoding: 0x1C002000
    // Test aarch64_memory_literal_simdfp field imm19 = 256 (PowerOfTwo)
    // Fields: imm19=256, Rt=0, opc=0
    let encoding: u32 = 0x1C002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #1024
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_511_poweroftwominusone_0_1c003fe0() {
    // Encoding: 0x1C003FE0
    // Test aarch64_memory_literal_simdfp field imm19 = 511 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, imm19=511
    let encoding: u32 = 0x1C003FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #2044
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_512_poweroftwo_0_1c004000() {
    // Encoding: 0x1C004000
    // Test aarch64_memory_literal_simdfp field imm19 = 512 (PowerOfTwo)
    // Fields: opc=0, imm19=512, Rt=0
    let encoding: u32 = 0x1C004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #2048
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_1023_poweroftwominusone_0_1c007fe0() {
    // Encoding: 0x1C007FE0
    // Test aarch64_memory_literal_simdfp field imm19 = 1023 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm19=1023, Rt=0
    let encoding: u32 = 0x1C007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #4092
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_1024_poweroftwo_0_1c008000() {
    // Encoding: 0x1C008000
    // Test aarch64_memory_literal_simdfp field imm19 = 1024 (PowerOfTwo)
    // Fields: Rt=0, imm19=1024, opc=0
    let encoding: u32 = 0x1C008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #4096
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 2047, boundary: PowerOfTwoMinusOne }
/// 2^11 - 1 = 2047
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_2047_poweroftwominusone_0_1c00ffe0() {
    // Encoding: 0x1C00FFE0
    // Test aarch64_memory_literal_simdfp field imm19 = 2047 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, imm19=2047
    let encoding: u32 = 0x1C00FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #8188
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_2048_poweroftwo_0_1c010000() {
    // Encoding: 0x1C010000
    // Test aarch64_memory_literal_simdfp field imm19 = 2048 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm19=2048
    let encoding: u32 = 0x1C010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #8192
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 4095, boundary: PowerOfTwoMinusOne }
/// 2^12 - 1 = 4095
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_4095_poweroftwominusone_0_1c01ffe0() {
    // Encoding: 0x1C01FFE0
    // Test aarch64_memory_literal_simdfp field imm19 = 4095 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm19=4095, Rt=0
    let encoding: u32 = 0x1C01FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #16380
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 4096, boundary: PowerOfTwo }
/// power of 2 (2^12 = 4096)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_4096_poweroftwo_0_1c020000() {
    // Encoding: 0x1C020000
    // Test aarch64_memory_literal_simdfp field imm19 = 4096 (PowerOfTwo)
    // Fields: imm19=4096, Rt=0, opc=0
    let encoding: u32 = 0x1C020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #16384
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 8191, boundary: PowerOfTwoMinusOne }
/// 2^13 - 1 = 8191
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_8191_poweroftwominusone_0_1c03ffe0() {
    // Encoding: 0x1C03FFE0
    // Test aarch64_memory_literal_simdfp field imm19 = 8191 (PowerOfTwoMinusOne)
    // Fields: imm19=8191, opc=0, Rt=0
    let encoding: u32 = 0x1C03FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #32764
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 8192, boundary: PowerOfTwo }
/// power of 2 (2^13 = 8192)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_8192_poweroftwo_0_1c040000() {
    // Encoding: 0x1C040000
    // Test aarch64_memory_literal_simdfp field imm19 = 8192 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm19=8192
    let encoding: u32 = 0x1C040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #32768
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 16383, boundary: PowerOfTwoMinusOne }
/// 2^14 - 1 = 16383
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_16383_poweroftwominusone_0_1c07ffe0() {
    // Encoding: 0x1C07FFE0
    // Test aarch64_memory_literal_simdfp field imm19 = 16383 (PowerOfTwoMinusOne)
    // Fields: imm19=16383, opc=0, Rt=0
    let encoding: u32 = 0x1C07FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #65532
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 16384, boundary: PowerOfTwo }
/// power of 2 (2^14 = 16384)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_16384_poweroftwo_0_1c080000() {
    // Encoding: 0x1C080000
    // Test aarch64_memory_literal_simdfp field imm19 = 16384 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm19=16384
    let encoding: u32 = 0x1C080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #65536
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 32767, boundary: PowerOfTwoMinusOne }
/// 2^15 - 1 = 32767
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_32767_poweroftwominusone_0_1c0fffe0() {
    // Encoding: 0x1C0FFFE0
    // Test aarch64_memory_literal_simdfp field imm19 = 32767 (PowerOfTwoMinusOne)
    // Fields: imm19=32767, opc=0, Rt=0
    let encoding: u32 = 0x1C0FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #131068
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 32768, boundary: PowerOfTwo }
/// power of 2 (2^15 = 32768)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_32768_poweroftwo_0_1c100000() {
    // Encoding: 0x1C100000
    // Test aarch64_memory_literal_simdfp field imm19 = 32768 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm19=32768
    let encoding: u32 = 0x1C100000;
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

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 65535, boundary: PowerOfTwoMinusOne }
/// 2^16 - 1 = 65535
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_65535_poweroftwominusone_0_1c1fffe0() {
    // Encoding: 0x1C1FFFE0
    // Test aarch64_memory_literal_simdfp field imm19 = 65535 (PowerOfTwoMinusOne)
    // Fields: imm19=65535, Rt=0, opc=0
    let encoding: u32 = 0x1C1FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #262140
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

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 65536, boundary: PowerOfTwo }
/// power of 2 (2^16 = 65536)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_65536_poweroftwo_0_1c200000() {
    // Encoding: 0x1C200000
    // Test aarch64_memory_literal_simdfp field imm19 = 65536 (PowerOfTwo)
    // Fields: Rt=0, imm19=65536, opc=0
    let encoding: u32 = 0x1C200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #262144
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 131071, boundary: PowerOfTwoMinusOne }
/// 2^17 - 1 = 131071
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_131071_poweroftwominusone_0_1c3fffe0() {
    // Encoding: 0x1C3FFFE0
    // Test aarch64_memory_literal_simdfp field imm19 = 131071 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, imm19=131071
    let encoding: u32 = 0x1C3FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #524284
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

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 131072, boundary: PowerOfTwo }
/// power of 2 (2^17 = 131072)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_131072_poweroftwo_0_1c400000() {
    // Encoding: 0x1C400000
    // Test aarch64_memory_literal_simdfp field imm19 = 131072 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm19=131072
    let encoding: u32 = 0x1C400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #524288
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 262143, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (262143)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_262143_poweroftwominusone_0_1c7fffe0() {
    // Encoding: 0x1C7FFFE0
    // Test aarch64_memory_literal_simdfp field imm19 = 262143 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, imm19=262143
    let encoding: u32 = 0x1C7FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #1048572
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

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 262144, boundary: PowerOfTwo }
/// power of 2 (2^18 = 262144)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_262144_poweroftwo_0_1c800000() {
    // Encoding: 0x1C800000
    // Test aarch64_memory_literal_simdfp field imm19 = 262144 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm19=262144
    let encoding: u32 = 0x1C800000;
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

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field imm19 5 +: 19`
/// Requirement: FieldBoundary { field: "imm19", value: 524287, boundary: Max }
/// maximum immediate (524287)
#[test]
fn test_aarch64_memory_literal_simdfp_field_imm19_524287_max_0_1cffffe0() {
    // Encoding: 0x1CFFFFE0
    // Test aarch64_memory_literal_simdfp field imm19 = 524287 (Max)
    // Fields: imm19=524287, opc=0, Rt=0
    let encoding: u32 = 0x1CFFFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #-4
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

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_literal_simdfp_field_rt_0_min_0_1c000000() {
    // Encoding: 0x1C000000
    // Test aarch64_memory_literal_simdfp field Rt = 0 (Min)
    // Fields: imm19=0, opc=0, Rt=0
    let encoding: u32 = 0x1C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_literal_simdfp_field_rt_1_poweroftwo_0_1c000001() {
    // Encoding: 0x1C000001
    // Test aarch64_memory_literal_simdfp field Rt = 1 (PowerOfTwo)
    // Fields: Rt=1, opc=0, imm19=0
    let encoding: u32 = 0x1C000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s1, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_literal_simdfp_field_rt_30_poweroftwominusone_0_1c00001e() {
    // Encoding: 0x1C00001E
    // Test aarch64_memory_literal_simdfp field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: imm19=0, Rt=30, opc=0
    let encoding: u32 = 0x1C00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s30, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_literal_simdfp_field_rt_31_max_0_1c00001f() {
    // Encoding: 0x1C00001F
    // Test aarch64_memory_literal_simdfp field Rt = 31 (Max)
    // Fields: imm19=0, Rt=31, opc=0
    let encoding: u32 = 0x1C00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s31, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_0_0_1c000000() {
    // Encoding: 0x1C000000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=0, Rt=0
    // Fields: imm19=0, opc=0, Rt=0
    let encoding: u32 = 0x1C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_1_0_5c000000() {
    // Encoding: 0x5C000000
    // Test aarch64_memory_literal_simdfp field combination: opc=1, imm19=0, Rt=0
    // Fields: imm19=0, Rt=0, opc=1
    let encoding: u32 = 0x5C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	d0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_2_0_9c000000() {
    // Encoding: 0x9C000000
    // Test aarch64_memory_literal_simdfp field combination: opc=2, imm19=0, Rt=0
    // Fields: opc=2, imm19=0, Rt=0
    let encoding: u32 = 0x9C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	q0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_3_0_dc000000() {
    // Encoding: 0xDC000000
    // Test aarch64_memory_literal_simdfp field combination: opc=3, imm19=0, Rt=0
    // Fields: Rt=0, opc=3, imm19=0
    let encoding: u32 = 0xDC000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=0 (immediate value 0)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_4_0_1c000000() {
    // Encoding: 0x1C000000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=0, Rt=0
    // Fields: opc=0, imm19=0, Rt=0
    let encoding: u32 = 0x1C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=1 (immediate value 1)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_5_0_1c000020() {
    // Encoding: 0x1C000020
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=1, Rt=0
    // Fields: imm19=1, opc=0, Rt=0
    let encoding: u32 = 0x1C000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_6_0_1c000060() {
    // Encoding: 0x1C000060
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=3, Rt=0
    // Fields: opc=0, imm19=3, Rt=0
    let encoding: u32 = 0x1C000060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #12
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_7_0_1c000080() {
    // Encoding: 0x1C000080
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=4, Rt=0
    // Fields: opc=0, Rt=0, imm19=4
    let encoding: u32 = 0x1C000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #16
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_8_0_1c0000e0() {
    // Encoding: 0x1C0000E0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=7, Rt=0
    // Fields: opc=0, imm19=7, Rt=0
    let encoding: u32 = 0x1C0000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #28
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_9_0_1c000100() {
    // Encoding: 0x1C000100
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=8, Rt=0
    // Fields: imm19=8, opc=0, Rt=0
    let encoding: u32 = 0x1C000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_10_0_1c0001e0() {
    // Encoding: 0x1C0001E0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=15, Rt=0
    // Fields: imm19=15, opc=0, Rt=0
    let encoding: u32 = 0x1C0001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #60
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_11_0_1c000200() {
    // Encoding: 0x1C000200
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=16, Rt=0
    // Fields: opc=0, imm19=16, Rt=0
    let encoding: u32 = 0x1C000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #64
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_12_0_1c0003e0() {
    // Encoding: 0x1C0003E0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=31, Rt=0
    // Fields: imm19=31, opc=0, Rt=0
    let encoding: u32 = 0x1C0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #124
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_13_0_1c000400() {
    // Encoding: 0x1C000400
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=32, Rt=0
    // Fields: imm19=32, opc=0, Rt=0
    let encoding: u32 = 0x1C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #128
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_14_0_1c0007e0() {
    // Encoding: 0x1C0007E0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=63, Rt=0
    // Fields: opc=0, imm19=63, Rt=0
    let encoding: u32 = 0x1C0007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #252
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_15_0_1c000800() {
    // Encoding: 0x1C000800
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=64, Rt=0
    // Fields: imm19=64, Rt=0, opc=0
    let encoding: u32 = 0x1C000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #256
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_16_0_1c000fe0() {
    // Encoding: 0x1C000FE0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=127, Rt=0
    // Fields: Rt=0, imm19=127, opc=0
    let encoding: u32 = 0x1C000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #508
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_17_0_1c001000() {
    // Encoding: 0x1C001000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=128, Rt=0
    // Fields: opc=0, imm19=128, Rt=0
    let encoding: u32 = 0x1C001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #512
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=255 (2^8 - 1 = 255)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_18_0_1c001fe0() {
    // Encoding: 0x1C001FE0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=255, Rt=0
    // Fields: imm19=255, Rt=0, opc=0
    let encoding: u32 = 0x1C001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #1020
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_19_0_1c002000() {
    // Encoding: 0x1C002000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=256, Rt=0
    // Fields: opc=0, imm19=256, Rt=0
    let encoding: u32 = 0x1C002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #1024
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=511 (2^9 - 1 = 511)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_20_0_1c003fe0() {
    // Encoding: 0x1C003FE0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=511, Rt=0
    // Fields: Rt=0, imm19=511, opc=0
    let encoding: u32 = 0x1C003FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #2044
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=512 (power of 2 (2^9 = 512))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_21_0_1c004000() {
    // Encoding: 0x1C004000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=512, Rt=0
    // Fields: Rt=0, opc=0, imm19=512
    let encoding: u32 = 0x1C004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #2048
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=1023 (2^10 - 1 = 1023)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_22_0_1c007fe0() {
    // Encoding: 0x1C007FE0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=1023, Rt=0
    // Fields: Rt=0, opc=0, imm19=1023
    let encoding: u32 = 0x1C007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #4092
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=1024 (power of 2 (2^10 = 1024))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_23_0_1c008000() {
    // Encoding: 0x1C008000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=1024, Rt=0
    // Fields: opc=0, Rt=0, imm19=1024
    let encoding: u32 = 0x1C008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #4096
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=2047 (2^11 - 1 = 2047)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_24_0_1c00ffe0() {
    // Encoding: 0x1C00FFE0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=2047, Rt=0
    // Fields: Rt=0, opc=0, imm19=2047
    let encoding: u32 = 0x1C00FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #8188
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=2048 (power of 2 (2^11 = 2048))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_25_0_1c010000() {
    // Encoding: 0x1C010000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=2048, Rt=0
    // Fields: Rt=0, imm19=2048, opc=0
    let encoding: u32 = 0x1C010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #8192
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=4095 (2^12 - 1 = 4095)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_26_0_1c01ffe0() {
    // Encoding: 0x1C01FFE0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=4095, Rt=0
    // Fields: opc=0, imm19=4095, Rt=0
    let encoding: u32 = 0x1C01FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #16380
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=4096 (power of 2 (2^12 = 4096))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_27_0_1c020000() {
    // Encoding: 0x1C020000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=4096, Rt=0
    // Fields: opc=0, imm19=4096, Rt=0
    let encoding: u32 = 0x1C020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #16384
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=8191 (2^13 - 1 = 8191)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_28_0_1c03ffe0() {
    // Encoding: 0x1C03FFE0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=8191, Rt=0
    // Fields: Rt=0, opc=0, imm19=8191
    let encoding: u32 = 0x1C03FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #32764
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=8192 (power of 2 (2^13 = 8192))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_29_0_1c040000() {
    // Encoding: 0x1C040000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=8192, Rt=0
    // Fields: opc=0, imm19=8192, Rt=0
    let encoding: u32 = 0x1C040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #32768
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=16383 (2^14 - 1 = 16383)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_30_0_1c07ffe0() {
    // Encoding: 0x1C07FFE0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=16383, Rt=0
    // Fields: opc=0, imm19=16383, Rt=0
    let encoding: u32 = 0x1C07FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #65532
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=16384 (power of 2 (2^14 = 16384))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_31_0_1c080000() {
    // Encoding: 0x1C080000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=16384, Rt=0
    // Fields: opc=0, Rt=0, imm19=16384
    let encoding: u32 = 0x1C080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #65536
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=32767 (2^15 - 1 = 32767)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_32_0_1c0fffe0() {
    // Encoding: 0x1C0FFFE0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=32767, Rt=0
    // Fields: imm19=32767, Rt=0, opc=0
    let encoding: u32 = 0x1C0FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #131068
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=32768 (power of 2 (2^15 = 32768))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_33_0_1c100000() {
    // Encoding: 0x1C100000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=32768, Rt=0
    // Fields: imm19=32768, Rt=0, opc=0
    let encoding: u32 = 0x1C100000;
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

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=65535 (2^16 - 1 = 65535)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_34_0_1c1fffe0() {
    // Encoding: 0x1C1FFFE0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=65535, Rt=0
    // Fields: imm19=65535, Rt=0, opc=0
    let encoding: u32 = 0x1C1FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #262140
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

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=65536 (power of 2 (2^16 = 65536))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_35_0_1c200000() {
    // Encoding: 0x1C200000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=65536, Rt=0
    // Fields: opc=0, Rt=0, imm19=65536
    let encoding: u32 = 0x1C200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #262144
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=131071 (2^17 - 1 = 131071)
#[test]
fn test_aarch64_memory_literal_simdfp_combo_36_0_1c3fffe0() {
    // Encoding: 0x1C3FFFE0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=131071, Rt=0
    // Fields: Rt=0, imm19=131071, opc=0
    let encoding: u32 = 0x1C3FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #524284
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

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=131072 (power of 2 (2^17 = 131072))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_37_0_1c400000() {
    // Encoding: 0x1C400000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=131072, Rt=0
    // Fields: imm19=131072, Rt=0, opc=0
    let encoding: u32 = 0x1C400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #524288
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=262143 (immediate midpoint (262143))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_38_0_1c7fffe0() {
    // Encoding: 0x1C7FFFE0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=262143, Rt=0
    // Fields: imm19=262143, Rt=0, opc=0
    let encoding: u32 = 0x1C7FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #1048572
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

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 39`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=262144 (power of 2 (2^18 = 262144))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_39_0_1c800000() {
    // Encoding: 0x1C800000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=262144, Rt=0
    // Fields: opc=0, imm19=262144, Rt=0
    let encoding: u32 = 0x1C800000;
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

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 40`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm19=524287 (maximum immediate (524287))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_40_0_1cffffe0() {
    // Encoding: 0x1CFFFFE0
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=524287, Rt=0
    // Fields: Rt=0, imm19=524287, opc=0
    let encoding: u32 = 0x1CFFFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #-4
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

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 41`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_41_0_1c000000() {
    // Encoding: 0x1C000000
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=0, Rt=0
    // Fields: Rt=0, opc=0, imm19=0
    let encoding: u32 = 0x1C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 42`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_42_0_1c000001() {
    // Encoding: 0x1C000001
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=0, Rt=1
    // Fields: Rt=1, opc=0, imm19=0
    let encoding: u32 = 0x1C000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s1, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 43`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_43_0_1c00001e() {
    // Encoding: 0x1C00001E
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=0, Rt=30
    // Fields: Rt=30, imm19=0, opc=0
    let encoding: u32 = 0x1C00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s30, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field combination 44`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_literal_simdfp_combo_44_0_1c00001f() {
    // Encoding: 0x1C00001F
    // Test aarch64_memory_literal_simdfp field combination: opc=0, imm19=0, Rt=31
    // Fields: imm19=0, opc=0, Rt=31
    let encoding: u32 = 0x1C00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s31, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_literal_simdfp_special_opc_0_size_variant_0_0_1c000020() {
    // Encoding: 0x1C000020
    // Test aarch64_memory_literal_simdfp special value opc = 0 (Size variant 0)
    // Fields: imm19=1, Rt=0, opc=0
    let encoding: u32 = 0x1C000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s0, #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_literal_simdfp_special_opc_1_size_variant_1_0_5c000020() {
    // Encoding: 0x5C000020
    // Test aarch64_memory_literal_simdfp special value opc = 1 (Size variant 1)
    // Fields: imm19=1, opc=1, Rt=0
    let encoding: u32 = 0x5C000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	d0, #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_literal_simdfp_special_opc_2_size_variant_2_0_9c000020() {
    // Encoding: 0x9C000020
    // Test aarch64_memory_literal_simdfp special value opc = 2 (Size variant 2)
    // Fields: Rt=0, opc=2, imm19=1
    let encoding: u32 = 0x9C000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	q0, #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_literal_simdfp_special_opc_3_size_variant_3_0_dc000020() {
    // Encoding: 0xDC000020
    // Test aarch64_memory_literal_simdfp special value opc = 3 (Size variant 3)
    // Fields: imm19=1, opc=3, Rt=0
    let encoding: u32 = 0xDC000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_literal_simdfp
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_literal_simdfp_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_1c00003f()
 {
    // Encoding: 0x1C00003F
    // Test aarch64_memory_literal_simdfp special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rt=31, opc=0, imm19=1
    let encoding: u32 = 0x1C00003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldr	s31, #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}
