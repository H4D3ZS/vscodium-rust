//! A64 integer other tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_integer_crc Tests
// ============================================================================

/// Provenance: aarch64_integer_crc
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_crc_field_sf_0_min_4000_1ac04000() {
    // Encoding: 0x1AC04000
    // Test aarch64_integer_crc field sf = 0 (Min)
    // Fields: Rm=0, sf=0, C=0, sz=0, Rn=0, Rd=0
    let encoding: u32 = 0x1AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_crc_field_sf_1_max_4000_9ac04000() {
    // Encoding: 0x9AC04000
    // Test aarch64_integer_crc field sf = 1 (Max)
    // Fields: Rm=0, C=0, Rn=0, sz=0, Rd=0, sf=1
    let encoding: u32 = 0x9AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_crc_field_rm_0_min_4000_1ac04000() {
    // Encoding: 0x1AC04000
    // Test aarch64_integer_crc field Rm = 0 (Min)
    // Fields: C=0, sf=0, Rn=0, Rm=0, sz=0, Rd=0
    let encoding: u32 = 0x1AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_crc_field_rm_1_poweroftwo_4000_1ac14000() {
    // Encoding: 0x1AC14000
    // Test aarch64_integer_crc field Rm = 1 (PowerOfTwo)
    // Fields: sz=0, Rn=0, C=0, Rd=0, sf=0, Rm=1
    let encoding: u32 = 0x1AC14000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_crc_field_rm_30_poweroftwominusone_4000_1ade4000() {
    // Encoding: 0x1ADE4000
    // Test aarch64_integer_crc field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=30, sz=0, Rn=0, Rd=0, sf=0, C=0
    let encoding: u32 = 0x1ADE4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_crc_field_rm_31_max_4000_1adf4000() {
    // Encoding: 0x1ADF4000
    // Test aarch64_integer_crc field Rm = 31 (Max)
    // Fields: sz=0, C=0, Rm=31, sf=0, Rn=0, Rd=0
    let encoding: u32 = 0x1ADF4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field C 12 +: 1`
/// Requirement: FieldBoundary { field: "C", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_crc_field_c_0_min_4000_1ac04000() {
    // Encoding: 0x1AC04000
    // Test aarch64_integer_crc field C = 0 (Min)
    // Fields: Rd=0, C=0, sf=0, Rn=0, sz=0, Rm=0
    let encoding: u32 = 0x1AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field C 12 +: 1`
/// Requirement: FieldBoundary { field: "C", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_crc_field_c_1_max_4000_1ac05000() {
    // Encoding: 0x1AC05000
    // Test aarch64_integer_crc field C = 1 (Max)
    // Fields: Rm=0, C=1, sf=0, Rn=0, Rd=0, sz=0
    let encoding: u32 = 0x1AC05000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field sz 10 +: 2`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_crc_field_sz_0_min_4000_1ac04000() {
    // Encoding: 0x1AC04000
    // Test aarch64_integer_crc field sz = 0 (Min)
    // Fields: Rm=0, sf=0, Rn=0, C=0, Rd=0, sz=0
    let encoding: u32 = 0x1AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field sz 10 +: 2`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_crc_field_sz_1_poweroftwo_4000_1ac04400() {
    // Encoding: 0x1AC04400
    // Test aarch64_integer_crc field sz = 1 (PowerOfTwo)
    // Fields: Rm=0, C=0, sf=0, Rn=0, sz=1, Rd=0
    let encoding: u32 = 0x1AC04400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field sz 10 +: 2`
/// Requirement: FieldBoundary { field: "sz", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_integer_crc_field_sz_2_poweroftwo_4000_1ac04800() {
    // Encoding: 0x1AC04800
    // Test aarch64_integer_crc field sz = 2 (PowerOfTwo)
    // Fields: Rn=0, C=0, sz=2, sf=0, Rm=0, Rd=0
    let encoding: u32 = 0x1AC04800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field sz 10 +: 2`
/// Requirement: FieldBoundary { field: "sz", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_integer_crc_field_sz_3_max_4000_1ac04c00() {
    // Encoding: 0x1AC04C00
    // Test aarch64_integer_crc field sz = 3 (Max)
    // Fields: sz=3, Rn=0, C=0, Rm=0, sf=0, Rd=0
    let encoding: u32 = 0x1AC04C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_crc_field_rn_0_min_4000_1ac04000() {
    // Encoding: 0x1AC04000
    // Test aarch64_integer_crc field Rn = 0 (Min)
    // Fields: C=0, sz=0, Rm=0, sf=0, Rn=0, Rd=0
    let encoding: u32 = 0x1AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_crc_field_rn_1_poweroftwo_4000_1ac04020() {
    // Encoding: 0x1AC04020
    // Test aarch64_integer_crc field Rn = 1 (PowerOfTwo)
    // Fields: sf=0, Rm=0, sz=0, Rn=1, Rd=0, C=0
    let encoding: u32 = 0x1AC04020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_crc_field_rn_30_poweroftwominusone_4000_1ac043c0() {
    // Encoding: 0x1AC043C0
    // Test aarch64_integer_crc field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rd=0, Rm=0, C=0, sz=0, sf=0
    let encoding: u32 = 0x1AC043C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_crc_field_rn_31_max_4000_1ac043e0() {
    // Encoding: 0x1AC043E0
    // Test aarch64_integer_crc field Rn = 31 (Max)
    // Fields: sz=0, Rm=0, C=0, Rn=31, sf=0, Rd=0
    let encoding: u32 = 0x1AC043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_crc_field_rd_0_min_4000_1ac04000() {
    // Encoding: 0x1AC04000
    // Test aarch64_integer_crc field Rd = 0 (Min)
    // Fields: sz=0, Rn=0, Rm=0, sf=0, Rd=0, C=0
    let encoding: u32 = 0x1AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_crc_field_rd_1_poweroftwo_4000_1ac04001() {
    // Encoding: 0x1AC04001
    // Test aarch64_integer_crc field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, C=0, sf=0, sz=0, Rm=0, Rn=0
    let encoding: u32 = 0x1AC04001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_crc_field_rd_30_poweroftwominusone_4000_1ac0401e() {
    // Encoding: 0x1AC0401E
    // Test aarch64_integer_crc field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rn=0, C=0, Rm=0, sz=0, Rd=30
    let encoding: u32 = 0x1AC0401E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_crc_field_rd_31_max_4000_1ac0401f() {
    // Encoding: 0x1AC0401F
    // Test aarch64_integer_crc field Rd = 31 (Max)
    // Fields: Rd=31, C=0, sz=0, sf=0, Rm=0, Rn=0
    let encoding: u32 = 0x1AC0401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_crc_combo_0_4000_1ac04000() {
    // Encoding: 0x1AC04000
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, sf=0, sz=0, Rn=0, C=0
    let encoding: u32 = 0x1AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_crc_combo_1_4000_9ac04000() {
    // Encoding: 0x9AC04000
    // Test aarch64_integer_crc field combination: sf=1, Rm=0, C=0, sz=0, Rn=0, Rd=0
    // Fields: Rn=0, C=0, Rm=0, sf=1, sz=0, Rd=0
    let encoding: u32 = 0x9AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_crc_combo_2_4000_1ac04000() {
    // Encoding: 0x1AC04000
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=0, Rd=0
    // Fields: sz=0, Rn=0, Rm=0, Rd=0, sf=0, C=0
    let encoding: u32 = 0x1AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_crc_combo_3_4000_1ac14000() {
    // Encoding: 0x1AC14000
    // Test aarch64_integer_crc field combination: sf=0, Rm=1, C=0, sz=0, Rn=0, Rd=0
    // Fields: Rn=0, sz=0, Rd=0, Rm=1, C=0, sf=0
    let encoding: u32 = 0x1AC14000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_crc_combo_4_4000_1ade4000() {
    // Encoding: 0x1ADE4000
    // Test aarch64_integer_crc field combination: sf=0, Rm=30, C=0, sz=0, Rn=0, Rd=0
    // Fields: Rm=30, sz=0, Rn=0, Rd=0, C=0, sf=0
    let encoding: u32 = 0x1ADE4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_crc_combo_5_4000_1adf4000() {
    // Encoding: 0x1ADF4000
    // Test aarch64_integer_crc field combination: sf=0, Rm=31, C=0, sz=0, Rn=0, Rd=0
    // Fields: Rm=31, Rd=0, Rn=0, sf=0, sz=0, C=0
    let encoding: u32 = 0x1ADF4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// C=0 (minimum value)
#[test]
fn test_aarch64_integer_crc_combo_6_4000_1ac04000() {
    // Encoding: 0x1AC04000
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=0, Rd=0
    // Fields: C=0, sz=0, Rn=0, sf=0, Rd=0, Rm=0
    let encoding: u32 = 0x1AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// C=1 (maximum value (1))
#[test]
fn test_aarch64_integer_crc_combo_7_4000_1ac05000() {
    // Encoding: 0x1AC05000
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=1, sz=0, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, Rd=0, Rm=0, C=1, sz=0
    let encoding: u32 = 0x1AC05000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_crc_combo_8_4000_1ac04000() {
    // Encoding: 0x1AC04000
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=0, Rd=0
    // Fields: sf=0, C=0, Rn=0, sz=0, Rd=0, Rm=0
    let encoding: u32 = 0x1AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_crc_combo_9_4000_1ac04400() {
    // Encoding: 0x1AC04400
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=1, Rn=0, Rd=0
    // Fields: sz=1, Rd=0, Rm=0, C=0, sf=0, Rn=0
    let encoding: u32 = 0x1AC04400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=2 (32-bit / word size)
#[test]
fn test_aarch64_integer_crc_combo_10_4000_1ac04800() {
    // Encoding: 0x1AC04800
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=2, Rn=0, Rd=0
    // Fields: Rn=0, C=0, Rd=0, sf=0, Rm=0, sz=2
    let encoding: u32 = 0x1AC04800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_integer_crc_combo_11_4000_1ac04c00() {
    // Encoding: 0x1AC04C00
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=3, Rn=0, Rd=0
    // Fields: Rn=0, sf=0, Rd=0, Rm=0, C=0, sz=3
    let encoding: u32 = 0x1AC04C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_crc_combo_12_4000_1ac04000() {
    // Encoding: 0x1AC04000
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=0, Rd=0
    // Fields: Rd=0, sf=0, C=0, Rn=0, Rm=0, sz=0
    let encoding: u32 = 0x1AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_crc_combo_13_4000_1ac04020() {
    // Encoding: 0x1AC04020
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=1, Rd=0
    // Fields: sz=0, Rn=1, Rd=0, Rm=0, sf=0, C=0
    let encoding: u32 = 0x1AC04020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_crc_combo_14_4000_1ac043c0() {
    // Encoding: 0x1AC043C0
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=30, Rd=0
    // Fields: Rm=0, sz=0, Rn=30, Rd=0, C=0, sf=0
    let encoding: u32 = 0x1AC043C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_crc_combo_15_4000_1ac043e0() {
    // Encoding: 0x1AC043E0
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=31, Rd=0
    // Fields: C=0, Rm=0, sf=0, sz=0, Rn=31, Rd=0
    let encoding: u32 = 0x1AC043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_crc_combo_16_4000_1ac04000() {
    // Encoding: 0x1AC04000
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, Rm=0, Rd=0, C=0, sz=0
    let encoding: u32 = 0x1AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_crc_combo_17_4000_1ac04001() {
    // Encoding: 0x1AC04001
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=0, Rd=1
    // Fields: C=0, sz=0, Rn=0, Rd=1, sf=0, Rm=0
    let encoding: u32 = 0x1AC04001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_crc_combo_18_4000_1ac0401e() {
    // Encoding: 0x1AC0401E
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=0, Rd=30
    // Fields: sz=0, C=0, sf=0, Rm=0, Rn=0, Rd=30
    let encoding: u32 = 0x1AC0401E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_crc_combo_19_4000_1ac0401f() {
    // Encoding: 0x1AC0401F
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=0, Rd=31
    // Fields: sz=0, sf=0, C=0, Rn=0, Rd=31, Rm=0
    let encoding: u32 = 0x1AC0401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_crc_combo_20_4000_1ac14020() {
    // Encoding: 0x1AC14020
    // Test aarch64_integer_crc field combination: sf=0, Rm=1, C=0, sz=0, Rn=1, Rd=0
    // Fields: Rm=1, sf=0, sz=0, C=0, Rn=1, Rd=0
    let encoding: u32 = 0x1AC14020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_crc_combo_21_4000_1adf43e0() {
    // Encoding: 0x1ADF43E0
    // Test aarch64_integer_crc field combination: sf=0, Rm=31, C=0, sz=0, Rn=31, Rd=0
    // Fields: sf=0, Rd=0, Rm=31, Rn=31, sz=0, C=0
    let encoding: u32 = 0x1ADF43E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_crc_combo_22_4000_1ac14001() {
    // Encoding: 0x1AC14001
    // Test aarch64_integer_crc field combination: sf=0, Rm=1, C=0, sz=0, Rn=0, Rd=1
    // Fields: sf=0, sz=0, Rd=1, C=0, Rm=1, Rn=0
    let encoding: u32 = 0x1AC14001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_crc_combo_23_4000_1adf401f() {
    // Encoding: 0x1ADF401F
    // Test aarch64_integer_crc field combination: sf=0, Rm=31, C=0, sz=0, Rn=0, Rd=31
    // Fields: Rm=31, Rn=0, C=0, sf=0, sz=0, Rd=31
    let encoding: u32 = 0x1ADF401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_crc_combo_24_4000_1ac04021() {
    // Encoding: 0x1AC04021
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=1, Rd=1
    // Fields: Rm=0, C=0, sf=0, sz=0, Rn=1, Rd=1
    let encoding: u32 = 0x1AC04021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_crc_combo_25_4000_1ac043ff() {
    // Encoding: 0x1AC043FF
    // Test aarch64_integer_crc field combination: sf=0, Rm=0, C=0, sz=0, Rn=31, Rd=31
    // Fields: Rn=31, Rm=0, sz=0, Rd=31, sf=0, C=0
    let encoding: u32 = 0x1AC043FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_crc_special_sf_0_size_variant_0_16384_1ac04400() {
    // Encoding: 0x1AC04400
    // Test aarch64_integer_crc special value sf = 0 (Size variant 0)
    // Fields: C=0, Rm=0, Rd=0, Rn=0, sz=1, sf=0
    let encoding: u32 = 0x1AC04400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_crc_special_sf_1_size_variant_1_16384_9ac04400() {
    // Encoding: 0x9AC04400
    // Test aarch64_integer_crc special value sf = 1 (Size variant 1)
    // Fields: Rn=0, sz=1, Rd=0, sf=1, C=0, Rm=0
    let encoding: u32 = 0x9AC04400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_crc_special_sz_0_size_variant_0_16384_1ac04000() {
    // Encoding: 0x1AC04000
    // Test aarch64_integer_crc special value sz = 0 (Size variant 0)
    // Fields: Rd=0, C=0, Rm=0, sf=0, Rn=0, sz=0
    let encoding: u32 = 0x1AC04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_crc_special_sz_1_size_variant_1_16384_1ac04400() {
    // Encoding: 0x1AC04400
    // Test aarch64_integer_crc special value sz = 1 (Size variant 1)
    // Fields: Rm=0, C=0, sz=1, sf=0, Rd=0, Rn=0
    let encoding: u32 = 0x1AC04400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field sz = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "sz", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_integer_crc_special_sz_2_size_variant_2_16384_1ac04800() {
    // Encoding: 0x1AC04800
    // Test aarch64_integer_crc special value sz = 2 (Size variant 2)
    // Fields: sf=0, Rm=0, C=0, sz=2, Rn=0, Rd=0
    let encoding: u32 = 0x1AC04800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field sz = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "sz", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_integer_crc_special_sz_3_size_variant_3_16384_1ac04c00() {
    // Encoding: 0x1AC04C00
    // Test aarch64_integer_crc special value sz = 3 (Size variant 3)
    // Fields: sz=3, Rn=0, Rd=0, sf=0, Rm=0, C=0
    let encoding: u32 = 0x1AC04C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_crc_special_rn_31_stack_pointer_sp_may_require_alignment_16384_1ac047e0() {
    // Encoding: 0x1AC047E0
    // Test aarch64_integer_crc special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sf=0, Rn=31, Rd=0, sz=1, C=0, Rm=0
    let encoding: u32 = 0x1AC047E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_crc_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_16384_1ac0441f()
 {
    // Encoding: 0x1AC0441F
    // Test aarch64_integer_crc special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: sz=1, Rn=0, Rd=31, C=0, sf=0, Rm=0
    let encoding: u32 = 0x1AC0441F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_crc
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values - high bits zero
#[test]
fn test_aarch64_integer_crc_umulh_oracle_0_9bc27c20() {
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

/// Provenance: aarch64_integer_crc
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// large value * 2 - produces high bits
#[test]
fn test_aarch64_integer_crc_umulh_oracle_1_9bc27c20() {
    // Test UMULH: large value * 2 - produces high bits (oracle)
    // Encoding: 0x9BC27C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x2);
    set_x(&mut cpu, 1, 0x8000000000000000);
    let encoding: u32 = 0x9BC27C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x1, "X0 should be 0x0000000000000001");
}

/// Provenance: aarch64_integer_crc
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max * max unsigned
#[test]
fn test_aarch64_integer_crc_umulh_oracle_2_9bc27c20() {
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

/// Provenance: aarch64_integer_crc
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max positive * max positive
#[test]
fn test_aarch64_integer_crc_umulh_oracle_3_9bc27c20() {
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

/// Provenance: aarch64_integer_crc
/// ASL: `UMULH X0, X1, X2`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// 2^32 * 2^32
#[test]
fn test_aarch64_integer_crc_umulh_oracle_4_9bc27c20() {
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

/// Provenance: aarch64_integer_crc
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_crc_reg_write_0_1ac04000() {
    // Test aarch64_integer_crc register write: GpFromField("d")
    // Encoding: 0x1AC04000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1AC04000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_crc
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_crc_sp_rn_1ac043e0() {
    // Test aarch64_integer_crc with Rn = SP (31)
    // Encoding: 0x1AC043E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1AC043E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_crc
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_crc_zr_rd_1ac0401f() {
    // Test aarch64_integer_crc with Rd = ZR (31)
    // Encoding: 0x1AC0401F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1AC0401F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_integer_arithmetic_rev Tests
// ============================================================================

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_arithmetic_rev_field_sf_0_min_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rev field sf = 0 (Min)
    // Fields: Rn=0, opc=0, sf=0, Rd=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_arithmetic_rev_field_sf_1_max_0_dac00000() {
    // Encoding: 0xDAC00000
    // Test aarch64_integer_arithmetic_rev field sf = 1 (Max)
    // Fields: opc=0, sf=1, Rn=0, Rd=0
    let encoding: u32 = 0xDAC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field opc 10 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_arithmetic_rev_field_opc_0_min_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rev field opc = 0 (Min)
    // Fields: opc=0, Rn=0, sf=0, Rd=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field opc 10 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_arithmetic_rev_field_opc_1_poweroftwo_0_5ac00400() {
    // Encoding: 0x5AC00400
    // Test aarch64_integer_arithmetic_rev field opc = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=0, sf=0, opc=1
    let encoding: u32 = 0x5AC00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rev16	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field opc 10 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_integer_arithmetic_rev_field_opc_2_poweroftwo_0_5ac00800() {
    // Encoding: 0x5AC00800
    // Test aarch64_integer_arithmetic_rev field opc = 2 (PowerOfTwo)
    // Fields: sf=0, opc=2, Rd=0, Rn=0
    let encoding: u32 = 0x5AC00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rev	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field opc 10 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_integer_arithmetic_rev_field_opc_3_max_0_5ac00c00() {
    // Encoding: 0x5AC00C00
    // Test aarch64_integer_arithmetic_rev field opc = 3 (Max)
    // Fields: opc=3, sf=0, Rn=0, Rd=0
    let encoding: u32 = 0x5AC00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_rev_field_rn_0_min_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rev field Rn = 0 (Min)
    // Fields: opc=0, Rd=0, Rn=0, sf=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_rev_field_rn_1_poweroftwo_0_5ac00020() {
    // Encoding: 0x5AC00020
    // Test aarch64_integer_arithmetic_rev field Rn = 1 (PowerOfTwo)
    // Fields: sf=0, Rn=1, opc=0, Rd=0
    let encoding: u32 = 0x5AC00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_rev_field_rn_30_poweroftwominusone_0_5ac003c0() {
    // Encoding: 0x5AC003C0
    // Test aarch64_integer_arithmetic_rev field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=30, opc=0, sf=0
    let encoding: u32 = 0x5AC003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_arithmetic_rev_field_rn_31_max_0_5ac003e0() {
    // Encoding: 0x5AC003E0
    // Test aarch64_integer_arithmetic_rev field Rn = 31 (Max)
    // Fields: Rd=0, sf=0, opc=0, Rn=31
    let encoding: u32 = 0x5AC003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, wzr
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_rev_field_rd_0_min_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rev field Rd = 0 (Min)
    // Fields: opc=0, Rd=0, sf=0, Rn=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_rev_field_rd_1_poweroftwo_0_5ac00001() {
    // Encoding: 0x5AC00001
    // Test aarch64_integer_arithmetic_rev field Rd = 1 (PowerOfTwo)
    // Fields: opc=0, Rd=1, sf=0, Rn=0
    let encoding: u32 = 0x5AC00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w1, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_rev_field_rd_30_poweroftwominusone_0_5ac0001e() {
    // Encoding: 0x5AC0001E
    // Test aarch64_integer_arithmetic_rev field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: sf=0, opc=0, Rd=30, Rn=0
    let encoding: u32 = 0x5AC0001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w30, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_arithmetic_rev_field_rd_31_max_0_5ac0001f() {
    // Encoding: 0x5AC0001F
    // Test aarch64_integer_arithmetic_rev field Rd = 31 (Max)
    // Fields: sf=0, Rn=0, opc=0, Rd=31
    let encoding: u32 = 0x5AC0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	wzr, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_0_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=0, Rn=0, Rd=0
    // Fields: Rd=0, opc=0, sf=0, Rn=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_1_0_dac00000() {
    // Encoding: 0xDAC00000
    // Test aarch64_integer_arithmetic_rev field combination: sf=1, opc=0, Rn=0, Rd=0
    // Fields: sf=1, opc=0, Rd=0, Rn=0
    let encoding: u32 = 0xDAC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_2_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=0, Rn=0, Rd=0
    // Fields: sf=0, Rd=0, opc=0, Rn=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_3_0_5ac00400() {
    // Encoding: 0x5AC00400
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=1, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, opc=1, Rd=0
    let encoding: u32 = 0x5AC00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rev16	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_4_0_5ac00800() {
    // Encoding: 0x5AC00800
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=2, Rn=0, Rd=0
    // Fields: opc=2, Rd=0, Rn=0, sf=0
    let encoding: u32 = 0x5AC00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rev	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_5_0_5ac00c00() {
    // Encoding: 0x5AC00C00
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=3, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, Rd=0, opc=3
    let encoding: u32 = 0x5AC00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_6_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, opc=0, sf=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_7_0_5ac00020() {
    // Encoding: 0x5AC00020
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=0, Rn=1, Rd=0
    // Fields: sf=0, Rn=1, opc=0, Rd=0
    let encoding: u32 = 0x5AC00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_8_0_5ac003c0() {
    // Encoding: 0x5AC003C0
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=0, Rn=30, Rd=0
    // Fields: sf=0, opc=0, Rn=30, Rd=0
    let encoding: u32 = 0x5AC003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_9_0_5ac003e0() {
    // Encoding: 0x5AC003E0
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=0, Rn=31, Rd=0
    // Fields: opc=0, sf=0, Rn=31, Rd=0
    let encoding: u32 = 0x5AC003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, wzr
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_10_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=0, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, Rd=0, opc=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_11_0_5ac00001() {
    // Encoding: 0x5AC00001
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, opc=0, sf=0
    let encoding: u32 = 0x5AC00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w1, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_12_0_5ac0001e() {
    // Encoding: 0x5AC0001E
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=0, Rn=0, Rd=30
    // Fields: sf=0, opc=0, Rd=30, Rn=0
    let encoding: u32 = 0x5AC0001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w30, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_13_0_5ac0001f() {
    // Encoding: 0x5AC0001F
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=0, Rn=0, Rd=31
    // Fields: opc=0, Rd=31, sf=0, Rn=0
    let encoding: u32 = 0x5AC0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	wzr, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_14_0_5ac00021() {
    // Encoding: 0x5AC00021
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=0, Rn=1, Rd=1
    // Fields: opc=0, sf=0, Rn=1, Rd=1
    let encoding: u32 = 0x5AC00021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w1, w1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_rev_combo_15_0_5ac003ff() {
    // Encoding: 0x5AC003FF
    // Test aarch64_integer_arithmetic_rev field combination: sf=0, opc=0, Rn=31, Rd=31
    // Fields: Rn=31, sf=0, opc=0, Rd=31
    let encoding: u32 = 0x5AC003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	wzr, wzr
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_arithmetic_rev_special_sf_0_size_variant_0_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rev special value sf = 0 (Size variant 0)
    // Fields: Rn=0, sf=0, Rd=0, opc=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_arithmetic_rev_special_sf_1_size_variant_1_0_dac00000() {
    // Encoding: 0xDAC00000
    // Test aarch64_integer_arithmetic_rev special value sf = 1 (Size variant 1)
    // Fields: Rd=0, opc=0, Rn=0, sf=1
    let encoding: u32 = 0xDAC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_arithmetic_rev_special_opc_0_size_variant_0_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rev special value opc = 0 (Size variant 0)
    // Fields: Rn=0, sf=0, opc=0, Rd=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_arithmetic_rev_special_opc_1_size_variant_1_0_5ac00400() {
    // Encoding: 0x5AC00400
    // Test aarch64_integer_arithmetic_rev special value opc = 1 (Size variant 1)
    // Fields: opc=1, Rd=0, Rn=0, sf=0
    let encoding: u32 = 0x5AC00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rev16	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_integer_arithmetic_rev_special_opc_2_size_variant_2_0_5ac00800() {
    // Encoding: 0x5AC00800
    // Test aarch64_integer_arithmetic_rev special value opc = 2 (Size variant 2)
    // Fields: Rd=0, Rn=0, opc=2, sf=0
    let encoding: u32 = 0x5AC00800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rev	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_integer_arithmetic_rev_special_opc_3_size_variant_3_0_5ac00c00() {
    // Encoding: 0x5AC00C00
    // Test aarch64_integer_arithmetic_rev special value opc = 3 (Size variant 3)
    // Fields: Rd=0, Rn=0, sf=0, opc=3
    let encoding: u32 = 0x5AC00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_arithmetic_rev_special_rn_31_stack_pointer_sp_may_require_alignment_0_5ac003e0()
 {
    // Encoding: 0x5AC003E0
    // Test aarch64_integer_arithmetic_rev special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, opc=0, sf=0, Rn=31
    let encoding: u32 = 0x5AC003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, wzr
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_arithmetic_rev_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_5ac0001f()
 {
    // Encoding: 0x5AC0001F
    // Test aarch64_integer_arithmetic_rev special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: sf=0, Rd=31, opc=0, Rn=0
    let encoding: u32 = 0x5AC0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	wzr, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_arithmetic_rev_reg_write_0_5ac00000() {
    // Test aarch64_integer_arithmetic_rev register write: GpFromField("d")
    // Encoding: 0x5AC00000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5AC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_arithmetic_rev_sp_rn_5ac003e0() {
    // Test aarch64_integer_arithmetic_rev with Rn = SP (31)
    // Encoding: 0x5AC003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5AC003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_rev
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_arithmetic_rev_zr_rd_5ac0001f() {
    // Test aarch64_integer_arithmetic_rev with Rd = ZR (31)
    // Encoding: 0x5AC0001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5AC0001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_integer_arithmetic_cnt Tests
// ============================================================================

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_arithmetic_cnt_field_sf_0_min_1000_5ac01000() {
    // Encoding: 0x5AC01000
    // Test aarch64_integer_arithmetic_cnt field sf = 0 (Min)
    // Fields: Rd=0, op=0, sf=0, Rn=0
    let encoding: u32 = 0x5AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_arithmetic_cnt_field_sf_1_max_1000_dac01000() {
    // Encoding: 0xDAC01000
    // Test aarch64_integer_arithmetic_cnt field sf = 1 (Max)
    // Fields: Rn=0, op=0, sf=1, Rd=0
    let encoding: u32 = 0xDAC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field op 10 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_arithmetic_cnt_field_op_0_min_1000_5ac01000() {
    // Encoding: 0x5AC01000
    // Test aarch64_integer_arithmetic_cnt field op = 0 (Min)
    // Fields: sf=0, Rn=0, Rd=0, op=0
    let encoding: u32 = 0x5AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field op 10 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_arithmetic_cnt_field_op_1_max_1000_5ac01400() {
    // Encoding: 0x5AC01400
    // Test aarch64_integer_arithmetic_cnt field op = 1 (Max)
    // Fields: sf=0, Rn=0, Rd=0, op=1
    let encoding: u32 = 0x5AC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: cls	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_cnt_field_rn_0_min_1000_5ac01000() {
    // Encoding: 0x5AC01000
    // Test aarch64_integer_arithmetic_cnt field Rn = 0 (Min)
    // Fields: Rd=0, op=0, sf=0, Rn=0
    let encoding: u32 = 0x5AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_cnt_field_rn_1_poweroftwo_1000_5ac01020() {
    // Encoding: 0x5AC01020
    // Test aarch64_integer_arithmetic_cnt field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, op=0, Rd=0, sf=0
    let encoding: u32 = 0x5AC01020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, w1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_cnt_field_rn_30_poweroftwominusone_1000_5ac013c0() {
    // Encoding: 0x5AC013C0
    // Test aarch64_integer_arithmetic_cnt field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rd=0, op=0, sf=0
    let encoding: u32 = 0x5AC013C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, w30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_arithmetic_cnt_field_rn_31_max_1000_5ac013e0() {
    // Encoding: 0x5AC013E0
    // Test aarch64_integer_arithmetic_cnt field Rn = 31 (Max)
    // Fields: sf=0, Rn=31, Rd=0, op=0
    let encoding: u32 = 0x5AC013E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, wzr
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_cnt_field_rd_0_min_1000_5ac01000() {
    // Encoding: 0x5AC01000
    // Test aarch64_integer_arithmetic_cnt field Rd = 0 (Min)
    // Fields: sf=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x5AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_cnt_field_rd_1_poweroftwo_1000_5ac01001() {
    // Encoding: 0x5AC01001
    // Test aarch64_integer_arithmetic_cnt field Rd = 1 (PowerOfTwo)
    // Fields: op=0, Rn=0, sf=0, Rd=1
    let encoding: u32 = 0x5AC01001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w1, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_cnt_field_rd_30_poweroftwominusone_1000_5ac0101e() {
    // Encoding: 0x5AC0101E
    // Test aarch64_integer_arithmetic_cnt field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=30, sf=0, op=0
    let encoding: u32 = 0x5AC0101E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w30, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_arithmetic_cnt_field_rd_31_max_1000_5ac0101f() {
    // Encoding: 0x5AC0101F
    // Test aarch64_integer_arithmetic_cnt field Rd = 31 (Max)
    // Fields: sf=0, op=0, Rn=0, Rd=31
    let encoding: u32 = 0x5AC0101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	wzr, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_0_1000_5ac01000() {
    // Encoding: 0x5AC01000
    // Test aarch64_integer_arithmetic_cnt field combination: sf=0, op=0, Rn=0, Rd=0
    // Fields: op=0, Rd=0, Rn=0, sf=0
    let encoding: u32 = 0x5AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_1_1000_dac01000() {
    // Encoding: 0xDAC01000
    // Test aarch64_integer_arithmetic_cnt field combination: sf=1, op=0, Rn=0, Rd=0
    // Fields: Rn=0, sf=1, op=0, Rd=0
    let encoding: u32 = 0xDAC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_2_1000_5ac01000() {
    // Encoding: 0x5AC01000
    // Test aarch64_integer_arithmetic_cnt field combination: sf=0, op=0, Rn=0, Rd=0
    // Fields: op=0, Rd=0, sf=0, Rn=0
    let encoding: u32 = 0x5AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_3_1000_5ac01400() {
    // Encoding: 0x5AC01400
    // Test aarch64_integer_arithmetic_cnt field combination: sf=0, op=1, Rn=0, Rd=0
    // Fields: op=1, sf=0, Rd=0, Rn=0
    let encoding: u32 = 0x5AC01400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: cls	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_4_1000_5ac01000() {
    // Encoding: 0x5AC01000
    // Test aarch64_integer_arithmetic_cnt field combination: sf=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, sf=0, op=0, Rn=0
    let encoding: u32 = 0x5AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_5_1000_5ac01020() {
    // Encoding: 0x5AC01020
    // Test aarch64_integer_arithmetic_cnt field combination: sf=0, op=0, Rn=1, Rd=0
    // Fields: Rn=1, op=0, Rd=0, sf=0
    let encoding: u32 = 0x5AC01020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, w1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_6_1000_5ac013c0() {
    // Encoding: 0x5AC013C0
    // Test aarch64_integer_arithmetic_cnt field combination: sf=0, op=0, Rn=30, Rd=0
    // Fields: sf=0, Rn=30, Rd=0, op=0
    let encoding: u32 = 0x5AC013C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, w30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_7_1000_5ac013e0() {
    // Encoding: 0x5AC013E0
    // Test aarch64_integer_arithmetic_cnt field combination: sf=0, op=0, Rn=31, Rd=0
    // Fields: sf=0, op=0, Rd=0, Rn=31
    let encoding: u32 = 0x5AC013E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, wzr
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_8_1000_5ac01000() {
    // Encoding: 0x5AC01000
    // Test aarch64_integer_arithmetic_cnt field combination: sf=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, sf=0, op=0, Rd=0
    let encoding: u32 = 0x5AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_9_1000_5ac01001() {
    // Encoding: 0x5AC01001
    // Test aarch64_integer_arithmetic_cnt field combination: sf=0, op=0, Rn=0, Rd=1
    // Fields: Rd=1, sf=0, op=0, Rn=0
    let encoding: u32 = 0x5AC01001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w1, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_10_1000_5ac0101e() {
    // Encoding: 0x5AC0101E
    // Test aarch64_integer_arithmetic_cnt field combination: sf=0, op=0, Rn=0, Rd=30
    // Fields: Rn=0, sf=0, op=0, Rd=30
    let encoding: u32 = 0x5AC0101E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w30, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_11_1000_5ac0101f() {
    // Encoding: 0x5AC0101F
    // Test aarch64_integer_arithmetic_cnt field combination: sf=0, op=0, Rn=0, Rd=31
    // Fields: sf=0, op=0, Rd=31, Rn=0
    let encoding: u32 = 0x5AC0101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	wzr, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_12_1000_5ac01021() {
    // Encoding: 0x5AC01021
    // Test aarch64_integer_arithmetic_cnt field combination: sf=0, op=0, Rn=1, Rd=1
    // Fields: op=0, Rd=1, sf=0, Rn=1
    let encoding: u32 = 0x5AC01021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w1, w1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_cnt_combo_13_1000_5ac013ff() {
    // Encoding: 0x5AC013FF
    // Test aarch64_integer_arithmetic_cnt field combination: sf=0, op=0, Rn=31, Rd=31
    // Fields: sf=0, op=0, Rd=31, Rn=31
    let encoding: u32 = 0x5AC013FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	wzr, wzr
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_arithmetic_cnt_special_sf_0_size_variant_0_4096_5ac01000() {
    // Encoding: 0x5AC01000
    // Test aarch64_integer_arithmetic_cnt special value sf = 0 (Size variant 0)
    // Fields: Rn=0, op=0, Rd=0, sf=0
    let encoding: u32 = 0x5AC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_arithmetic_cnt_special_sf_1_size_variant_1_4096_dac01000() {
    // Encoding: 0xDAC01000
    // Test aarch64_integer_arithmetic_cnt special value sf = 1 (Size variant 1)
    // Fields: Rn=0, sf=1, op=0, Rd=0
    let encoding: u32 = 0xDAC01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_arithmetic_cnt_special_rn_31_stack_pointer_sp_may_require_alignment_4096_5ac013e0()
 {
    // Encoding: 0x5AC013E0
    // Test aarch64_integer_arithmetic_cnt special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sf=0, Rd=0, op=0, Rn=31
    let encoding: u32 = 0x5AC013E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	w0, wzr
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_arithmetic_cnt_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_4096_5ac0101f()
 {
    // Encoding: 0x5AC0101F
    // Test aarch64_integer_arithmetic_cnt special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: op=0, sf=0, Rn=0, Rd=31
    let encoding: u32 = 0x5AC0101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: clz	wzr, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_arithmetic_cnt_reg_write_0_5ac01000() {
    // Test aarch64_integer_arithmetic_cnt register write: GpFromField("d")
    // Encoding: 0x5AC01000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5AC01000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_arithmetic_cnt_sp_rn_5ac013e0() {
    // Test aarch64_integer_arithmetic_cnt with Rn = SP (31)
    // Encoding: 0x5AC013E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5AC013E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_cnt
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_arithmetic_cnt_zr_rd_5ac0101f() {
    // Test aarch64_integer_arithmetic_cnt with Rd = ZR (31)
    // Encoding: 0x5AC0101F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5AC0101F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_integer_arithmetic_rbit Tests
// ============================================================================

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_arithmetic_rbit_field_sf_0_min_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rbit field sf = 0 (Min)
    // Fields: sf=0, Rn=0, Rd=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_arithmetic_rbit_field_sf_1_max_0_dac00000() {
    // Encoding: 0xDAC00000
    // Test aarch64_integer_arithmetic_rbit field sf = 1 (Max)
    // Fields: Rd=0, sf=1, Rn=0
    let encoding: u32 = 0xDAC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_rbit_field_rn_0_min_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rbit field Rn = 0 (Min)
    // Fields: Rd=0, sf=0, Rn=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_rbit_field_rn_1_poweroftwo_0_5ac00020() {
    // Encoding: 0x5AC00020
    // Test aarch64_integer_arithmetic_rbit field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Rd=0, sf=0
    let encoding: u32 = 0x5AC00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_rbit_field_rn_30_poweroftwominusone_0_5ac003c0() {
    // Encoding: 0x5AC003C0
    // Test aarch64_integer_arithmetic_rbit field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rd=0, Rn=30
    let encoding: u32 = 0x5AC003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_arithmetic_rbit_field_rn_31_max_0_5ac003e0() {
    // Encoding: 0x5AC003E0
    // Test aarch64_integer_arithmetic_rbit field Rn = 31 (Max)
    // Fields: Rd=0, Rn=31, sf=0
    let encoding: u32 = 0x5AC003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, wzr
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_arithmetic_rbit_field_rd_0_min_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rbit field Rd = 0 (Min)
    // Fields: Rd=0, sf=0, Rn=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_arithmetic_rbit_field_rd_1_poweroftwo_0_5ac00001() {
    // Encoding: 0x5AC00001
    // Test aarch64_integer_arithmetic_rbit field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, Rn=0, sf=0
    let encoding: u32 = 0x5AC00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w1, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_arithmetic_rbit_field_rd_30_poweroftwominusone_0_5ac0001e() {
    // Encoding: 0x5AC0001E
    // Test aarch64_integer_arithmetic_rbit field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rn=0, Rd=30
    let encoding: u32 = 0x5AC0001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w30, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_arithmetic_rbit_field_rd_31_max_0_5ac0001f() {
    // Encoding: 0x5AC0001F
    // Test aarch64_integer_arithmetic_rbit field Rd = 31 (Max)
    // Fields: Rd=31, sf=0, Rn=0
    let encoding: u32 = 0x5AC0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	wzr, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_arithmetic_rbit_combo_0_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rbit field combination: sf=0, Rn=0, Rd=0
    // Fields: sf=0, Rd=0, Rn=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_arithmetic_rbit_combo_1_0_dac00000() {
    // Encoding: 0xDAC00000
    // Test aarch64_integer_arithmetic_rbit field combination: sf=1, Rn=0, Rd=0
    // Fields: Rd=0, sf=1, Rn=0
    let encoding: u32 = 0xDAC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_rbit_combo_2_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rbit field combination: sf=0, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, Rd=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_rbit_combo_3_0_5ac00020() {
    // Encoding: 0x5AC00020
    // Test aarch64_integer_arithmetic_rbit field combination: sf=0, Rn=1, Rd=0
    // Fields: Rn=1, Rd=0, sf=0
    let encoding: u32 = 0x5AC00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_rbit_combo_4_0_5ac003c0() {
    // Encoding: 0x5AC003C0
    // Test aarch64_integer_arithmetic_rbit field combination: sf=0, Rn=30, Rd=0
    // Fields: sf=0, Rn=30, Rd=0
    let encoding: u32 = 0x5AC003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_arithmetic_rbit_combo_5_0_5ac003e0() {
    // Encoding: 0x5AC003E0
    // Test aarch64_integer_arithmetic_rbit field combination: sf=0, Rn=31, Rd=0
    // Fields: sf=0, Rd=0, Rn=31
    let encoding: u32 = 0x5AC003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, wzr
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_arithmetic_rbit_combo_6_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rbit field combination: sf=0, Rn=0, Rd=0
    // Fields: sf=0, Rd=0, Rn=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_arithmetic_rbit_combo_7_0_5ac00001() {
    // Encoding: 0x5AC00001
    // Test aarch64_integer_arithmetic_rbit field combination: sf=0, Rn=0, Rd=1
    // Fields: Rd=1, Rn=0, sf=0
    let encoding: u32 = 0x5AC00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w1, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_arithmetic_rbit_combo_8_0_5ac0001e() {
    // Encoding: 0x5AC0001E
    // Test aarch64_integer_arithmetic_rbit field combination: sf=0, Rn=0, Rd=30
    // Fields: sf=0, Rn=0, Rd=30
    let encoding: u32 = 0x5AC0001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w30, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_arithmetic_rbit_combo_9_0_5ac0001f() {
    // Encoding: 0x5AC0001F
    // Test aarch64_integer_arithmetic_rbit field combination: sf=0, Rn=0, Rd=31
    // Fields: Rd=31, sf=0, Rn=0
    let encoding: u32 = 0x5AC0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	wzr, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_arithmetic_rbit_combo_10_0_5ac00021() {
    // Encoding: 0x5AC00021
    // Test aarch64_integer_arithmetic_rbit field combination: sf=0, Rn=1, Rd=1
    // Fields: sf=0, Rn=1, Rd=1
    let encoding: u32 = 0x5AC00021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w1, w1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_arithmetic_rbit_combo_11_0_5ac003ff() {
    // Encoding: 0x5AC003FF
    // Test aarch64_integer_arithmetic_rbit field combination: sf=0, Rn=31, Rd=31
    // Fields: Rd=31, sf=0, Rn=31
    let encoding: u32 = 0x5AC003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	wzr, wzr
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_arithmetic_rbit_special_sf_0_size_variant_0_0_5ac00000() {
    // Encoding: 0x5AC00000
    // Test aarch64_integer_arithmetic_rbit special value sf = 0 (Size variant 0)
    // Fields: Rn=0, sf=0, Rd=0
    let encoding: u32 = 0x5AC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_arithmetic_rbit_special_sf_1_size_variant_1_0_dac00000() {
    // Encoding: 0xDAC00000
    // Test aarch64_integer_arithmetic_rbit special value sf = 1 (Size variant 1)
    // Fields: sf=1, Rn=0, Rd=0
    let encoding: u32 = 0xDAC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	x0, x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_arithmetic_rbit_special_rn_31_stack_pointer_sp_may_require_alignment_0_5ac003e0()
 {
    // Encoding: 0x5AC003E0
    // Test aarch64_integer_arithmetic_rbit special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sf=0, Rd=0, Rn=31
    let encoding: u32 = 0x5AC003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	w0, wzr
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_arithmetic_rbit_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_5ac0001f()
 {
    // Encoding: 0x5AC0001F
    // Test aarch64_integer_arithmetic_rbit special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Rd=31, sf=0
    let encoding: u32 = 0x5AC0001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: rbit	wzr, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_arithmetic_rbit_reg_write_0_5ac00000() {
    // Test aarch64_integer_arithmetic_rbit register write: GpFromField("d")
    // Encoding: 0x5AC00000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5AC00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_arithmetic_rbit_sp_rn_5ac003e0() {
    // Test aarch64_integer_arithmetic_rbit with Rn = SP (31)
    // Encoding: 0x5AC003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5AC003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_arithmetic_rbit
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_arithmetic_rbit_zr_rd_5ac0001f() {
    // Test aarch64_integer_arithmetic_rbit with Rd = ZR (31)
    // Encoding: 0x5AC0001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5AC0001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}
