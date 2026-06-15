//! A64 misc other tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_udf Tests
// ============================================================================

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_udf_field_imm16_0_zero_0_00000000() {
    // Encoding: 0x00000000
    // Test aarch64_udf field imm16 = 0 (Zero)
    // Fields: imm16=0
    let encoding: u32 = 0x00000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#0
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_udf_field_imm16_1_poweroftwo_0_00000001() {
    // Encoding: 0x00000001
    // Test aarch64_udf field imm16 = 1 (PowerOfTwo)
    // Fields: imm16=1
    let encoding: u32 = 0x00000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#1
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_udf_field_imm16_3_poweroftwominusone_0_00000003() {
    // Encoding: 0x00000003
    // Test aarch64_udf field imm16 = 3 (PowerOfTwoMinusOne)
    // Fields: imm16=3
    let encoding: u32 = 0x00000003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#3
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_udf_field_imm16_4_poweroftwo_0_00000004() {
    // Encoding: 0x00000004
    // Test aarch64_udf field imm16 = 4 (PowerOfTwo)
    // Fields: imm16=4
    let encoding: u32 = 0x00000004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#4
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_udf_field_imm16_7_poweroftwominusone_0_00000007() {
    // Encoding: 0x00000007
    // Test aarch64_udf field imm16 = 7 (PowerOfTwoMinusOne)
    // Fields: imm16=7
    let encoding: u32 = 0x00000007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#7
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_udf_field_imm16_8_poweroftwo_0_00000008() {
    // Encoding: 0x00000008
    // Test aarch64_udf field imm16 = 8 (PowerOfTwo)
    // Fields: imm16=8
    let encoding: u32 = 0x00000008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#8
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_udf_field_imm16_15_poweroftwominusone_0_0000000f() {
    // Encoding: 0x0000000F
    // Test aarch64_udf field imm16 = 15 (PowerOfTwoMinusOne)
    // Fields: imm16=15
    let encoding: u32 = 0x0000000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#15
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_udf_field_imm16_16_poweroftwo_0_00000010() {
    // Encoding: 0x00000010
    // Test aarch64_udf field imm16 = 16 (PowerOfTwo)
    // Fields: imm16=16
    let encoding: u32 = 0x00000010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#16
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_udf_field_imm16_31_poweroftwominusone_0_0000001f() {
    // Encoding: 0x0000001F
    // Test aarch64_udf field imm16 = 31 (PowerOfTwoMinusOne)
    // Fields: imm16=31
    let encoding: u32 = 0x0000001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#31
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_udf_field_imm16_32_poweroftwo_0_00000020() {
    // Encoding: 0x00000020
    // Test aarch64_udf field imm16 = 32 (PowerOfTwo)
    // Fields: imm16=32
    let encoding: u32 = 0x00000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#32
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_udf_field_imm16_63_poweroftwominusone_0_0000003f() {
    // Encoding: 0x0000003F
    // Test aarch64_udf field imm16 = 63 (PowerOfTwoMinusOne)
    // Fields: imm16=63
    let encoding: u32 = 0x0000003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#63
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_udf_field_imm16_64_poweroftwo_0_00000040() {
    // Encoding: 0x00000040
    // Test aarch64_udf field imm16 = 64 (PowerOfTwo)
    // Fields: imm16=64
    let encoding: u32 = 0x00000040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#64
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_udf_field_imm16_127_poweroftwominusone_0_0000007f() {
    // Encoding: 0x0000007F
    // Test aarch64_udf field imm16 = 127 (PowerOfTwoMinusOne)
    // Fields: imm16=127
    let encoding: u32 = 0x0000007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#127
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_udf_field_imm16_128_poweroftwo_0_00000080() {
    // Encoding: 0x00000080
    // Test aarch64_udf field imm16 = 128 (PowerOfTwo)
    // Fields: imm16=128
    let encoding: u32 = 0x00000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#128
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch64_udf_field_imm16_255_poweroftwominusone_0_000000ff() {
    // Encoding: 0x000000FF
    // Test aarch64_udf field imm16 = 255 (PowerOfTwoMinusOne)
    // Fields: imm16=255
    let encoding: u32 = 0x000000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#255
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_udf_field_imm16_256_poweroftwo_0_00000100() {
    // Encoding: 0x00000100
    // Test aarch64_udf field imm16 = 256 (PowerOfTwo)
    // Fields: imm16=256
    let encoding: u32 = 0x00000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#256
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch64_udf_field_imm16_511_poweroftwominusone_0_000001ff() {
    // Encoding: 0x000001FF
    // Test aarch64_udf field imm16 = 511 (PowerOfTwoMinusOne)
    // Fields: imm16=511
    let encoding: u32 = 0x000001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#511
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch64_udf_field_imm16_512_poweroftwo_0_00000200() {
    // Encoding: 0x00000200
    // Test aarch64_udf field imm16 = 512 (PowerOfTwo)
    // Fields: imm16=512
    let encoding: u32 = 0x00000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#512
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch64_udf_field_imm16_1023_poweroftwominusone_0_000003ff() {
    // Encoding: 0x000003FF
    // Test aarch64_udf field imm16 = 1023 (PowerOfTwoMinusOne)
    // Fields: imm16=1023
    let encoding: u32 = 0x000003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#1023
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch64_udf_field_imm16_1024_poweroftwo_0_00000400() {
    // Encoding: 0x00000400
    // Test aarch64_udf field imm16 = 1024 (PowerOfTwo)
    // Fields: imm16=1024
    let encoding: u32 = 0x00000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#1024
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 2047, boundary: PowerOfTwoMinusOne }
/// 2^11 - 1 = 2047
#[test]
fn test_aarch64_udf_field_imm16_2047_poweroftwominusone_0_000007ff() {
    // Encoding: 0x000007FF
    // Test aarch64_udf field imm16 = 2047 (PowerOfTwoMinusOne)
    // Fields: imm16=2047
    let encoding: u32 = 0x000007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#2047
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch64_udf_field_imm16_2048_poweroftwo_0_00000800() {
    // Encoding: 0x00000800
    // Test aarch64_udf field imm16 = 2048 (PowerOfTwo)
    // Fields: imm16=2048
    let encoding: u32 = 0x00000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#2048
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 4095, boundary: PowerOfTwoMinusOne }
/// 2^12 - 1 = 4095
#[test]
fn test_aarch64_udf_field_imm16_4095_poweroftwominusone_0_00000fff() {
    // Encoding: 0x00000FFF
    // Test aarch64_udf field imm16 = 4095 (PowerOfTwoMinusOne)
    // Fields: imm16=4095
    let encoding: u32 = 0x00000FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#4095
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 4096, boundary: PowerOfTwo }
/// power of 2 (2^12 = 4096)
#[test]
fn test_aarch64_udf_field_imm16_4096_poweroftwo_0_00001000() {
    // Encoding: 0x00001000
    // Test aarch64_udf field imm16 = 4096 (PowerOfTwo)
    // Fields: imm16=4096
    let encoding: u32 = 0x00001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#4096
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 8191, boundary: PowerOfTwoMinusOne }
/// 2^13 - 1 = 8191
#[test]
fn test_aarch64_udf_field_imm16_8191_poweroftwominusone_0_00001fff() {
    // Encoding: 0x00001FFF
    // Test aarch64_udf field imm16 = 8191 (PowerOfTwoMinusOne)
    // Fields: imm16=8191
    let encoding: u32 = 0x00001FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#8191
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 8192, boundary: PowerOfTwo }
/// power of 2 (2^13 = 8192)
#[test]
fn test_aarch64_udf_field_imm16_8192_poweroftwo_0_00002000() {
    // Encoding: 0x00002000
    // Test aarch64_udf field imm16 = 8192 (PowerOfTwo)
    // Fields: imm16=8192
    let encoding: u32 = 0x00002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#8192
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 16383, boundary: PowerOfTwoMinusOne }
/// 2^14 - 1 = 16383
#[test]
fn test_aarch64_udf_field_imm16_16383_poweroftwominusone_0_00003fff() {
    // Encoding: 0x00003FFF
    // Test aarch64_udf field imm16 = 16383 (PowerOfTwoMinusOne)
    // Fields: imm16=16383
    let encoding: u32 = 0x00003FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#16383
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 16384, boundary: PowerOfTwo }
/// power of 2 (2^14 = 16384)
#[test]
fn test_aarch64_udf_field_imm16_16384_poweroftwo_0_00004000() {
    // Encoding: 0x00004000
    // Test aarch64_udf field imm16 = 16384 (PowerOfTwo)
    // Fields: imm16=16384
    let encoding: u32 = 0x00004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#16384
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 32767, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (32767)
#[test]
fn test_aarch64_udf_field_imm16_32767_poweroftwominusone_0_00007fff() {
    // Encoding: 0x00007FFF
    // Test aarch64_udf field imm16 = 32767 (PowerOfTwoMinusOne)
    // Fields: imm16=32767
    let encoding: u32 = 0x00007FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#32767
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 32768, boundary: PowerOfTwo }
/// power of 2 (2^15 = 32768)
#[test]
fn test_aarch64_udf_field_imm16_32768_poweroftwo_0_00008000() {
    // Encoding: 0x00008000
    // Test aarch64_udf field imm16 = 32768 (PowerOfTwo)
    // Fields: imm16=32768
    let encoding: u32 = 0x00008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#32768
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field imm16 0 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 65535, boundary: Max }
/// maximum immediate (65535)
#[test]
fn test_aarch64_udf_field_imm16_65535_max_0_0000ffff() {
    // Encoding: 0x0000FFFF
    // Test aarch64_udf field imm16 = 65535 (Max)
    // Fields: imm16=65535
    let encoding: u32 = 0x0000FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#65535
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=0 (immediate value 0)
#[test]
fn test_aarch64_udf_combo_0_0_00000000() {
    // Encoding: 0x00000000
    // Test aarch64_udf field combination: imm16=0
    // Fields: imm16=0
    let encoding: u32 = 0x00000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#0
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=1 (immediate value 1)
#[test]
fn test_aarch64_udf_combo_1_0_00000001() {
    // Encoding: 0x00000001
    // Test aarch64_udf field combination: imm16=1
    // Fields: imm16=1
    let encoding: u32 = 0x00000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#1
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_udf_combo_2_0_00000003() {
    // Encoding: 0x00000003
    // Test aarch64_udf field combination: imm16=3
    // Fields: imm16=3
    let encoding: u32 = 0x00000003;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#3
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_udf_combo_3_0_00000004() {
    // Encoding: 0x00000004
    // Test aarch64_udf field combination: imm16=4
    // Fields: imm16=4
    let encoding: u32 = 0x00000004;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#4
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_udf_combo_4_0_00000007() {
    // Encoding: 0x00000007
    // Test aarch64_udf field combination: imm16=7
    // Fields: imm16=7
    let encoding: u32 = 0x00000007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#7
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_udf_combo_5_0_00000008() {
    // Encoding: 0x00000008
    // Test aarch64_udf field combination: imm16=8
    // Fields: imm16=8
    let encoding: u32 = 0x00000008;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#8
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_udf_combo_6_0_0000000f() {
    // Encoding: 0x0000000F
    // Test aarch64_udf field combination: imm16=15
    // Fields: imm16=15
    let encoding: u32 = 0x0000000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#15
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_udf_combo_7_0_00000010() {
    // Encoding: 0x00000010
    // Test aarch64_udf field combination: imm16=16
    // Fields: imm16=16
    let encoding: u32 = 0x00000010;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#16
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_udf_combo_8_0_0000001f() {
    // Encoding: 0x0000001F
    // Test aarch64_udf field combination: imm16=31
    // Fields: imm16=31
    let encoding: u32 = 0x0000001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#31
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_udf_combo_9_0_00000020() {
    // Encoding: 0x00000020
    // Test aarch64_udf field combination: imm16=32
    // Fields: imm16=32
    let encoding: u32 = 0x00000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#32
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_udf_combo_10_0_0000003f() {
    // Encoding: 0x0000003F
    // Test aarch64_udf field combination: imm16=63
    // Fields: imm16=63
    let encoding: u32 = 0x0000003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#63
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_udf_combo_11_0_00000040() {
    // Encoding: 0x00000040
    // Test aarch64_udf field combination: imm16=64
    // Fields: imm16=64
    let encoding: u32 = 0x00000040;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#64
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_udf_combo_12_0_0000007f() {
    // Encoding: 0x0000007F
    // Test aarch64_udf field combination: imm16=127
    // Fields: imm16=127
    let encoding: u32 = 0x0000007F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#127
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_udf_combo_13_0_00000080() {
    // Encoding: 0x00000080
    // Test aarch64_udf field combination: imm16=128
    // Fields: imm16=128
    let encoding: u32 = 0x00000080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#128
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=255 (2^8 - 1 = 255)
#[test]
fn test_aarch64_udf_combo_14_0_000000ff() {
    // Encoding: 0x000000FF
    // Test aarch64_udf field combination: imm16=255
    // Fields: imm16=255
    let encoding: u32 = 0x000000FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#255
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_udf_combo_15_0_00000100() {
    // Encoding: 0x00000100
    // Test aarch64_udf field combination: imm16=256
    // Fields: imm16=256
    let encoding: u32 = 0x00000100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#256
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=511 (2^9 - 1 = 511)
#[test]
fn test_aarch64_udf_combo_16_0_000001ff() {
    // Encoding: 0x000001FF
    // Test aarch64_udf field combination: imm16=511
    // Fields: imm16=511
    let encoding: u32 = 0x000001FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#511
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=512 (power of 2 (2^9 = 512))
#[test]
fn test_aarch64_udf_combo_17_0_00000200() {
    // Encoding: 0x00000200
    // Test aarch64_udf field combination: imm16=512
    // Fields: imm16=512
    let encoding: u32 = 0x00000200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#512
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=1023 (2^10 - 1 = 1023)
#[test]
fn test_aarch64_udf_combo_18_0_000003ff() {
    // Encoding: 0x000003FF
    // Test aarch64_udf field combination: imm16=1023
    // Fields: imm16=1023
    let encoding: u32 = 0x000003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#1023
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=1024 (power of 2 (2^10 = 1024))
#[test]
fn test_aarch64_udf_combo_19_0_00000400() {
    // Encoding: 0x00000400
    // Test aarch64_udf field combination: imm16=1024
    // Fields: imm16=1024
    let encoding: u32 = 0x00000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#1024
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=2047 (2^11 - 1 = 2047)
#[test]
fn test_aarch64_udf_combo_20_0_000007ff() {
    // Encoding: 0x000007FF
    // Test aarch64_udf field combination: imm16=2047
    // Fields: imm16=2047
    let encoding: u32 = 0x000007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#2047
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=2048 (power of 2 (2^11 = 2048))
#[test]
fn test_aarch64_udf_combo_21_0_00000800() {
    // Encoding: 0x00000800
    // Test aarch64_udf field combination: imm16=2048
    // Fields: imm16=2048
    let encoding: u32 = 0x00000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#2048
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=4095 (2^12 - 1 = 4095)
#[test]
fn test_aarch64_udf_combo_22_0_00000fff() {
    // Encoding: 0x00000FFF
    // Test aarch64_udf field combination: imm16=4095
    // Fields: imm16=4095
    let encoding: u32 = 0x00000FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#4095
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=4096 (power of 2 (2^12 = 4096))
#[test]
fn test_aarch64_udf_combo_23_0_00001000() {
    // Encoding: 0x00001000
    // Test aarch64_udf field combination: imm16=4096
    // Fields: imm16=4096
    let encoding: u32 = 0x00001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#4096
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=8191 (2^13 - 1 = 8191)
#[test]
fn test_aarch64_udf_combo_24_0_00001fff() {
    // Encoding: 0x00001FFF
    // Test aarch64_udf field combination: imm16=8191
    // Fields: imm16=8191
    let encoding: u32 = 0x00001FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#8191
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=8192 (power of 2 (2^13 = 8192))
#[test]
fn test_aarch64_udf_combo_25_0_00002000() {
    // Encoding: 0x00002000
    // Test aarch64_udf field combination: imm16=8192
    // Fields: imm16=8192
    let encoding: u32 = 0x00002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#8192
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=16383 (2^14 - 1 = 16383)
#[test]
fn test_aarch64_udf_combo_26_0_00003fff() {
    // Encoding: 0x00003FFF
    // Test aarch64_udf field combination: imm16=16383
    // Fields: imm16=16383
    let encoding: u32 = 0x00003FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#16383
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=16384 (power of 2 (2^14 = 16384))
#[test]
fn test_aarch64_udf_combo_27_0_00004000() {
    // Encoding: 0x00004000
    // Test aarch64_udf field combination: imm16=16384
    // Fields: imm16=16384
    let encoding: u32 = 0x00004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#16384
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=32767 (immediate midpoint (32767))
#[test]
fn test_aarch64_udf_combo_28_0_00007fff() {
    // Encoding: 0x00007FFF
    // Test aarch64_udf field combination: imm16=32767
    // Fields: imm16=32767
    let encoding: u32 = 0x00007FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#32767
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=32768 (power of 2 (2^15 = 32768))
#[test]
fn test_aarch64_udf_combo_29_0_00008000() {
    // Encoding: 0x00008000
    // Test aarch64_udf field combination: imm16=32768
    // Fields: imm16=32768
    let encoding: u32 = 0x00008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#32768
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_udf
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=65535 (maximum immediate (65535))
#[test]
fn test_aarch64_udf_combo_30_0_0000ffff() {
    // Encoding: 0x0000FFFF
    // Test aarch64_udf field combination: imm16=65535
    // Fields: imm16=65535
    let encoding: u32 = 0x0000FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: udf	#65535
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}
