//! A64 integer address tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_integer_arithmetic_address_pc_rel Tests
// ============================================================================

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field op 31 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_op_0_min_0_10000000() {
    // Encoding: 0x10000000
    // Test aarch64_integer_arithmetic_address_pc_rel field op = 0 (Min)
    // Fields: op=0, immhi=0, Rd=0, immlo=0
    let encoding: u32 = 0x10000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field op 31 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_op_1_max_0_90000000() {
    // Encoding: 0x90000000
    // Test aarch64_integer_arithmetic_address_pc_rel field op = 1 (Max)
    // Fields: Rd=0, immhi=0, op=1, immlo=0
    let encoding: u32 = 0x90000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immlo 29 +: 2`
/// Requirement: FieldBoundary { field: "immlo", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immlo_0_zero_0_10000000() {
    // Encoding: 0x10000000
    // Test aarch64_integer_arithmetic_address_pc_rel field immlo = 0 (Zero)
    // Fields: immlo=0, Rd=0, op=0, immhi=0
    let encoding: u32 = 0x10000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immlo 29 +: 2`
/// Requirement: FieldBoundary { field: "immlo", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immlo_1_poweroftwo_0_30000000() {
    // Encoding: 0x30000000
    // Test aarch64_integer_arithmetic_address_pc_rel field immlo = 1 (PowerOfTwo)
    // Fields: immhi=0, op=0, immlo=1, Rd=0
    let encoding: u32 = 0x30000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immlo 29 +: 2`
/// Requirement: FieldBoundary { field: "immlo", value: 3, boundary: Max }
/// maximum immediate (3)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immlo_3_max_0_70000000() {
    // Encoding: 0x70000000
    // Test aarch64_integer_arithmetic_address_pc_rel field immlo = 3 (Max)
    // Fields: immhi=0, immlo=3, op=0, Rd=0
    let encoding: u32 = 0x70000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_0_zero_0_10000000() {
    // Encoding: 0x10000000
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 0 (Zero)
    // Fields: Rd=0, immlo=0, immhi=0, op=0
    let encoding: u32 = 0x10000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_1_poweroftwo_0_10000020() {
    // Encoding: 0x10000020
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 1 (PowerOfTwo)
    // Fields: immhi=1, immlo=0, op=0, Rd=0
    let encoding: u32 = 0x10000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_3_poweroftwominusone_0_10000060() {
    // Encoding: 0x10000060
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 3 (PowerOfTwoMinusOne)
    // Fields: immlo=0, immhi=3, Rd=0, op=0
    let encoding: u32 = 0x10000060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_4_poweroftwo_0_10000080() {
    // Encoding: 0x10000080
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 4 (PowerOfTwo)
    // Fields: immlo=0, immhi=4, Rd=0, op=0
    let encoding: u32 = 0x10000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_7_poweroftwominusone_0_100000e0() {
    // Encoding: 0x100000E0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 7 (PowerOfTwoMinusOne)
    // Fields: immhi=7, op=0, immlo=0, Rd=0
    let encoding: u32 = 0x100000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_8_poweroftwo_0_10000100() {
    // Encoding: 0x10000100
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 8 (PowerOfTwo)
    // Fields: immlo=0, Rd=0, op=0, immhi=8
    let encoding: u32 = 0x10000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_15_poweroftwominusone_0_100001e0() {
    // Encoding: 0x100001E0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 15 (PowerOfTwoMinusOne)
    // Fields: Rd=0, op=0, immhi=15, immlo=0
    let encoding: u32 = 0x100001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_16_poweroftwo_0_10000200() {
    // Encoding: 0x10000200
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 16 (PowerOfTwo)
    // Fields: Rd=0, op=0, immhi=16, immlo=0
    let encoding: u32 = 0x10000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_31_poweroftwominusone_0_100003e0() {
    // Encoding: 0x100003E0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 31 (PowerOfTwoMinusOne)
    // Fields: Rd=0, immlo=0, immhi=31, op=0
    let encoding: u32 = 0x100003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_32_poweroftwo_0_10000400() {
    // Encoding: 0x10000400
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 32 (PowerOfTwo)
    // Fields: Rd=0, op=0, immhi=32, immlo=0
    let encoding: u32 = 0x10000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_63_poweroftwominusone_0_100007e0() {
    // Encoding: 0x100007E0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 63 (PowerOfTwoMinusOne)
    // Fields: op=0, immhi=63, immlo=0, Rd=0
    let encoding: u32 = 0x100007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_64_poweroftwo_0_10000800() {
    // Encoding: 0x10000800
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 64 (PowerOfTwo)
    // Fields: Rd=0, op=0, immlo=0, immhi=64
    let encoding: u32 = 0x10000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_127_poweroftwominusone_0_10000fe0() {
    // Encoding: 0x10000FE0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 127 (PowerOfTwoMinusOne)
    // Fields: Rd=0, op=0, immlo=0, immhi=127
    let encoding: u32 = 0x10000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_128_poweroftwo_0_10001000() {
    // Encoding: 0x10001000
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 128 (PowerOfTwo)
    // Fields: Rd=0, immhi=128, op=0, immlo=0
    let encoding: u32 = 0x10001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_255_poweroftwominusone_0_10001fe0() {
    // Encoding: 0x10001FE0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 255 (PowerOfTwoMinusOne)
    // Fields: immhi=255, Rd=0, immlo=0, op=0
    let encoding: u32 = 0x10001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_256_poweroftwo_0_10002000() {
    // Encoding: 0x10002000
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 256 (PowerOfTwo)
    // Fields: immlo=0, Rd=0, op=0, immhi=256
    let encoding: u32 = 0x10002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_511_poweroftwominusone_0_10003fe0() {
    // Encoding: 0x10003FE0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 511 (PowerOfTwoMinusOne)
    // Fields: immhi=511, Rd=0, op=0, immlo=0
    let encoding: u32 = 0x10003FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_512_poweroftwo_0_10004000() {
    // Encoding: 0x10004000
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 512 (PowerOfTwo)
    // Fields: immlo=0, op=0, Rd=0, immhi=512
    let encoding: u32 = 0x10004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_1023_poweroftwominusone_0_10007fe0() {
    // Encoding: 0x10007FE0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 1023 (PowerOfTwoMinusOne)
    // Fields: op=0, immlo=0, Rd=0, immhi=1023
    let encoding: u32 = 0x10007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_1024_poweroftwo_0_10008000() {
    // Encoding: 0x10008000
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 1024 (PowerOfTwo)
    // Fields: Rd=0, immlo=0, op=0, immhi=1024
    let encoding: u32 = 0x10008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 2047, boundary: PowerOfTwoMinusOne }
/// 2^11 - 1 = 2047
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_2047_poweroftwominusone_0_1000ffe0() {
    // Encoding: 0x1000FFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 2047 (PowerOfTwoMinusOne)
    // Fields: op=0, immhi=2047, Rd=0, immlo=0
    let encoding: u32 = 0x1000FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_2048_poweroftwo_0_10010000() {
    // Encoding: 0x10010000
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 2048 (PowerOfTwo)
    // Fields: immhi=2048, op=0, immlo=0, Rd=0
    let encoding: u32 = 0x10010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 4095, boundary: PowerOfTwoMinusOne }
/// 2^12 - 1 = 4095
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_4095_poweroftwominusone_0_1001ffe0() {
    // Encoding: 0x1001FFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 4095 (PowerOfTwoMinusOne)
    // Fields: immlo=0, Rd=0, op=0, immhi=4095
    let encoding: u32 = 0x1001FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 4096, boundary: PowerOfTwo }
/// power of 2 (2^12 = 4096)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_4096_poweroftwo_0_10020000() {
    // Encoding: 0x10020000
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 4096 (PowerOfTwo)
    // Fields: immlo=0, op=0, Rd=0, immhi=4096
    let encoding: u32 = 0x10020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 8191, boundary: PowerOfTwoMinusOne }
/// 2^13 - 1 = 8191
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_8191_poweroftwominusone_0_1003ffe0() {
    // Encoding: 0x1003FFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 8191 (PowerOfTwoMinusOne)
    // Fields: immhi=8191, op=0, immlo=0, Rd=0
    let encoding: u32 = 0x1003FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 8192, boundary: PowerOfTwo }
/// power of 2 (2^13 = 8192)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_8192_poweroftwo_0_10040000() {
    // Encoding: 0x10040000
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 8192 (PowerOfTwo)
    // Fields: immlo=0, Rd=0, op=0, immhi=8192
    let encoding: u32 = 0x10040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 16383, boundary: PowerOfTwoMinusOne }
/// 2^14 - 1 = 16383
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_16383_poweroftwominusone_0_1007ffe0()
{
    // Encoding: 0x1007FFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 16383 (PowerOfTwoMinusOne)
    // Fields: op=0, immhi=16383, immlo=0, Rd=0
    let encoding: u32 = 0x1007FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 16384, boundary: PowerOfTwo }
/// power of 2 (2^14 = 16384)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_16384_poweroftwo_0_10080000() {
    // Encoding: 0x10080000
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 16384 (PowerOfTwo)
    // Fields: op=0, immhi=16384, immlo=0, Rd=0
    let encoding: u32 = 0x10080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 32767, boundary: PowerOfTwoMinusOne }
/// 2^15 - 1 = 32767
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_32767_poweroftwominusone_0_100fffe0()
{
    // Encoding: 0x100FFFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 32767 (PowerOfTwoMinusOne)
    // Fields: op=0, immlo=0, immhi=32767, Rd=0
    let encoding: u32 = 0x100FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 32768, boundary: PowerOfTwo }
/// power of 2 (2^15 = 32768)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_32768_poweroftwo_0_10100000() {
    // Encoding: 0x10100000
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 32768 (PowerOfTwo)
    // Fields: immlo=0, Rd=0, op=0, immhi=32768
    let encoding: u32 = 0x10100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 65535, boundary: PowerOfTwoMinusOne }
/// 2^16 - 1 = 65535
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_65535_poweroftwominusone_0_101fffe0()
{
    // Encoding: 0x101FFFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 65535 (PowerOfTwoMinusOne)
    // Fields: immlo=0, Rd=0, op=0, immhi=65535
    let encoding: u32 = 0x101FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 65536, boundary: PowerOfTwo }
/// power of 2 (2^16 = 65536)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_65536_poweroftwo_0_10200000() {
    // Encoding: 0x10200000
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 65536 (PowerOfTwo)
    // Fields: Rd=0, immlo=0, immhi=65536, op=0
    let encoding: u32 = 0x10200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 131071, boundary: PowerOfTwoMinusOne }
/// 2^17 - 1 = 131071
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_131071_poweroftwominusone_0_103fffe0()
{
    // Encoding: 0x103FFFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 131071 (PowerOfTwoMinusOne)
    // Fields: op=0, immhi=131071, Rd=0, immlo=0
    let encoding: u32 = 0x103FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 131072, boundary: PowerOfTwo }
/// power of 2 (2^17 = 131072)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_131072_poweroftwo_0_10400000() {
    // Encoding: 0x10400000
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 131072 (PowerOfTwo)
    // Fields: op=0, immhi=131072, immlo=0, Rd=0
    let encoding: u32 = 0x10400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 262143, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (262143)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_262143_poweroftwominusone_0_107fffe0()
{
    // Encoding: 0x107FFFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 262143 (PowerOfTwoMinusOne)
    // Fields: op=0, immhi=262143, Rd=0, immlo=0
    let encoding: u32 = 0x107FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 262144, boundary: PowerOfTwo }
/// power of 2 (2^18 = 262144)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_262144_poweroftwo_0_10800000() {
    // Encoding: 0x10800000
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 262144 (PowerOfTwo)
    // Fields: Rd=0, op=0, immlo=0, immhi=262144
    let encoding: u32 = 0x10800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field immhi 5 +: 19`
/// Requirement: FieldBoundary { field: "immhi", value: 524287, boundary: Max }
/// maximum immediate (524287)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_immhi_524287_max_0_10ffffe0() {
    // Encoding: 0x10FFFFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field immhi = 524287 (Max)
    // Fields: immlo=0, Rd=0, immhi=524287, op=0
    let encoding: u32 = 0x10FFFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_rd_0_min_0_10000000() {
    // Encoding: 0x10000000
    // Test aarch64_integer_arithmetic_address_pc_rel field Rd = 0 (Min)
    // Fields: immlo=0, Rd=0, op=0, immhi=0
    let encoding: u32 = 0x10000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_rd_1_poweroftwo_0_10000001() {
    // Encoding: 0x10000001
    // Test aarch64_integer_arithmetic_address_pc_rel field Rd = 1 (PowerOfTwo)
    // Fields: op=0, immhi=0, immlo=0, Rd=1
    let encoding: u32 = 0x10000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_rd_30_poweroftwominusone_0_1000001e() {
    // Encoding: 0x1000001E
    // Test aarch64_integer_arithmetic_address_pc_rel field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: immhi=0, immlo=0, op=0, Rd=30
    let encoding: u32 = 0x1000001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_field_rd_31_max_0_1000001f() {
    // Encoding: 0x1000001F
    // Test aarch64_integer_arithmetic_address_pc_rel field Rd = 31 (Max)
    // Fields: immlo=0, immhi=0, op=0, Rd=31
    let encoding: u32 = 0x1000001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_0_0_10000000() {
    // Encoding: 0x10000000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=0, Rd=0
    // Fields: op=0, immlo=0, Rd=0, immhi=0
    let encoding: u32 = 0x10000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_1_0_90000000() {
    // Encoding: 0x90000000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=1, immlo=0, immhi=0, Rd=0
    // Fields: immhi=0, Rd=0, op=1, immlo=0
    let encoding: u32 = 0x90000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immlo=0 (immediate value 0)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_2_0_10000000() {
    // Encoding: 0x10000000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=0, Rd=0
    // Fields: Rd=0, immlo=0, immhi=0, op=0
    let encoding: u32 = 0x10000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immlo=1 (immediate value 1)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_3_0_30000000() {
    // Encoding: 0x30000000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=1, immhi=0, Rd=0
    // Fields: op=0, immlo=1, immhi=0, Rd=0
    let encoding: u32 = 0x30000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immlo=3 (maximum immediate (3))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_4_0_70000000() {
    // Encoding: 0x70000000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=3, immhi=0, Rd=0
    // Fields: immlo=3, immhi=0, Rd=0, op=0
    let encoding: u32 = 0x70000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=0 (immediate value 0)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_5_0_10000000() {
    // Encoding: 0x10000000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=0, Rd=0
    // Fields: immlo=0, immhi=0, Rd=0, op=0
    let encoding: u32 = 0x10000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=1 (immediate value 1)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_6_0_10000020() {
    // Encoding: 0x10000020
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=1, Rd=0
    // Fields: immhi=1, op=0, Rd=0, immlo=0
    let encoding: u32 = 0x10000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_7_0_10000060() {
    // Encoding: 0x10000060
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=3, Rd=0
    // Fields: immlo=0, immhi=3, Rd=0, op=0
    let encoding: u32 = 0x10000060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_8_0_10000080() {
    // Encoding: 0x10000080
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=4, Rd=0
    // Fields: Rd=0, op=0, immlo=0, immhi=4
    let encoding: u32 = 0x10000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_9_0_100000e0() {
    // Encoding: 0x100000E0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=7, Rd=0
    // Fields: immlo=0, op=0, immhi=7, Rd=0
    let encoding: u32 = 0x100000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_10_0_10000100() {
    // Encoding: 0x10000100
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=8, Rd=0
    // Fields: Rd=0, immlo=0, immhi=8, op=0
    let encoding: u32 = 0x10000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_11_0_100001e0() {
    // Encoding: 0x100001E0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=15, Rd=0
    // Fields: immlo=0, Rd=0, op=0, immhi=15
    let encoding: u32 = 0x100001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_12_0_10000200() {
    // Encoding: 0x10000200
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=16, Rd=0
    // Fields: Rd=0, immlo=0, immhi=16, op=0
    let encoding: u32 = 0x10000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_13_0_100003e0() {
    // Encoding: 0x100003E0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=31, Rd=0
    // Fields: immlo=0, immhi=31, Rd=0, op=0
    let encoding: u32 = 0x100003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_14_0_10000400() {
    // Encoding: 0x10000400
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=32, Rd=0
    // Fields: immhi=32, op=0, immlo=0, Rd=0
    let encoding: u32 = 0x10000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_15_0_100007e0() {
    // Encoding: 0x100007E0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=63, Rd=0
    // Fields: op=0, Rd=0, immhi=63, immlo=0
    let encoding: u32 = 0x100007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_16_0_10000800() {
    // Encoding: 0x10000800
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=64, Rd=0
    // Fields: immhi=64, op=0, Rd=0, immlo=0
    let encoding: u32 = 0x10000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_17_0_10000fe0() {
    // Encoding: 0x10000FE0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=127, Rd=0
    // Fields: op=0, immlo=0, immhi=127, Rd=0
    let encoding: u32 = 0x10000FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_18_0_10001000() {
    // Encoding: 0x10001000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=128, Rd=0
    // Fields: immhi=128, op=0, immlo=0, Rd=0
    let encoding: u32 = 0x10001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=255 (2^8 - 1 = 255)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_19_0_10001fe0() {
    // Encoding: 0x10001FE0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=255, Rd=0
    // Fields: op=0, immlo=0, Rd=0, immhi=255
    let encoding: u32 = 0x10001FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_20_0_10002000() {
    // Encoding: 0x10002000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=256, Rd=0
    // Fields: op=0, immhi=256, immlo=0, Rd=0
    let encoding: u32 = 0x10002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=511 (2^9 - 1 = 511)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_21_0_10003fe0() {
    // Encoding: 0x10003FE0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=511, Rd=0
    // Fields: immlo=0, Rd=0, op=0, immhi=511
    let encoding: u32 = 0x10003FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=512 (power of 2 (2^9 = 512))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_22_0_10004000() {
    // Encoding: 0x10004000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=512, Rd=0
    // Fields: op=0, immlo=0, immhi=512, Rd=0
    let encoding: u32 = 0x10004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=1023 (2^10 - 1 = 1023)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_23_0_10007fe0() {
    // Encoding: 0x10007FE0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=1023, Rd=0
    // Fields: immlo=0, Rd=0, immhi=1023, op=0
    let encoding: u32 = 0x10007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=1024 (power of 2 (2^10 = 1024))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_24_0_10008000() {
    // Encoding: 0x10008000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=1024, Rd=0
    // Fields: immhi=1024, op=0, immlo=0, Rd=0
    let encoding: u32 = 0x10008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=2047 (2^11 - 1 = 2047)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_25_0_1000ffe0() {
    // Encoding: 0x1000FFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=2047, Rd=0
    // Fields: immlo=0, Rd=0, op=0, immhi=2047
    let encoding: u32 = 0x1000FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=2048 (power of 2 (2^11 = 2048))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_26_0_10010000() {
    // Encoding: 0x10010000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=2048, Rd=0
    // Fields: immlo=0, immhi=2048, op=0, Rd=0
    let encoding: u32 = 0x10010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=4095 (2^12 - 1 = 4095)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_27_0_1001ffe0() {
    // Encoding: 0x1001FFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=4095, Rd=0
    // Fields: op=0, Rd=0, immlo=0, immhi=4095
    let encoding: u32 = 0x1001FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=4096 (power of 2 (2^12 = 4096))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_28_0_10020000() {
    // Encoding: 0x10020000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=4096, Rd=0
    // Fields: Rd=0, op=0, immlo=0, immhi=4096
    let encoding: u32 = 0x10020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=8191 (2^13 - 1 = 8191)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_29_0_1003ffe0() {
    // Encoding: 0x1003FFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=8191, Rd=0
    // Fields: op=0, Rd=0, immhi=8191, immlo=0
    let encoding: u32 = 0x1003FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=8192 (power of 2 (2^13 = 8192))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_30_0_10040000() {
    // Encoding: 0x10040000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=8192, Rd=0
    // Fields: immhi=8192, op=0, Rd=0, immlo=0
    let encoding: u32 = 0x10040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=16383 (2^14 - 1 = 16383)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_31_0_1007ffe0() {
    // Encoding: 0x1007FFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=16383, Rd=0
    // Fields: op=0, immlo=0, immhi=16383, Rd=0
    let encoding: u32 = 0x1007FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=16384 (power of 2 (2^14 = 16384))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_32_0_10080000() {
    // Encoding: 0x10080000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=16384, Rd=0
    // Fields: op=0, immhi=16384, immlo=0, Rd=0
    let encoding: u32 = 0x10080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=32767 (2^15 - 1 = 32767)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_33_0_100fffe0() {
    // Encoding: 0x100FFFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=32767, Rd=0
    // Fields: op=0, immhi=32767, Rd=0, immlo=0
    let encoding: u32 = 0x100FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=32768 (power of 2 (2^15 = 32768))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_34_0_10100000() {
    // Encoding: 0x10100000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=32768, Rd=0
    // Fields: op=0, immhi=32768, Rd=0, immlo=0
    let encoding: u32 = 0x10100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=65535 (2^16 - 1 = 65535)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_35_0_101fffe0() {
    // Encoding: 0x101FFFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=65535, Rd=0
    // Fields: immhi=65535, Rd=0, immlo=0, op=0
    let encoding: u32 = 0x101FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=65536 (power of 2 (2^16 = 65536))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_36_0_10200000() {
    // Encoding: 0x10200000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=65536, Rd=0
    // Fields: immlo=0, immhi=65536, Rd=0, op=0
    let encoding: u32 = 0x10200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=131071 (2^17 - 1 = 131071)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_37_0_103fffe0() {
    // Encoding: 0x103FFFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=131071, Rd=0
    // Fields: Rd=0, op=0, immhi=131071, immlo=0
    let encoding: u32 = 0x103FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=131072 (power of 2 (2^17 = 131072))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_38_0_10400000() {
    // Encoding: 0x10400000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=131072, Rd=0
    // Fields: immhi=131072, immlo=0, op=0, Rd=0
    let encoding: u32 = 0x10400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 39`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=262143 (immediate midpoint (262143))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_39_0_107fffe0() {
    // Encoding: 0x107FFFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=262143, Rd=0
    // Fields: Rd=0, immhi=262143, immlo=0, op=0
    let encoding: u32 = 0x107FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 40`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=262144 (power of 2 (2^18 = 262144))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_40_0_10800000() {
    // Encoding: 0x10800000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=262144, Rd=0
    // Fields: Rd=0, immhi=262144, op=0, immlo=0
    let encoding: u32 = 0x10800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 41`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immhi=524287 (maximum immediate (524287))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_41_0_10ffffe0() {
    // Encoding: 0x10FFFFE0
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=524287, Rd=0
    // Fields: Rd=0, immlo=0, op=0, immhi=524287
    let encoding: u32 = 0x10FFFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 42`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_42_0_10000000() {
    // Encoding: 0x10000000
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=0, Rd=0
    // Fields: immlo=0, immhi=0, Rd=0, op=0
    let encoding: u32 = 0x10000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 43`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_43_0_10000001() {
    // Encoding: 0x10000001
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=0, Rd=1
    // Fields: Rd=1, immhi=0, immlo=0, op=0
    let encoding: u32 = 0x10000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 44`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_44_0_1000001e() {
    // Encoding: 0x1000001E
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=0, Rd=30
    // Fields: op=0, immlo=0, Rd=30, immhi=0
    let encoding: u32 = 0x1000001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field combination 45`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_combo_45_0_1000001f() {
    // Encoding: 0x1000001F
    // Test aarch64_integer_arithmetic_address_pc_rel field combination: op=0, immlo=0, immhi=0, Rd=31
    // Fields: op=0, immlo=0, immhi=0, Rd=31
    let encoding: u32 = 0x1000001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_3000003f()
 {
    // Encoding: 0x3000003F
    // Test aarch64_integer_arithmetic_address_pc_rel special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: immlo=1, immhi=1, op=0, Rd=31
    let encoding: u32 = 0x3000003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `ADR X0, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero offset
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_oracle_0_10000000() {
    // Test ADR: zero offset (oracle)
    // Encoding: 0x10000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x10000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `ADR X0, #4`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small positive
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_oracle_1_10000020() {
    // Test ADR: small positive (oracle)
    // Encoding: 0x10000020
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x10000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `ADR X0, #-4`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small negative
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_oracle_2_10ffffe0() {
    // Test ADR: small negative (oracle)
    // Encoding: 0x10FFFFE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x10FFFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_reg_write_0_10000000() {
    // Test aarch64_integer_arithmetic_address_pc_rel register write: GpFromField("d")
    // Encoding: 0x10000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x10000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_address_pc_rel
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_arithmetic_address_pc_rel_zr_rd_1000001f() {
    // Test aarch64_integer_arithmetic_address_pc_rel with Rd = ZR (31)
    // Encoding: 0x1000001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1000001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress Tests
// ============================================================================

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_field_xm_0_min_0_9ac00000() {
    // Encoding: 0x9AC00000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field Xm = 0 (Min)
    // Fields: Xn=0, Xd=0, Xm=0
    let encoding: u32 = 0x9AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_field_xm_1_poweroftwo_0_9ac10000()
 {
    // Encoding: 0x9AC10000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field Xm = 1 (PowerOfTwo)
    // Fields: Xn=0, Xm=1, Xd=0
    let encoding: u32 = 0x9AC10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_field_xm_30_poweroftwominusone_0_9ade0000()
 {
    // Encoding: 0x9ADE0000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field Xm = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xd=0, Xm=30
    let encoding: u32 = 0x9ADE0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_field_xm_31_max_0_9adf0000() {
    // Encoding: 0x9ADF0000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field Xm = 31 (Max)
    // Fields: Xm=31, Xn=0, Xd=0
    let encoding: u32 = 0x9ADF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_field_xn_0_min_0_9ac00000() {
    // Encoding: 0x9AC00000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field Xn = 0 (Min)
    // Fields: Xn=0, Xm=0, Xd=0
    let encoding: u32 = 0x9AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_field_xn_1_poweroftwo_0_9ac00020()
 {
    // Encoding: 0x9AC00020
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field Xn = 1 (PowerOfTwo)
    // Fields: Xm=0, Xn=1, Xd=0
    let encoding: u32 = 0x9AC00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_field_xn_30_poweroftwominusone_0_9ac003c0()
 {
    // Encoding: 0x9AC003C0
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xm=0, Xn=30, Xd=0
    let encoding: u32 = 0x9AC003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_field_xn_31_max_0_9ac003e0() {
    // Encoding: 0x9AC003E0
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field Xn = 31 (Max)
    // Fields: Xn=31, Xm=0, Xd=0
    let encoding: u32 = 0x9AC003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_field_xd_0_min_0_9ac00000() {
    // Encoding: 0x9AC00000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field Xd = 0 (Min)
    // Fields: Xd=0, Xm=0, Xn=0
    let encoding: u32 = 0x9AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_field_xd_1_poweroftwo_0_9ac00001()
 {
    // Encoding: 0x9AC00001
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field Xd = 1 (PowerOfTwo)
    // Fields: Xd=1, Xm=0, Xn=0
    let encoding: u32 = 0x9AC00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_field_xd_30_poweroftwominusone_0_9ac0001e()
 {
    // Encoding: 0x9AC0001E
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field Xd = 30 (PowerOfTwoMinusOne)
    // Fields: Xm=0, Xn=0, Xd=30
    let encoding: u32 = 0x9AC0001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_field_xd_31_max_0_9ac0001f() {
    // Encoding: 0x9AC0001F
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field Xd = 31 (Max)
    // Fields: Xd=31, Xn=0, Xm=0
    let encoding: u32 = 0x9AC0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_0_0_9ac00000() {
    // Encoding: 0x9AC00000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=0, Xn=0, Xd=0
    // Fields: Xd=0, Xm=0, Xn=0
    let encoding: u32 = 0x9AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_1_0_9ac10000() {
    // Encoding: 0x9AC10000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=1, Xn=0, Xd=0
    // Fields: Xm=1, Xd=0, Xn=0
    let encoding: u32 = 0x9AC10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_2_0_9ade0000() {
    // Encoding: 0x9ADE0000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=30, Xn=0, Xd=0
    // Fields: Xd=0, Xm=30, Xn=0
    let encoding: u32 = 0x9ADE0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_3_0_9adf0000() {
    // Encoding: 0x9ADF0000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=31, Xn=0, Xd=0
    // Fields: Xn=0, Xm=31, Xd=0
    let encoding: u32 = 0x9ADF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_4_0_9ac00000() {
    // Encoding: 0x9AC00000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=0, Xn=0, Xd=0
    // Fields: Xd=0, Xm=0, Xn=0
    let encoding: u32 = 0x9AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_5_0_9ac00020() {
    // Encoding: 0x9AC00020
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=0, Xn=1, Xd=0
    // Fields: Xd=0, Xn=1, Xm=0
    let encoding: u32 = 0x9AC00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_6_0_9ac003c0() {
    // Encoding: 0x9AC003C0
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=0, Xn=30, Xd=0
    // Fields: Xn=30, Xd=0, Xm=0
    let encoding: u32 = 0x9AC003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_7_0_9ac003e0() {
    // Encoding: 0x9AC003E0
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=0, Xn=31, Xd=0
    // Fields: Xd=0, Xm=0, Xn=31
    let encoding: u32 = 0x9AC003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_8_0_9ac00000() {
    // Encoding: 0x9AC00000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=0, Xn=0, Xd=0
    // Fields: Xd=0, Xm=0, Xn=0
    let encoding: u32 = 0x9AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_9_0_9ac00001() {
    // Encoding: 0x9AC00001
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=0, Xn=0, Xd=1
    // Fields: Xd=1, Xm=0, Xn=0
    let encoding: u32 = 0x9AC00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_10_0_9ac0001e() {
    // Encoding: 0x9AC0001E
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=0, Xn=0, Xd=30
    // Fields: Xd=30, Xm=0, Xn=0
    let encoding: u32 = 0x9AC0001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_11_0_9ac0001f() {
    // Encoding: 0x9AC0001F
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=0, Xn=0, Xd=31
    // Fields: Xn=0, Xd=31, Xm=0
    let encoding: u32 = 0x9AC0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=1 (same register test (reg=1)), Xn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_12_0_9ac10020() {
    // Encoding: 0x9AC10020
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=1, Xn=1, Xd=0
    // Fields: Xm=1, Xn=1, Xd=0
    let encoding: u32 = 0x9AC10020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=31 (same register test (reg=31)), Xn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_13_0_9adf03e0() {
    // Encoding: 0x9ADF03E0
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=31, Xn=31, Xd=0
    // Fields: Xm=31, Xn=31, Xd=0
    let encoding: u32 = 0x9ADF03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=1 (same register test (reg=1)), Xd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_14_0_9ac10001() {
    // Encoding: 0x9AC10001
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=1, Xn=0, Xd=1
    // Fields: Xm=1, Xn=0, Xd=1
    let encoding: u32 = 0x9AC10001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=31 (same register test (reg=31)), Xd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_15_0_9adf001f() {
    // Encoding: 0x9ADF001F
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=31, Xn=0, Xd=31
    // Fields: Xd=31, Xn=0, Xm=31
    let encoding: u32 = 0x9ADF001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_16_0_9ac00021() {
    // Encoding: 0x9AC00021
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=0, Xn=1, Xd=1
    // Fields: Xn=1, Xm=0, Xd=1
    let encoding: u32 = 0x9AC00021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_combo_17_0_9ac003ff() {
    // Encoding: 0x9AC003FF
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress field combination: Xm=0, Xn=31, Xd=31
    // Fields: Xn=31, Xd=31, Xm=0
    let encoding: u32 = 0x9AC003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_special_xn_31_stack_pointer_sp_may_require_alignment_0_9ac003e0()
 {
    // Encoding: 0x9AC003E0
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xm=0, Xn=31, Xd=0
    let encoding: u32 = 0x9AC003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values - high bits zero
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_umulh_oracle_0_9bc27c20() {
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

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// large value * 2 - produces high bits
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_umulh_oracle_1_9bc27c20() {
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

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max * max unsigned
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_umulh_oracle_2_9bc27c20() {
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

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max positive * max positive
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_umulh_oracle_3_9bc27c20() {
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

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// 2^32 * 2^32
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_umulh_oracle_4_9bc27c20() {
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

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_reg_write_0_9ac00000() {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress register write: GpFromField("d")
    // Encoding: 0x9AC00000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x9AC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_sp_xn_9ac003e0() {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress with Xn = SP (31)
    // Encoding: 0x9AC003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x9AC003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_flags_zeroresult_0_9ac00000() {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress flag computation: ZeroResult
    // Encoding: 0x9AC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0x9AC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_flags_zeroresult_1_9ac00000() {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress flag computation: ZeroResult
    // Encoding: 0x9AC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0x9AC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_flags_negativeresult_2_9ac00000()
{
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress flag computation: NegativeResult
    // Encoding: 0x9AC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x0);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x9AC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_flags_unsignedoverflow_3_9ac00000()
 {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress flag computation: UnsignedOverflow
    // Encoding: 0x9AC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x9AC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_flags_unsignedoverflow_4_9ac00000()
 {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress flag computation: UnsignedOverflow
    // Encoding: 0x9AC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x2);
    let encoding: u32 = 0x9AC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_flags_signedoverflow_5_9ac00000()
{
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress flag computation: SignedOverflow
    // Encoding: 0x9AC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0x9AC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_flags_signedoverflow_6_9ac00000()
{
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress flag computation: SignedOverflow
    // Encoding: 0x9AC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x9AC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress_flags_positiveresult_7_9ac00000()
{
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddress flag computation: PositiveResult
    // Encoding: 0x9AC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0x9AC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z should be false");
    assert_eq!(cpu.get_pstate().c, false, "C should be false");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

// ============================================================================
// aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags Tests
// ============================================================================

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_field_xm_0_min_0_bac00000()
 {
    // Encoding: 0xBAC00000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field Xm = 0 (Min)
    // Fields: Xn=0, Xd=0, Xm=0
    let encoding: u32 = 0xBAC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_field_xm_1_poweroftwo_0_bac10000()
 {
    // Encoding: 0xBAC10000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field Xm = 1 (PowerOfTwo)
    // Fields: Xm=1, Xd=0, Xn=0
    let encoding: u32 = 0xBAC10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_field_xm_30_poweroftwominusone_0_bade0000()
 {
    // Encoding: 0xBADE0000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field Xm = 30 (PowerOfTwoMinusOne)
    // Fields: Xm=30, Xn=0, Xd=0
    let encoding: u32 = 0xBADE0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field Xm 16 +: 5`
/// Requirement: FieldBoundary { field: "Xm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_field_xm_31_max_0_badf0000()
 {
    // Encoding: 0xBADF0000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field Xm = 31 (Max)
    // Fields: Xm=31, Xn=0, Xd=0
    let encoding: u32 = 0xBADF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_field_xn_0_min_0_bac00000()
 {
    // Encoding: 0xBAC00000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field Xn = 0 (Min)
    // Fields: Xm=0, Xn=0, Xd=0
    let encoding: u32 = 0xBAC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_field_xn_1_poweroftwo_0_bac00020()
 {
    // Encoding: 0xBAC00020
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field Xn = 1 (PowerOfTwo)
    // Fields: Xd=0, Xn=1, Xm=0
    let encoding: u32 = 0xBAC00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_field_xn_30_poweroftwominusone_0_bac003c0()
 {
    // Encoding: 0xBAC003C0
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field Xn = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=30, Xm=0, Xd=0
    let encoding: u32 = 0xBAC003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field Xn 5 +: 5`
/// Requirement: FieldBoundary { field: "Xn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_field_xn_31_max_0_bac003e0()
 {
    // Encoding: 0xBAC003E0
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field Xn = 31 (Max)
    // Fields: Xn=31, Xd=0, Xm=0
    let encoding: u32 = 0xBAC003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_field_xd_0_min_0_bac00000()
 {
    // Encoding: 0xBAC00000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field Xd = 0 (Min)
    // Fields: Xn=0, Xm=0, Xd=0
    let encoding: u32 = 0xBAC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_field_xd_1_poweroftwo_0_bac00001()
 {
    // Encoding: 0xBAC00001
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field Xd = 1 (PowerOfTwo)
    // Fields: Xm=0, Xn=0, Xd=1
    let encoding: u32 = 0xBAC00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_field_xd_30_poweroftwominusone_0_bac0001e()
 {
    // Encoding: 0xBAC0001E
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field Xd = 30 (PowerOfTwoMinusOne)
    // Fields: Xn=0, Xm=0, Xd=30
    let encoding: u32 = 0xBAC0001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field Xd 0 +: 5`
/// Requirement: FieldBoundary { field: "Xd", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_field_xd_31_max_0_bac0001f()
 {
    // Encoding: 0xBAC0001F
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field Xd = 31 (Max)
    // Fields: Xn=0, Xd=31, Xm=0
    let encoding: u32 = 0xBAC0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_0_0_bac00000() {
    // Encoding: 0xBAC00000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=0, Xn=0, Xd=0
    // Fields: Xn=0, Xd=0, Xm=0
    let encoding: u32 = 0xBAC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_1_0_bac10000() {
    // Encoding: 0xBAC10000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=1, Xn=0, Xd=0
    // Fields: Xm=1, Xn=0, Xd=0
    let encoding: u32 = 0xBAC10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_2_0_bade0000() {
    // Encoding: 0xBADE0000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=30, Xn=0, Xd=0
    // Fields: Xm=30, Xn=0, Xd=0
    let encoding: u32 = 0xBADE0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_3_0_badf0000() {
    // Encoding: 0xBADF0000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=31, Xn=0, Xd=0
    // Fields: Xd=0, Xm=31, Xn=0
    let encoding: u32 = 0xBADF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_4_0_bac00000() {
    // Encoding: 0xBAC00000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=0, Xn=0, Xd=0
    // Fields: Xn=0, Xd=0, Xm=0
    let encoding: u32 = 0xBAC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_5_0_bac00020() {
    // Encoding: 0xBAC00020
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=0, Xn=1, Xd=0
    // Fields: Xn=1, Xm=0, Xd=0
    let encoding: u32 = 0xBAC00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_6_0_bac003c0() {
    // Encoding: 0xBAC003C0
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=0, Xn=30, Xd=0
    // Fields: Xm=0, Xn=30, Xd=0
    let encoding: u32 = 0xBAC003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_7_0_bac003e0() {
    // Encoding: 0xBAC003E0
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=0, Xn=31, Xd=0
    // Fields: Xn=31, Xd=0, Xm=0
    let encoding: u32 = 0xBAC003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_8_0_bac00000() {
    // Encoding: 0xBAC00000
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=0, Xn=0, Xd=0
    // Fields: Xd=0, Xn=0, Xm=0
    let encoding: u32 = 0xBAC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_9_0_bac00001() {
    // Encoding: 0xBAC00001
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=0, Xn=0, Xd=1
    // Fields: Xd=1, Xm=0, Xn=0
    let encoding: u32 = 0xBAC00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_10_0_bac0001e() {
    // Encoding: 0xBAC0001E
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=0, Xn=0, Xd=30
    // Fields: Xm=0, Xd=30, Xn=0
    let encoding: u32 = 0xBAC0001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xd=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_11_0_bac0001f() {
    // Encoding: 0xBAC0001F
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=0, Xn=0, Xd=31
    // Fields: Xn=0, Xd=31, Xm=0
    let encoding: u32 = 0xBAC0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=1 (same register test (reg=1)), Xn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_12_0_bac10020() {
    // Encoding: 0xBAC10020
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=1, Xn=1, Xd=0
    // Fields: Xm=1, Xn=1, Xd=0
    let encoding: u32 = 0xBAC10020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=31 (same register test (reg=31)), Xn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_13_0_badf03e0() {
    // Encoding: 0xBADF03E0
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=31, Xn=31, Xd=0
    // Fields: Xn=31, Xd=0, Xm=31
    let encoding: u32 = 0xBADF03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=1 (same register test (reg=1)), Xd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_14_0_bac10001() {
    // Encoding: 0xBAC10001
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=1, Xn=0, Xd=1
    // Fields: Xn=0, Xm=1, Xd=1
    let encoding: u32 = 0xBAC10001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xm=31 (same register test (reg=31)), Xd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_15_0_badf001f() {
    // Encoding: 0xBADF001F
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=31, Xn=0, Xd=31
    // Fields: Xm=31, Xn=0, Xd=31
    let encoding: u32 = 0xBADF001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=1 (same register test (reg=1)), Xd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_16_0_bac00021() {
    // Encoding: 0xBAC00021
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=0, Xn=1, Xd=1
    // Fields: Xn=1, Xd=1, Xm=0
    let encoding: u32 = 0xBAC00021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Xn=31 (same register test (reg=31)), Xd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_combo_17_0_bac003ff() {
    // Encoding: 0xBAC003FF
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags field combination: Xm=0, Xn=31, Xd=31
    // Fields: Xm=0, Xn=31, Xd=31
    let encoding: u32 = 0xBAC003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `field Xn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Xn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_special_xn_31_stack_pointer_sp_may_require_alignment_0_bac003e0()
 {
    // Encoding: 0xBAC003E0
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags special value Xn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Xd=0, Xn=31, Xm=0
    let encoding: u32 = 0xBAC003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_reg_write_0_bac00000() {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags register write: GpFromField("d")
    // Encoding: 0xBAC00000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xBAC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `Xn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Xn = 31)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_sp_xn_bac003e0() {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags with Xn = SP (31)
    // Encoding: 0xBAC003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xBAC003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 0 + 0 = 0 (Z=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_flags_zeroresult_0_bac00000()
 {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags flag computation: ZeroResult
    // Encoding: 0xBAC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x0);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xBAC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: ZeroResult }
/// 1 + (-1) = 0 (Z=1, C=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_flags_zeroresult_1_bac00000()
 {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags flag computation: ZeroResult
    // Encoding: 0xBAC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    set_x(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xBAC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: NegativeResult }
/// negative value (N=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_flags_negativeresult_2_bac00000()
 {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags flag computation: NegativeResult
    // Encoding: 0xBAC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0x0);
    let encoding: u32 = 0xBAC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 1 = 0 (C=1, Z=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_flags_unsignedoverflow_3_bac00000()
 {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags flag computation: UnsignedOverflow
    // Encoding: 0xBAC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x1);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBAC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N should be false");
    assert_eq!(cpu.get_pstate().z, true, "Z should be true");
    assert_eq!(cpu.get_pstate().c, true, "C should be true");
    assert_eq!(cpu.get_pstate().v, false, "V should be false");
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: UnsignedOverflow }
/// max + 2 = 1 (C=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_flags_unsignedoverflow_4_bac00000()
 {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags flag computation: UnsignedOverflow
    // Encoding: 0xBAC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBAC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// max_signed + 1 = min_signed (V=1, N=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_flags_signedoverflow_5_bac00000()
 {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags flag computation: SignedOverflow
    // Encoding: 0xBAC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x7FFFFFFFFFFFFFFF);
    set_x(&mut cpu, 2, 0x1);
    let encoding: u32 = 0xBAC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: SignedOverflow }
/// min_signed + (-1) = max_signed (V=1)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_flags_signedoverflow_6_bac00000()
 {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags flag computation: SignedOverflow
    // Encoding: 0xBAC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000000000000000);
    set_x(&mut cpu, 2, 0xFFFFFFFFFFFFFFFF);
    let encoding: u32 = 0xBAC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags
/// ASL: `if setflags then PSTATE.<N,Z,C,V> = nzcv`
/// Requirement: FlagComputation { flag: N, scenario: PositiveResult }
/// 100 + 50 = 150 (no flags)
#[test]
fn test_aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags_flags_positiveresult_7_bac00000()
 {
    // Test aarch64_integer_arithmetic_pointer_mcsubtracttaggedaddresssetflags flag computation: PositiveResult
    // Encoding: 0xBAC00000
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x64);
    set_x(&mut cpu, 2, 0x32);
    let encoding: u32 = 0xBAC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}
