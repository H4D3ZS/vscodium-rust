//! A64 integer bitfield tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_integer_ins_ext_extract_immediate Tests
// ============================================================================

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_sf_0_min_0_13800000() {
    // Encoding: 0x13800000
    // Test aarch64_integer_ins_ext_extract_immediate field sf = 0 (Min)
    // Fields: N=0, imms=0, Rm=0, Rn=0, sf=0, Rd=0
    let encoding: u32 = 0x13800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_sf_1_max_0_93800000() {
    // Encoding: 0x93800000
    // Test aarch64_integer_ins_ext_extract_immediate field sf = 1 (Max)
    // Fields: N=0, Rd=0, Rm=0, sf=1, imms=0, Rn=0
    let encoding: u32 = 0x93800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field N 22 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_n_0_min_0_13800000() {
    // Encoding: 0x13800000
    // Test aarch64_integer_ins_ext_extract_immediate field N = 0 (Min)
    // Fields: N=0, sf=0, imms=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x13800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field N 22 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_n_1_max_0_13c00000() {
    // Encoding: 0x13C00000
    // Test aarch64_integer_ins_ext_extract_immediate field N = 1 (Max)
    // Fields: N=1, Rd=0, Rm=0, Rn=0, imms=0, sf=0
    let encoding: u32 = 0x13C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_rm_0_min_0_13800000() {
    // Encoding: 0x13800000
    // Test aarch64_integer_ins_ext_extract_immediate field Rm = 0 (Min)
    // Fields: imms=0, Rn=0, Rm=0, Rd=0, sf=0, N=0
    let encoding: u32 = 0x13800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_rm_1_poweroftwo_0_13810000() {
    // Encoding: 0x13810000
    // Test aarch64_integer_ins_ext_extract_immediate field Rm = 1 (PowerOfTwo)
    // Fields: Rd=0, sf=0, Rn=0, N=0, Rm=1, imms=0
    let encoding: u32 = 0x13810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_rm_30_poweroftwominusone_0_139e0000() {
    // Encoding: 0x139E0000
    // Test aarch64_integer_ins_ext_extract_immediate field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: imms=0, Rn=0, Rd=0, sf=0, Rm=30, N=0
    let encoding: u32 = 0x139E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_rm_31_max_0_139f0000() {
    // Encoding: 0x139F0000
    // Test aarch64_integer_ins_ext_extract_immediate field Rm = 31 (Max)
    // Fields: Rn=0, Rd=0, Rm=31, N=0, imms=0, sf=0
    let encoding: u32 = 0x139F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_imms_0_zero_0_13800000() {
    // Encoding: 0x13800000
    // Test aarch64_integer_ins_ext_extract_immediate field imms = 0 (Zero)
    // Fields: Rd=0, Rm=0, sf=0, Rn=0, imms=0, N=0
    let encoding: u32 = 0x13800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_imms_1_poweroftwo_0_13800400() {
    // Encoding: 0x13800400
    // Test aarch64_integer_ins_ext_extract_immediate field imms = 1 (PowerOfTwo)
    // Fields: Rd=0, N=0, imms=1, sf=0, Rn=0, Rm=0
    let encoding: u32 = 0x13800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_imms_3_poweroftwominusone_0_13800c00() {
    // Encoding: 0x13800C00
    // Test aarch64_integer_ins_ext_extract_immediate field imms = 3 (PowerOfTwoMinusOne)
    // Fields: N=0, Rd=0, imms=3, sf=0, Rm=0, Rn=0
    let encoding: u32 = 0x13800C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_imms_4_poweroftwo_0_13801000() {
    // Encoding: 0x13801000
    // Test aarch64_integer_ins_ext_extract_immediate field imms = 4 (PowerOfTwo)
    // Fields: sf=0, N=0, Rd=0, Rn=0, imms=4, Rm=0
    let encoding: u32 = 0x13801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_imms_7_poweroftwominusone_0_13801c00() {
    // Encoding: 0x13801C00
    // Test aarch64_integer_ins_ext_extract_immediate field imms = 7 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rm=0, imms=7, Rn=0, Rd=0, N=0
    let encoding: u32 = 0x13801C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_imms_8_poweroftwo_0_13802000() {
    // Encoding: 0x13802000
    // Test aarch64_integer_ins_ext_extract_immediate field imms = 8 (PowerOfTwo)
    // Fields: sf=0, N=0, Rm=0, imms=8, Rn=0, Rd=0
    let encoding: u32 = 0x13802000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_imms_15_poweroftwominusone_0_13803c00() {
    // Encoding: 0x13803C00
    // Test aarch64_integer_ins_ext_extract_immediate field imms = 15 (PowerOfTwoMinusOne)
    // Fields: imms=15, sf=0, Rm=0, Rd=0, N=0, Rn=0
    let encoding: u32 = 0x13803C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_imms_16_poweroftwo_0_13804000() {
    // Encoding: 0x13804000
    // Test aarch64_integer_ins_ext_extract_immediate field imms = 16 (PowerOfTwo)
    // Fields: imms=16, Rn=0, N=0, Rm=0, Rd=0, sf=0
    let encoding: u32 = 0x13804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_imms_31_poweroftwominusone_0_13807c00() {
    // Encoding: 0x13807C00
    // Test aarch64_integer_ins_ext_extract_immediate field imms = 31 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=0, Rm=0, imms=31, sf=0, N=0
    let encoding: u32 = 0x13807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_imms_32_poweroftwo_0_13808000() {
    // Encoding: 0x13808000
    // Test aarch64_integer_ins_ext_extract_immediate field imms = 32 (PowerOfTwo)
    // Fields: sf=0, Rd=0, Rm=0, N=0, imms=32, Rn=0
    let encoding: u32 = 0x13808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_imms_63_max_0_1380fc00() {
    // Encoding: 0x1380FC00
    // Test aarch64_integer_ins_ext_extract_immediate field imms = 63 (Max)
    // Fields: Rm=0, N=0, Rn=0, Rd=0, sf=0, imms=63
    let encoding: u32 = 0x1380FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_rn_0_min_0_13800000() {
    // Encoding: 0x13800000
    // Test aarch64_integer_ins_ext_extract_immediate field Rn = 0 (Min)
    // Fields: imms=0, N=0, Rn=0, sf=0, Rm=0, Rd=0
    let encoding: u32 = 0x13800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_rn_1_poweroftwo_0_13800020() {
    // Encoding: 0x13800020
    // Test aarch64_integer_ins_ext_extract_immediate field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, imms=0, N=0, sf=0, Rm=0, Rd=0
    let encoding: u32 = 0x13800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_rn_30_poweroftwominusone_0_138003c0() {
    // Encoding: 0x138003C0
    // Test aarch64_integer_ins_ext_extract_immediate field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=30, sf=0, imms=0, Rm=0, N=0
    let encoding: u32 = 0x138003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_rn_31_max_0_138003e0() {
    // Encoding: 0x138003E0
    // Test aarch64_integer_ins_ext_extract_immediate field Rn = 31 (Max)
    // Fields: Rd=0, N=0, Rm=0, imms=0, Rn=31, sf=0
    let encoding: u32 = 0x138003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_rd_0_min_0_13800000() {
    // Encoding: 0x13800000
    // Test aarch64_integer_ins_ext_extract_immediate field Rd = 0 (Min)
    // Fields: sf=0, N=0, Rm=0, Rd=0, Rn=0, imms=0
    let encoding: u32 = 0x13800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_rd_1_poweroftwo_0_13800001() {
    // Encoding: 0x13800001
    // Test aarch64_integer_ins_ext_extract_immediate field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, imms=0, Rm=0, N=0, sf=0, Rn=0
    let encoding: u32 = 0x13800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_rd_30_poweroftwominusone_0_1380001e() {
    // Encoding: 0x1380001E
    // Test aarch64_integer_ins_ext_extract_immediate field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: imms=0, Rn=0, Rm=0, sf=0, Rd=30, N=0
    let encoding: u32 = 0x1380001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_field_rd_31_max_0_1380001f() {
    // Encoding: 0x1380001F
    // Test aarch64_integer_ins_ext_extract_immediate field Rd = 31 (Max)
    // Fields: Rn=0, N=0, Rd=31, Rm=0, imms=0, sf=0
    let encoding: u32 = 0x1380001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_0_0_13800000() {
    // Encoding: 0x13800000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=0, Rd=0
    // Fields: Rm=0, sf=0, imms=0, Rd=0, Rn=0, N=0
    let encoding: u32 = 0x13800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_1_0_93800000() {
    // Encoding: 0x93800000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=1, N=0, Rm=0, imms=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, N=0, imms=0, Rn=0, sf=1
    let encoding: u32 = 0x93800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// N=0 (minimum value)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_2_0_13800000() {
    // Encoding: 0x13800000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=0, Rd=0
    // Fields: sf=0, imms=0, Rn=0, N=0, Rm=0, Rd=0
    let encoding: u32 = 0x13800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// N=1 (maximum value (1))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_3_0_13c00000() {
    // Encoding: 0x13C00000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=1, Rm=0, imms=0, Rn=0, Rd=0
    // Fields: Rd=0, imms=0, Rm=0, sf=0, N=1, Rn=0
    let encoding: u32 = 0x13C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_4_0_13800000() {
    // Encoding: 0x13800000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=0, Rd=0
    // Fields: sf=0, N=0, imms=0, Rd=0, Rn=0, Rm=0
    let encoding: u32 = 0x13800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_5_0_13810000() {
    // Encoding: 0x13810000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=1, imms=0, Rn=0, Rd=0
    // Fields: N=0, Rm=1, imms=0, sf=0, Rn=0, Rd=0
    let encoding: u32 = 0x13810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_6_0_139e0000() {
    // Encoding: 0x139E0000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=30, imms=0, Rn=0, Rd=0
    // Fields: imms=0, Rm=30, Rn=0, Rd=0, sf=0, N=0
    let encoding: u32 = 0x139E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_7_0_139f0000() {
    // Encoding: 0x139F0000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=31, imms=0, Rn=0, Rd=0
    // Fields: Rm=31, Rn=0, imms=0, Rd=0, sf=0, N=0
    let encoding: u32 = 0x139F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=0 (immediate value 0)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_8_0_13800000() {
    // Encoding: 0x13800000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=0, Rd=0
    // Fields: sf=0, Rd=0, imms=0, Rm=0, Rn=0, N=0
    let encoding: u32 = 0x13800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=1 (immediate value 1)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_9_0_13800400() {
    // Encoding: 0x13800400
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=1, Rn=0, Rd=0
    // Fields: Rd=0, sf=0, Rm=0, Rn=0, N=0, imms=1
    let encoding: u32 = 0x13800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_10_0_13800c00() {
    // Encoding: 0x13800C00
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=3, Rn=0, Rd=0
    // Fields: Rd=0, sf=0, imms=3, Rm=0, Rn=0, N=0
    let encoding: u32 = 0x13800C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_11_0_13801000() {
    // Encoding: 0x13801000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=4, Rn=0, Rd=0
    // Fields: Rm=0, imms=4, N=0, Rn=0, sf=0, Rd=0
    let encoding: u32 = 0x13801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_12_0_13801c00() {
    // Encoding: 0x13801C00
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=7, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, Rd=0, imms=7, N=0, Rm=0
    let encoding: u32 = 0x13801C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_13_0_13802000() {
    // Encoding: 0x13802000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=8, Rn=0, Rd=0
    // Fields: sf=0, N=0, Rd=0, imms=8, Rn=0, Rm=0
    let encoding: u32 = 0x13802000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_14_0_13803c00() {
    // Encoding: 0x13803C00
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=15, Rn=0, Rd=0
    // Fields: N=0, sf=0, Rm=0, Rn=0, Rd=0, imms=15
    let encoding: u32 = 0x13803C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_15_0_13804000() {
    // Encoding: 0x13804000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=16, Rn=0, Rd=0
    // Fields: sf=0, Rm=0, imms=16, Rn=0, N=0, Rd=0
    let encoding: u32 = 0x13804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=31 (immediate midpoint (31))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_16_0_13807c00() {
    // Encoding: 0x13807C00
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=31, Rn=0, Rd=0
    // Fields: N=0, Rn=0, Rm=0, sf=0, imms=31, Rd=0
    let encoding: u32 = 0x13807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_17_0_13808000() {
    // Encoding: 0x13808000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=32, Rn=0, Rd=0
    // Fields: sf=0, Rm=0, imms=32, Rn=0, Rd=0, N=0
    let encoding: u32 = 0x13808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=63 (maximum immediate (63))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_18_0_1380fc00() {
    // Encoding: 0x1380FC00
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=63, Rn=0, Rd=0
    // Fields: imms=63, Rm=0, Rd=0, Rn=0, sf=0, N=0
    let encoding: u32 = 0x1380FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_19_0_13800000() {
    // Encoding: 0x13800000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=0, Rd=0
    // Fields: imms=0, Rm=0, Rn=0, sf=0, N=0, Rd=0
    let encoding: u32 = 0x13800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_20_0_13800020() {
    // Encoding: 0x13800020
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=1, Rd=0
    // Fields: Rd=0, N=0, Rn=1, sf=0, Rm=0, imms=0
    let encoding: u32 = 0x13800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_21_0_138003c0() {
    // Encoding: 0x138003C0
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=30, Rd=0
    // Fields: Rn=30, Rm=0, Rd=0, N=0, sf=0, imms=0
    let encoding: u32 = 0x138003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_22_0_138003e0() {
    // Encoding: 0x138003E0
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=31, Rd=0
    // Fields: Rn=31, Rm=0, sf=0, Rd=0, imms=0, N=0
    let encoding: u32 = 0x138003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_23_0_13800000() {
    // Encoding: 0x13800000
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, Rn=0, N=0, imms=0, sf=0
    let encoding: u32 = 0x13800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_24_0_13800001() {
    // Encoding: 0x13800001
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=0, Rd=1
    // Fields: sf=0, Rd=1, Rn=0, N=0, Rm=0, imms=0
    let encoding: u32 = 0x13800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_25_0_1380001e() {
    // Encoding: 0x1380001E
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=0, Rd=30
    // Fields: Rn=0, Rd=30, Rm=0, sf=0, imms=0, N=0
    let encoding: u32 = 0x1380001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_26_0_1380001f() {
    // Encoding: 0x1380001F
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=0, Rd=31
    // Fields: Rd=31, N=0, Rm=0, Rn=0, imms=0, sf=0
    let encoding: u32 = 0x1380001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_27_0_13810020() {
    // Encoding: 0x13810020
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=1, imms=0, Rn=1, Rd=0
    // Fields: imms=0, sf=0, Rn=1, Rd=0, N=0, Rm=1
    let encoding: u32 = 0x13810020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_28_0_139f03e0() {
    // Encoding: 0x139F03E0
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=31, imms=0, Rn=31, Rd=0
    // Fields: Rn=31, N=0, sf=0, Rm=31, imms=0, Rd=0
    let encoding: u32 = 0x139F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_29_0_13810001() {
    // Encoding: 0x13810001
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=1, imms=0, Rn=0, Rd=1
    // Fields: imms=0, Rd=1, N=0, Rm=1, Rn=0, sf=0
    let encoding: u32 = 0x13810001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_30_0_139f001f() {
    // Encoding: 0x139F001F
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=31, imms=0, Rn=0, Rd=31
    // Fields: Rd=31, sf=0, Rn=0, Rm=31, N=0, imms=0
    let encoding: u32 = 0x139F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_31_0_13800021() {
    // Encoding: 0x13800021
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=1, Rd=1
    // Fields: sf=0, Rd=1, N=0, Rm=0, imms=0, Rn=1
    let encoding: u32 = 0x13800021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_combo_32_0_138003ff() {
    // Encoding: 0x138003FF
    // Test aarch64_integer_ins_ext_extract_immediate field combination: sf=0, N=0, Rm=0, imms=0, Rn=31, Rd=31
    // Fields: Rn=31, N=0, Rd=31, imms=0, sf=0, Rm=0
    let encoding: u32 = 0x138003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_special_sf_0_size_variant_0_0_13800400() {
    // Encoding: 0x13800400
    // Test aarch64_integer_ins_ext_extract_immediate special value sf = 0 (Size variant 0)
    // Fields: N=0, imms=1, Rd=0, Rn=0, Rm=0, sf=0
    let encoding: u32 = 0x13800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_special_sf_1_size_variant_1_0_93800400() {
    // Encoding: 0x93800400
    // Test aarch64_integer_ins_ext_extract_immediate special value sf = 1 (Size variant 1)
    // Fields: Rm=0, Rn=0, imms=1, Rd=0, sf=1, N=0
    let encoding: u32 = 0x93800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_special_rn_31_stack_pointer_sp_may_require_alignment_0_138007e0()
 {
    // Encoding: 0x138007E0
    // Test aarch64_integer_ins_ext_extract_immediate special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: imms=1, Rd=0, Rn=31, N=0, Rm=0, sf=0
    let encoding: u32 = 0x138007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_1380041f()
 {
    // Encoding: 0x1380041F
    // Test aarch64_integer_ins_ext_extract_immediate special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: sf=0, Rm=0, N=0, imms=1, Rn=0, Rd=31
    let encoding: u32 = 0x1380041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `EXTR X0, X1, X2, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// extract at 0 (32)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_extr_oracle_32_0_13820020() {
    // Test EXTR 32-bit: extract at 0 (oracle)
    // Encoding: 0x13820020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0xCAFEBABE);
    set_x(&mut cpu, 1, 0xDEADBEEF);
    let encoding: u32 = 0x13820020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xCAFEBABE, "W0 should be 0xCAFEBABE");
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `EXTR X0, X1, X2, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// extract at 0 (64)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_extr_oracle_64_0_93c20020() {
    // Test EXTR 64-bit: extract at 0 (oracle)
    // Encoding: 0x93C20020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xDEADBEEF);
    set_x(&mut cpu, 2, 0xCAFEBABE);
    let encoding: u32 = 0x93C20020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xCAFEBABE,
        "X0 should be 0x00000000CAFEBABE"
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `EXTR X0, X1, X2, #16`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// extract at 16 (32)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_extr_oracle_32_1_13824020() {
    // Test EXTR 32-bit: extract at 16 (oracle)
    // Encoding: 0x13824020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xDEADBEEF);
    set_x(&mut cpu, 2, 0xCAFEBABE);
    let encoding: u32 = 0x13824020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xBEEFCAFE, "W0 should be 0xBEEFCAFE");
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `EXTR X0, X1, X2, #16`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// extract at 16 (64)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_extr_oracle_64_1_93c24020() {
    // Test EXTR 64-bit: extract at 16 (oracle)
    // Encoding: 0x93C24020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xDEADBEEF);
    set_x(&mut cpu, 2, 0xCAFEBABE);
    let encoding: u32 = 0x93C24020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xBEEF00000000CAFE,
        "X0 should be 0xBEEF00000000CAFE"
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `EXTR X0, X1, X2, #8`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// extract at 8 (32)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_extr_oracle_32_2_13822020() {
    // Test EXTR 32-bit: extract at 8 (oracle)
    // Encoding: 0x13822020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xDEADBEEF);
    set_x(&mut cpu, 2, 0xCAFEBABE);
    let encoding: u32 = 0x13822020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xEFCAFEBA, "W0 should be 0xEFCAFEBA");
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `EXTR X0, X1, X2, #8`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// extract at 8 (64)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_extr_oracle_64_2_93c22020() {
    // Test EXTR 64-bit: extract at 8 (oracle)
    // Encoding: 0x93C22020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xDEADBEEF);
    set_x(&mut cpu, 2, 0xCAFEBABE);
    let encoding: u32 = 0x93C22020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xEF00000000CAFEBA,
        "X0 should be 0xEF00000000CAFEBA"
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `EXTR X0, X1, X2, #32`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// extract at 32 (64-bit) (64)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_extr_oracle_64_3_93c28020() {
    // Test EXTR 64-bit: extract at 32 (64-bit) (oracle)
    // Encoding: 0x93C28020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x123456789ABCDEF0);
    set_x(&mut cpu, 2, 0xFEDCBA9876543210);
    let encoding: u32 = 0x93C28020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x9ABCDEF0FEDCBA98,
        "X0 should be 0x9ABCDEF0FEDCBA98"
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `EXTR X0, X1, X2, #4`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// alternating bits (32)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_extr_oracle_32_4_13821020() {
    // Test EXTR 32-bit: alternating bits (oracle)
    // Encoding: 0x13821020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xAAAAAAAA);
    set_x(&mut cpu, 2, 0x55555555);
    let encoding: u32 = 0x13821020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA5555555, "W0 should be 0xA5555555");
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `EXTR X0, X1, X2, #4`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// alternating bits (64)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_extr_oracle_64_4_93c21020() {
    // Test EXTR 64-bit: alternating bits (oracle)
    // Encoding: 0x93C21020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 2, 0x55555555);
    set_x(&mut cpu, 1, 0xAAAAAAAA);
    let encoding: u32 = 0x93C21020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xA000000005555555,
        "X0 should be 0xA000000005555555"
    );
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_reg_write_0_13800000() {
    // Test aarch64_integer_ins_ext_extract_immediate register write: GpFromField("d")
    // Encoding: 0x13800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x13800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_sp_rn_138003e0() {
    // Test aarch64_integer_ins_ext_extract_immediate with Rn = SP (31)
    // Encoding: 0x138003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x138003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_ins_ext_extract_immediate
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_ins_ext_extract_immediate_zr_rd_1380001f() {
    // Test aarch64_integer_ins_ext_extract_immediate with Rd = ZR (31)
    // Encoding: 0x1380001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1380001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_integer_bitfield Tests
// ============================================================================

/// Provenance: aarch64_integer_bitfield
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_bitfield_field_sf_0_min_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field sf = 0 (Min)
    // Fields: opc=0, Rn=0, sf=0, Rd=0, imms=0, immr=0, N=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_bitfield_field_sf_1_max_0_93000000() {
    // Encoding: 0x93000000
    // Test aarch64_integer_bitfield field sf = 1 (Max)
    // Fields: immr=0, sf=1, Rn=0, Rd=0, N=0, imms=0, opc=0
    let encoding: u32 = 0x93000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field opc 29 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_bitfield_field_opc_0_min_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field opc = 0 (Min)
    // Fields: immr=0, sf=0, imms=0, opc=0, Rn=0, Rd=0, N=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field opc 29 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_bitfield_field_opc_1_poweroftwo_0_33000000() {
    // Encoding: 0x33000000
    // Test aarch64_integer_bitfield field opc = 1 (PowerOfTwo)
    // Fields: opc=1, N=0, Rd=0, immr=0, sf=0, Rn=0, imms=0
    let encoding: u32 = 0x33000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field opc 29 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_integer_bitfield_field_opc_2_poweroftwo_0_53000000() {
    // Encoding: 0x53000000
    // Test aarch64_integer_bitfield field opc = 2 (PowerOfTwo)
    // Fields: imms=0, N=0, immr=0, opc=2, Rn=0, Rd=0, sf=0
    let encoding: u32 = 0x53000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field opc 29 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_integer_bitfield_field_opc_3_max_0_73000000() {
    // Encoding: 0x73000000
    // Test aarch64_integer_bitfield field opc = 3 (Max)
    // Fields: opc=3, immr=0, sf=0, N=0, imms=0, Rd=0, Rn=0
    let encoding: u32 = 0x73000000;
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

/// Provenance: aarch64_integer_bitfield
/// ASL: `field N 22 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_bitfield_field_n_0_min_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field N = 0 (Min)
    // Fields: sf=0, N=0, immr=0, imms=0, Rn=0, Rd=0, opc=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field N 22 +: 1`
/// Requirement: FieldBoundary { field: "N", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_integer_bitfield_field_n_1_max_0_13400000() {
    // Encoding: 0x13400000
    // Test aarch64_integer_bitfield field N = 1 (Max)
    // Fields: Rn=0, opc=0, sf=0, Rd=0, immr=0, N=1, imms=0
    let encoding: u32 = 0x13400000;
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

/// Provenance: aarch64_integer_bitfield
/// ASL: `field immr 16 +: 6`
/// Requirement: FieldBoundary { field: "immr", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_bitfield_field_immr_0_zero_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field immr = 0 (Zero)
    // Fields: imms=0, Rn=0, Rd=0, opc=0, immr=0, sf=0, N=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field immr 16 +: 6`
/// Requirement: FieldBoundary { field: "immr", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_bitfield_field_immr_1_poweroftwo_0_13010000() {
    // Encoding: 0x13010000
    // Test aarch64_integer_bitfield field immr = 1 (PowerOfTwo)
    // Fields: Rd=0, imms=0, opc=0, sf=0, N=0, immr=1, Rn=0
    let encoding: u32 = 0x13010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field immr 16 +: 6`
/// Requirement: FieldBoundary { field: "immr", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_bitfield_field_immr_3_poweroftwominusone_0_13030000() {
    // Encoding: 0x13030000
    // Test aarch64_integer_bitfield field immr = 3 (PowerOfTwoMinusOne)
    // Fields: opc=0, N=0, Rd=0, Rn=0, imms=0, sf=0, immr=3
    let encoding: u32 = 0x13030000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field immr 16 +: 6`
/// Requirement: FieldBoundary { field: "immr", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_bitfield_field_immr_4_poweroftwo_0_13040000() {
    // Encoding: 0x13040000
    // Test aarch64_integer_bitfield field immr = 4 (PowerOfTwo)
    // Fields: sf=0, opc=0, N=0, Rd=0, immr=4, imms=0, Rn=0
    let encoding: u32 = 0x13040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field immr 16 +: 6`
/// Requirement: FieldBoundary { field: "immr", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_bitfield_field_immr_7_poweroftwominusone_0_13070000() {
    // Encoding: 0x13070000
    // Test aarch64_integer_bitfield field immr = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=0, imms=0, opc=0, N=0, immr=7, sf=0
    let encoding: u32 = 0x13070000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field immr 16 +: 6`
/// Requirement: FieldBoundary { field: "immr", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_bitfield_field_immr_8_poweroftwo_0_13080000() {
    // Encoding: 0x13080000
    // Test aarch64_integer_bitfield field immr = 8 (PowerOfTwo)
    // Fields: imms=0, sf=0, Rn=0, opc=0, N=0, immr=8, Rd=0
    let encoding: u32 = 0x13080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field immr 16 +: 6`
/// Requirement: FieldBoundary { field: "immr", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_bitfield_field_immr_15_poweroftwominusone_0_130f0000() {
    // Encoding: 0x130F0000
    // Test aarch64_integer_bitfield field immr = 15 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rn=0, imms=0, N=0, Rd=0, immr=15, opc=0
    let encoding: u32 = 0x130F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field immr 16 +: 6`
/// Requirement: FieldBoundary { field: "immr", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_bitfield_field_immr_16_poweroftwo_0_13100000() {
    // Encoding: 0x13100000
    // Test aarch64_integer_bitfield field immr = 16 (PowerOfTwo)
    // Fields: Rd=0, sf=0, opc=0, N=0, Rn=0, immr=16, imms=0
    let encoding: u32 = 0x13100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field immr 16 +: 6`
/// Requirement: FieldBoundary { field: "immr", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_aarch64_integer_bitfield_field_immr_31_poweroftwominusone_0_131f0000() {
    // Encoding: 0x131F0000
    // Test aarch64_integer_bitfield field immr = 31 (PowerOfTwoMinusOne)
    // Fields: opc=0, N=0, imms=0, Rn=0, Rd=0, sf=0, immr=31
    let encoding: u32 = 0x131F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field immr 16 +: 6`
/// Requirement: FieldBoundary { field: "immr", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_bitfield_field_immr_32_poweroftwo_0_13200000() {
    // Encoding: 0x13200000
    // Test aarch64_integer_bitfield field immr = 32 (PowerOfTwo)
    // Fields: sf=0, imms=0, opc=0, N=0, immr=32, Rd=0, Rn=0
    let encoding: u32 = 0x13200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field immr 16 +: 6`
/// Requirement: FieldBoundary { field: "immr", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_aarch64_integer_bitfield_field_immr_63_max_0_133f0000() {
    // Encoding: 0x133F0000
    // Test aarch64_integer_bitfield field immr = 63 (Max)
    // Fields: N=0, Rn=0, imms=0, immr=63, opc=0, Rd=0, sf=0
    let encoding: u32 = 0x133F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_bitfield_field_imms_0_zero_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field imms = 0 (Zero)
    // Fields: Rd=0, immr=0, N=0, opc=0, imms=0, Rn=0, sf=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_bitfield_field_imms_1_poweroftwo_0_13000400() {
    // Encoding: 0x13000400
    // Test aarch64_integer_bitfield field imms = 1 (PowerOfTwo)
    // Fields: sf=0, N=0, imms=1, immr=0, Rn=0, Rd=0, opc=0
    let encoding: u32 = 0x13000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_bitfield_field_imms_3_poweroftwominusone_0_13000c00() {
    // Encoding: 0x13000C00
    // Test aarch64_integer_bitfield field imms = 3 (PowerOfTwoMinusOne)
    // Fields: Rd=0, sf=0, imms=3, opc=0, N=0, immr=0, Rn=0
    let encoding: u32 = 0x13000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_bitfield_field_imms_4_poweroftwo_0_13001000() {
    // Encoding: 0x13001000
    // Test aarch64_integer_bitfield field imms = 4 (PowerOfTwo)
    // Fields: Rd=0, opc=0, sf=0, N=0, immr=0, Rn=0, imms=4
    let encoding: u32 = 0x13001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_bitfield_field_imms_7_poweroftwominusone_0_13001c00() {
    // Encoding: 0x13001C00
    // Test aarch64_integer_bitfield field imms = 7 (PowerOfTwoMinusOne)
    // Fields: Rd=0, N=0, opc=0, imms=7, immr=0, sf=0, Rn=0
    let encoding: u32 = 0x13001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_bitfield_field_imms_8_poweroftwo_0_13002000() {
    // Encoding: 0x13002000
    // Test aarch64_integer_bitfield field imms = 8 (PowerOfTwo)
    // Fields: sf=0, opc=0, N=0, Rd=0, immr=0, imms=8, Rn=0
    let encoding: u32 = 0x13002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_bitfield_field_imms_15_poweroftwominusone_0_13003c00() {
    // Encoding: 0x13003C00
    // Test aarch64_integer_bitfield field imms = 15 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rn=0, immr=0, sf=0, N=0, imms=15, Rd=0
    let encoding: u32 = 0x13003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_bitfield_field_imms_16_poweroftwo_0_13004000() {
    // Encoding: 0x13004000
    // Test aarch64_integer_bitfield field imms = 16 (PowerOfTwo)
    // Fields: N=0, imms=16, Rn=0, sf=0, immr=0, Rd=0, opc=0
    let encoding: u32 = 0x13004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_aarch64_integer_bitfield_field_imms_31_poweroftwominusone_0_13007c00() {
    // Encoding: 0x13007C00
    // Test aarch64_integer_bitfield field imms = 31 (PowerOfTwoMinusOne)
    // Fields: sf=0, N=0, opc=0, immr=0, imms=31, Rd=0, Rn=0
    let encoding: u32 = 0x13007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_bitfield_field_imms_32_poweroftwo_0_13008000() {
    // Encoding: 0x13008000
    // Test aarch64_integer_bitfield field imms = 32 (PowerOfTwo)
    // Fields: sf=0, N=0, immr=0, opc=0, imms=32, Rn=0, Rd=0
    let encoding: u32 = 0x13008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field imms 10 +: 6`
/// Requirement: FieldBoundary { field: "imms", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_aarch64_integer_bitfield_field_imms_63_max_0_1300fc00() {
    // Encoding: 0x1300FC00
    // Test aarch64_integer_bitfield field imms = 63 (Max)
    // Fields: imms=63, Rn=0, opc=0, Rd=0, N=0, sf=0, immr=0
    let encoding: u32 = 0x1300FC00;
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

/// Provenance: aarch64_integer_bitfield
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_bitfield_field_rn_0_min_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field Rn = 0 (Min)
    // Fields: immr=0, N=0, opc=0, imms=0, Rn=0, Rd=0, sf=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_bitfield_field_rn_1_poweroftwo_0_13000020() {
    // Encoding: 0x13000020
    // Test aarch64_integer_bitfield field Rn = 1 (PowerOfTwo)
    // Fields: imms=0, sf=0, N=0, Rn=1, opc=0, Rd=0, immr=0
    let encoding: u32 = 0x13000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_bitfield_field_rn_30_poweroftwominusone_0_130003c0() {
    // Encoding: 0x130003C0
    // Test aarch64_integer_bitfield field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: imms=0, sf=0, Rn=30, immr=0, N=0, Rd=0, opc=0
    let encoding: u32 = 0x130003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_integer_bitfield_field_rn_31_max_0_130003e0() {
    // Encoding: 0x130003E0
    // Test aarch64_integer_bitfield field Rn = 31 (Max)
    // Fields: opc=0, Rd=0, sf=0, Rn=31, imms=0, N=0, immr=0
    let encoding: u32 = 0x130003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_bitfield_field_rd_0_min_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field Rd = 0 (Min)
    // Fields: immr=0, opc=0, Rd=0, Rn=0, imms=0, sf=0, N=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_bitfield_field_rd_1_poweroftwo_0_13000001() {
    // Encoding: 0x13000001
    // Test aarch64_integer_bitfield field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, imms=0, sf=0, N=0, immr=0, Rn=0, opc=0
    let encoding: u32 = 0x13000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_bitfield_field_rd_30_poweroftwominusone_0_1300001e() {
    // Encoding: 0x1300001E
    // Test aarch64_integer_bitfield field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rd=30, imms=0, Rn=0, opc=0, immr=0, N=0
    let encoding: u32 = 0x1300001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_bitfield_field_rd_31_max_0_1300001f() {
    // Encoding: 0x1300001F
    // Test aarch64_integer_bitfield field Rd = 31 (Max)
    // Fields: Rd=31, sf=0, opc=0, N=0, imms=0, immr=0, Rn=0
    let encoding: u32 = 0x1300001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_bitfield_combo_0_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=0, Rd=0
    // Fields: Rn=0, sf=0, Rd=0, N=0, opc=0, immr=0, imms=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_bitfield_combo_1_0_93000000() {
    // Encoding: 0x93000000
    // Test aarch64_integer_bitfield field combination: sf=1, opc=0, N=0, immr=0, imms=0, Rn=0, Rd=0
    // Fields: immr=0, opc=0, imms=0, Rn=0, Rd=0, sf=1, N=0
    let encoding: u32 = 0x93000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_bitfield_combo_2_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=0, Rd=0
    // Fields: sf=0, immr=0, opc=0, N=0, imms=0, Rn=0, Rd=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_bitfield_combo_3_0_33000000() {
    // Encoding: 0x33000000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=1, N=0, immr=0, imms=0, Rn=0, Rd=0
    // Fields: N=0, sf=0, imms=0, opc=1, Rn=0, immr=0, Rd=0
    let encoding: u32 = 0x33000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_integer_bitfield_combo_4_0_53000000() {
    // Encoding: 0x53000000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=2, N=0, immr=0, imms=0, Rn=0, Rd=0
    // Fields: Rd=0, imms=0, N=0, sf=0, opc=2, immr=0, Rn=0
    let encoding: u32 = 0x53000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_integer_bitfield_combo_5_0_73000000() {
    // Encoding: 0x73000000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=3, N=0, immr=0, imms=0, Rn=0, Rd=0
    // Fields: N=0, imms=0, Rn=0, Rd=0, sf=0, opc=3, immr=0
    let encoding: u32 = 0x73000000;
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

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// N=0 (minimum value)
#[test]
fn test_aarch64_integer_bitfield_combo_6_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=0, Rd=0
    // Fields: sf=0, immr=0, imms=0, Rn=0, Rd=0, opc=0, N=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// N=1 (maximum value (1))
#[test]
fn test_aarch64_integer_bitfield_combo_7_0_13400000() {
    // Encoding: 0x13400000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=1, immr=0, imms=0, Rn=0, Rd=0
    // Fields: immr=0, N=1, opc=0, sf=0, Rn=0, imms=0, Rd=0
    let encoding: u32 = 0x13400000;
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

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immr=0 (immediate value 0)
#[test]
fn test_aarch64_integer_bitfield_combo_8_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=0, Rd=0
    // Fields: Rd=0, opc=0, sf=0, N=0, immr=0, Rn=0, imms=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immr=1 (immediate value 1)
#[test]
fn test_aarch64_integer_bitfield_combo_9_0_13010000() {
    // Encoding: 0x13010000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=1, imms=0, Rn=0, Rd=0
    // Fields: Rd=0, opc=0, sf=0, N=0, immr=1, imms=0, Rn=0
    let encoding: u32 = 0x13010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immr=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_bitfield_combo_10_0_13030000() {
    // Encoding: 0x13030000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=3, imms=0, Rn=0, Rd=0
    // Fields: opc=0, sf=0, Rd=0, N=0, immr=3, imms=0, Rn=0
    let encoding: u32 = 0x13030000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immr=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_bitfield_combo_11_0_13040000() {
    // Encoding: 0x13040000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=4, imms=0, Rn=0, Rd=0
    // Fields: immr=4, Rd=0, imms=0, Rn=0, sf=0, opc=0, N=0
    let encoding: u32 = 0x13040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immr=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_bitfield_combo_12_0_13070000() {
    // Encoding: 0x13070000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=7, imms=0, Rn=0, Rd=0
    // Fields: opc=0, sf=0, N=0, immr=7, imms=0, Rn=0, Rd=0
    let encoding: u32 = 0x13070000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immr=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_bitfield_combo_13_0_13080000() {
    // Encoding: 0x13080000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=8, imms=0, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, immr=8, imms=0, opc=0, Rd=0, N=0
    let encoding: u32 = 0x13080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immr=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_bitfield_combo_14_0_130f0000() {
    // Encoding: 0x130F0000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=15, imms=0, Rn=0, Rd=0
    // Fields: imms=0, Rn=0, opc=0, sf=0, N=0, immr=15, Rd=0
    let encoding: u32 = 0x130F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immr=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_bitfield_combo_15_0_13100000() {
    // Encoding: 0x13100000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=16, imms=0, Rn=0, Rd=0
    // Fields: sf=0, opc=0, N=0, immr=16, Rn=0, Rd=0, imms=0
    let encoding: u32 = 0x13100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immr=31 (immediate midpoint (31))
#[test]
fn test_aarch64_integer_bitfield_combo_16_0_131f0000() {
    // Encoding: 0x131F0000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=31, imms=0, Rn=0, Rd=0
    // Fields: sf=0, opc=0, N=0, imms=0, Rn=0, immr=31, Rd=0
    let encoding: u32 = 0x131F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immr=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_bitfield_combo_17_0_13200000() {
    // Encoding: 0x13200000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=32, imms=0, Rn=0, Rd=0
    // Fields: immr=32, sf=0, opc=0, Rd=0, Rn=0, imms=0, N=0
    let encoding: u32 = 0x13200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immr=63 (maximum immediate (63))
#[test]
fn test_aarch64_integer_bitfield_combo_18_0_133f0000() {
    // Encoding: 0x133F0000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=63, imms=0, Rn=0, Rd=0
    // Fields: opc=0, Rd=0, imms=0, immr=63, sf=0, Rn=0, N=0
    let encoding: u32 = 0x133F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=0 (immediate value 0)
#[test]
fn test_aarch64_integer_bitfield_combo_19_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, sf=0, imms=0, immr=0, N=0, opc=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=1 (immediate value 1)
#[test]
fn test_aarch64_integer_bitfield_combo_20_0_13000400() {
    // Encoding: 0x13000400
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=1, Rn=0, Rd=0
    // Fields: opc=0, N=0, imms=1, Rn=0, immr=0, Rd=0, sf=0
    let encoding: u32 = 0x13000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_bitfield_combo_21_0_13000c00() {
    // Encoding: 0x13000C00
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=3, Rn=0, Rd=0
    // Fields: opc=0, Rn=0, Rd=0, N=0, sf=0, immr=0, imms=3
    let encoding: u32 = 0x13000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_bitfield_combo_22_0_13001000() {
    // Encoding: 0x13001000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=4, Rn=0, Rd=0
    // Fields: N=0, sf=0, opc=0, imms=4, Rn=0, immr=0, Rd=0
    let encoding: u32 = 0x13001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_bitfield_combo_23_0_13001c00() {
    // Encoding: 0x13001C00
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=7, Rn=0, Rd=0
    // Fields: Rn=0, immr=0, N=0, imms=7, Rd=0, opc=0, sf=0
    let encoding: u32 = 0x13001C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_bitfield_combo_24_0_13002000() {
    // Encoding: 0x13002000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=8, Rn=0, Rd=0
    // Fields: opc=0, Rn=0, imms=8, Rd=0, sf=0, N=0, immr=0
    let encoding: u32 = 0x13002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_bitfield_combo_25_0_13003c00() {
    // Encoding: 0x13003C00
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=15, Rn=0, Rd=0
    // Fields: immr=0, N=0, sf=0, imms=15, Rn=0, Rd=0, opc=0
    let encoding: u32 = 0x13003C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_bitfield_combo_26_0_13004000() {
    // Encoding: 0x13004000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=16, Rn=0, Rd=0
    // Fields: imms=16, opc=0, N=0, Rn=0, immr=0, Rd=0, sf=0
    let encoding: u32 = 0x13004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=31 (immediate midpoint (31))
#[test]
fn test_aarch64_integer_bitfield_combo_27_0_13007c00() {
    // Encoding: 0x13007C00
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=31, Rn=0, Rd=0
    // Fields: immr=0, imms=31, sf=0, opc=0, N=0, Rd=0, Rn=0
    let encoding: u32 = 0x13007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_bitfield_combo_28_0_13008000() {
    // Encoding: 0x13008000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=32, Rn=0, Rd=0
    // Fields: imms=32, Rd=0, sf=0, N=0, opc=0, immr=0, Rn=0
    let encoding: u32 = 0x13008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imms=63 (maximum immediate (63))
#[test]
fn test_aarch64_integer_bitfield_combo_29_0_1300fc00() {
    // Encoding: 0x1300FC00
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=63, Rn=0, Rd=0
    // Fields: sf=0, opc=0, imms=63, immr=0, Rn=0, Rd=0, N=0
    let encoding: u32 = 0x1300FC00;
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

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_bitfield_combo_30_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=0, Rd=0
    // Fields: opc=0, immr=0, N=0, imms=0, Rn=0, sf=0, Rd=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_bitfield_combo_31_0_13000020() {
    // Encoding: 0x13000020
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=1, Rd=0
    // Fields: immr=0, sf=0, opc=0, imms=0, Rn=1, N=0, Rd=0
    let encoding: u32 = 0x13000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_bitfield_combo_32_0_130003c0() {
    // Encoding: 0x130003C0
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=30, Rd=0
    // Fields: Rd=0, sf=0, immr=0, N=0, imms=0, Rn=30, opc=0
    let encoding: u32 = 0x130003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_integer_bitfield_combo_33_0_130003e0() {
    // Encoding: 0x130003E0
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=31, Rd=0
    // Fields: Rn=31, Rd=0, sf=0, opc=0, imms=0, N=0, immr=0
    let encoding: u32 = 0x130003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_bitfield_combo_34_0_13000000() {
    // Encoding: 0x13000000
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=0, Rd=0
    // Fields: Rn=0, imms=0, immr=0, sf=0, Rd=0, opc=0, N=0
    let encoding: u32 = 0x13000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_bitfield_combo_35_0_13000001() {
    // Encoding: 0x13000001
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=0, Rd=1
    // Fields: opc=0, N=0, Rn=0, immr=0, Rd=1, sf=0, imms=0
    let encoding: u32 = 0x13000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_bitfield_combo_36_0_1300001e() {
    // Encoding: 0x1300001E
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=0, Rd=30
    // Fields: sf=0, opc=0, N=0, immr=0, imms=0, Rd=30, Rn=0
    let encoding: u32 = 0x1300001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_bitfield_combo_37_0_1300001f() {
    // Encoding: 0x1300001F
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=0, Rd=31
    // Fields: imms=0, Rn=0, opc=0, Rd=31, sf=0, immr=0, N=0
    let encoding: u32 = 0x1300001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_integer_bitfield_combo_38_0_13000021() {
    // Encoding: 0x13000021
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=1, Rd=1
    // Fields: Rd=1, immr=0, N=0, imms=0, opc=0, sf=0, Rn=1
    let encoding: u32 = 0x13000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field combination 39`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_integer_bitfield_combo_39_0_130003ff() {
    // Encoding: 0x130003FF
    // Test aarch64_integer_bitfield field combination: sf=0, opc=0, N=0, immr=0, imms=0, Rn=31, Rd=31
    // Fields: imms=0, Rd=31, N=0, sf=0, Rn=31, opc=0, immr=0
    let encoding: u32 = 0x130003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_bitfield_special_sf_0_size_variant_0_0_13010400() {
    // Encoding: 0x13010400
    // Test aarch64_integer_bitfield special value sf = 0 (Size variant 0)
    // Fields: N=0, sf=0, immr=1, imms=1, Rn=0, opc=0, Rd=0
    let encoding: u32 = 0x13010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_bitfield_special_sf_1_size_variant_1_0_93010400() {
    // Encoding: 0x93010400
    // Test aarch64_integer_bitfield special value sf = 1 (Size variant 1)
    // Fields: immr=1, Rn=0, Rd=0, opc=0, imms=1, N=0, sf=1
    let encoding: u32 = 0x93010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_bitfield_special_opc_0_size_variant_0_0_13010400() {
    // Encoding: 0x13010400
    // Test aarch64_integer_bitfield special value opc = 0 (Size variant 0)
    // Fields: Rd=0, opc=0, N=0, sf=0, imms=1, Rn=0, immr=1
    let encoding: u32 = 0x13010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_bitfield_special_opc_1_size_variant_1_0_33010400() {
    // Encoding: 0x33010400
    // Test aarch64_integer_bitfield special value opc = 1 (Size variant 1)
    // Fields: imms=1, Rn=0, opc=1, N=0, sf=0, immr=1, Rd=0
    let encoding: u32 = 0x33010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_integer_bitfield_special_opc_2_size_variant_2_0_53010400() {
    // Encoding: 0x53010400
    // Test aarch64_integer_bitfield special value opc = 2 (Size variant 2)
    // Fields: opc=2, immr=1, imms=1, sf=0, N=0, Rn=0, Rd=0
    let encoding: u32 = 0x53010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_integer_bitfield_special_opc_3_size_variant_3_0_73010400() {
    // Encoding: 0x73010400
    // Test aarch64_integer_bitfield special value opc = 3 (Size variant 3)
    // Fields: sf=0, opc=3, N=0, immr=1, Rd=0, imms=1, Rn=0
    let encoding: u32 = 0x73010400;
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

/// Provenance: aarch64_integer_bitfield
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_integer_bitfield_special_rn_31_stack_pointer_sp_may_require_alignment_0_130107e0() {
    // Encoding: 0x130107E0
    // Test aarch64_integer_bitfield special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: imms=1, Rn=31, Rd=0, sf=0, opc=0, N=0, immr=1
    let encoding: u32 = 0x130107E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_bitfield_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_1301041f()
 {
    // Encoding: 0x1301041F
    // Test aarch64_integer_bitfield special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: opc=0, sf=0, immr=1, imms=1, Rn=0, Rd=31, N=0
    let encoding: u32 = 0x1301041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #0, #7`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// extract byte (UXTB/SXTB) (32)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_32_0_13001c20() {
    // Test SBFM 32-bit: extract byte (UXTB/SXTB) (oracle)
    // Encoding: 0x13001C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0x13001C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #0, #7`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// extract byte (UXTB/SXTB) (64)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_64_0_93401c20() {
    // Test SBFM 64-bit: extract byte (UXTB/SXTB) (oracle)
    // Encoding: 0x93401C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFF);
    let encoding: u32 = 0x93401C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #0, #7`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// extract signed byte (32)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_32_1_13001c20() {
    // Test SBFM 32-bit: extract signed byte (oracle)
    // Encoding: 0x13001C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x80);
    let encoding: u32 = 0x13001C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFF80, "W0 should be 0xFFFFFF80");
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #0, #7`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// extract signed byte (64)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_64_1_93401c20() {
    // Test SBFM 64-bit: extract signed byte (oracle)
    // Encoding: 0x93401C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x80);
    let encoding: u32 = 0x93401C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFF80,
        "X0 should be 0xFFFFFFFFFFFFFF80"
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #0, #15`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// extract halfword (UXTH/SXTH) (32)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_32_2_13003c20() {
    // Test SBFM 32-bit: extract halfword (UXTH/SXTH) (oracle)
    // Encoding: 0x13003C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFF);
    let encoding: u32 = 0x13003C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #0, #15`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// extract halfword (UXTH/SXTH) (64)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_64_2_93403c20() {
    // Test SBFM 64-bit: extract halfword (UXTH/SXTH) (oracle)
    // Encoding: 0x93403C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFF);
    let encoding: u32 = 0x93403C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #0, #15`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// extract signed halfword (32)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_32_3_13003c20() {
    // Test SBFM 32-bit: extract signed halfword (oracle)
    // Encoding: 0x13003C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000);
    let encoding: u32 = 0x13003C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF8000, "W0 should be 0xFFFF8000");
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #0, #15`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// extract signed halfword (64)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_64_3_93403c20() {
    // Test SBFM 64-bit: extract signed halfword (oracle)
    // Encoding: 0x93403C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x8000);
    let encoding: u32 = 0x93403C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFF8000,
        "X0 should be 0xFFFFFFFFFFFF8000"
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #0, #31`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// extract word (32-bit extract) (32)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_32_4_13007c20() {
    // Test SBFM 32-bit: extract word (32-bit extract) (oracle)
    // Encoding: 0x13007C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x13007C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #0, #31`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// extract word (32-bit extract) (64)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_64_4_93407c20() {
    // Test SBFM 64-bit: extract word (32-bit extract) (oracle)
    // Encoding: 0x93407C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0x93407C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #8, #15`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// extract bits [15:8] (32)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_32_5_13083c20() {
    // Test SBFM 32-bit: extract bits [15:8] (oracle)
    // Encoding: 0x13083C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x12345678);
    let encoding: u32 = 0x13083C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x56, "W0 should be 0x00000056");
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #8, #15`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// extract bits [15:8] (64)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_64_5_93483c20() {
    // Test SBFM 64-bit: extract bits [15:8] (oracle)
    // Encoding: 0x93483C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x12345678);
    let encoding: u32 = 0x93483C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x56, "X0 should be 0x0000000000000056");
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #4, #7`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// extract nibble (32)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_32_6_13041c20() {
    // Test SBFM 32-bit: extract nibble (oracle)
    // Encoding: 0x13041C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xDEADBEEF);
    let encoding: u32 = 0x13041C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFE, "W0 should be 0xFFFFFFFE");
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #4, #7`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// extract nibble (64)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_64_6_93441c20() {
    // Test SBFM 64-bit: extract nibble (oracle)
    // Encoding: 0x93441C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xDEADBEEF);
    let encoding: u32 = 0x93441C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFE,
        "X0 should be 0xFFFFFFFFFFFFFFFE"
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #60, #3`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// insert at position 60 (UBFIZ) (64)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_64_7_937c0c20() {
    // Test SBFM 64-bit: insert at position 60 (UBFIZ) (oracle)
    // Encoding: 0x937C0C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xCAFE);
    let encoding: u32 = 0x937C0C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFE0,
        "X0 should be 0xFFFFFFFFFFFFFFE0"
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #16, #31`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// BFM insert (32)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_32_8_13107c20() {
    // Test SBFM 32-bit: BFM insert (oracle)
    // Encoding: 0x13107C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xCCCCDDDD);
    let encoding: u32 = 0x13107C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFCCCC, "W0 should be 0xFFFFCCCC");
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `SBFM X0, X1, #16, #31`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// BFM insert (64)
#[test]
fn test_aarch64_integer_bitfield_sbfm_oracle_64_8_93507c20() {
    // Test SBFM 64-bit: BFM insert (oracle)
    // Encoding: 0x93507C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0xAAAABBBBCCCCDDDD);
    let encoding: u32 = 0x93507C20;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFCCCC,
        "X0 should be 0xFFFFFFFFFFFFCCCC"
    );
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_bitfield_reg_write_0_13000000() {
    // Test aarch64_integer_bitfield register write: GpFromField("d")
    // Encoding: 0x13000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x13000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_integer_bitfield_sp_rn_130003e0() {
    // Test aarch64_integer_bitfield with Rn = SP (31)
    // Encoding: 0x130003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x130003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_bitfield
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_bitfield_zr_rd_1300001f() {
    // Test aarch64_integer_bitfield with Rd = ZR (31)
    // Encoding: 0x1300001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1300001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_integer_ins_ext_insert_movewide Tests
// ============================================================================

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_sf_0_min_0_12800000() {
    // Encoding: 0x12800000
    // Test aarch64_integer_ins_ext_insert_movewide field sf = 0 (Min)
    // Fields: sf=0, hw=0, opc=0, Rd=0, imm16=0
    let encoding: u32 = 0x12800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_sf_1_max_0_92800000() {
    // Encoding: 0x92800000
    // Test aarch64_integer_ins_ext_insert_movewide field sf = 1 (Max)
    // Fields: sf=1, opc=0, imm16=0, Rd=0, hw=0
    let encoding: u32 = 0x92800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field opc 29 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_opc_0_min_0_12800000() {
    // Encoding: 0x12800000
    // Test aarch64_integer_ins_ext_insert_movewide field opc = 0 (Min)
    // Fields: sf=0, opc=0, hw=0, imm16=0, Rd=0
    let encoding: u32 = 0x12800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field opc 29 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_opc_1_poweroftwo_0_32800000() {
    // Encoding: 0x32800000
    // Test aarch64_integer_ins_ext_insert_movewide field opc = 1 (PowerOfTwo)
    // Fields: Rd=0, opc=1, hw=0, imm16=0, sf=0
    let encoding: u32 = 0x32800000;
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

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field opc 29 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_opc_2_poweroftwo_0_52800000() {
    // Encoding: 0x52800000
    // Test aarch64_integer_ins_ext_insert_movewide field opc = 2 (PowerOfTwo)
    // Fields: sf=0, imm16=0, opc=2, Rd=0, hw=0
    let encoding: u32 = 0x52800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field opc 29 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_opc_3_max_0_72800000() {
    // Encoding: 0x72800000
    // Test aarch64_integer_ins_ext_insert_movewide field opc = 3 (Max)
    // Fields: sf=0, imm16=0, hw=0, opc=3, Rd=0
    let encoding: u32 = 0x72800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field hw 21 +: 2`
/// Requirement: FieldBoundary { field: "hw", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_hw_0_min_0_12800000() {
    // Encoding: 0x12800000
    // Test aarch64_integer_ins_ext_insert_movewide field hw = 0 (Min)
    // Fields: opc=0, Rd=0, imm16=0, hw=0, sf=0
    let encoding: u32 = 0x12800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field hw 21 +: 2`
/// Requirement: FieldBoundary { field: "hw", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_hw_1_poweroftwo_0_12a00000() {
    // Encoding: 0x12A00000
    // Test aarch64_integer_ins_ext_insert_movewide field hw = 1 (PowerOfTwo)
    // Fields: Rd=0, hw=1, imm16=0, sf=0, opc=0
    let encoding: u32 = 0x12A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field hw 21 +: 2`
/// Requirement: FieldBoundary { field: "hw", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_hw_3_max_0_12e00000() {
    // Encoding: 0x12E00000
    // Test aarch64_integer_ins_ext_insert_movewide field hw = 3 (Max)
    // Fields: Rd=0, opc=0, imm16=0, hw=3, sf=0
    let encoding: u32 = 0x12E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_0_zero_0_12800000() {
    // Encoding: 0x12800000
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 0 (Zero)
    // Fields: hw=0, imm16=0, Rd=0, sf=0, opc=0
    let encoding: u32 = 0x12800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_1_poweroftwo_0_12800020() {
    // Encoding: 0x12800020
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 1 (PowerOfTwo)
    // Fields: hw=0, Rd=0, sf=0, opc=0, imm16=1
    let encoding: u32 = 0x12800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_3_poweroftwominusone_0_12800060() {
    // Encoding: 0x12800060
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 3 (PowerOfTwoMinusOne)
    // Fields: hw=0, opc=0, imm16=3, Rd=0, sf=0
    let encoding: u32 = 0x12800060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_4_poweroftwo_0_12800080() {
    // Encoding: 0x12800080
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 4 (PowerOfTwo)
    // Fields: Rd=0, sf=0, imm16=4, hw=0, opc=0
    let encoding: u32 = 0x12800080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_7_poweroftwominusone_0_128000e0() {
    // Encoding: 0x128000E0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 7 (PowerOfTwoMinusOne)
    // Fields: Rd=0, opc=0, sf=0, hw=0, imm16=7
    let encoding: u32 = 0x128000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_8_poweroftwo_0_12800100() {
    // Encoding: 0x12800100
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 8 (PowerOfTwo)
    // Fields: hw=0, sf=0, Rd=0, imm16=8, opc=0
    let encoding: u32 = 0x12800100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_15_poweroftwominusone_0_128001e0() {
    // Encoding: 0x128001E0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 15 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rd=0, imm16=15, opc=0, hw=0
    let encoding: u32 = 0x128001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_16_poweroftwo_0_12800200() {
    // Encoding: 0x12800200
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 16 (PowerOfTwo)
    // Fields: opc=0, hw=0, sf=0, Rd=0, imm16=16
    let encoding: u32 = 0x12800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_31_poweroftwominusone_0_128003e0() {
    // Encoding: 0x128003E0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 31 (PowerOfTwoMinusOne)
    // Fields: imm16=31, Rd=0, hw=0, opc=0, sf=0
    let encoding: u32 = 0x128003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_32_poweroftwo_0_12800400() {
    // Encoding: 0x12800400
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 32 (PowerOfTwo)
    // Fields: opc=0, imm16=32, Rd=0, sf=0, hw=0
    let encoding: u32 = 0x12800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_63_poweroftwominusone_0_128007e0() {
    // Encoding: 0x128007E0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 63 (PowerOfTwoMinusOne)
    // Fields: imm16=63, Rd=0, sf=0, hw=0, opc=0
    let encoding: u32 = 0x128007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_64_poweroftwo_0_12800800() {
    // Encoding: 0x12800800
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 64 (PowerOfTwo)
    // Fields: hw=0, Rd=0, opc=0, imm16=64, sf=0
    let encoding: u32 = 0x12800800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 127, boundary: PowerOfTwoMinusOne }
/// 2^7 - 1 = 127
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_127_poweroftwominusone_0_12800fe0() {
    // Encoding: 0x12800FE0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 127 (PowerOfTwoMinusOne)
    // Fields: hw=0, imm16=127, opc=0, Rd=0, sf=0
    let encoding: u32 = 0x12800FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_128_poweroftwo_0_12801000() {
    // Encoding: 0x12801000
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 128 (PowerOfTwo)
    // Fields: opc=0, Rd=0, hw=0, sf=0, imm16=128
    let encoding: u32 = 0x12801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 255, boundary: PowerOfTwoMinusOne }
/// 2^8 - 1 = 255
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_255_poweroftwominusone_0_12801fe0() {
    // Encoding: 0x12801FE0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 255 (PowerOfTwoMinusOne)
    // Fields: sf=0, Rd=0, imm16=255, hw=0, opc=0
    let encoding: u32 = 0x12801FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 256, boundary: PowerOfTwo }
/// power of 2 (2^8 = 256)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_256_poweroftwo_0_12802000() {
    // Encoding: 0x12802000
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 256 (PowerOfTwo)
    // Fields: Rd=0, sf=0, opc=0, hw=0, imm16=256
    let encoding: u32 = 0x12802000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 511, boundary: PowerOfTwoMinusOne }
/// 2^9 - 1 = 511
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_511_poweroftwominusone_0_12803fe0() {
    // Encoding: 0x12803FE0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 511 (PowerOfTwoMinusOne)
    // Fields: hw=0, opc=0, sf=0, Rd=0, imm16=511
    let encoding: u32 = 0x12803FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 512, boundary: PowerOfTwo }
/// power of 2 (2^9 = 512)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_512_poweroftwo_0_12804000() {
    // Encoding: 0x12804000
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 512 (PowerOfTwo)
    // Fields: sf=0, opc=0, Rd=0, imm16=512, hw=0
    let encoding: u32 = 0x12804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 1023, boundary: PowerOfTwoMinusOne }
/// 2^10 - 1 = 1023
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_1023_poweroftwominusone_0_12807fe0() {
    // Encoding: 0x12807FE0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 1023 (PowerOfTwoMinusOne)
    // Fields: hw=0, sf=0, opc=0, imm16=1023, Rd=0
    let encoding: u32 = 0x12807FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 1024, boundary: PowerOfTwo }
/// power of 2 (2^10 = 1024)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_1024_poweroftwo_0_12808000() {
    // Encoding: 0x12808000
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 1024 (PowerOfTwo)
    // Fields: sf=0, imm16=1024, Rd=0, opc=0, hw=0
    let encoding: u32 = 0x12808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 2047, boundary: PowerOfTwoMinusOne }
/// 2^11 - 1 = 2047
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_2047_poweroftwominusone_0_1280ffe0() {
    // Encoding: 0x1280FFE0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 2047 (PowerOfTwoMinusOne)
    // Fields: imm16=2047, hw=0, opc=0, Rd=0, sf=0
    let encoding: u32 = 0x1280FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 2048, boundary: PowerOfTwo }
/// power of 2 (2^11 = 2048)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_2048_poweroftwo_0_12810000() {
    // Encoding: 0x12810000
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 2048 (PowerOfTwo)
    // Fields: Rd=0, imm16=2048, hw=0, sf=0, opc=0
    let encoding: u32 = 0x12810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 4095, boundary: PowerOfTwoMinusOne }
/// 2^12 - 1 = 4095
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_4095_poweroftwominusone_0_1281ffe0() {
    // Encoding: 0x1281FFE0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 4095 (PowerOfTwoMinusOne)
    // Fields: imm16=4095, Rd=0, hw=0, opc=0, sf=0
    let encoding: u32 = 0x1281FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 4096, boundary: PowerOfTwo }
/// power of 2 (2^12 = 4096)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_4096_poweroftwo_0_12820000() {
    // Encoding: 0x12820000
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 4096 (PowerOfTwo)
    // Fields: opc=0, sf=0, Rd=0, hw=0, imm16=4096
    let encoding: u32 = 0x12820000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 8191, boundary: PowerOfTwoMinusOne }
/// 2^13 - 1 = 8191
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_8191_poweroftwominusone_0_1283ffe0() {
    // Encoding: 0x1283FFE0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 8191 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm16=8191, Rd=0, sf=0, hw=0
    let encoding: u32 = 0x1283FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 8192, boundary: PowerOfTwo }
/// power of 2 (2^13 = 8192)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_8192_poweroftwo_0_12840000() {
    // Encoding: 0x12840000
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 8192 (PowerOfTwo)
    // Fields: imm16=8192, Rd=0, opc=0, sf=0, hw=0
    let encoding: u32 = 0x12840000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 16383, boundary: PowerOfTwoMinusOne }
/// 2^14 - 1 = 16383
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_16383_poweroftwominusone_0_1287ffe0() {
    // Encoding: 0x1287FFE0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 16383 (PowerOfTwoMinusOne)
    // Fields: Rd=0, opc=0, imm16=16383, sf=0, hw=0
    let encoding: u32 = 0x1287FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 16384, boundary: PowerOfTwo }
/// power of 2 (2^14 = 16384)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_16384_poweroftwo_0_12880000() {
    // Encoding: 0x12880000
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 16384 (PowerOfTwo)
    // Fields: imm16=16384, opc=0, hw=0, Rd=0, sf=0
    let encoding: u32 = 0x12880000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 32767, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (32767)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_32767_poweroftwominusone_0_128fffe0() {
    // Encoding: 0x128FFFE0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 32767 (PowerOfTwoMinusOne)
    // Fields: sf=0, imm16=32767, Rd=0, hw=0, opc=0
    let encoding: u32 = 0x128FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 32768, boundary: PowerOfTwo }
/// power of 2 (2^15 = 32768)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_32768_poweroftwo_0_12900000() {
    // Encoding: 0x12900000
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 32768 (PowerOfTwo)
    // Fields: opc=0, hw=0, imm16=32768, sf=0, Rd=0
    let encoding: u32 = 0x12900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field imm16 5 +: 16`
/// Requirement: FieldBoundary { field: "imm16", value: 65535, boundary: Max }
/// maximum immediate (65535)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_imm16_65535_max_0_129fffe0() {
    // Encoding: 0x129FFFE0
    // Test aarch64_integer_ins_ext_insert_movewide field imm16 = 65535 (Max)
    // Fields: imm16=65535, hw=0, sf=0, Rd=0, opc=0
    let encoding: u32 = 0x129FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_rd_0_min_0_12800000() {
    // Encoding: 0x12800000
    // Test aarch64_integer_ins_ext_insert_movewide field Rd = 0 (Min)
    // Fields: opc=0, sf=0, hw=0, imm16=0, Rd=0
    let encoding: u32 = 0x12800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_rd_1_poweroftwo_0_12800001() {
    // Encoding: 0x12800001
    // Test aarch64_integer_ins_ext_insert_movewide field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, opc=0, imm16=0, sf=0, hw=0
    let encoding: u32 = 0x12800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_rd_30_poweroftwominusone_0_1280001e() {
    // Encoding: 0x1280001E
    // Test aarch64_integer_ins_ext_insert_movewide field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm16=0, hw=0, Rd=30, sf=0
    let encoding: u32 = 0x1280001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_field_rd_31_max_0_1280001f() {
    // Encoding: 0x1280001F
    // Test aarch64_integer_ins_ext_insert_movewide field Rd = 31 (Max)
    // Fields: Rd=31, sf=0, opc=0, hw=0, imm16=0
    let encoding: u32 = 0x1280001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_0_0_12800000() {
    // Encoding: 0x12800000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=0, Rd=0
    // Fields: opc=0, hw=0, sf=0, Rd=0, imm16=0
    let encoding: u32 = 0x12800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_1_0_92800000() {
    // Encoding: 0x92800000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=1, opc=0, hw=0, imm16=0, Rd=0
    // Fields: sf=1, hw=0, opc=0, imm16=0, Rd=0
    let encoding: u32 = 0x92800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_2_0_12800000() {
    // Encoding: 0x12800000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=0, Rd=0
    // Fields: sf=0, hw=0, imm16=0, opc=0, Rd=0
    let encoding: u32 = 0x12800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_3_0_32800000() {
    // Encoding: 0x32800000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=1, hw=0, imm16=0, Rd=0
    // Fields: imm16=0, Rd=0, hw=0, opc=1, sf=0
    let encoding: u32 = 0x32800000;
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

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_4_0_52800000() {
    // Encoding: 0x52800000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=2, hw=0, imm16=0, Rd=0
    // Fields: hw=0, imm16=0, sf=0, Rd=0, opc=2
    let encoding: u32 = 0x52800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_5_0_72800000() {
    // Encoding: 0x72800000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=3, hw=0, imm16=0, Rd=0
    // Fields: opc=3, sf=0, hw=0, Rd=0, imm16=0
    let encoding: u32 = 0x72800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// hw=0 (minimum value)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_6_0_12800000() {
    // Encoding: 0x12800000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=0, Rd=0
    // Fields: opc=0, hw=0, imm16=0, sf=0, Rd=0
    let encoding: u32 = 0x12800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// hw=1 (value 1)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_7_0_12a00000() {
    // Encoding: 0x12A00000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=1, imm16=0, Rd=0
    // Fields: Rd=0, hw=1, opc=0, sf=0, imm16=0
    let encoding: u32 = 0x12A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// hw=3 (maximum value (3))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_8_0_12e00000() {
    // Encoding: 0x12E00000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=3, imm16=0, Rd=0
    // Fields: sf=0, hw=3, opc=0, Rd=0, imm16=0
    let encoding: u32 = 0x12E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=0 (immediate value 0)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_9_0_12800000() {
    // Encoding: 0x12800000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=0, Rd=0
    // Fields: sf=0, opc=0, imm16=0, hw=0, Rd=0
    let encoding: u32 = 0x12800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=1 (immediate value 1)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_10_0_12800020() {
    // Encoding: 0x12800020
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=1, Rd=0
    // Fields: sf=0, Rd=0, opc=0, imm16=1, hw=0
    let encoding: u32 = 0x12800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_11_0_12800060() {
    // Encoding: 0x12800060
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=3, Rd=0
    // Fields: imm16=3, opc=0, Rd=0, hw=0, sf=0
    let encoding: u32 = 0x12800060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_12_0_12800080() {
    // Encoding: 0x12800080
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=4, Rd=0
    // Fields: opc=0, imm16=4, sf=0, Rd=0, hw=0
    let encoding: u32 = 0x12800080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_13_0_128000e0() {
    // Encoding: 0x128000E0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=7, Rd=0
    // Fields: sf=0, opc=0, imm16=7, hw=0, Rd=0
    let encoding: u32 = 0x128000E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_14_0_12800100() {
    // Encoding: 0x12800100
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=8, Rd=0
    // Fields: imm16=8, Rd=0, opc=0, hw=0, sf=0
    let encoding: u32 = 0x12800100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_15_0_128001e0() {
    // Encoding: 0x128001E0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=15, Rd=0
    // Fields: sf=0, opc=0, Rd=0, hw=0, imm16=15
    let encoding: u32 = 0x128001E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_16_0_12800200() {
    // Encoding: 0x12800200
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=16, Rd=0
    // Fields: opc=0, Rd=0, hw=0, imm16=16, sf=0
    let encoding: u32 = 0x12800200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_17_0_128003e0() {
    // Encoding: 0x128003E0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=31, Rd=0
    // Fields: sf=0, imm16=31, Rd=0, opc=0, hw=0
    let encoding: u32 = 0x128003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_18_0_12800400() {
    // Encoding: 0x12800400
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=32, Rd=0
    // Fields: sf=0, opc=0, hw=0, imm16=32, Rd=0
    let encoding: u32 = 0x12800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_19_0_128007e0() {
    // Encoding: 0x128007E0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=63, Rd=0
    // Fields: imm16=63, sf=0, opc=0, Rd=0, hw=0
    let encoding: u32 = 0x128007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_20_0_12800800() {
    // Encoding: 0x12800800
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=64, Rd=0
    // Fields: sf=0, hw=0, imm16=64, opc=0, Rd=0
    let encoding: u32 = 0x12800800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=127 (2^7 - 1 = 127)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_21_0_12800fe0() {
    // Encoding: 0x12800FE0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=127, Rd=0
    // Fields: opc=0, imm16=127, Rd=0, sf=0, hw=0
    let encoding: u32 = 0x12800FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_22_0_12801000() {
    // Encoding: 0x12801000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=128, Rd=0
    // Fields: hw=0, opc=0, imm16=128, Rd=0, sf=0
    let encoding: u32 = 0x12801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=255 (2^8 - 1 = 255)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_23_0_12801fe0() {
    // Encoding: 0x12801FE0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=255, Rd=0
    // Fields: Rd=0, imm16=255, opc=0, hw=0, sf=0
    let encoding: u32 = 0x12801FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=256 (power of 2 (2^8 = 256))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_24_0_12802000() {
    // Encoding: 0x12802000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=256, Rd=0
    // Fields: Rd=0, sf=0, hw=0, opc=0, imm16=256
    let encoding: u32 = 0x12802000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=511 (2^9 - 1 = 511)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_25_0_12803fe0() {
    // Encoding: 0x12803FE0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=511, Rd=0
    // Fields: opc=0, imm16=511, Rd=0, sf=0, hw=0
    let encoding: u32 = 0x12803FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=512 (power of 2 (2^9 = 512))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_26_0_12804000() {
    // Encoding: 0x12804000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=512, Rd=0
    // Fields: sf=0, hw=0, imm16=512, opc=0, Rd=0
    let encoding: u32 = 0x12804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=1023 (2^10 - 1 = 1023)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_27_0_12807fe0() {
    // Encoding: 0x12807FE0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=1023, Rd=0
    // Fields: Rd=0, opc=0, imm16=1023, hw=0, sf=0
    let encoding: u32 = 0x12807FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=1024 (power of 2 (2^10 = 1024))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_28_0_12808000() {
    // Encoding: 0x12808000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=1024, Rd=0
    // Fields: sf=0, imm16=1024, opc=0, hw=0, Rd=0
    let encoding: u32 = 0x12808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=2047 (2^11 - 1 = 2047)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_29_0_1280ffe0() {
    // Encoding: 0x1280FFE0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=2047, Rd=0
    // Fields: sf=0, opc=0, imm16=2047, hw=0, Rd=0
    let encoding: u32 = 0x1280FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=2048 (power of 2 (2^11 = 2048))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_30_0_12810000() {
    // Encoding: 0x12810000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=2048, Rd=0
    // Fields: opc=0, hw=0, imm16=2048, sf=0, Rd=0
    let encoding: u32 = 0x12810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=4095 (2^12 - 1 = 4095)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_31_0_1281ffe0() {
    // Encoding: 0x1281FFE0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=4095, Rd=0
    // Fields: opc=0, imm16=4095, sf=0, Rd=0, hw=0
    let encoding: u32 = 0x1281FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=4096 (power of 2 (2^12 = 4096))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_32_0_12820000() {
    // Encoding: 0x12820000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=4096, Rd=0
    // Fields: sf=0, Rd=0, opc=0, hw=0, imm16=4096
    let encoding: u32 = 0x12820000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=8191 (2^13 - 1 = 8191)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_33_0_1283ffe0() {
    // Encoding: 0x1283FFE0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=8191, Rd=0
    // Fields: opc=0, hw=0, sf=0, imm16=8191, Rd=0
    let encoding: u32 = 0x1283FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=8192 (power of 2 (2^13 = 8192))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_34_0_12840000() {
    // Encoding: 0x12840000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=8192, Rd=0
    // Fields: hw=0, sf=0, imm16=8192, opc=0, Rd=0
    let encoding: u32 = 0x12840000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=16383 (2^14 - 1 = 16383)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_35_0_1287ffe0() {
    // Encoding: 0x1287FFE0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=16383, Rd=0
    // Fields: sf=0, opc=0, imm16=16383, Rd=0, hw=0
    let encoding: u32 = 0x1287FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=16384 (power of 2 (2^14 = 16384))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_36_0_12880000() {
    // Encoding: 0x12880000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=16384, Rd=0
    // Fields: sf=0, opc=0, hw=0, Rd=0, imm16=16384
    let encoding: u32 = 0x12880000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 37`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=32767 (immediate midpoint (32767))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_37_0_128fffe0() {
    // Encoding: 0x128FFFE0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=32767, Rd=0
    // Fields: hw=0, imm16=32767, opc=0, Rd=0, sf=0
    let encoding: u32 = 0x128FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 38`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=32768 (power of 2 (2^15 = 32768))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_38_0_12900000() {
    // Encoding: 0x12900000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=32768, Rd=0
    // Fields: imm16=32768, sf=0, opc=0, hw=0, Rd=0
    let encoding: u32 = 0x12900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 39`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm16=65535 (maximum immediate (65535))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_39_0_129fffe0() {
    // Encoding: 0x129FFFE0
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=65535, Rd=0
    // Fields: imm16=65535, sf=0, opc=0, Rd=0, hw=0
    let encoding: u32 = 0x129FFFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 40`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_40_0_12800000() {
    // Encoding: 0x12800000
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=0, Rd=0
    // Fields: hw=0, imm16=0, Rd=0, sf=0, opc=0
    let encoding: u32 = 0x12800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 41`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_41_0_12800001() {
    // Encoding: 0x12800001
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=0, Rd=1
    // Fields: sf=0, imm16=0, Rd=1, opc=0, hw=0
    let encoding: u32 = 0x12800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 42`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_42_0_1280001e() {
    // Encoding: 0x1280001E
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=0, Rd=30
    // Fields: hw=0, Rd=30, opc=0, sf=0, imm16=0
    let encoding: u32 = 0x1280001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field combination 43`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_combo_43_0_1280001f() {
    // Encoding: 0x1280001F
    // Test aarch64_integer_ins_ext_insert_movewide field combination: sf=0, opc=0, hw=0, imm16=0, Rd=31
    // Fields: opc=0, Rd=31, sf=0, hw=0, imm16=0
    let encoding: u32 = 0x1280001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_special_sf_0_size_variant_0_0_12800020() {
    // Encoding: 0x12800020
    // Test aarch64_integer_ins_ext_insert_movewide special value sf = 0 (Size variant 0)
    // Fields: opc=0, hw=0, imm16=1, Rd=0, sf=0
    let encoding: u32 = 0x12800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_special_sf_1_size_variant_1_0_92800020() {
    // Encoding: 0x92800020
    // Test aarch64_integer_ins_ext_insert_movewide special value sf = 1 (Size variant 1)
    // Fields: Rd=0, sf=1, opc=0, hw=0, imm16=1
    let encoding: u32 = 0x92800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_special_opc_0_size_variant_0_0_12800020() {
    // Encoding: 0x12800020
    // Test aarch64_integer_ins_ext_insert_movewide special value opc = 0 (Size variant 0)
    // Fields: Rd=0, sf=0, opc=0, imm16=1, hw=0
    let encoding: u32 = 0x12800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_special_opc_1_size_variant_1_0_32800020() {
    // Encoding: 0x32800020
    // Test aarch64_integer_ins_ext_insert_movewide special value opc = 1 (Size variant 1)
    // Fields: hw=0, opc=1, imm16=1, Rd=0, sf=0
    let encoding: u32 = 0x32800020;
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

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_special_opc_2_size_variant_2_0_52800020() {
    // Encoding: 0x52800020
    // Test aarch64_integer_ins_ext_insert_movewide special value opc = 2 (Size variant 2)
    // Fields: Rd=0, opc=2, sf=0, imm16=1, hw=0
    let encoding: u32 = 0x52800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_special_opc_3_size_variant_3_0_72800020() {
    // Encoding: 0x72800020
    // Test aarch64_integer_ins_ext_insert_movewide special value opc = 3 (Size variant 3)
    // Fields: sf=0, Rd=0, imm16=1, hw=0, opc=3
    let encoding: u32 = 0x72800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_1280003f()
 {
    // Encoding: 0x1280003F
    // Test aarch64_integer_ins_ext_insert_movewide special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: sf=0, opc=0, imm16=1, hw=0, Rd=31
    let encoding: u32 = 0x1280003F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `MOVN X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// lower 16 bits (32)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_movn_oracle_32_0_12824680() {
    // Test MOVN 32-bit: lower 16 bits (oracle)
    // Encoding: 0x12824680
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x12824680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFEDCB, "W0 should be 0xFFFFEDCB");
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `MOVN X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// lower 16 bits (64)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_movn_oracle_64_0_92824680() {
    // Test MOVN 64-bit: lower 16 bits (oracle)
    // Encoding: 0x92824680
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x92824680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFEDCB,
        "X0 should be 0xFFFFFFFFFFFFEDCB"
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `MOVN X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shifted 16 bits (32)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_movn_oracle_32_1_12b579a0() {
    // Test MOVN 32-bit: shifted 16 bits (oracle)
    // Encoding: 0x12B579A0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x12B579A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x5432FFFF, "W0 should be 0x5432FFFF");
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `MOVN X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 16 bits (64)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_movn_oracle_64_1_92b579a0() {
    // Test MOVN 64-bit: shifted 16 bits (oracle)
    // Encoding: 0x92B579A0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x92B579A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFF5432FFFF,
        "X0 should be 0xFFFFFFFF5432FFFF"
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `MOVN X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm16 (32)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_movn_oracle_32_2_129fffe0() {
    // Test MOVN 32-bit: max imm16 (oracle)
    // Encoding: 0x129FFFE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x129FFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF0000, "W0 should be 0xFFFF0000");
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `MOVN X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm16 (64)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_movn_oracle_64_2_929fffe0() {
    // Test MOVN 64-bit: max imm16 (oracle)
    // Encoding: 0x929FFFE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x929FFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFF0000,
        "X0 should be 0xFFFFFFFFFFFF0000"
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `MOVN X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero imm16 (32)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_movn_oracle_32_3_12800000() {
    // Test MOVN 32-bit: zero imm16 (oracle)
    // Encoding: 0x12800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x12800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "W0 should be 0xFFFFFFFF");
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `MOVN X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero imm16 (64)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_movn_oracle_64_3_92800000() {
    // Test MOVN 64-bit: zero imm16 (oracle)
    // Encoding: 0x92800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x92800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `MOVN X0, #0x5678, LSL #32`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 32 bits (64)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_movn_oracle_64_4_92cacf00() {
    // Test MOVN 64-bit: shifted 32 bits (oracle)
    // Encoding: 0x92CACF00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x92CACF00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFA987FFFFFFFF,
        "X0 should be 0xFFFFA987FFFFFFFF"
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `MOVN X0, #0xDEAD, LSL #48`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 48 bits (64)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_movn_oracle_64_5_92fbd5a0() {
    // Test MOVN 64-bit: shifted 48 bits (oracle)
    // Encoding: 0x92FBD5A0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x92FBD5A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0x2152FFFFFFFFFFFF,
        "X0 should be 0x2152FFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `GpFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "d" }
/// verify register write to GpFromField("d")
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_reg_write_0_12800000() {
    // Test aarch64_integer_ins_ext_insert_movewide register write: GpFromField("d")
    // Encoding: 0x12800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x12800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_integer_ins_ext_insert_movewide
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_integer_ins_ext_insert_movewide_zr_rd_1280001f() {
    // Test aarch64_integer_ins_ext_insert_movewide with Rd = ZR (31)
    // Encoding: 0x1280001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x1280001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}
