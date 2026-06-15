//! A64 sve convert tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// UCVTF_Z.P.Z_H2FP16 Tests
// ============================================================================

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_ucvtf_z_p_z_h2fp16_field_pg_0_min_a000_6553a000() {
    // Encoding: 0x6553A000
    // Test UCVTF_Z.P.Z_H2FP16 field Pg = 0 (Min)
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x6553A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_ucvtf_z_p_z_h2fp16_field_pg_1_poweroftwo_a000_6553a400() {
    // Encoding: 0x6553A400
    // Test UCVTF_Z.P.Z_H2FP16 field Pg = 1 (PowerOfTwo)
    // Fields: Zd=0, Pg=1, Zn=0
    let encoding: u32 = 0x6553A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p1/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_h2fp16_field_zn_0_min_a000_6553a000() {
    // Encoding: 0x6553A000
    // Test UCVTF_Z.P.Z_H2FP16 field Zn = 0 (Min)
    // Fields: Zn=0, Pg=0, Zd=0
    let encoding: u32 = 0x6553A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_h2fp16_field_zn_1_poweroftwo_a000_6553a020() {
    // Encoding: 0x6553A020
    // Test UCVTF_Z.P.Z_H2FP16 field Zn = 1 (PowerOfTwo)
    // Fields: Zn=1, Zd=0, Pg=0
    let encoding: u32 = 0x6553A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z1.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_h2fp16_field_zn_30_poweroftwominusone_a000_6553a3c0() {
    // Encoding: 0x6553A3C0
    // Test UCVTF_Z.P.Z_H2FP16 field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Zn=30, Zd=0, Pg=0
    let encoding: u32 = 0x6553A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z30.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_h2fp16_field_zn_31_max_a000_6553a3e0() {
    // Encoding: 0x6553A3E0
    // Test UCVTF_Z.P.Z_H2FP16 field Zn = 31 (Max)
    // Fields: Pg=0, Zd=0, Zn=31
    let encoding: u32 = 0x6553A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z31.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_h2fp16_field_zd_0_min_a000_6553a000() {
    // Encoding: 0x6553A000
    // Test UCVTF_Z.P.Z_H2FP16 field Zd = 0 (Min)
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x6553A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_h2fp16_field_zd_1_poweroftwo_a000_6553a001() {
    // Encoding: 0x6553A001
    // Test UCVTF_Z.P.Z_H2FP16 field Zd = 1 (PowerOfTwo)
    // Fields: Pg=0, Zn=0, Zd=1
    let encoding: u32 = 0x6553A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_h2fp16_field_zd_30_poweroftwominusone_a000_6553a01e() {
    // Encoding: 0x6553A01E
    // Test UCVTF_Z.P.Z_H2FP16 field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Zd=30, Pg=0, Zn=0
    let encoding: u32 = 0x6553A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_h2fp16_field_zd_31_max_a000_6553a01f() {
    // Encoding: 0x6553A01F
    // Test UCVTF_Z.P.Z_H2FP16 field Zd = 31 (Max)
    // Fields: Pg=0, Zd=31, Zn=0
    let encoding: u32 = 0x6553A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_ucvtf_z_p_z_h2fp16_combo_0_a000_6553a000() {
    // Encoding: 0x6553A000
    // Test UCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zd=0, Zn=0, Pg=0
    let encoding: u32 = 0x6553A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_ucvtf_z_p_z_h2fp16_combo_1_a000_6553a400() {
    // Encoding: 0x6553A400
    // Test UCVTF_Z.P.Z_H2FP16 field combination: Pg=1, Zn=0, Zd=0
    // Fields: Pg=1, Zd=0, Zn=0
    let encoding: u32 = 0x6553A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p1/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_h2fp16_combo_2_a000_6553a000() {
    // Encoding: 0x6553A000
    // Test UCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x6553A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_h2fp16_combo_3_a000_6553a020() {
    // Encoding: 0x6553A020
    // Test UCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=1, Zd=0
    // Fields: Zn=1, Zd=0, Pg=0
    let encoding: u32 = 0x6553A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z1.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_h2fp16_combo_4_a000_6553a3c0() {
    // Encoding: 0x6553A3C0
    // Test UCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=30, Zd=0
    // Fields: Zn=30, Pg=0, Zd=0
    let encoding: u32 = 0x6553A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z30.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_h2fp16_combo_5_a000_6553a3e0() {
    // Encoding: 0x6553A3E0
    // Test UCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=31, Zd=0
    // Fields: Zd=0, Pg=0, Zn=31
    let encoding: u32 = 0x6553A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z31.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_h2fp16_combo_6_a000_6553a000() {
    // Encoding: 0x6553A000
    // Test UCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x6553A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_h2fp16_combo_7_a000_6553a001() {
    // Encoding: 0x6553A001
    // Test UCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=0, Zd=1
    // Fields: Pg=0, Zn=0, Zd=1
    let encoding: u32 = 0x6553A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_h2fp16_combo_8_a000_6553a01e() {
    // Encoding: 0x6553A01E
    // Test UCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=0, Zd=30
    // Fields: Zd=30, Pg=0, Zn=0
    let encoding: u32 = 0x6553A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_h2fp16_combo_9_a000_6553a01f() {
    // Encoding: 0x6553A01F
    // Test UCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=0, Zd=31
    // Fields: Pg=0, Zn=0, Zd=31
    let encoding: u32 = 0x6553A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_ucvtf_z_p_z_w2fp16_field_pg_0_min_a000_6555a000() {
    // Encoding: 0x6555A000
    // Test UCVTF_Z.P.Z_W2FP16 field Pg = 0 (Min)
    // Fields: Zn=0, Pg=0, Zd=0
    let encoding: u32 = 0x6555A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_ucvtf_z_p_z_w2fp16_field_pg_1_poweroftwo_a000_6555a400() {
    // Encoding: 0x6555A400
    // Test UCVTF_Z.P.Z_W2FP16 field Pg = 1 (PowerOfTwo)
    // Fields: Zd=0, Pg=1, Zn=0
    let encoding: u32 = 0x6555A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p1/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_w2fp16_field_zn_0_min_a000_6555a000() {
    // Encoding: 0x6555A000
    // Test UCVTF_Z.P.Z_W2FP16 field Zn = 0 (Min)
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x6555A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_w2fp16_field_zn_1_poweroftwo_a000_6555a020() {
    // Encoding: 0x6555A020
    // Test UCVTF_Z.P.Z_W2FP16 field Zn = 1 (PowerOfTwo)
    // Fields: Pg=0, Zn=1, Zd=0
    let encoding: u32 = 0x6555A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z1.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_w2fp16_field_zn_30_poweroftwominusone_a000_6555a3c0() {
    // Encoding: 0x6555A3C0
    // Test UCVTF_Z.P.Z_W2FP16 field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Zn=30, Pg=0, Zd=0
    let encoding: u32 = 0x6555A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z30.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_w2fp16_field_zn_31_max_a000_6555a3e0() {
    // Encoding: 0x6555A3E0
    // Test UCVTF_Z.P.Z_W2FP16 field Zn = 31 (Max)
    // Fields: Zn=31, Zd=0, Pg=0
    let encoding: u32 = 0x6555A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z31.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_w2fp16_field_zd_0_min_a000_6555a000() {
    // Encoding: 0x6555A000
    // Test UCVTF_Z.P.Z_W2FP16 field Zd = 0 (Min)
    // Fields: Zn=0, Pg=0, Zd=0
    let encoding: u32 = 0x6555A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_w2fp16_field_zd_1_poweroftwo_a000_6555a001() {
    // Encoding: 0x6555A001
    // Test UCVTF_Z.P.Z_W2FP16 field Zd = 1 (PowerOfTwo)
    // Fields: Zn=0, Pg=0, Zd=1
    let encoding: u32 = 0x6555A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_w2fp16_field_zd_30_poweroftwominusone_a000_6555a01e() {
    // Encoding: 0x6555A01E
    // Test UCVTF_Z.P.Z_W2FP16 field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Zn=0, Zd=30, Pg=0
    let encoding: u32 = 0x6555A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_w2fp16_field_zd_31_max_a000_6555a01f() {
    // Encoding: 0x6555A01F
    // Test UCVTF_Z.P.Z_W2FP16 field Zd = 31 (Max)
    // Fields: Zd=31, Pg=0, Zn=0
    let encoding: u32 = 0x6555A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_ucvtf_z_p_z_w2fp16_combo_0_a000_6555a000() {
    // Encoding: 0x6555A000
    // Test UCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x6555A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_ucvtf_z_p_z_w2fp16_combo_1_a000_6555a400() {
    // Encoding: 0x6555A400
    // Test UCVTF_Z.P.Z_W2FP16 field combination: Pg=1, Zn=0, Zd=0
    // Fields: Pg=1, Zn=0, Zd=0
    let encoding: u32 = 0x6555A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p1/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_w2fp16_combo_2_a000_6555a000() {
    // Encoding: 0x6555A000
    // Test UCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x6555A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_w2fp16_combo_3_a000_6555a020() {
    // Encoding: 0x6555A020
    // Test UCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=1, Zd=0
    // Fields: Zn=1, Pg=0, Zd=0
    let encoding: u32 = 0x6555A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z1.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_w2fp16_combo_4_a000_6555a3c0() {
    // Encoding: 0x6555A3C0
    // Test UCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=30, Zd=0
    // Fields: Zd=0, Zn=30, Pg=0
    let encoding: u32 = 0x6555A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z30.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_w2fp16_combo_5_a000_6555a3e0() {
    // Encoding: 0x6555A3E0
    // Test UCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=31, Zd=0
    // Fields: Pg=0, Zd=0, Zn=31
    let encoding: u32 = 0x6555A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z31.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_w2fp16_combo_6_a000_6555a000() {
    // Encoding: 0x6555A000
    // Test UCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x6555A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_w2fp16_combo_7_a000_6555a001() {
    // Encoding: 0x6555A001
    // Test UCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=0, Zd=1
    // Fields: Pg=0, Zd=1, Zn=0
    let encoding: u32 = 0x6555A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_w2fp16_combo_8_a000_6555a01e() {
    // Encoding: 0x6555A01E
    // Test UCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=0, Zd=30
    // Fields: Pg=0, Zn=0, Zd=30
    let encoding: u32 = 0x6555A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_w2fp16_combo_9_a000_6555a01f() {
    // Encoding: 0x6555A01F
    // Test UCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=0, Zd=31
    // Fields: Zn=0, Zd=31, Pg=0
    let encoding: u32 = 0x6555A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_ucvtf_z_p_z_w2s_field_pg_0_min_a000_6595a000() {
    // Encoding: 0x6595A000
    // Test UCVTF_Z.P.Z_W2S field Pg = 0 (Min)
    // Fields: Zn=0, Pg=0, Zd=0
    let encoding: u32 = 0x6595A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_ucvtf_z_p_z_w2s_field_pg_1_poweroftwo_a000_6595a400() {
    // Encoding: 0x6595A400
    // Test UCVTF_Z.P.Z_W2S field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Zn=0, Zd=0
    let encoding: u32 = 0x6595A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p1/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_w2s_field_zn_0_min_a000_6595a000() {
    // Encoding: 0x6595A000
    // Test UCVTF_Z.P.Z_W2S field Zn = 0 (Min)
    // Fields: Pg=0, Zd=0, Zn=0
    let encoding: u32 = 0x6595A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_w2s_field_zn_1_poweroftwo_a000_6595a020() {
    // Encoding: 0x6595A020
    // Test UCVTF_Z.P.Z_W2S field Zn = 1 (PowerOfTwo)
    // Fields: Pg=0, Zn=1, Zd=0
    let encoding: u32 = 0x6595A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z1.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_w2s_field_zn_30_poweroftwominusone_a000_6595a3c0() {
    // Encoding: 0x6595A3C0
    // Test UCVTF_Z.P.Z_W2S field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Zn=30, Zd=0, Pg=0
    let encoding: u32 = 0x6595A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z30.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_w2s_field_zn_31_max_a000_6595a3e0() {
    // Encoding: 0x6595A3E0
    // Test UCVTF_Z.P.Z_W2S field Zn = 31 (Max)
    // Fields: Zd=0, Pg=0, Zn=31
    let encoding: u32 = 0x6595A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z31.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_w2s_field_zd_0_min_a000_6595a000() {
    // Encoding: 0x6595A000
    // Test UCVTF_Z.P.Z_W2S field Zd = 0 (Min)
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x6595A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_w2s_field_zd_1_poweroftwo_a000_6595a001() {
    // Encoding: 0x6595A001
    // Test UCVTF_Z.P.Z_W2S field Zd = 1 (PowerOfTwo)
    // Fields: Zn=0, Pg=0, Zd=1
    let encoding: u32 = 0x6595A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_w2s_field_zd_30_poweroftwominusone_a000_6595a01e() {
    // Encoding: 0x6595A01E
    // Test UCVTF_Z.P.Z_W2S field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Zn=0, Pg=0, Zd=30
    let encoding: u32 = 0x6595A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_w2s_field_zd_31_max_a000_6595a01f() {
    // Encoding: 0x6595A01F
    // Test UCVTF_Z.P.Z_W2S field Zd = 31 (Max)
    // Fields: Pg=0, Zn=0, Zd=31
    let encoding: u32 = 0x6595A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_ucvtf_z_p_z_w2s_combo_0_a000_6595a000() {
    // Encoding: 0x6595A000
    // Test UCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zd=0, Zn=0, Pg=0
    let encoding: u32 = 0x6595A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_ucvtf_z_p_z_w2s_combo_1_a000_6595a400() {
    // Encoding: 0x6595A400
    // Test UCVTF_Z.P.Z_W2S field combination: Pg=1, Zn=0, Zd=0
    // Fields: Pg=1, Zd=0, Zn=0
    let encoding: u32 = 0x6595A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p1/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_w2s_combo_2_a000_6595a000() {
    // Encoding: 0x6595A000
    // Test UCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x6595A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_w2s_combo_3_a000_6595a020() {
    // Encoding: 0x6595A020
    // Test UCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=1, Zd=0
    // Fields: Pg=0, Zd=0, Zn=1
    let encoding: u32 = 0x6595A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z1.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_w2s_combo_4_a000_6595a3c0() {
    // Encoding: 0x6595A3C0
    // Test UCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=30, Zd=0
    // Fields: Zd=0, Pg=0, Zn=30
    let encoding: u32 = 0x6595A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z30.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_w2s_combo_5_a000_6595a3e0() {
    // Encoding: 0x6595A3E0
    // Test UCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=31, Zd=0
    // Fields: Zd=0, Pg=0, Zn=31
    let encoding: u32 = 0x6595A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z31.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_w2s_combo_6_a000_6595a000() {
    // Encoding: 0x6595A000
    // Test UCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x6595A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_w2s_combo_7_a000_6595a001() {
    // Encoding: 0x6595A001
    // Test UCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=0, Zd=1
    // Fields: Pg=0, Zd=1, Zn=0
    let encoding: u32 = 0x6595A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_w2s_combo_8_a000_6595a01e() {
    // Encoding: 0x6595A01E
    // Test UCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=0, Zd=30
    // Fields: Zd=30, Pg=0, Zn=0
    let encoding: u32 = 0x6595A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_w2s_combo_9_a000_6595a01f() {
    // Encoding: 0x6595A01F
    // Test UCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=0, Zd=31
    // Fields: Pg=0, Zd=31, Zn=0
    let encoding: u32 = 0x6595A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_ucvtf_z_p_z_w2d_field_pg_0_min_a000_65d1a000() {
    // Encoding: 0x65D1A000
    // Test UCVTF_Z.P.Z_W2D field Pg = 0 (Min)
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x65D1A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_ucvtf_z_p_z_w2d_field_pg_1_poweroftwo_a000_65d1a400() {
    // Encoding: 0x65D1A400
    // Test UCVTF_Z.P.Z_W2D field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Zn=0, Zd=0
    let encoding: u32 = 0x65D1A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p1/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_w2d_field_zn_0_min_a000_65d1a000() {
    // Encoding: 0x65D1A000
    // Test UCVTF_Z.P.Z_W2D field Zn = 0 (Min)
    // Fields: Pg=0, Zd=0, Zn=0
    let encoding: u32 = 0x65D1A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_w2d_field_zn_1_poweroftwo_a000_65d1a020() {
    // Encoding: 0x65D1A020
    // Test UCVTF_Z.P.Z_W2D field Zn = 1 (PowerOfTwo)
    // Fields: Zd=0, Zn=1, Pg=0
    let encoding: u32 = 0x65D1A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z1.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_w2d_field_zn_30_poweroftwominusone_a000_65d1a3c0() {
    // Encoding: 0x65D1A3C0
    // Test UCVTF_Z.P.Z_W2D field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zn=30, Zd=0
    let encoding: u32 = 0x65D1A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z30.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_w2d_field_zn_31_max_a000_65d1a3e0() {
    // Encoding: 0x65D1A3E0
    // Test UCVTF_Z.P.Z_W2D field Zn = 31 (Max)
    // Fields: Pg=0, Zn=31, Zd=0
    let encoding: u32 = 0x65D1A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z31.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_w2d_field_zd_0_min_a000_65d1a000() {
    // Encoding: 0x65D1A000
    // Test UCVTF_Z.P.Z_W2D field Zd = 0 (Min)
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x65D1A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_w2d_field_zd_1_poweroftwo_a000_65d1a001() {
    // Encoding: 0x65D1A001
    // Test UCVTF_Z.P.Z_W2D field Zd = 1 (PowerOfTwo)
    // Fields: Zd=1, Pg=0, Zn=0
    let encoding: u32 = 0x65D1A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_w2d_field_zd_30_poweroftwominusone_a000_65d1a01e() {
    // Encoding: 0x65D1A01E
    // Test UCVTF_Z.P.Z_W2D field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Zn=0, Zd=30, Pg=0
    let encoding: u32 = 0x65D1A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_w2d_field_zd_31_max_a000_65d1a01f() {
    // Encoding: 0x65D1A01F
    // Test UCVTF_Z.P.Z_W2D field Zd = 31 (Max)
    // Fields: Zn=0, Zd=31, Pg=0
    let encoding: u32 = 0x65D1A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_ucvtf_z_p_z_w2d_combo_0_a000_65d1a000() {
    // Encoding: 0x65D1A000
    // Test UCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x65D1A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_ucvtf_z_p_z_w2d_combo_1_a000_65d1a400() {
    // Encoding: 0x65D1A400
    // Test UCVTF_Z.P.Z_W2D field combination: Pg=1, Zn=0, Zd=0
    // Fields: Pg=1, Zn=0, Zd=0
    let encoding: u32 = 0x65D1A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p1/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_w2d_combo_2_a000_65d1a000() {
    // Encoding: 0x65D1A000
    // Test UCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x65D1A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_w2d_combo_3_a000_65d1a020() {
    // Encoding: 0x65D1A020
    // Test UCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=1, Zd=0
    // Fields: Zd=0, Pg=0, Zn=1
    let encoding: u32 = 0x65D1A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z1.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_w2d_combo_4_a000_65d1a3c0() {
    // Encoding: 0x65D1A3C0
    // Test UCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=30, Zd=0
    // Fields: Zn=30, Zd=0, Pg=0
    let encoding: u32 = 0x65D1A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z30.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_w2d_combo_5_a000_65d1a3e0() {
    // Encoding: 0x65D1A3E0
    // Test UCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=31, Zd=0
    // Fields: Zd=0, Pg=0, Zn=31
    let encoding: u32 = 0x65D1A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z31.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_w2d_combo_6_a000_65d1a000() {
    // Encoding: 0x65D1A000
    // Test UCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x65D1A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_w2d_combo_7_a000_65d1a001() {
    // Encoding: 0x65D1A001
    // Test UCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=0, Zd=1
    // Fields: Zn=0, Zd=1, Pg=0
    let encoding: u32 = 0x65D1A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_w2d_combo_8_a000_65d1a01e() {
    // Encoding: 0x65D1A01E
    // Test UCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=0, Zd=30
    // Fields: Pg=0, Zn=0, Zd=30
    let encoding: u32 = 0x65D1A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_W2D
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_w2d_combo_9_a000_65d1a01f() {
    // Encoding: 0x65D1A01F
    // Test UCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=0, Zd=31
    // Fields: Zd=31, Pg=0, Zn=0
    let encoding: u32 = 0x65D1A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_ucvtf_z_p_z_x2fp16_field_pg_0_min_a000_6557a000() {
    // Encoding: 0x6557A000
    // Test UCVTF_Z.P.Z_X2FP16 field Pg = 0 (Min)
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x6557A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_ucvtf_z_p_z_x2fp16_field_pg_1_poweroftwo_a000_6557a400() {
    // Encoding: 0x6557A400
    // Test UCVTF_Z.P.Z_X2FP16 field Pg = 1 (PowerOfTwo)
    // Fields: Zn=0, Pg=1, Zd=0
    let encoding: u32 = 0x6557A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p1/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_x2fp16_field_zn_0_min_a000_6557a000() {
    // Encoding: 0x6557A000
    // Test UCVTF_Z.P.Z_X2FP16 field Zn = 0 (Min)
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x6557A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_x2fp16_field_zn_1_poweroftwo_a000_6557a020() {
    // Encoding: 0x6557A020
    // Test UCVTF_Z.P.Z_X2FP16 field Zn = 1 (PowerOfTwo)
    // Fields: Zd=0, Pg=0, Zn=1
    let encoding: u32 = 0x6557A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z1.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_x2fp16_field_zn_30_poweroftwominusone_a000_6557a3c0() {
    // Encoding: 0x6557A3C0
    // Test UCVTF_Z.P.Z_X2FP16 field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zd=0, Zn=30
    let encoding: u32 = 0x6557A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z30.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_x2fp16_field_zn_31_max_a000_6557a3e0() {
    // Encoding: 0x6557A3E0
    // Test UCVTF_Z.P.Z_X2FP16 field Zn = 31 (Max)
    // Fields: Zn=31, Pg=0, Zd=0
    let encoding: u32 = 0x6557A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z31.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_x2fp16_field_zd_0_min_a000_6557a000() {
    // Encoding: 0x6557A000
    // Test UCVTF_Z.P.Z_X2FP16 field Zd = 0 (Min)
    // Fields: Zd=0, Zn=0, Pg=0
    let encoding: u32 = 0x6557A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_x2fp16_field_zd_1_poweroftwo_a000_6557a001() {
    // Encoding: 0x6557A001
    // Test UCVTF_Z.P.Z_X2FP16 field Zd = 1 (PowerOfTwo)
    // Fields: Pg=0, Zd=1, Zn=0
    let encoding: u32 = 0x6557A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_x2fp16_field_zd_30_poweroftwominusone_a000_6557a01e() {
    // Encoding: 0x6557A01E
    // Test UCVTF_Z.P.Z_X2FP16 field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Zd=30, Pg=0, Zn=0
    let encoding: u32 = 0x6557A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_x2fp16_field_zd_31_max_a000_6557a01f() {
    // Encoding: 0x6557A01F
    // Test UCVTF_Z.P.Z_X2FP16 field Zd = 31 (Max)
    // Fields: Zn=0, Zd=31, Pg=0
    let encoding: u32 = 0x6557A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_ucvtf_z_p_z_x2fp16_combo_0_a000_6557a000() {
    // Encoding: 0x6557A000
    // Test UCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zn=0, Pg=0, Zd=0
    let encoding: u32 = 0x6557A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_ucvtf_z_p_z_x2fp16_combo_1_a000_6557a400() {
    // Encoding: 0x6557A400
    // Test UCVTF_Z.P.Z_X2FP16 field combination: Pg=1, Zn=0, Zd=0
    // Fields: Pg=1, Zd=0, Zn=0
    let encoding: u32 = 0x6557A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p1/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_x2fp16_combo_2_a000_6557a000() {
    // Encoding: 0x6557A000
    // Test UCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x6557A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_x2fp16_combo_3_a000_6557a020() {
    // Encoding: 0x6557A020
    // Test UCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=1, Zd=0
    // Fields: Pg=0, Zd=0, Zn=1
    let encoding: u32 = 0x6557A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z1.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_x2fp16_combo_4_a000_6557a3c0() {
    // Encoding: 0x6557A3C0
    // Test UCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=30, Zd=0
    // Fields: Pg=0, Zn=30, Zd=0
    let encoding: u32 = 0x6557A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z30.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_x2fp16_combo_5_a000_6557a3e0() {
    // Encoding: 0x6557A3E0
    // Test UCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=31, Zd=0
    // Fields: Pg=0, Zn=31, Zd=0
    let encoding: u32 = 0x6557A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z31.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_x2fp16_combo_6_a000_6557a000() {
    // Encoding: 0x6557A000
    // Test UCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x6557A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_x2fp16_combo_7_a000_6557a001() {
    // Encoding: 0x6557A001
    // Test UCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=0, Zd=1
    // Fields: Zd=1, Zn=0, Pg=0
    let encoding: u32 = 0x6557A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_x2fp16_combo_8_a000_6557a01e() {
    // Encoding: 0x6557A01E
    // Test UCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=0, Zd=30
    // Fields: Zn=0, Zd=30, Pg=0
    let encoding: u32 = 0x6557A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_x2fp16_combo_9_a000_6557a01f() {
    // Encoding: 0x6557A01F
    // Test UCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=0, Zd=31
    // Fields: Pg=0, Zn=0, Zd=31
    let encoding: u32 = 0x6557A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_ucvtf_z_p_z_x2s_field_pg_0_min_a000_65d5a000() {
    // Encoding: 0x65D5A000
    // Test UCVTF_Z.P.Z_X2S field Pg = 0 (Min)
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x65D5A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_ucvtf_z_p_z_x2s_field_pg_1_poweroftwo_a000_65d5a400() {
    // Encoding: 0x65D5A400
    // Test UCVTF_Z.P.Z_X2S field Pg = 1 (PowerOfTwo)
    // Fields: Zn=0, Zd=0, Pg=1
    let encoding: u32 = 0x65D5A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p1/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_x2s_field_zn_0_min_a000_65d5a000() {
    // Encoding: 0x65D5A000
    // Test UCVTF_Z.P.Z_X2S field Zn = 0 (Min)
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x65D5A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_x2s_field_zn_1_poweroftwo_a000_65d5a020() {
    // Encoding: 0x65D5A020
    // Test UCVTF_Z.P.Z_X2S field Zn = 1 (PowerOfTwo)
    // Fields: Zd=0, Pg=0, Zn=1
    let encoding: u32 = 0x65D5A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z1.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_x2s_field_zn_30_poweroftwominusone_a000_65d5a3c0() {
    // Encoding: 0x65D5A3C0
    // Test UCVTF_Z.P.Z_X2S field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Zd=0, Zn=30, Pg=0
    let encoding: u32 = 0x65D5A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z30.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_x2s_field_zn_31_max_a000_65d5a3e0() {
    // Encoding: 0x65D5A3E0
    // Test UCVTF_Z.P.Z_X2S field Zn = 31 (Max)
    // Fields: Zn=31, Zd=0, Pg=0
    let encoding: u32 = 0x65D5A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z31.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_x2s_field_zd_0_min_a000_65d5a000() {
    // Encoding: 0x65D5A000
    // Test UCVTF_Z.P.Z_X2S field Zd = 0 (Min)
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x65D5A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_x2s_field_zd_1_poweroftwo_a000_65d5a001() {
    // Encoding: 0x65D5A001
    // Test UCVTF_Z.P.Z_X2S field Zd = 1 (PowerOfTwo)
    // Fields: Pg=0, Zd=1, Zn=0
    let encoding: u32 = 0x65D5A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_x2s_field_zd_30_poweroftwominusone_a000_65d5a01e() {
    // Encoding: 0x65D5A01E
    // Test UCVTF_Z.P.Z_X2S field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zd=30, Zn=0
    let encoding: u32 = 0x65D5A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_x2s_field_zd_31_max_a000_65d5a01f() {
    // Encoding: 0x65D5A01F
    // Test UCVTF_Z.P.Z_X2S field Zd = 31 (Max)
    // Fields: Zn=0, Zd=31, Pg=0
    let encoding: u32 = 0x65D5A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_ucvtf_z_p_z_x2s_combo_0_a000_65d5a000() {
    // Encoding: 0x65D5A000
    // Test UCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zd=0, Zn=0
    let encoding: u32 = 0x65D5A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_ucvtf_z_p_z_x2s_combo_1_a000_65d5a400() {
    // Encoding: 0x65D5A400
    // Test UCVTF_Z.P.Z_X2S field combination: Pg=1, Zn=0, Zd=0
    // Fields: Zn=0, Pg=1, Zd=0
    let encoding: u32 = 0x65D5A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p1/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_x2s_combo_2_a000_65d5a000() {
    // Encoding: 0x65D5A000
    // Test UCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x65D5A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_x2s_combo_3_a000_65d5a020() {
    // Encoding: 0x65D5A020
    // Test UCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=1, Zd=0
    // Fields: Zn=1, Zd=0, Pg=0
    let encoding: u32 = 0x65D5A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z1.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_x2s_combo_4_a000_65d5a3c0() {
    // Encoding: 0x65D5A3C0
    // Test UCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=30, Zd=0
    // Fields: Zd=0, Pg=0, Zn=30
    let encoding: u32 = 0x65D5A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z30.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_x2s_combo_5_a000_65d5a3e0() {
    // Encoding: 0x65D5A3E0
    // Test UCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=31, Zd=0
    // Fields: Pg=0, Zd=0, Zn=31
    let encoding: u32 = 0x65D5A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z31.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_x2s_combo_6_a000_65d5a000() {
    // Encoding: 0x65D5A000
    // Test UCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x65D5A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_x2s_combo_7_a000_65d5a001() {
    // Encoding: 0x65D5A001
    // Test UCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=0, Zd=1
    // Fields: Zd=1, Zn=0, Pg=0
    let encoding: u32 = 0x65D5A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_x2s_combo_8_a000_65d5a01e() {
    // Encoding: 0x65D5A01E
    // Test UCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=0, Zd=30
    // Fields: Zd=30, Zn=0, Pg=0
    let encoding: u32 = 0x65D5A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_x2s_combo_9_a000_65d5a01f() {
    // Encoding: 0x65D5A01F
    // Test UCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=0, Zd=31
    // Fields: Zd=31, Zn=0, Pg=0
    let encoding: u32 = 0x65D5A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_ucvtf_z_p_z_x2d_field_pg_0_min_a000_65d7a000() {
    // Encoding: 0x65D7A000
    // Test UCVTF_Z.P.Z_X2D field Pg = 0 (Min)
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x65D7A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_ucvtf_z_p_z_x2d_field_pg_1_poweroftwo_a000_65d7a400() {
    // Encoding: 0x65D7A400
    // Test UCVTF_Z.P.Z_X2D field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Zn=0, Zd=0
    let encoding: u32 = 0x65D7A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p1/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_x2d_field_zn_0_min_a000_65d7a000() {
    // Encoding: 0x65D7A000
    // Test UCVTF_Z.P.Z_X2D field Zn = 0 (Min)
    // Fields: Pg=0, Zd=0, Zn=0
    let encoding: u32 = 0x65D7A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_x2d_field_zn_1_poweroftwo_a000_65d7a020() {
    // Encoding: 0x65D7A020
    // Test UCVTF_Z.P.Z_X2D field Zn = 1 (PowerOfTwo)
    // Fields: Pg=0, Zn=1, Zd=0
    let encoding: u32 = 0x65D7A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z1.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_x2d_field_zn_30_poweroftwominusone_a000_65d7a3c0() {
    // Encoding: 0x65D7A3C0
    // Test UCVTF_Z.P.Z_X2D field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zn=30, Zd=0
    let encoding: u32 = 0x65D7A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z30.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_x2d_field_zn_31_max_a000_65d7a3e0() {
    // Encoding: 0x65D7A3E0
    // Test UCVTF_Z.P.Z_X2D field Zn = 31 (Max)
    // Fields: Zn=31, Zd=0, Pg=0
    let encoding: u32 = 0x65D7A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z31.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_ucvtf_z_p_z_x2d_field_zd_0_min_a000_65d7a000() {
    // Encoding: 0x65D7A000
    // Test UCVTF_Z.P.Z_X2D field Zd = 0 (Min)
    // Fields: Zn=0, Pg=0, Zd=0
    let encoding: u32 = 0x65D7A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_ucvtf_z_p_z_x2d_field_zd_1_poweroftwo_a000_65d7a001() {
    // Encoding: 0x65D7A001
    // Test UCVTF_Z.P.Z_X2D field Zd = 1 (PowerOfTwo)
    // Fields: Zn=0, Pg=0, Zd=1
    let encoding: u32 = 0x65D7A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_ucvtf_z_p_z_x2d_field_zd_30_poweroftwominusone_a000_65d7a01e() {
    // Encoding: 0x65D7A01E
    // Test UCVTF_Z.P.Z_X2D field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zn=0, Zd=30
    let encoding: u32 = 0x65D7A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_ucvtf_z_p_z_x2d_field_zd_31_max_a000_65d7a01f() {
    // Encoding: 0x65D7A01F
    // Test UCVTF_Z.P.Z_X2D field Zd = 31 (Max)
    // Fields: Zn=0, Pg=0, Zd=31
    let encoding: u32 = 0x65D7A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_ucvtf_z_p_z_x2d_combo_0_a000_65d7a000() {
    // Encoding: 0x65D7A000
    // Test UCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zd=0, Zn=0, Pg=0
    let encoding: u32 = 0x65D7A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_ucvtf_z_p_z_x2d_combo_1_a000_65d7a400() {
    // Encoding: 0x65D7A400
    // Test UCVTF_Z.P.Z_X2D field combination: Pg=1, Zn=0, Zd=0
    // Fields: Zd=0, Zn=0, Pg=1
    let encoding: u32 = 0x65D7A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p1/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_x2d_combo_2_a000_65d7a000() {
    // Encoding: 0x65D7A000
    // Test UCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zn=0, Pg=0, Zd=0
    let encoding: u32 = 0x65D7A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_x2d_combo_3_a000_65d7a020() {
    // Encoding: 0x65D7A020
    // Test UCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=1, Zd=0
    // Fields: Pg=0, Zn=1, Zd=0
    let encoding: u32 = 0x65D7A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z1.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_x2d_combo_4_a000_65d7a3c0() {
    // Encoding: 0x65D7A3C0
    // Test UCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=30, Zd=0
    // Fields: Pg=0, Zd=0, Zn=30
    let encoding: u32 = 0x65D7A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z30.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_x2d_combo_5_a000_65d7a3e0() {
    // Encoding: 0x65D7A3E0
    // Test UCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=31, Zd=0
    // Fields: Zd=0, Pg=0, Zn=31
    let encoding: u32 = 0x65D7A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z31.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_ucvtf_z_p_z_x2d_combo_6_a000_65d7a000() {
    // Encoding: 0x65D7A000
    // Test UCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x65D7A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z0.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_ucvtf_z_p_z_x2d_combo_7_a000_65d7a001() {
    // Encoding: 0x65D7A001
    // Test UCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=0, Zd=1
    // Fields: Zn=0, Zd=1, Pg=0
    let encoding: u32 = 0x65D7A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z1.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_ucvtf_z_p_z_x2d_combo_8_a000_65d7a01e() {
    // Encoding: 0x65D7A01E
    // Test UCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=0, Zd=30
    // Fields: Zn=0, Pg=0, Zd=30
    let encoding: u32 = 0x65D7A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z30.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: UCVTF_Z.P.Z_X2D
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_ucvtf_z_p_z_x2d_combo_9_a000_65d7a01f() {
    // Encoding: 0x65D7A01F
    // Test UCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=0, Zd=31
    // Fields: Pg=0, Zn=0, Zd=31
    let encoding: u32 = 0x65D7A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ucvtf	z31.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// SCVTF_Z.P.Z_H2FP16 Tests
// ============================================================================

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_scvtf_z_p_z_h2fp16_field_pg_0_min_a000_6552a000() {
    // Encoding: 0x6552A000
    // Test SCVTF_Z.P.Z_H2FP16 field Pg = 0 (Min)
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x6552A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_scvtf_z_p_z_h2fp16_field_pg_1_poweroftwo_a000_6552a400() {
    // Encoding: 0x6552A400
    // Test SCVTF_Z.P.Z_H2FP16 field Pg = 1 (PowerOfTwo)
    // Fields: Zd=0, Zn=0, Pg=1
    let encoding: u32 = 0x6552A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p1/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_h2fp16_field_zn_0_min_a000_6552a000() {
    // Encoding: 0x6552A000
    // Test SCVTF_Z.P.Z_H2FP16 field Zn = 0 (Min)
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x6552A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_h2fp16_field_zn_1_poweroftwo_a000_6552a020() {
    // Encoding: 0x6552A020
    // Test SCVTF_Z.P.Z_H2FP16 field Zn = 1 (PowerOfTwo)
    // Fields: Zd=0, Zn=1, Pg=0
    let encoding: u32 = 0x6552A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z1.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_h2fp16_field_zn_30_poweroftwominusone_a000_6552a3c0() {
    // Encoding: 0x6552A3C0
    // Test SCVTF_Z.P.Z_H2FP16 field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zn=30, Zd=0
    let encoding: u32 = 0x6552A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z30.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_h2fp16_field_zn_31_max_a000_6552a3e0() {
    // Encoding: 0x6552A3E0
    // Test SCVTF_Z.P.Z_H2FP16 field Zn = 31 (Max)
    // Fields: Pg=0, Zd=0, Zn=31
    let encoding: u32 = 0x6552A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z31.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_h2fp16_field_zd_0_min_a000_6552a000() {
    // Encoding: 0x6552A000
    // Test SCVTF_Z.P.Z_H2FP16 field Zd = 0 (Min)
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x6552A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_h2fp16_field_zd_1_poweroftwo_a000_6552a001() {
    // Encoding: 0x6552A001
    // Test SCVTF_Z.P.Z_H2FP16 field Zd = 1 (PowerOfTwo)
    // Fields: Zn=0, Pg=0, Zd=1
    let encoding: u32 = 0x6552A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_h2fp16_field_zd_30_poweroftwominusone_a000_6552a01e() {
    // Encoding: 0x6552A01E
    // Test SCVTF_Z.P.Z_H2FP16 field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Zd=30, Pg=0, Zn=0
    let encoding: u32 = 0x6552A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_h2fp16_field_zd_31_max_a000_6552a01f() {
    // Encoding: 0x6552A01F
    // Test SCVTF_Z.P.Z_H2FP16 field Zd = 31 (Max)
    // Fields: Zn=0, Pg=0, Zd=31
    let encoding: u32 = 0x6552A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_scvtf_z_p_z_h2fp16_combo_0_a000_6552a000() {
    // Encoding: 0x6552A000
    // Test SCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zn=0, Pg=0, Zd=0
    let encoding: u32 = 0x6552A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_scvtf_z_p_z_h2fp16_combo_1_a000_6552a400() {
    // Encoding: 0x6552A400
    // Test SCVTF_Z.P.Z_H2FP16 field combination: Pg=1, Zn=0, Zd=0
    // Fields: Pg=1, Zd=0, Zn=0
    let encoding: u32 = 0x6552A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p1/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_h2fp16_combo_2_a000_6552a000() {
    // Encoding: 0x6552A000
    // Test SCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x6552A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_h2fp16_combo_3_a000_6552a020() {
    // Encoding: 0x6552A020
    // Test SCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=1, Zd=0
    // Fields: Zn=1, Pg=0, Zd=0
    let encoding: u32 = 0x6552A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z1.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_h2fp16_combo_4_a000_6552a3c0() {
    // Encoding: 0x6552A3C0
    // Test SCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=30, Zd=0
    // Fields: Pg=0, Zd=0, Zn=30
    let encoding: u32 = 0x6552A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z30.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_h2fp16_combo_5_a000_6552a3e0() {
    // Encoding: 0x6552A3E0
    // Test SCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=31, Zd=0
    // Fields: Zd=0, Pg=0, Zn=31
    let encoding: u32 = 0x6552A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z31.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_h2fp16_combo_6_a000_6552a000() {
    // Encoding: 0x6552A000
    // Test SCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x6552A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_h2fp16_combo_7_a000_6552a001() {
    // Encoding: 0x6552A001
    // Test SCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=0, Zd=1
    // Fields: Zn=0, Pg=0, Zd=1
    let encoding: u32 = 0x6552A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_h2fp16_combo_8_a000_6552a01e() {
    // Encoding: 0x6552A01E
    // Test SCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=0, Zd=30
    // Fields: Zn=0, Zd=30, Pg=0
    let encoding: u32 = 0x6552A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_H2FP16
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_h2fp16_combo_9_a000_6552a01f() {
    // Encoding: 0x6552A01F
    // Test SCVTF_Z.P.Z_H2FP16 field combination: Pg=0, Zn=0, Zd=31
    // Fields: Zd=31, Pg=0, Zn=0
    let encoding: u32 = 0x6552A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.h, p0/m, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_scvtf_z_p_z_w2fp16_field_pg_0_min_a000_6554a000() {
    // Encoding: 0x6554A000
    // Test SCVTF_Z.P.Z_W2FP16 field Pg = 0 (Min)
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x6554A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_scvtf_z_p_z_w2fp16_field_pg_1_poweroftwo_a000_6554a400() {
    // Encoding: 0x6554A400
    // Test SCVTF_Z.P.Z_W2FP16 field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Zn=0, Zd=0
    let encoding: u32 = 0x6554A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p1/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_w2fp16_field_zn_0_min_a000_6554a000() {
    // Encoding: 0x6554A000
    // Test SCVTF_Z.P.Z_W2FP16 field Zn = 0 (Min)
    // Fields: Zn=0, Pg=0, Zd=0
    let encoding: u32 = 0x6554A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_w2fp16_field_zn_1_poweroftwo_a000_6554a020() {
    // Encoding: 0x6554A020
    // Test SCVTF_Z.P.Z_W2FP16 field Zn = 1 (PowerOfTwo)
    // Fields: Zd=0, Pg=0, Zn=1
    let encoding: u32 = 0x6554A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z1.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_w2fp16_field_zn_30_poweroftwominusone_a000_6554a3c0() {
    // Encoding: 0x6554A3C0
    // Test SCVTF_Z.P.Z_W2FP16 field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zn=30, Zd=0
    let encoding: u32 = 0x6554A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z30.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_w2fp16_field_zn_31_max_a000_6554a3e0() {
    // Encoding: 0x6554A3E0
    // Test SCVTF_Z.P.Z_W2FP16 field Zn = 31 (Max)
    // Fields: Zd=0, Zn=31, Pg=0
    let encoding: u32 = 0x6554A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z31.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_w2fp16_field_zd_0_min_a000_6554a000() {
    // Encoding: 0x6554A000
    // Test SCVTF_Z.P.Z_W2FP16 field Zd = 0 (Min)
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x6554A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_w2fp16_field_zd_1_poweroftwo_a000_6554a001() {
    // Encoding: 0x6554A001
    // Test SCVTF_Z.P.Z_W2FP16 field Zd = 1 (PowerOfTwo)
    // Fields: Zd=1, Zn=0, Pg=0
    let encoding: u32 = 0x6554A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_w2fp16_field_zd_30_poweroftwominusone_a000_6554a01e() {
    // Encoding: 0x6554A01E
    // Test SCVTF_Z.P.Z_W2FP16 field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Zd=30, Zn=0, Pg=0
    let encoding: u32 = 0x6554A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_w2fp16_field_zd_31_max_a000_6554a01f() {
    // Encoding: 0x6554A01F
    // Test SCVTF_Z.P.Z_W2FP16 field Zd = 31 (Max)
    // Fields: Pg=0, Zn=0, Zd=31
    let encoding: u32 = 0x6554A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_scvtf_z_p_z_w2fp16_combo_0_a000_6554a000() {
    // Encoding: 0x6554A000
    // Test SCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x6554A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_scvtf_z_p_z_w2fp16_combo_1_a000_6554a400() {
    // Encoding: 0x6554A400
    // Test SCVTF_Z.P.Z_W2FP16 field combination: Pg=1, Zn=0, Zd=0
    // Fields: Zd=0, Zn=0, Pg=1
    let encoding: u32 = 0x6554A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p1/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_w2fp16_combo_2_a000_6554a000() {
    // Encoding: 0x6554A000
    // Test SCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x6554A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_w2fp16_combo_3_a000_6554a020() {
    // Encoding: 0x6554A020
    // Test SCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=1, Zd=0
    // Fields: Zn=1, Pg=0, Zd=0
    let encoding: u32 = 0x6554A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z1.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_w2fp16_combo_4_a000_6554a3c0() {
    // Encoding: 0x6554A3C0
    // Test SCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=30, Zd=0
    // Fields: Zn=30, Pg=0, Zd=0
    let encoding: u32 = 0x6554A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z30.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_w2fp16_combo_5_a000_6554a3e0() {
    // Encoding: 0x6554A3E0
    // Test SCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=31, Zd=0
    // Fields: Zd=0, Zn=31, Pg=0
    let encoding: u32 = 0x6554A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z31.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_w2fp16_combo_6_a000_6554a000() {
    // Encoding: 0x6554A000
    // Test SCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zd=0, Zn=0
    let encoding: u32 = 0x6554A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_w2fp16_combo_7_a000_6554a001() {
    // Encoding: 0x6554A001
    // Test SCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=0, Zd=1
    // Fields: Pg=0, Zd=1, Zn=0
    let encoding: u32 = 0x6554A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_w2fp16_combo_8_a000_6554a01e() {
    // Encoding: 0x6554A01E
    // Test SCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=0, Zd=30
    // Fields: Pg=0, Zn=0, Zd=30
    let encoding: u32 = 0x6554A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2FP16
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_w2fp16_combo_9_a000_6554a01f() {
    // Encoding: 0x6554A01F
    // Test SCVTF_Z.P.Z_W2FP16 field combination: Pg=0, Zn=0, Zd=31
    // Fields: Zn=0, Zd=31, Pg=0
    let encoding: u32 = 0x6554A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.h, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_scvtf_z_p_z_w2s_field_pg_0_min_a000_6594a000() {
    // Encoding: 0x6594A000
    // Test SCVTF_Z.P.Z_W2S field Pg = 0 (Min)
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x6594A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_scvtf_z_p_z_w2s_field_pg_1_poweroftwo_a000_6594a400() {
    // Encoding: 0x6594A400
    // Test SCVTF_Z.P.Z_W2S field Pg = 1 (PowerOfTwo)
    // Fields: Zn=0, Pg=1, Zd=0
    let encoding: u32 = 0x6594A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p1/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_w2s_field_zn_0_min_a000_6594a000() {
    // Encoding: 0x6594A000
    // Test SCVTF_Z.P.Z_W2S field Zn = 0 (Min)
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x6594A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_w2s_field_zn_1_poweroftwo_a000_6594a020() {
    // Encoding: 0x6594A020
    // Test SCVTF_Z.P.Z_W2S field Zn = 1 (PowerOfTwo)
    // Fields: Pg=0, Zn=1, Zd=0
    let encoding: u32 = 0x6594A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z1.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_w2s_field_zn_30_poweroftwominusone_a000_6594a3c0() {
    // Encoding: 0x6594A3C0
    // Test SCVTF_Z.P.Z_W2S field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Zd=0, Pg=0, Zn=30
    let encoding: u32 = 0x6594A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z30.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_w2s_field_zn_31_max_a000_6594a3e0() {
    // Encoding: 0x6594A3E0
    // Test SCVTF_Z.P.Z_W2S field Zn = 31 (Max)
    // Fields: Zd=0, Zn=31, Pg=0
    let encoding: u32 = 0x6594A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z31.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_w2s_field_zd_0_min_a000_6594a000() {
    // Encoding: 0x6594A000
    // Test SCVTF_Z.P.Z_W2S field Zd = 0 (Min)
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x6594A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_w2s_field_zd_1_poweroftwo_a000_6594a001() {
    // Encoding: 0x6594A001
    // Test SCVTF_Z.P.Z_W2S field Zd = 1 (PowerOfTwo)
    // Fields: Zd=1, Pg=0, Zn=0
    let encoding: u32 = 0x6594A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_w2s_field_zd_30_poweroftwominusone_a000_6594a01e() {
    // Encoding: 0x6594A01E
    // Test SCVTF_Z.P.Z_W2S field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zd=30, Zn=0
    let encoding: u32 = 0x6594A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_w2s_field_zd_31_max_a000_6594a01f() {
    // Encoding: 0x6594A01F
    // Test SCVTF_Z.P.Z_W2S field Zd = 31 (Max)
    // Fields: Zn=0, Zd=31, Pg=0
    let encoding: u32 = 0x6594A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_scvtf_z_p_z_w2s_combo_0_a000_6594a000() {
    // Encoding: 0x6594A000
    // Test SCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x6594A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_scvtf_z_p_z_w2s_combo_1_a000_6594a400() {
    // Encoding: 0x6594A400
    // Test SCVTF_Z.P.Z_W2S field combination: Pg=1, Zn=0, Zd=0
    // Fields: Zn=0, Zd=0, Pg=1
    let encoding: u32 = 0x6594A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p1/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_w2s_combo_2_a000_6594a000() {
    // Encoding: 0x6594A000
    // Test SCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zd=0, Zn=0, Pg=0
    let encoding: u32 = 0x6594A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_w2s_combo_3_a000_6594a020() {
    // Encoding: 0x6594A020
    // Test SCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=1, Zd=0
    // Fields: Zn=1, Pg=0, Zd=0
    let encoding: u32 = 0x6594A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z1.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_w2s_combo_4_a000_6594a3c0() {
    // Encoding: 0x6594A3C0
    // Test SCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=30, Zd=0
    // Fields: Pg=0, Zd=0, Zn=30
    let encoding: u32 = 0x6594A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z30.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_w2s_combo_5_a000_6594a3e0() {
    // Encoding: 0x6594A3E0
    // Test SCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=31, Zd=0
    // Fields: Pg=0, Zn=31, Zd=0
    let encoding: u32 = 0x6594A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z31.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_w2s_combo_6_a000_6594a000() {
    // Encoding: 0x6594A000
    // Test SCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x6594A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_w2s_combo_7_a000_6594a001() {
    // Encoding: 0x6594A001
    // Test SCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=0, Zd=1
    // Fields: Pg=0, Zd=1, Zn=0
    let encoding: u32 = 0x6594A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_w2s_combo_8_a000_6594a01e() {
    // Encoding: 0x6594A01E
    // Test SCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=0, Zd=30
    // Fields: Pg=0, Zn=0, Zd=30
    let encoding: u32 = 0x6594A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_w2s_combo_9_a000_6594a01f() {
    // Encoding: 0x6594A01F
    // Test SCVTF_Z.P.Z_W2S field combination: Pg=0, Zn=0, Zd=31
    // Fields: Zd=31, Pg=0, Zn=0
    let encoding: u32 = 0x6594A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.s, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_scvtf_z_p_z_w2d_field_pg_0_min_a000_65d0a000() {
    // Encoding: 0x65D0A000
    // Test SCVTF_Z.P.Z_W2D field Pg = 0 (Min)
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x65D0A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_scvtf_z_p_z_w2d_field_pg_1_poweroftwo_a000_65d0a400() {
    // Encoding: 0x65D0A400
    // Test SCVTF_Z.P.Z_W2D field Pg = 1 (PowerOfTwo)
    // Fields: Zn=0, Zd=0, Pg=1
    let encoding: u32 = 0x65D0A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p1/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_w2d_field_zn_0_min_a000_65d0a000() {
    // Encoding: 0x65D0A000
    // Test SCVTF_Z.P.Z_W2D field Zn = 0 (Min)
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x65D0A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_w2d_field_zn_1_poweroftwo_a000_65d0a020() {
    // Encoding: 0x65D0A020
    // Test SCVTF_Z.P.Z_W2D field Zn = 1 (PowerOfTwo)
    // Fields: Zd=0, Pg=0, Zn=1
    let encoding: u32 = 0x65D0A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z1.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_w2d_field_zn_30_poweroftwominusone_a000_65d0a3c0() {
    // Encoding: 0x65D0A3C0
    // Test SCVTF_Z.P.Z_W2D field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Zd=0, Pg=0, Zn=30
    let encoding: u32 = 0x65D0A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z30.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_w2d_field_zn_31_max_a000_65d0a3e0() {
    // Encoding: 0x65D0A3E0
    // Test SCVTF_Z.P.Z_W2D field Zn = 31 (Max)
    // Fields: Zd=0, Pg=0, Zn=31
    let encoding: u32 = 0x65D0A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z31.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_w2d_field_zd_0_min_a000_65d0a000() {
    // Encoding: 0x65D0A000
    // Test SCVTF_Z.P.Z_W2D field Zd = 0 (Min)
    // Fields: Pg=0, Zd=0, Zn=0
    let encoding: u32 = 0x65D0A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_w2d_field_zd_1_poweroftwo_a000_65d0a001() {
    // Encoding: 0x65D0A001
    // Test SCVTF_Z.P.Z_W2D field Zd = 1 (PowerOfTwo)
    // Fields: Zd=1, Zn=0, Pg=0
    let encoding: u32 = 0x65D0A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_w2d_field_zd_30_poweroftwominusone_a000_65d0a01e() {
    // Encoding: 0x65D0A01E
    // Test SCVTF_Z.P.Z_W2D field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Zn=0, Zd=30, Pg=0
    let encoding: u32 = 0x65D0A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_w2d_field_zd_31_max_a000_65d0a01f() {
    // Encoding: 0x65D0A01F
    // Test SCVTF_Z.P.Z_W2D field Zd = 31 (Max)
    // Fields: Zn=0, Zd=31, Pg=0
    let encoding: u32 = 0x65D0A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_scvtf_z_p_z_w2d_combo_0_a000_65d0a000() {
    // Encoding: 0x65D0A000
    // Test SCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x65D0A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_scvtf_z_p_z_w2d_combo_1_a000_65d0a400() {
    // Encoding: 0x65D0A400
    // Test SCVTF_Z.P.Z_W2D field combination: Pg=1, Zn=0, Zd=0
    // Fields: Pg=1, Zn=0, Zd=0
    let encoding: u32 = 0x65D0A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p1/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_w2d_combo_2_a000_65d0a000() {
    // Encoding: 0x65D0A000
    // Test SCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x65D0A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_w2d_combo_3_a000_65d0a020() {
    // Encoding: 0x65D0A020
    // Test SCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=1, Zd=0
    // Fields: Zn=1, Zd=0, Pg=0
    let encoding: u32 = 0x65D0A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z1.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_w2d_combo_4_a000_65d0a3c0() {
    // Encoding: 0x65D0A3C0
    // Test SCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=30, Zd=0
    // Fields: Pg=0, Zd=0, Zn=30
    let encoding: u32 = 0x65D0A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z30.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_w2d_combo_5_a000_65d0a3e0() {
    // Encoding: 0x65D0A3E0
    // Test SCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=31, Zd=0
    // Fields: Zn=31, Pg=0, Zd=0
    let encoding: u32 = 0x65D0A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z31.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_w2d_combo_6_a000_65d0a000() {
    // Encoding: 0x65D0A000
    // Test SCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zn=0, Pg=0, Zd=0
    let encoding: u32 = 0x65D0A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_w2d_combo_7_a000_65d0a001() {
    // Encoding: 0x65D0A001
    // Test SCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=0, Zd=1
    // Fields: Zn=0, Zd=1, Pg=0
    let encoding: u32 = 0x65D0A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_w2d_combo_8_a000_65d0a01e() {
    // Encoding: 0x65D0A01E
    // Test SCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=0, Zd=30
    // Fields: Zd=30, Pg=0, Zn=0
    let encoding: u32 = 0x65D0A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_W2D
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_w2d_combo_9_a000_65d0a01f() {
    // Encoding: 0x65D0A01F
    // Test SCVTF_Z.P.Z_W2D field combination: Pg=0, Zn=0, Zd=31
    // Fields: Zd=31, Pg=0, Zn=0
    let encoding: u32 = 0x65D0A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.d, p0/m, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_scvtf_z_p_z_x2fp16_field_pg_0_min_a000_6556a000() {
    // Encoding: 0x6556A000
    // Test SCVTF_Z.P.Z_X2FP16 field Pg = 0 (Min)
    // Fields: Zn=0, Pg=0, Zd=0
    let encoding: u32 = 0x6556A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_scvtf_z_p_z_x2fp16_field_pg_1_poweroftwo_a000_6556a400() {
    // Encoding: 0x6556A400
    // Test SCVTF_Z.P.Z_X2FP16 field Pg = 1 (PowerOfTwo)
    // Fields: Zn=0, Pg=1, Zd=0
    let encoding: u32 = 0x6556A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p1/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_x2fp16_field_zn_0_min_a000_6556a000() {
    // Encoding: 0x6556A000
    // Test SCVTF_Z.P.Z_X2FP16 field Zn = 0 (Min)
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x6556A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_x2fp16_field_zn_1_poweroftwo_a000_6556a020() {
    // Encoding: 0x6556A020
    // Test SCVTF_Z.P.Z_X2FP16 field Zn = 1 (PowerOfTwo)
    // Fields: Zn=1, Zd=0, Pg=0
    let encoding: u32 = 0x6556A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z1.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_x2fp16_field_zn_30_poweroftwominusone_a000_6556a3c0() {
    // Encoding: 0x6556A3C0
    // Test SCVTF_Z.P.Z_X2FP16 field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zn=30, Zd=0
    let encoding: u32 = 0x6556A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z30.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_x2fp16_field_zn_31_max_a000_6556a3e0() {
    // Encoding: 0x6556A3E0
    // Test SCVTF_Z.P.Z_X2FP16 field Zn = 31 (Max)
    // Fields: Pg=0, Zd=0, Zn=31
    let encoding: u32 = 0x6556A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z31.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_x2fp16_field_zd_0_min_a000_6556a000() {
    // Encoding: 0x6556A000
    // Test SCVTF_Z.P.Z_X2FP16 field Zd = 0 (Min)
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x6556A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_x2fp16_field_zd_1_poweroftwo_a000_6556a001() {
    // Encoding: 0x6556A001
    // Test SCVTF_Z.P.Z_X2FP16 field Zd = 1 (PowerOfTwo)
    // Fields: Pg=0, Zd=1, Zn=0
    let encoding: u32 = 0x6556A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_x2fp16_field_zd_30_poweroftwominusone_a000_6556a01e() {
    // Encoding: 0x6556A01E
    // Test SCVTF_Z.P.Z_X2FP16 field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zn=0, Zd=30
    let encoding: u32 = 0x6556A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_x2fp16_field_zd_31_max_a000_6556a01f() {
    // Encoding: 0x6556A01F
    // Test SCVTF_Z.P.Z_X2FP16 field Zd = 31 (Max)
    // Fields: Zn=0, Pg=0, Zd=31
    let encoding: u32 = 0x6556A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_scvtf_z_p_z_x2fp16_combo_0_a000_6556a000() {
    // Encoding: 0x6556A000
    // Test SCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x6556A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_scvtf_z_p_z_x2fp16_combo_1_a000_6556a400() {
    // Encoding: 0x6556A400
    // Test SCVTF_Z.P.Z_X2FP16 field combination: Pg=1, Zn=0, Zd=0
    // Fields: Pg=1, Zn=0, Zd=0
    let encoding: u32 = 0x6556A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p1/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_x2fp16_combo_2_a000_6556a000() {
    // Encoding: 0x6556A000
    // Test SCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x6556A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_x2fp16_combo_3_a000_6556a020() {
    // Encoding: 0x6556A020
    // Test SCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=1, Zd=0
    // Fields: Zn=1, Pg=0, Zd=0
    let encoding: u32 = 0x6556A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z1.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_x2fp16_combo_4_a000_6556a3c0() {
    // Encoding: 0x6556A3C0
    // Test SCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=30, Zd=0
    // Fields: Zn=30, Pg=0, Zd=0
    let encoding: u32 = 0x6556A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z30.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_x2fp16_combo_5_a000_6556a3e0() {
    // Encoding: 0x6556A3E0
    // Test SCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=31, Zd=0
    // Fields: Pg=0, Zd=0, Zn=31
    let encoding: u32 = 0x6556A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z31.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_x2fp16_combo_6_a000_6556a000() {
    // Encoding: 0x6556A000
    // Test SCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x6556A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_x2fp16_combo_7_a000_6556a001() {
    // Encoding: 0x6556A001
    // Test SCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=0, Zd=1
    // Fields: Pg=0, Zn=0, Zd=1
    let encoding: u32 = 0x6556A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_x2fp16_combo_8_a000_6556a01e() {
    // Encoding: 0x6556A01E
    // Test SCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=0, Zd=30
    // Fields: Zn=0, Pg=0, Zd=30
    let encoding: u32 = 0x6556A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2FP16
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_x2fp16_combo_9_a000_6556a01f() {
    // Encoding: 0x6556A01F
    // Test SCVTF_Z.P.Z_X2FP16 field combination: Pg=0, Zn=0, Zd=31
    // Fields: Pg=0, Zd=31, Zn=0
    let encoding: u32 = 0x6556A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.h, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_scvtf_z_p_z_x2s_field_pg_0_min_a000_65d4a000() {
    // Encoding: 0x65D4A000
    // Test SCVTF_Z.P.Z_X2S field Pg = 0 (Min)
    // Fields: Pg=0, Zd=0, Zn=0
    let encoding: u32 = 0x65D4A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_scvtf_z_p_z_x2s_field_pg_1_poweroftwo_a000_65d4a400() {
    // Encoding: 0x65D4A400
    // Test SCVTF_Z.P.Z_X2S field Pg = 1 (PowerOfTwo)
    // Fields: Zd=0, Pg=1, Zn=0
    let encoding: u32 = 0x65D4A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p1/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_x2s_field_zn_0_min_a000_65d4a000() {
    // Encoding: 0x65D4A000
    // Test SCVTF_Z.P.Z_X2S field Zn = 0 (Min)
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x65D4A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_x2s_field_zn_1_poweroftwo_a000_65d4a020() {
    // Encoding: 0x65D4A020
    // Test SCVTF_Z.P.Z_X2S field Zn = 1 (PowerOfTwo)
    // Fields: Pg=0, Zn=1, Zd=0
    let encoding: u32 = 0x65D4A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z1.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_x2s_field_zn_30_poweroftwominusone_a000_65d4a3c0() {
    // Encoding: 0x65D4A3C0
    // Test SCVTF_Z.P.Z_X2S field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Zd=0, Pg=0, Zn=30
    let encoding: u32 = 0x65D4A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z30.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_x2s_field_zn_31_max_a000_65d4a3e0() {
    // Encoding: 0x65D4A3E0
    // Test SCVTF_Z.P.Z_X2S field Zn = 31 (Max)
    // Fields: Pg=0, Zn=31, Zd=0
    let encoding: u32 = 0x65D4A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z31.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_x2s_field_zd_0_min_a000_65d4a000() {
    // Encoding: 0x65D4A000
    // Test SCVTF_Z.P.Z_X2S field Zd = 0 (Min)
    // Fields: Zd=0, Pg=0, Zn=0
    let encoding: u32 = 0x65D4A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_x2s_field_zd_1_poweroftwo_a000_65d4a001() {
    // Encoding: 0x65D4A001
    // Test SCVTF_Z.P.Z_X2S field Zd = 1 (PowerOfTwo)
    // Fields: Zn=0, Zd=1, Pg=0
    let encoding: u32 = 0x65D4A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_x2s_field_zd_30_poweroftwominusone_a000_65d4a01e() {
    // Encoding: 0x65D4A01E
    // Test SCVTF_Z.P.Z_X2S field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Zd=30, Zn=0, Pg=0
    let encoding: u32 = 0x65D4A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_x2s_field_zd_31_max_a000_65d4a01f() {
    // Encoding: 0x65D4A01F
    // Test SCVTF_Z.P.Z_X2S field Zd = 31 (Max)
    // Fields: Pg=0, Zn=0, Zd=31
    let encoding: u32 = 0x65D4A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_scvtf_z_p_z_x2s_combo_0_a000_65d4a000() {
    // Encoding: 0x65D4A000
    // Test SCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=0, Zd=0
    // Fields: Zd=0, Zn=0, Pg=0
    let encoding: u32 = 0x65D4A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_scvtf_z_p_z_x2s_combo_1_a000_65d4a400() {
    // Encoding: 0x65D4A400
    // Test SCVTF_Z.P.Z_X2S field combination: Pg=1, Zn=0, Zd=0
    // Fields: Zd=0, Pg=1, Zn=0
    let encoding: u32 = 0x65D4A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p1/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_x2s_combo_2_a000_65d4a000() {
    // Encoding: 0x65D4A000
    // Test SCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zd=0, Zn=0
    let encoding: u32 = 0x65D4A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_x2s_combo_3_a000_65d4a020() {
    // Encoding: 0x65D4A020
    // Test SCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=1, Zd=0
    // Fields: Pg=0, Zd=0, Zn=1
    let encoding: u32 = 0x65D4A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z1.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_x2s_combo_4_a000_65d4a3c0() {
    // Encoding: 0x65D4A3C0
    // Test SCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=30, Zd=0
    // Fields: Pg=0, Zd=0, Zn=30
    let encoding: u32 = 0x65D4A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z30.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_x2s_combo_5_a000_65d4a3e0() {
    // Encoding: 0x65D4A3E0
    // Test SCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=31, Zd=0
    // Fields: Zn=31, Zd=0, Pg=0
    let encoding: u32 = 0x65D4A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z31.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_x2s_combo_6_a000_65d4a000() {
    // Encoding: 0x65D4A000
    // Test SCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x65D4A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_x2s_combo_7_a000_65d4a001() {
    // Encoding: 0x65D4A001
    // Test SCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=0, Zd=1
    // Fields: Zn=0, Zd=1, Pg=0
    let encoding: u32 = 0x65D4A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_x2s_combo_8_a000_65d4a01e() {
    // Encoding: 0x65D4A01E
    // Test SCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=0, Zd=30
    // Fields: Pg=0, Zd=30, Zn=0
    let encoding: u32 = 0x65D4A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_x2s_combo_9_a000_65d4a01f() {
    // Encoding: 0x65D4A01F
    // Test SCVTF_Z.P.Z_X2S field combination: Pg=0, Zn=0, Zd=31
    // Fields: Zd=31, Pg=0, Zn=0
    let encoding: u32 = 0x65D4A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.s, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_scvtf_z_p_z_x2d_field_pg_0_min_a000_65d6a000() {
    // Encoding: 0x65D6A000
    // Test SCVTF_Z.P.Z_X2D field Pg = 0 (Min)
    // Fields: Zn=0, Zd=0, Pg=0
    let encoding: u32 = 0x65D6A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_scvtf_z_p_z_x2d_field_pg_1_poweroftwo_a000_65d6a400() {
    // Encoding: 0x65D6A400
    // Test SCVTF_Z.P.Z_X2D field Pg = 1 (PowerOfTwo)
    // Fields: Zd=0, Pg=1, Zn=0
    let encoding: u32 = 0x65D6A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p1/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_x2d_field_zn_0_min_a000_65d6a000() {
    // Encoding: 0x65D6A000
    // Test SCVTF_Z.P.Z_X2D field Zn = 0 (Min)
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x65D6A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_x2d_field_zn_1_poweroftwo_a000_65d6a020() {
    // Encoding: 0x65D6A020
    // Test SCVTF_Z.P.Z_X2D field Zn = 1 (PowerOfTwo)
    // Fields: Zd=0, Pg=0, Zn=1
    let encoding: u32 = 0x65D6A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z1.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_x2d_field_zn_30_poweroftwominusone_a000_65d6a3c0() {
    // Encoding: 0x65D6A3C0
    // Test SCVTF_Z.P.Z_X2D field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Zd=0, Zn=30, Pg=0
    let encoding: u32 = 0x65D6A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z30.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_x2d_field_zn_31_max_a000_65d6a3e0() {
    // Encoding: 0x65D6A3E0
    // Test SCVTF_Z.P.Z_X2D field Zn = 31 (Max)
    // Fields: Zn=31, Pg=0, Zd=0
    let encoding: u32 = 0x65D6A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z31.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_scvtf_z_p_z_x2d_field_zd_0_min_a000_65d6a000() {
    // Encoding: 0x65D6A000
    // Test SCVTF_Z.P.Z_X2D field Zd = 0 (Min)
    // Fields: Pg=0, Zd=0, Zn=0
    let encoding: u32 = 0x65D6A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_scvtf_z_p_z_x2d_field_zd_1_poweroftwo_a000_65d6a001() {
    // Encoding: 0x65D6A001
    // Test SCVTF_Z.P.Z_X2D field Zd = 1 (PowerOfTwo)
    // Fields: Pg=0, Zn=0, Zd=1
    let encoding: u32 = 0x65D6A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_scvtf_z_p_z_x2d_field_zd_30_poweroftwominusone_a000_65d6a01e() {
    // Encoding: 0x65D6A01E
    // Test SCVTF_Z.P.Z_X2D field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Zd=30, Zn=0, Pg=0
    let encoding: u32 = 0x65D6A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_scvtf_z_p_z_x2d_field_zd_31_max_a000_65d6a01f() {
    // Encoding: 0x65D6A01F
    // Test SCVTF_Z.P.Z_X2D field Zd = 31 (Max)
    // Fields: Pg=0, Zn=0, Zd=31
    let encoding: u32 = 0x65D6A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_scvtf_z_p_z_x2d_combo_0_a000_65d6a000() {
    // Encoding: 0x65D6A000
    // Test SCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zn=0, Zd=0
    let encoding: u32 = 0x65D6A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_scvtf_z_p_z_x2d_combo_1_a000_65d6a400() {
    // Encoding: 0x65D6A400
    // Test SCVTF_Z.P.Z_X2D field combination: Pg=1, Zn=0, Zd=0
    // Fields: Pg=1, Zd=0, Zn=0
    let encoding: u32 = 0x65D6A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p1/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_x2d_combo_2_a000_65d6a000() {
    // Encoding: 0x65D6A000
    // Test SCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zd=0, Zn=0
    let encoding: u32 = 0x65D6A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_x2d_combo_3_a000_65d6a020() {
    // Encoding: 0x65D6A020
    // Test SCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=1, Zd=0
    // Fields: Zd=0, Pg=0, Zn=1
    let encoding: u32 = 0x65D6A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z1.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_x2d_combo_4_a000_65d6a3c0() {
    // Encoding: 0x65D6A3C0
    // Test SCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=30, Zd=0
    // Fields: Zn=30, Zd=0, Pg=0
    let encoding: u32 = 0x65D6A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z30.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_x2d_combo_5_a000_65d6a3e0() {
    // Encoding: 0x65D6A3E0
    // Test SCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=31, Zd=0
    // Fields: Zd=0, Zn=31, Pg=0
    let encoding: u32 = 0x65D6A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z31.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_scvtf_z_p_z_x2d_combo_6_a000_65d6a000() {
    // Encoding: 0x65D6A000
    // Test SCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=0, Zd=0
    // Fields: Pg=0, Zd=0, Zn=0
    let encoding: u32 = 0x65D6A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z0.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_scvtf_z_p_z_x2d_combo_7_a000_65d6a001() {
    // Encoding: 0x65D6A001
    // Test SCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=0, Zd=1
    // Fields: Zd=1, Pg=0, Zn=0
    let encoding: u32 = 0x65D6A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z1.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_scvtf_z_p_z_x2d_combo_8_a000_65d6a01e() {
    // Encoding: 0x65D6A01E
    // Test SCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=0, Zd=30
    // Fields: Zd=30, Pg=0, Zn=0
    let encoding: u32 = 0x65D6A01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z30.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SCVTF_Z.P.Z_X2D
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_scvtf_z_p_z_x2d_combo_9_a000_65d6a01f() {
    // Encoding: 0x65D6A01F
    // Test SCVTF_Z.P.Z_X2D field combination: Pg=0, Zn=0, Zd=31
    // Fields: Zd=31, Pg=0, Zn=0
    let encoding: u32 = 0x65D6A01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: scvtf	z31.d, p0/m, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}
