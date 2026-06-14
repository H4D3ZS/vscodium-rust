//! A64 sve arithmetic tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// MLS_Z.P.ZZZ__ Tests
// ============================================================================

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_mls_z_p_zzz_field_size_0_min_6000_04006000() {
    // Encoding: 0x04006000
    // Test MLS_Z.P.ZZZ__ field size = 0 (Min)
    // Fields: size=0, Zda=0, Zm=0, Pg=0, Zn=0
    let encoding: u32 = 0x04006000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_mls_z_p_zzz_field_size_1_poweroftwo_6000_04406000() {
    // Encoding: 0x04406000
    // Test MLS_Z.P.ZZZ__ field size = 1 (PowerOfTwo)
    // Fields: Zn=0, Zm=0, size=1, Zda=0, Pg=0
    let encoding: u32 = 0x04406000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_mls_z_p_zzz_field_size_2_poweroftwo_6000_04806000() {
    // Encoding: 0x04806000
    // Test MLS_Z.P.ZZZ__ field size = 2 (PowerOfTwo)
    // Fields: Pg=0, size=2, Zn=0, Zda=0, Zm=0
    let encoding: u32 = 0x04806000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_mls_z_p_zzz_field_size_3_max_6000_04c06000() {
    // Encoding: 0x04C06000
    // Test MLS_Z.P.ZZZ__ field size = 3 (Max)
    // Fields: size=3, Zm=0, Pg=0, Zn=0, Zda=0
    let encoding: u32 = 0x04C06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_mls_z_p_zzz_field_zm_0_min_6000_04006000() {
    // Encoding: 0x04006000
    // Test MLS_Z.P.ZZZ__ field Zm = 0 (Min)
    // Fields: Pg=0, Zn=0, Zm=0, size=0, Zda=0
    let encoding: u32 = 0x04006000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_mls_z_p_zzz_field_zm_1_poweroftwo_6000_04016000() {
    // Encoding: 0x04016000
    // Test MLS_Z.P.ZZZ__ field Zm = 1 (PowerOfTwo)
    // Fields: size=0, Pg=0, Zn=0, Zda=0, Zm=1
    let encoding: u32 = 0x04016000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_mls_z_p_zzz_field_zm_30_poweroftwominusone_6000_041e6000() {
    // Encoding: 0x041E6000
    // Test MLS_Z.P.ZZZ__ field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, size=0, Zn=0, Zda=0, Zm=30
    let encoding: u32 = 0x041E6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_mls_z_p_zzz_field_zm_31_max_6000_041f6000() {
    // Encoding: 0x041F6000
    // Test MLS_Z.P.ZZZ__ field Zm = 31 (Max)
    // Fields: Pg=0, size=0, Zda=0, Zn=0, Zm=31
    let encoding: u32 = 0x041F6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_mls_z_p_zzz_field_pg_0_min_6000_04006000() {
    // Encoding: 0x04006000
    // Test MLS_Z.P.ZZZ__ field Pg = 0 (Min)
    // Fields: size=0, Zm=0, Pg=0, Zn=0, Zda=0
    let encoding: u32 = 0x04006000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_mls_z_p_zzz_field_pg_1_poweroftwo_6000_04006400() {
    // Encoding: 0x04006400
    // Test MLS_Z.P.ZZZ__ field Pg = 1 (PowerOfTwo)
    // Fields: Zm=0, Pg=1, size=0, Zda=0, Zn=0
    let encoding: u32 = 0x04006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p1/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_mls_z_p_zzz_field_zn_0_min_6000_04006000() {
    // Encoding: 0x04006000
    // Test MLS_Z.P.ZZZ__ field Zn = 0 (Min)
    // Fields: Pg=0, size=0, Zn=0, Zda=0, Zm=0
    let encoding: u32 = 0x04006000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_mls_z_p_zzz_field_zn_1_poweroftwo_6000_04006020() {
    // Encoding: 0x04006020
    // Test MLS_Z.P.ZZZ__ field Zn = 1 (PowerOfTwo)
    // Fields: Zn=1, size=0, Pg=0, Zda=0, Zm=0
    let encoding: u32 = 0x04006020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_mls_z_p_zzz_field_zn_30_poweroftwominusone_6000_040063c0() {
    // Encoding: 0x040063C0
    // Test MLS_Z.P.ZZZ__ field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Zn=30, Pg=0, Zda=0, size=0, Zm=0
    let encoding: u32 = 0x040063C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z30.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_mls_z_p_zzz_field_zn_31_max_6000_040063e0() {
    // Encoding: 0x040063E0
    // Test MLS_Z.P.ZZZ__ field Zn = 31 (Max)
    // Fields: Pg=0, Zm=0, size=0, Zda=0, Zn=31
    let encoding: u32 = 0x040063E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Zda 0 +: 5`
/// Requirement: FieldBoundary { field: "Zda", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_mls_z_p_zzz_field_zda_0_min_6000_04006000() {
    // Encoding: 0x04006000
    // Test MLS_Z.P.ZZZ__ field Zda = 0 (Min)
    // Fields: size=0, Zm=0, Zn=0, Pg=0, Zda=0
    let encoding: u32 = 0x04006000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Zda 0 +: 5`
/// Requirement: FieldBoundary { field: "Zda", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_mls_z_p_zzz_field_zda_1_poweroftwo_6000_04006001() {
    // Encoding: 0x04006001
    // Test MLS_Z.P.ZZZ__ field Zda = 1 (PowerOfTwo)
    // Fields: Pg=0, Zm=0, Zda=1, size=0, Zn=0
    let encoding: u32 = 0x04006001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z1.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Zda 0 +: 5`
/// Requirement: FieldBoundary { field: "Zda", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_mls_z_p_zzz_field_zda_15_poweroftwominusone_6000_0400600f() {
    // Encoding: 0x0400600F
    // Test MLS_Z.P.ZZZ__ field Zda = 15 (PowerOfTwoMinusOne)
    // Fields: Zm=0, size=0, Pg=0, Zda=15, Zn=0
    let encoding: u32 = 0x0400600F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z15.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field Zda 0 +: 5`
/// Requirement: FieldBoundary { field: "Zda", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_mls_z_p_zzz_field_zda_31_max_6000_0400601f() {
    // Encoding: 0x0400601F
    // Test MLS_Z.P.ZZZ__ field Zda = 31 (Max)
    // Fields: Zn=0, size=0, Pg=0, Zm=0, Zda=31
    let encoding: u32 = 0x0400601F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z31.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_mls_z_p_zzz_combo_0_6000_04006000() {
    // Encoding: 0x04006000
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: Zm=0, Zn=0, size=0, Pg=0, Zda=0
    let encoding: u32 = 0x04006000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_mls_z_p_zzz_combo_1_6000_04406000() {
    // Encoding: 0x04406000
    // Test MLS_Z.P.ZZZ__ field combination: size=1, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: size=1, Zda=0, Zm=0, Zn=0, Pg=0
    let encoding: u32 = 0x04406000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_mls_z_p_zzz_combo_2_6000_04806000() {
    // Encoding: 0x04806000
    // Test MLS_Z.P.ZZZ__ field combination: size=2, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: Pg=0, Zm=0, Zda=0, Zn=0, size=2
    let encoding: u32 = 0x04806000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_mls_z_p_zzz_combo_3_6000_04c06000() {
    // Encoding: 0x04C06000
    // Test MLS_Z.P.ZZZ__ field combination: size=3, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: Zm=0, Zn=0, Zda=0, size=3, Pg=0
    let encoding: u32 = 0x04C06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_mls_z_p_zzz_combo_4_6000_04006000() {
    // Encoding: 0x04006000
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: Zda=0, size=0, Zm=0, Zn=0, Pg=0
    let encoding: u32 = 0x04006000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_mls_z_p_zzz_combo_5_6000_04016000() {
    // Encoding: 0x04016000
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=1, Pg=0, Zn=0, Zda=0
    // Fields: Pg=0, size=0, Zm=1, Zn=0, Zda=0
    let encoding: u32 = 0x04016000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_mls_z_p_zzz_combo_6_6000_041e6000() {
    // Encoding: 0x041E6000
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=30, Pg=0, Zn=0, Zda=0
    // Fields: size=0, Zm=30, Pg=0, Zda=0, Zn=0
    let encoding: u32 = 0x041E6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_mls_z_p_zzz_combo_7_6000_041f6000() {
    // Encoding: 0x041F6000
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=31, Pg=0, Zn=0, Zda=0
    // Fields: Zn=0, Pg=0, Zda=0, size=0, Zm=31
    let encoding: u32 = 0x041F6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_mls_z_p_zzz_combo_8_6000_04006000() {
    // Encoding: 0x04006000
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: size=0, Zda=0, Zm=0, Pg=0, Zn=0
    let encoding: u32 = 0x04006000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_mls_z_p_zzz_combo_9_6000_04006400() {
    // Encoding: 0x04006400
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=1, Zn=0, Zda=0
    // Fields: Zn=0, size=0, Zm=0, Zda=0, Pg=1
    let encoding: u32 = 0x04006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p1/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_mls_z_p_zzz_combo_10_6000_04006000() {
    // Encoding: 0x04006000
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: size=0, Zn=0, Zm=0, Zda=0, Pg=0
    let encoding: u32 = 0x04006000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_mls_z_p_zzz_combo_11_6000_04006020() {
    // Encoding: 0x04006020
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=1, Zda=0
    // Fields: Zn=1, size=0, Zm=0, Zda=0, Pg=0
    let encoding: u32 = 0x04006020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_mls_z_p_zzz_combo_12_6000_040063c0() {
    // Encoding: 0x040063C0
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=30, Zda=0
    // Fields: Zn=30, size=0, Zm=0, Pg=0, Zda=0
    let encoding: u32 = 0x040063C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z30.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_mls_z_p_zzz_combo_13_6000_040063e0() {
    // Encoding: 0x040063E0
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=31, Zda=0
    // Fields: Zda=0, size=0, Zm=0, Zn=31, Pg=0
    let encoding: u32 = 0x040063E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zda=0 (minimum value)
#[test]
fn test_mls_z_p_zzz_combo_14_6000_04006000() {
    // Encoding: 0x04006000
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: size=0, Zda=0, Zm=0, Zn=0, Pg=0
    let encoding: u32 = 0x04006000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zda=1 (value 1)
#[test]
fn test_mls_z_p_zzz_combo_15_6000_04006001() {
    // Encoding: 0x04006001
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=1
    // Fields: size=0, Zn=0, Zm=0, Zda=1, Pg=0
    let encoding: u32 = 0x04006001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z1.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zda=15 (midpoint (15))
#[test]
fn test_mls_z_p_zzz_combo_16_6000_0400600f() {
    // Encoding: 0x0400600F
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=15
    // Fields: size=0, Zn=0, Zda=15, Zm=0, Pg=0
    let encoding: u32 = 0x0400600F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z15.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zda=31 (maximum value (31))
#[test]
fn test_mls_z_p_zzz_combo_17_6000_0400601f() {
    // Encoding: 0x0400601F
    // Test MLS_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=31
    // Fields: size=0, Pg=0, Zda=31, Zn=0, Zm=0
    let encoding: u32 = 0x0400601F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z31.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_mls_z_p_zzz_special_size_0_size_variant_0_24576_04006000() {
    // Encoding: 0x04006000
    // Test MLS_Z.P.ZZZ__ special value size = 0 (Size variant 0)
    // Fields: Pg=0, Zm=0, size=0, Zn=0, Zda=0
    let encoding: u32 = 0x04006000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_mls_z_p_zzz_special_size_1_size_variant_1_24576_04406000() {
    // Encoding: 0x04406000
    // Test MLS_Z.P.ZZZ__ special value size = 1 (Size variant 1)
    // Fields: Zm=0, Pg=0, Zn=0, size=1, Zda=0
    let encoding: u32 = 0x04406000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_mls_z_p_zzz_special_size_2_size_variant_2_24576_04806000() {
    // Encoding: 0x04806000
    // Test MLS_Z.P.ZZZ__ special value size = 2 (Size variant 2)
    // Fields: size=2, Zm=0, Zn=0, Pg=0, Zda=0
    let encoding: u32 = 0x04806000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLS_Z.P.ZZZ__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_mls_z_p_zzz_special_size_3_size_variant_3_24576_04c06000() {
    // Encoding: 0x04C06000
    // Test MLS_Z.P.ZZZ__ special value size = 3 (Size variant 3)
    // Fields: size=3, Zm=0, Pg=0, Zda=0, Zn=0
    let encoding: u32 = 0x04C06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mls	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// ADD_Z.P.ZZ__ Tests
// ============================================================================

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_add_z_p_zz_field_size_0_min_0_04000000() {
    // Encoding: 0x04000000
    // Test ADD_Z.P.ZZ__ field size = 0 (Min)
    // Fields: Pg=0, size=0, Zm=0, Zdn=0
    let encoding: u32 = 0x04000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_add_z_p_zz_field_size_1_poweroftwo_0_04400000() {
    // Encoding: 0x04400000
    // Test ADD_Z.P.ZZ__ field size = 1 (PowerOfTwo)
    // Fields: Zdn=0, Zm=0, Pg=0, size=1
    let encoding: u32 = 0x04400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_add_z_p_zz_field_size_2_poweroftwo_0_04800000() {
    // Encoding: 0x04800000
    // Test ADD_Z.P.ZZ__ field size = 2 (PowerOfTwo)
    // Fields: size=2, Zm=0, Pg=0, Zdn=0
    let encoding: u32 = 0x04800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_add_z_p_zz_field_size_3_max_0_04c00000() {
    // Encoding: 0x04C00000
    // Test ADD_Z.P.ZZ__ field size = 3 (Max)
    // Fields: size=3, Zm=0, Zdn=0, Pg=0
    let encoding: u32 = 0x04C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_add_z_p_zz_field_pg_0_min_0_04000000() {
    // Encoding: 0x04000000
    // Test ADD_Z.P.ZZ__ field Pg = 0 (Min)
    // Fields: size=0, Pg=0, Zm=0, Zdn=0
    let encoding: u32 = 0x04000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_add_z_p_zz_field_pg_1_poweroftwo_0_04000400() {
    // Encoding: 0x04000400
    // Test ADD_Z.P.ZZ__ field Pg = 1 (PowerOfTwo)
    // Fields: size=0, Zdn=0, Pg=1, Zm=0
    let encoding: u32 = 0x04000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p1/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_add_z_p_zz_field_zm_0_min_0_04000000() {
    // Encoding: 0x04000000
    // Test ADD_Z.P.ZZ__ field Zm = 0 (Min)
    // Fields: size=0, Pg=0, Zdn=0, Zm=0
    let encoding: u32 = 0x04000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_add_z_p_zz_field_zm_1_poweroftwo_0_04000020() {
    // Encoding: 0x04000020
    // Test ADD_Z.P.ZZ__ field Zm = 1 (PowerOfTwo)
    // Fields: Zm=1, size=0, Pg=0, Zdn=0
    let encoding: u32 = 0x04000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_add_z_p_zz_field_zm_30_poweroftwominusone_0_040003c0() {
    // Encoding: 0x040003C0
    // Test ADD_Z.P.ZZ__ field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Pg=0, Zm=30, Zdn=0
    let encoding: u32 = 0x040003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_add_z_p_zz_field_zm_31_max_0_040003e0() {
    // Encoding: 0x040003E0
    // Test ADD_Z.P.ZZ__ field Zm = 31 (Max)
    // Fields: Zm=31, Pg=0, size=0, Zdn=0
    let encoding: u32 = 0x040003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_add_z_p_zz_field_zdn_0_min_0_04000000() {
    // Encoding: 0x04000000
    // Test ADD_Z.P.ZZ__ field Zdn = 0 (Min)
    // Fields: Zm=0, size=0, Zdn=0, Pg=0
    let encoding: u32 = 0x04000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_add_z_p_zz_field_zdn_1_poweroftwo_0_04000001() {
    // Encoding: 0x04000001
    // Test ADD_Z.P.ZZ__ field Zdn = 1 (PowerOfTwo)
    // Fields: Zdn=1, Pg=0, size=0, Zm=0
    let encoding: u32 = 0x04000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z1.b, p0/m, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_add_z_p_zz_field_zdn_15_poweroftwominusone_0_0400000f() {
    // Encoding: 0x0400000F
    // Test ADD_Z.P.ZZ__ field Zdn = 15 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zdn=15, size=0, Zm=0
    let encoding: u32 = 0x0400000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z15.b, p0/m, z15.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_add_z_p_zz_field_zdn_31_max_0_0400001f() {
    // Encoding: 0x0400001F
    // Test ADD_Z.P.ZZ__ field Zdn = 31 (Max)
    // Fields: Pg=0, size=0, Zm=0, Zdn=31
    let encoding: u32 = 0x0400001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z31.b, p0/m, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_add_z_p_zz_combo_0_0_04000000() {
    // Encoding: 0x04000000
    // Test ADD_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: Zdn=0, Pg=0, size=0, Zm=0
    let encoding: u32 = 0x04000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_add_z_p_zz_combo_1_0_04400000() {
    // Encoding: 0x04400000
    // Test ADD_Z.P.ZZ__ field combination: size=1, Pg=0, Zm=0, Zdn=0
    // Fields: Zdn=0, Pg=0, size=1, Zm=0
    let encoding: u32 = 0x04400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_add_z_p_zz_combo_2_0_04800000() {
    // Encoding: 0x04800000
    // Test ADD_Z.P.ZZ__ field combination: size=2, Pg=0, Zm=0, Zdn=0
    // Fields: Pg=0, Zm=0, Zdn=0, size=2
    let encoding: u32 = 0x04800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_add_z_p_zz_combo_3_0_04c00000() {
    // Encoding: 0x04C00000
    // Test ADD_Z.P.ZZ__ field combination: size=3, Pg=0, Zm=0, Zdn=0
    // Fields: Zm=0, Zdn=0, Pg=0, size=3
    let encoding: u32 = 0x04C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_add_z_p_zz_combo_4_0_04000000() {
    // Encoding: 0x04000000
    // Test ADD_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: size=0, Zm=0, Pg=0, Zdn=0
    let encoding: u32 = 0x04000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_add_z_p_zz_combo_5_0_04000400() {
    // Encoding: 0x04000400
    // Test ADD_Z.P.ZZ__ field combination: size=0, Pg=1, Zm=0, Zdn=0
    // Fields: Zm=0, size=0, Pg=1, Zdn=0
    let encoding: u32 = 0x04000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p1/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_add_z_p_zz_combo_6_0_04000000() {
    // Encoding: 0x04000000
    // Test ADD_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: Zm=0, size=0, Zdn=0, Pg=0
    let encoding: u32 = 0x04000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_add_z_p_zz_combo_7_0_04000020() {
    // Encoding: 0x04000020
    // Test ADD_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=1, Zdn=0
    // Fields: Zm=1, size=0, Pg=0, Zdn=0
    let encoding: u32 = 0x04000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_add_z_p_zz_combo_8_0_040003c0() {
    // Encoding: 0x040003C0
    // Test ADD_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=30, Zdn=0
    // Fields: Zm=30, Pg=0, size=0, Zdn=0
    let encoding: u32 = 0x040003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_add_z_p_zz_combo_9_0_040003e0() {
    // Encoding: 0x040003E0
    // Test ADD_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=31, Zdn=0
    // Fields: Pg=0, Zm=31, Zdn=0, size=0
    let encoding: u32 = 0x040003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=0 (minimum value)
#[test]
fn test_add_z_p_zz_combo_10_0_04000000() {
    // Encoding: 0x04000000
    // Test ADD_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: Pg=0, Zm=0, Zdn=0, size=0
    let encoding: u32 = 0x04000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=1 (value 1)
#[test]
fn test_add_z_p_zz_combo_11_0_04000001() {
    // Encoding: 0x04000001
    // Test ADD_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=1
    // Fields: size=0, Zdn=1, Zm=0, Pg=0
    let encoding: u32 = 0x04000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z1.b, p0/m, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=15 (midpoint (15))
#[test]
fn test_add_z_p_zz_combo_12_0_0400000f() {
    // Encoding: 0x0400000F
    // Test ADD_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=15
    // Fields: Pg=0, Zdn=15, Zm=0, size=0
    let encoding: u32 = 0x0400000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z15.b, p0/m, z15.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=31 (maximum value (31))
#[test]
fn test_add_z_p_zz_combo_13_0_0400001f() {
    // Encoding: 0x0400001F
    // Test ADD_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=31
    // Fields: size=0, Zdn=31, Zm=0, Pg=0
    let encoding: u32 = 0x0400001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z31.b, p0/m, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_add_z_p_zz_special_size_0_size_variant_0_0_04000000() {
    // Encoding: 0x04000000
    // Test ADD_Z.P.ZZ__ special value size = 0 (Size variant 0)
    // Fields: Pg=0, Zm=0, size=0, Zdn=0
    let encoding: u32 = 0x04000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_add_z_p_zz_special_size_1_size_variant_1_0_04400000() {
    // Encoding: 0x04400000
    // Test ADD_Z.P.ZZ__ special value size = 1 (Size variant 1)
    // Fields: size=1, Pg=0, Zdn=0, Zm=0
    let encoding: u32 = 0x04400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_add_z_p_zz_special_size_2_size_variant_2_0_04800000() {
    // Encoding: 0x04800000
    // Test ADD_Z.P.ZZ__ special value size = 2 (Size variant 2)
    // Fields: Pg=0, size=2, Zm=0, Zdn=0
    let encoding: u32 = 0x04800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.P.ZZ__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_add_z_p_zz_special_size_3_size_variant_3_0_04c00000() {
    // Encoding: 0x04C00000
    // Test ADD_Z.P.ZZ__ special value size = 3 (Size variant 3)
    // Fields: Zdn=0, Zm=0, size=3, Pg=0
    let encoding: u32 = 0x04C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// ADDVL_R.RI__ Tests
// ============================================================================

/// Provenance: ADDVL_R.RI__
/// ASL: `field Rn 16 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_addvl_r_ri_field_rn_0_min_5000_04205000() {
    // Encoding: 0x04205000
    // Test ADDVL_R.RI__ field Rn = 0 (Min)
    // Fields: Rd=0, Rn=0, imm6=0
    let encoding: u32 = 0x04205000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field Rn 16 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_addvl_r_ri_field_rn_1_poweroftwo_5000_04215000() {
    // Encoding: 0x04215000
    // Test ADDVL_R.RI__ field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=1, imm6=0
    let encoding: u32 = 0x04215000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x1, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field Rn 16 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_addvl_r_ri_field_rn_30_poweroftwominusone_5000_043e5000() {
    // Encoding: 0x043E5000
    // Test ADDVL_R.RI__ field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, imm6=0, Rd=0
    let encoding: u32 = 0x043E5000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x30, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field Rn 16 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_addvl_r_ri_field_rn_31_max_5000_043f5000() {
    // Encoding: 0x043F5000
    // Test ADDVL_R.RI__ field Rn = 31 (Max)
    // Fields: Rn=31, Rd=0, imm6=0
    let encoding: u32 = 0x043F5000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, sp, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_addvl_r_ri_field_imm6_0_zero_5000_04205000() {
    // Encoding: 0x04205000
    // Test ADDVL_R.RI__ field imm6 = 0 (Zero)
    // Fields: Rd=0, imm6=0, Rn=0
    let encoding: u32 = 0x04205000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_addvl_r_ri_field_imm6_1_poweroftwo_5000_04205020() {
    // Encoding: 0x04205020
    // Test ADDVL_R.RI__ field imm6 = 1 (PowerOfTwo)
    // Fields: Rd=0, imm6=1, Rn=0
    let encoding: u32 = 0x04205020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_addvl_r_ri_field_imm6_3_poweroftwominusone_5000_04205060() {
    // Encoding: 0x04205060
    // Test ADDVL_R.RI__ field imm6 = 3 (PowerOfTwoMinusOne)
    // Fields: imm6=3, Rn=0, Rd=0
    let encoding: u32 = 0x04205060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #3
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_addvl_r_ri_field_imm6_4_poweroftwo_5000_04205080() {
    // Encoding: 0x04205080
    // Test ADDVL_R.RI__ field imm6 = 4 (PowerOfTwo)
    // Fields: imm6=4, Rn=0, Rd=0
    let encoding: u32 = 0x04205080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_addvl_r_ri_field_imm6_7_poweroftwominusone_5000_042050e0() {
    // Encoding: 0x042050E0
    // Test ADDVL_R.RI__ field imm6 = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, imm6=7, Rd=0
    let encoding: u32 = 0x042050E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #7
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_addvl_r_ri_field_imm6_8_poweroftwo_5000_04205100() {
    // Encoding: 0x04205100
    // Test ADDVL_R.RI__ field imm6 = 8 (PowerOfTwo)
    // Fields: Rn=0, imm6=8, Rd=0
    let encoding: u32 = 0x04205100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #8
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_addvl_r_ri_field_imm6_15_poweroftwominusone_5000_042051e0() {
    // Encoding: 0x042051E0
    // Test ADDVL_R.RI__ field imm6 = 15 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=0, imm6=15
    let encoding: u32 = 0x042051E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #15
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_addvl_r_ri_field_imm6_16_poweroftwo_5000_04205200() {
    // Encoding: 0x04205200
    // Test ADDVL_R.RI__ field imm6 = 16 (PowerOfTwo)
    // Fields: Rd=0, Rn=0, imm6=16
    let encoding: u32 = 0x04205200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #16
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_addvl_r_ri_field_imm6_31_poweroftwominusone_5000_042053e0() {
    // Encoding: 0x042053E0
    // Test ADDVL_R.RI__ field imm6 = 31 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=0, imm6=31
    let encoding: u32 = 0x042053E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #31
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_addvl_r_ri_field_imm6_32_poweroftwo_5000_04205400() {
    // Encoding: 0x04205400
    // Test ADDVL_R.RI__ field imm6 = 32 (PowerOfTwo)
    // Fields: Rd=0, Rn=0, imm6=32
    let encoding: u32 = 0x04205400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #-32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_addvl_r_ri_field_imm6_63_max_5000_042057e0() {
    // Encoding: 0x042057E0
    // Test ADDVL_R.RI__ field imm6 = 63 (Max)
    // Fields: Rn=0, Rd=0, imm6=63
    let encoding: u32 = 0x042057E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #-1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_addvl_r_ri_field_rd_0_min_5000_04205000() {
    // Encoding: 0x04205000
    // Test ADDVL_R.RI__ field Rd = 0 (Min)
    // Fields: Rd=0, imm6=0, Rn=0
    let encoding: u32 = 0x04205000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_addvl_r_ri_field_rd_1_poweroftwo_5000_04205001() {
    // Encoding: 0x04205001
    // Test ADDVL_R.RI__ field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, imm6=0, Rd=1
    let encoding: u32 = 0x04205001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x1, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_addvl_r_ri_field_rd_30_poweroftwominusone_5000_0420501e() {
    // Encoding: 0x0420501E
    // Test ADDVL_R.RI__ field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=30, imm6=0
    let encoding: u32 = 0x0420501E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x30, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_addvl_r_ri_field_rd_31_max_5000_0420501f() {
    // Encoding: 0x0420501F
    // Test ADDVL_R.RI__ field Rd = 31 (Max)
    // Fields: Rn=0, imm6=0, Rd=31
    let encoding: u32 = 0x0420501F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	sp, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_addvl_r_ri_combo_0_5000_04205000() {
    // Encoding: 0x04205000
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=0, Rd=0
    // Fields: Rn=0, imm6=0, Rd=0
    let encoding: u32 = 0x04205000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_addvl_r_ri_combo_1_5000_04215000() {
    // Encoding: 0x04215000
    // Test ADDVL_R.RI__ field combination: Rn=1, imm6=0, Rd=0
    // Fields: Rd=0, Rn=1, imm6=0
    let encoding: u32 = 0x04215000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x1, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_addvl_r_ri_combo_2_5000_043e5000() {
    // Encoding: 0x043E5000
    // Test ADDVL_R.RI__ field combination: Rn=30, imm6=0, Rd=0
    // Fields: Rn=30, imm6=0, Rd=0
    let encoding: u32 = 0x043E5000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x30, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_addvl_r_ri_combo_3_5000_043f5000() {
    // Encoding: 0x043F5000
    // Test ADDVL_R.RI__ field combination: Rn=31, imm6=0, Rd=0
    // Fields: imm6=0, Rd=0, Rn=31
    let encoding: u32 = 0x043F5000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, sp, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=0 (immediate value 0)
#[test]
fn test_addvl_r_ri_combo_4_5000_04205000() {
    // Encoding: 0x04205000
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=0, Rd=0
    // Fields: Rn=0, imm6=0, Rd=0
    let encoding: u32 = 0x04205000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=1 (immediate value 1)
#[test]
fn test_addvl_r_ri_combo_5_5000_04205020() {
    // Encoding: 0x04205020
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=1, Rd=0
    // Fields: imm6=1, Rd=0, Rn=0
    let encoding: u32 = 0x04205020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=3 (2^2 - 1 = 3)
#[test]
fn test_addvl_r_ri_combo_6_5000_04205060() {
    // Encoding: 0x04205060
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=3, Rd=0
    // Fields: Rd=0, Rn=0, imm6=3
    let encoding: u32 = 0x04205060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #3
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=4 (power of 2 (2^2 = 4))
#[test]
fn test_addvl_r_ri_combo_7_5000_04205080() {
    // Encoding: 0x04205080
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=4, Rd=0
    // Fields: imm6=4, Rn=0, Rd=0
    let encoding: u32 = 0x04205080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=7 (2^3 - 1 = 7)
#[test]
fn test_addvl_r_ri_combo_8_5000_042050e0() {
    // Encoding: 0x042050E0
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=7, Rd=0
    // Fields: Rn=0, imm6=7, Rd=0
    let encoding: u32 = 0x042050E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #7
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=8 (power of 2 (2^3 = 8))
#[test]
fn test_addvl_r_ri_combo_9_5000_04205100() {
    // Encoding: 0x04205100
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=8, Rd=0
    // Fields: imm6=8, Rn=0, Rd=0
    let encoding: u32 = 0x04205100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #8
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=15 (2^4 - 1 = 15)
#[test]
fn test_addvl_r_ri_combo_10_5000_042051e0() {
    // Encoding: 0x042051E0
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=15, Rd=0
    // Fields: imm6=15, Rn=0, Rd=0
    let encoding: u32 = 0x042051E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #15
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=16 (power of 2 (2^4 = 16))
#[test]
fn test_addvl_r_ri_combo_11_5000_04205200() {
    // Encoding: 0x04205200
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=16, Rd=0
    // Fields: Rn=0, Rd=0, imm6=16
    let encoding: u32 = 0x04205200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #16
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=31 (immediate midpoint (31))
#[test]
fn test_addvl_r_ri_combo_12_5000_042053e0() {
    // Encoding: 0x042053E0
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=31, Rd=0
    // Fields: imm6=31, Rd=0, Rn=0
    let encoding: u32 = 0x042053E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #31
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=32 (power of 2 (2^5 = 32))
#[test]
fn test_addvl_r_ri_combo_13_5000_04205400() {
    // Encoding: 0x04205400
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=32, Rd=0
    // Fields: imm6=32, Rn=0, Rd=0
    let encoding: u32 = 0x04205400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #-32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=63 (maximum immediate (63))
#[test]
fn test_addvl_r_ri_combo_14_5000_042057e0() {
    // Encoding: 0x042057E0
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=63, Rd=0
    // Fields: imm6=63, Rd=0, Rn=0
    let encoding: u32 = 0x042057E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #-1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_addvl_r_ri_combo_15_5000_04205000() {
    // Encoding: 0x04205000
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=0, Rd=0
    // Fields: imm6=0, Rd=0, Rn=0
    let encoding: u32 = 0x04205000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_addvl_r_ri_combo_16_5000_04205001() {
    // Encoding: 0x04205001
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=0, Rd=1
    // Fields: Rd=1, Rn=0, imm6=0
    let encoding: u32 = 0x04205001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x1, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_addvl_r_ri_combo_17_5000_0420501e() {
    // Encoding: 0x0420501E
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=0, Rd=30
    // Fields: Rd=30, Rn=0, imm6=0
    let encoding: u32 = 0x0420501E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x30, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_addvl_r_ri_combo_18_5000_0420501f() {
    // Encoding: 0x0420501F
    // Test ADDVL_R.RI__ field combination: Rn=0, imm6=0, Rd=31
    // Fields: Rd=31, Rn=0, imm6=0
    let encoding: u32 = 0x0420501F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	sp, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_addvl_r_ri_combo_19_5000_04215001() {
    // Encoding: 0x04215001
    // Test ADDVL_R.RI__ field combination: Rn=1, imm6=0, Rd=1
    // Fields: imm6=0, Rn=1, Rd=1
    let encoding: u32 = 0x04215001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x1, x1, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_addvl_r_ri_combo_20_5000_043f501f() {
    // Encoding: 0x043F501F
    // Test ADDVL_R.RI__ field combination: Rn=31, imm6=0, Rd=31
    // Fields: imm6=0, Rd=31, Rn=31
    let encoding: u32 = 0x043F501F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	sp, sp, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_addvl_r_ri_special_rn_31_stack_pointer_sp_may_require_alignment_20480_043f5020() {
    // Encoding: 0x043F5020
    // Test ADDVL_R.RI__ special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, imm6=1, Rd=0
    let encoding: u32 = 0x043F5020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	x0, sp, #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDVL_R.RI__
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_addvl_r_ri_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_20480_0420503f()
{
    // Encoding: 0x0420503F
    // Test ADDVL_R.RI__ special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, imm6=1, Rd=31
    let encoding: u32 = 0x0420503F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addvl	sp, x0, #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// SUBR_Z.P.ZZ__ Tests
// ============================================================================

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_subr_z_p_zz_field_size_0_min_0_04030000() {
    // Encoding: 0x04030000
    // Test SUBR_Z.P.ZZ__ field size = 0 (Min)
    // Fields: Zdn=0, Pg=0, Zm=0, size=0
    let encoding: u32 = 0x04030000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_subr_z_p_zz_field_size_1_poweroftwo_0_04430000() {
    // Encoding: 0x04430000
    // Test SUBR_Z.P.ZZ__ field size = 1 (PowerOfTwo)
    // Fields: Zdn=0, size=1, Zm=0, Pg=0
    let encoding: u32 = 0x04430000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_subr_z_p_zz_field_size_2_poweroftwo_0_04830000() {
    // Encoding: 0x04830000
    // Test SUBR_Z.P.ZZ__ field size = 2 (PowerOfTwo)
    // Fields: size=2, Pg=0, Zm=0, Zdn=0
    let encoding: u32 = 0x04830000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_subr_z_p_zz_field_size_3_max_0_04c30000() {
    // Encoding: 0x04C30000
    // Test SUBR_Z.P.ZZ__ field size = 3 (Max)
    // Fields: Pg=0, Zdn=0, size=3, Zm=0
    let encoding: u32 = 0x04C30000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_subr_z_p_zz_field_pg_0_min_0_04030000() {
    // Encoding: 0x04030000
    // Test SUBR_Z.P.ZZ__ field Pg = 0 (Min)
    // Fields: size=0, Zm=0, Pg=0, Zdn=0
    let encoding: u32 = 0x04030000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_subr_z_p_zz_field_pg_1_poweroftwo_0_04030400() {
    // Encoding: 0x04030400
    // Test SUBR_Z.P.ZZ__ field Pg = 1 (PowerOfTwo)
    // Fields: Zm=0, Pg=1, Zdn=0, size=0
    let encoding: u32 = 0x04030400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p1/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_subr_z_p_zz_field_zm_0_min_0_04030000() {
    // Encoding: 0x04030000
    // Test SUBR_Z.P.ZZ__ field Zm = 0 (Min)
    // Fields: size=0, Zdn=0, Zm=0, Pg=0
    let encoding: u32 = 0x04030000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_subr_z_p_zz_field_zm_1_poweroftwo_0_04030020() {
    // Encoding: 0x04030020
    // Test SUBR_Z.P.ZZ__ field Zm = 1 (PowerOfTwo)
    // Fields: size=0, Zm=1, Zdn=0, Pg=0
    let encoding: u32 = 0x04030020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_subr_z_p_zz_field_zm_30_poweroftwominusone_0_040303c0() {
    // Encoding: 0x040303C0
    // Test SUBR_Z.P.ZZ__ field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: Zdn=0, Pg=0, Zm=30, size=0
    let encoding: u32 = 0x040303C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_subr_z_p_zz_field_zm_31_max_0_040303e0() {
    // Encoding: 0x040303E0
    // Test SUBR_Z.P.ZZ__ field Zm = 31 (Max)
    // Fields: Zm=31, Pg=0, size=0, Zdn=0
    let encoding: u32 = 0x040303E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_subr_z_p_zz_field_zdn_0_min_0_04030000() {
    // Encoding: 0x04030000
    // Test SUBR_Z.P.ZZ__ field Zdn = 0 (Min)
    // Fields: Pg=0, Zdn=0, size=0, Zm=0
    let encoding: u32 = 0x04030000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_subr_z_p_zz_field_zdn_1_poweroftwo_0_04030001() {
    // Encoding: 0x04030001
    // Test SUBR_Z.P.ZZ__ field Zdn = 1 (PowerOfTwo)
    // Fields: Pg=0, size=0, Zm=0, Zdn=1
    let encoding: u32 = 0x04030001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z1.b, p0/m, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_subr_z_p_zz_field_zdn_15_poweroftwominusone_0_0403000f() {
    // Encoding: 0x0403000F
    // Test SUBR_Z.P.ZZ__ field Zdn = 15 (PowerOfTwoMinusOne)
    // Fields: Zm=0, Pg=0, size=0, Zdn=15
    let encoding: u32 = 0x0403000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z15.b, p0/m, z15.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_subr_z_p_zz_field_zdn_31_max_0_0403001f() {
    // Encoding: 0x0403001F
    // Test SUBR_Z.P.ZZ__ field Zdn = 31 (Max)
    // Fields: Zdn=31, Pg=0, size=0, Zm=0
    let encoding: u32 = 0x0403001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z31.b, p0/m, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_subr_z_p_zz_combo_0_0_04030000() {
    // Encoding: 0x04030000
    // Test SUBR_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: size=0, Pg=0, Zdn=0, Zm=0
    let encoding: u32 = 0x04030000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_subr_z_p_zz_combo_1_0_04430000() {
    // Encoding: 0x04430000
    // Test SUBR_Z.P.ZZ__ field combination: size=1, Pg=0, Zm=0, Zdn=0
    // Fields: Pg=0, size=1, Zm=0, Zdn=0
    let encoding: u32 = 0x04430000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_subr_z_p_zz_combo_2_0_04830000() {
    // Encoding: 0x04830000
    // Test SUBR_Z.P.ZZ__ field combination: size=2, Pg=0, Zm=0, Zdn=0
    // Fields: Zm=0, size=2, Pg=0, Zdn=0
    let encoding: u32 = 0x04830000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_subr_z_p_zz_combo_3_0_04c30000() {
    // Encoding: 0x04C30000
    // Test SUBR_Z.P.ZZ__ field combination: size=3, Pg=0, Zm=0, Zdn=0
    // Fields: Pg=0, Zm=0, Zdn=0, size=3
    let encoding: u32 = 0x04C30000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_subr_z_p_zz_combo_4_0_04030000() {
    // Encoding: 0x04030000
    // Test SUBR_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: Zm=0, Zdn=0, size=0, Pg=0
    let encoding: u32 = 0x04030000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_subr_z_p_zz_combo_5_0_04030400() {
    // Encoding: 0x04030400
    // Test SUBR_Z.P.ZZ__ field combination: size=0, Pg=1, Zm=0, Zdn=0
    // Fields: Pg=1, size=0, Zm=0, Zdn=0
    let encoding: u32 = 0x04030400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p1/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_subr_z_p_zz_combo_6_0_04030000() {
    // Encoding: 0x04030000
    // Test SUBR_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: Zdn=0, size=0, Pg=0, Zm=0
    let encoding: u32 = 0x04030000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_subr_z_p_zz_combo_7_0_04030020() {
    // Encoding: 0x04030020
    // Test SUBR_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=1, Zdn=0
    // Fields: Pg=0, Zm=1, size=0, Zdn=0
    let encoding: u32 = 0x04030020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_subr_z_p_zz_combo_8_0_040303c0() {
    // Encoding: 0x040303C0
    // Test SUBR_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=30, Zdn=0
    // Fields: size=0, Zm=30, Pg=0, Zdn=0
    let encoding: u32 = 0x040303C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_subr_z_p_zz_combo_9_0_040303e0() {
    // Encoding: 0x040303E0
    // Test SUBR_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=31, Zdn=0
    // Fields: Pg=0, Zdn=0, Zm=31, size=0
    let encoding: u32 = 0x040303E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=0 (minimum value)
#[test]
fn test_subr_z_p_zz_combo_10_0_04030000() {
    // Encoding: 0x04030000
    // Test SUBR_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: Pg=0, Zdn=0, size=0, Zm=0
    let encoding: u32 = 0x04030000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=1 (value 1)
#[test]
fn test_subr_z_p_zz_combo_11_0_04030001() {
    // Encoding: 0x04030001
    // Test SUBR_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=1
    // Fields: Zdn=1, Pg=0, size=0, Zm=0
    let encoding: u32 = 0x04030001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z1.b, p0/m, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=15 (midpoint (15))
#[test]
fn test_subr_z_p_zz_combo_12_0_0403000f() {
    // Encoding: 0x0403000F
    // Test SUBR_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=15
    // Fields: size=0, Pg=0, Zdn=15, Zm=0
    let encoding: u32 = 0x0403000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z15.b, p0/m, z15.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=31 (maximum value (31))
#[test]
fn test_subr_z_p_zz_combo_13_0_0403001f() {
    // Encoding: 0x0403001F
    // Test SUBR_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=31
    // Fields: Zdn=31, Pg=0, Zm=0, size=0
    let encoding: u32 = 0x0403001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z31.b, p0/m, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_subr_z_p_zz_special_size_0_size_variant_0_0_04030000() {
    // Encoding: 0x04030000
    // Test SUBR_Z.P.ZZ__ special value size = 0 (Size variant 0)
    // Fields: Zdn=0, Pg=0, size=0, Zm=0
    let encoding: u32 = 0x04030000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_subr_z_p_zz_special_size_1_size_variant_1_0_04430000() {
    // Encoding: 0x04430000
    // Test SUBR_Z.P.ZZ__ special value size = 1 (Size variant 1)
    // Fields: size=1, Pg=0, Zm=0, Zdn=0
    let encoding: u32 = 0x04430000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_subr_z_p_zz_special_size_2_size_variant_2_0_04830000() {
    // Encoding: 0x04830000
    // Test SUBR_Z.P.ZZ__ special value size = 2 (Size variant 2)
    // Fields: size=2, Zm=0, Pg=0, Zdn=0
    let encoding: u32 = 0x04830000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.P.ZZ__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_subr_z_p_zz_special_size_3_size_variant_3_0_04c30000() {
    // Encoding: 0x04C30000
    // Test SUBR_Z.P.ZZ__ special value size = 3 (Size variant 3)
    // Fields: Pg=0, Zm=0, size=3, Zdn=0
    let encoding: u32 = 0x04C30000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// ADDPL_R.RI__ Tests
// ============================================================================

/// Provenance: ADDPL_R.RI__
/// ASL: `field Rn 16 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_addpl_r_ri_field_rn_0_min_5000_04605000() {
    // Encoding: 0x04605000
    // Test ADDPL_R.RI__ field Rn = 0 (Min)
    // Fields: imm6=0, Rn=0, Rd=0
    let encoding: u32 = 0x04605000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field Rn 16 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_addpl_r_ri_field_rn_1_poweroftwo_5000_04615000() {
    // Encoding: 0x04615000
    // Test ADDPL_R.RI__ field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=1, imm6=0
    let encoding: u32 = 0x04615000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x1, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field Rn 16 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_addpl_r_ri_field_rn_30_poweroftwominusone_5000_047e5000() {
    // Encoding: 0x047E5000
    // Test ADDPL_R.RI__ field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: imm6=0, Rn=30, Rd=0
    let encoding: u32 = 0x047E5000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x30, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field Rn 16 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_addpl_r_ri_field_rn_31_max_5000_047f5000() {
    // Encoding: 0x047F5000
    // Test ADDPL_R.RI__ field Rn = 31 (Max)
    // Fields: Rd=0, Rn=31, imm6=0
    let encoding: u32 = 0x047F5000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, sp, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_addpl_r_ri_field_imm6_0_zero_5000_04605000() {
    // Encoding: 0x04605000
    // Test ADDPL_R.RI__ field imm6 = 0 (Zero)
    // Fields: Rn=0, imm6=0, Rd=0
    let encoding: u32 = 0x04605000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_addpl_r_ri_field_imm6_1_poweroftwo_5000_04605020() {
    // Encoding: 0x04605020
    // Test ADDPL_R.RI__ field imm6 = 1 (PowerOfTwo)
    // Fields: imm6=1, Rn=0, Rd=0
    let encoding: u32 = 0x04605020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_addpl_r_ri_field_imm6_3_poweroftwominusone_5000_04605060() {
    // Encoding: 0x04605060
    // Test ADDPL_R.RI__ field imm6 = 3 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=0, imm6=3
    let encoding: u32 = 0x04605060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #3
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_addpl_r_ri_field_imm6_4_poweroftwo_5000_04605080() {
    // Encoding: 0x04605080
    // Test ADDPL_R.RI__ field imm6 = 4 (PowerOfTwo)
    // Fields: Rd=0, Rn=0, imm6=4
    let encoding: u32 = 0x04605080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_addpl_r_ri_field_imm6_7_poweroftwominusone_5000_046050e0() {
    // Encoding: 0x046050E0
    // Test ADDPL_R.RI__ field imm6 = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=0, imm6=7
    let encoding: u32 = 0x046050E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #7
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_addpl_r_ri_field_imm6_8_poweroftwo_5000_04605100() {
    // Encoding: 0x04605100
    // Test ADDPL_R.RI__ field imm6 = 8 (PowerOfTwo)
    // Fields: Rn=0, imm6=8, Rd=0
    let encoding: u32 = 0x04605100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #8
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_addpl_r_ri_field_imm6_15_poweroftwominusone_5000_046051e0() {
    // Encoding: 0x046051E0
    // Test ADDPL_R.RI__ field imm6 = 15 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=0, imm6=15
    let encoding: u32 = 0x046051E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #15
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_addpl_r_ri_field_imm6_16_poweroftwo_5000_04605200() {
    // Encoding: 0x04605200
    // Test ADDPL_R.RI__ field imm6 = 16 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, imm6=16
    let encoding: u32 = 0x04605200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #16
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_addpl_r_ri_field_imm6_31_poweroftwominusone_5000_046053e0() {
    // Encoding: 0x046053E0
    // Test ADDPL_R.RI__ field imm6 = 31 (PowerOfTwoMinusOne)
    // Fields: Rd=0, imm6=31, Rn=0
    let encoding: u32 = 0x046053E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #31
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_addpl_r_ri_field_imm6_32_poweroftwo_5000_04605400() {
    // Encoding: 0x04605400
    // Test ADDPL_R.RI__ field imm6 = 32 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, imm6=32
    let encoding: u32 = 0x04605400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #-32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field imm6 5 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_addpl_r_ri_field_imm6_63_max_5000_046057e0() {
    // Encoding: 0x046057E0
    // Test ADDPL_R.RI__ field imm6 = 63 (Max)
    // Fields: Rn=0, imm6=63, Rd=0
    let encoding: u32 = 0x046057E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #-1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_addpl_r_ri_field_rd_0_min_5000_04605000() {
    // Encoding: 0x04605000
    // Test ADDPL_R.RI__ field Rd = 0 (Min)
    // Fields: imm6=0, Rd=0, Rn=0
    let encoding: u32 = 0x04605000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_addpl_r_ri_field_rd_1_poweroftwo_5000_04605001() {
    // Encoding: 0x04605001
    // Test ADDPL_R.RI__ field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, imm6=0, Rd=1
    let encoding: u32 = 0x04605001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x1, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_addpl_r_ri_field_rd_30_poweroftwominusone_5000_0460501e() {
    // Encoding: 0x0460501E
    // Test ADDPL_R.RI__ field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: imm6=0, Rd=30, Rn=0
    let encoding: u32 = 0x0460501E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x30, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_addpl_r_ri_field_rd_31_max_5000_0460501f() {
    // Encoding: 0x0460501F
    // Test ADDPL_R.RI__ field Rd = 31 (Max)
    // Fields: Rn=0, imm6=0, Rd=31
    let encoding: u32 = 0x0460501F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	sp, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_addpl_r_ri_combo_0_5000_04605000() {
    // Encoding: 0x04605000
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=0, Rd=0
    // Fields: imm6=0, Rd=0, Rn=0
    let encoding: u32 = 0x04605000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_addpl_r_ri_combo_1_5000_04615000() {
    // Encoding: 0x04615000
    // Test ADDPL_R.RI__ field combination: Rn=1, imm6=0, Rd=0
    // Fields: Rn=1, imm6=0, Rd=0
    let encoding: u32 = 0x04615000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x1, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_addpl_r_ri_combo_2_5000_047e5000() {
    // Encoding: 0x047E5000
    // Test ADDPL_R.RI__ field combination: Rn=30, imm6=0, Rd=0
    // Fields: imm6=0, Rn=30, Rd=0
    let encoding: u32 = 0x047E5000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x30, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_addpl_r_ri_combo_3_5000_047f5000() {
    // Encoding: 0x047F5000
    // Test ADDPL_R.RI__ field combination: Rn=31, imm6=0, Rd=0
    // Fields: Rn=31, imm6=0, Rd=0
    let encoding: u32 = 0x047F5000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, sp, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=0 (immediate value 0)
#[test]
fn test_addpl_r_ri_combo_4_5000_04605000() {
    // Encoding: 0x04605000
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=0, Rd=0
    // Fields: imm6=0, Rn=0, Rd=0
    let encoding: u32 = 0x04605000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=1 (immediate value 1)
#[test]
fn test_addpl_r_ri_combo_5_5000_04605020() {
    // Encoding: 0x04605020
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=1, Rd=0
    // Fields: Rn=0, Rd=0, imm6=1
    let encoding: u32 = 0x04605020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=3 (2^2 - 1 = 3)
#[test]
fn test_addpl_r_ri_combo_6_5000_04605060() {
    // Encoding: 0x04605060
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=3, Rd=0
    // Fields: Rn=0, imm6=3, Rd=0
    let encoding: u32 = 0x04605060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #3
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=4 (power of 2 (2^2 = 4))
#[test]
fn test_addpl_r_ri_combo_7_5000_04605080() {
    // Encoding: 0x04605080
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=4, Rd=0
    // Fields: Rn=0, imm6=4, Rd=0
    let encoding: u32 = 0x04605080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=7 (2^3 - 1 = 7)
#[test]
fn test_addpl_r_ri_combo_8_5000_046050e0() {
    // Encoding: 0x046050E0
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=7, Rd=0
    // Fields: imm6=7, Rd=0, Rn=0
    let encoding: u32 = 0x046050E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #7
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=8 (power of 2 (2^3 = 8))
#[test]
fn test_addpl_r_ri_combo_9_5000_04605100() {
    // Encoding: 0x04605100
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=8, Rd=0
    // Fields: imm6=8, Rd=0, Rn=0
    let encoding: u32 = 0x04605100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #8
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=15 (2^4 - 1 = 15)
#[test]
fn test_addpl_r_ri_combo_10_5000_046051e0() {
    // Encoding: 0x046051E0
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=15, Rd=0
    // Fields: Rd=0, Rn=0, imm6=15
    let encoding: u32 = 0x046051E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #15
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=16 (power of 2 (2^4 = 16))
#[test]
fn test_addpl_r_ri_combo_11_5000_04605200() {
    // Encoding: 0x04605200
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=16, Rd=0
    // Fields: Rn=0, imm6=16, Rd=0
    let encoding: u32 = 0x04605200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #16
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=31 (immediate midpoint (31))
#[test]
fn test_addpl_r_ri_combo_12_5000_046053e0() {
    // Encoding: 0x046053E0
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=31, Rd=0
    // Fields: imm6=31, Rd=0, Rn=0
    let encoding: u32 = 0x046053E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #31
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=32 (power of 2 (2^5 = 32))
#[test]
fn test_addpl_r_ri_combo_13_5000_04605400() {
    // Encoding: 0x04605400
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=32, Rd=0
    // Fields: Rd=0, imm6=32, Rn=0
    let encoding: u32 = 0x04605400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #-32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=63 (maximum immediate (63))
#[test]
fn test_addpl_r_ri_combo_14_5000_046057e0() {
    // Encoding: 0x046057E0
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=63, Rd=0
    // Fields: imm6=63, Rd=0, Rn=0
    let encoding: u32 = 0x046057E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #-1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_addpl_r_ri_combo_15_5000_04605000() {
    // Encoding: 0x04605000
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=0, Rd=0
    // Fields: imm6=0, Rd=0, Rn=0
    let encoding: u32 = 0x04605000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_addpl_r_ri_combo_16_5000_04605001() {
    // Encoding: 0x04605001
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=0, Rd=1
    // Fields: Rn=0, Rd=1, imm6=0
    let encoding: u32 = 0x04605001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x1, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_addpl_r_ri_combo_17_5000_0460501e() {
    // Encoding: 0x0460501E
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=0, Rd=30
    // Fields: Rd=30, imm6=0, Rn=0
    let encoding: u32 = 0x0460501E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x30, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_addpl_r_ri_combo_18_5000_0460501f() {
    // Encoding: 0x0460501F
    // Test ADDPL_R.RI__ field combination: Rn=0, imm6=0, Rd=31
    // Fields: imm6=0, Rd=31, Rn=0
    let encoding: u32 = 0x0460501F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	sp, x0, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_addpl_r_ri_combo_19_5000_04615001() {
    // Encoding: 0x04615001
    // Test ADDPL_R.RI__ field combination: Rn=1, imm6=0, Rd=1
    // Fields: Rd=1, imm6=0, Rn=1
    let encoding: u32 = 0x04615001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x1, x1, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_addpl_r_ri_combo_20_5000_047f501f() {
    // Encoding: 0x047F501F
    // Test ADDPL_R.RI__ field combination: Rn=31, imm6=0, Rd=31
    // Fields: imm6=0, Rd=31, Rn=31
    let encoding: u32 = 0x047F501F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	sp, sp, #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_addpl_r_ri_special_rn_31_stack_pointer_sp_may_require_alignment_20480_047f5020() {
    // Encoding: 0x047F5020
    // Test ADDPL_R.RI__ special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: imm6=1, Rd=0, Rn=31
    let encoding: u32 = 0x047F5020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	x0, sp, #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADDPL_R.RI__
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_addpl_r_ri_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_20480_0460503f()
{
    // Encoding: 0x0460503F
    // Test ADDPL_R.RI__ special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Rd=31, imm6=1
    let encoding: u32 = 0x0460503F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: addpl	sp, x0, #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// SUB_Z.P.ZZ__ Tests
// ============================================================================

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_sub_z_p_zz_field_size_0_min_0_04010000() {
    // Encoding: 0x04010000
    // Test SUB_Z.P.ZZ__ field size = 0 (Min)
    // Fields: Pg=0, Zdn=0, Zm=0, size=0
    let encoding: u32 = 0x04010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_sub_z_p_zz_field_size_1_poweroftwo_0_04410000() {
    // Encoding: 0x04410000
    // Test SUB_Z.P.ZZ__ field size = 1 (PowerOfTwo)
    // Fields: size=1, Zm=0, Pg=0, Zdn=0
    let encoding: u32 = 0x04410000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_sub_z_p_zz_field_size_2_poweroftwo_0_04810000() {
    // Encoding: 0x04810000
    // Test SUB_Z.P.ZZ__ field size = 2 (PowerOfTwo)
    // Fields: size=2, Zm=0, Zdn=0, Pg=0
    let encoding: u32 = 0x04810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_sub_z_p_zz_field_size_3_max_0_04c10000() {
    // Encoding: 0x04C10000
    // Test SUB_Z.P.ZZ__ field size = 3 (Max)
    // Fields: size=3, Zm=0, Pg=0, Zdn=0
    let encoding: u32 = 0x04C10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_sub_z_p_zz_field_pg_0_min_0_04010000() {
    // Encoding: 0x04010000
    // Test SUB_Z.P.ZZ__ field Pg = 0 (Min)
    // Fields: Zdn=0, Pg=0, size=0, Zm=0
    let encoding: u32 = 0x04010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_sub_z_p_zz_field_pg_1_poweroftwo_0_04010400() {
    // Encoding: 0x04010400
    // Test SUB_Z.P.ZZ__ field Pg = 1 (PowerOfTwo)
    // Fields: size=0, Zm=0, Zdn=0, Pg=1
    let encoding: u32 = 0x04010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p1/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_sub_z_p_zz_field_zm_0_min_0_04010000() {
    // Encoding: 0x04010000
    // Test SUB_Z.P.ZZ__ field Zm = 0 (Min)
    // Fields: size=0, Pg=0, Zdn=0, Zm=0
    let encoding: u32 = 0x04010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_sub_z_p_zz_field_zm_1_poweroftwo_0_04010020() {
    // Encoding: 0x04010020
    // Test SUB_Z.P.ZZ__ field Zm = 1 (PowerOfTwo)
    // Fields: Pg=0, Zm=1, Zdn=0, size=0
    let encoding: u32 = 0x04010020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_sub_z_p_zz_field_zm_30_poweroftwominusone_0_040103c0() {
    // Encoding: 0x040103C0
    // Test SUB_Z.P.ZZ__ field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: Zm=30, Zdn=0, size=0, Pg=0
    let encoding: u32 = 0x040103C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_sub_z_p_zz_field_zm_31_max_0_040103e0() {
    // Encoding: 0x040103E0
    // Test SUB_Z.P.ZZ__ field Zm = 31 (Max)
    // Fields: Zdn=0, Pg=0, size=0, Zm=31
    let encoding: u32 = 0x040103E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_sub_z_p_zz_field_zdn_0_min_0_04010000() {
    // Encoding: 0x04010000
    // Test SUB_Z.P.ZZ__ field Zdn = 0 (Min)
    // Fields: Pg=0, Zdn=0, Zm=0, size=0
    let encoding: u32 = 0x04010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_sub_z_p_zz_field_zdn_1_poweroftwo_0_04010001() {
    // Encoding: 0x04010001
    // Test SUB_Z.P.ZZ__ field Zdn = 1 (PowerOfTwo)
    // Fields: Pg=0, Zdn=1, size=0, Zm=0
    let encoding: u32 = 0x04010001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z1.b, p0/m, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_sub_z_p_zz_field_zdn_15_poweroftwominusone_0_0401000f() {
    // Encoding: 0x0401000F
    // Test SUB_Z.P.ZZ__ field Zdn = 15 (PowerOfTwoMinusOne)
    // Fields: Zdn=15, size=0, Zm=0, Pg=0
    let encoding: u32 = 0x0401000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z15.b, p0/m, z15.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_sub_z_p_zz_field_zdn_31_max_0_0401001f() {
    // Encoding: 0x0401001F
    // Test SUB_Z.P.ZZ__ field Zdn = 31 (Max)
    // Fields: Zm=0, Pg=0, size=0, Zdn=31
    let encoding: u32 = 0x0401001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z31.b, p0/m, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_sub_z_p_zz_combo_0_0_04010000() {
    // Encoding: 0x04010000
    // Test SUB_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: Pg=0, Zdn=0, size=0, Zm=0
    let encoding: u32 = 0x04010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_sub_z_p_zz_combo_1_0_04410000() {
    // Encoding: 0x04410000
    // Test SUB_Z.P.ZZ__ field combination: size=1, Pg=0, Zm=0, Zdn=0
    // Fields: Zdn=0, size=1, Zm=0, Pg=0
    let encoding: u32 = 0x04410000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_sub_z_p_zz_combo_2_0_04810000() {
    // Encoding: 0x04810000
    // Test SUB_Z.P.ZZ__ field combination: size=2, Pg=0, Zm=0, Zdn=0
    // Fields: Zdn=0, size=2, Zm=0, Pg=0
    let encoding: u32 = 0x04810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_sub_z_p_zz_combo_3_0_04c10000() {
    // Encoding: 0x04C10000
    // Test SUB_Z.P.ZZ__ field combination: size=3, Pg=0, Zm=0, Zdn=0
    // Fields: Zm=0, Zdn=0, Pg=0, size=3
    let encoding: u32 = 0x04C10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_sub_z_p_zz_combo_4_0_04010000() {
    // Encoding: 0x04010000
    // Test SUB_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: Zm=0, size=0, Pg=0, Zdn=0
    let encoding: u32 = 0x04010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_sub_z_p_zz_combo_5_0_04010400() {
    // Encoding: 0x04010400
    // Test SUB_Z.P.ZZ__ field combination: size=0, Pg=1, Zm=0, Zdn=0
    // Fields: size=0, Zm=0, Zdn=0, Pg=1
    let encoding: u32 = 0x04010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p1/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_sub_z_p_zz_combo_6_0_04010000() {
    // Encoding: 0x04010000
    // Test SUB_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: Pg=0, Zm=0, size=0, Zdn=0
    let encoding: u32 = 0x04010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_sub_z_p_zz_combo_7_0_04010020() {
    // Encoding: 0x04010020
    // Test SUB_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=1, Zdn=0
    // Fields: size=0, Zdn=0, Zm=1, Pg=0
    let encoding: u32 = 0x04010020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_sub_z_p_zz_combo_8_0_040103c0() {
    // Encoding: 0x040103C0
    // Test SUB_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=30, Zdn=0
    // Fields: Zdn=0, Zm=30, size=0, Pg=0
    let encoding: u32 = 0x040103C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_sub_z_p_zz_combo_9_0_040103e0() {
    // Encoding: 0x040103E0
    // Test SUB_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=31, Zdn=0
    // Fields: Zdn=0, size=0, Zm=31, Pg=0
    let encoding: u32 = 0x040103E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=0 (minimum value)
#[test]
fn test_sub_z_p_zz_combo_10_0_04010000() {
    // Encoding: 0x04010000
    // Test SUB_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: size=0, Zm=0, Pg=0, Zdn=0
    let encoding: u32 = 0x04010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=1 (value 1)
#[test]
fn test_sub_z_p_zz_combo_11_0_04010001() {
    // Encoding: 0x04010001
    // Test SUB_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=1
    // Fields: Pg=0, Zdn=1, size=0, Zm=0
    let encoding: u32 = 0x04010001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z1.b, p0/m, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=15 (midpoint (15))
#[test]
fn test_sub_z_p_zz_combo_12_0_0401000f() {
    // Encoding: 0x0401000F
    // Test SUB_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=15
    // Fields: Pg=0, size=0, Zm=0, Zdn=15
    let encoding: u32 = 0x0401000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z15.b, p0/m, z15.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=31 (maximum value (31))
#[test]
fn test_sub_z_p_zz_combo_13_0_0401001f() {
    // Encoding: 0x0401001F
    // Test SUB_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=31
    // Fields: size=0, Pg=0, Zm=0, Zdn=31
    let encoding: u32 = 0x0401001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z31.b, p0/m, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_sub_z_p_zz_special_size_0_size_variant_0_0_04010000() {
    // Encoding: 0x04010000
    // Test SUB_Z.P.ZZ__ special value size = 0 (Size variant 0)
    // Fields: Pg=0, Zdn=0, size=0, Zm=0
    let encoding: u32 = 0x04010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_sub_z_p_zz_special_size_1_size_variant_1_0_04410000() {
    // Encoding: 0x04410000
    // Test SUB_Z.P.ZZ__ special value size = 1 (Size variant 1)
    // Fields: size=1, Zm=0, Zdn=0, Pg=0
    let encoding: u32 = 0x04410000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_sub_z_p_zz_special_size_2_size_variant_2_0_04810000() {
    // Encoding: 0x04810000
    // Test SUB_Z.P.ZZ__ special value size = 2 (Size variant 2)
    // Fields: Pg=0, Zm=0, size=2, Zdn=0
    let encoding: u32 = 0x04810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.P.ZZ__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_sub_z_p_zz_special_size_3_size_variant_3_0_04c10000() {
    // Encoding: 0x04C10000
    // Test SUB_Z.P.ZZ__ special value size = 3 (Size variant 3)
    // Fields: Zdn=0, Zm=0, size=3, Pg=0
    let encoding: u32 = 0x04C10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// MUL_Z.ZI__ Tests
// ============================================================================

/// Provenance: MUL_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_mul_z_zi_field_size_0_min_c000_2530c000() {
    // Encoding: 0x2530C000
    // Test MUL_Z.ZI__ field size = 0 (Min)
    // Fields: size=0, imm8=0, Zdn=0
    let encoding: u32 = 0x2530C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_mul_z_zi_field_size_1_poweroftwo_c000_2570c000() {
    // Encoding: 0x2570C000
    // Test MUL_Z.ZI__ field size = 1 (PowerOfTwo)
    // Fields: Zdn=0, size=1, imm8=0
    let encoding: u32 = 0x2570C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_mul_z_zi_field_size_2_poweroftwo_c000_25b0c000() {
    // Encoding: 0x25B0C000
    // Test MUL_Z.ZI__ field size = 2 (PowerOfTwo)
    // Fields: size=2, imm8=0, Zdn=0
    let encoding: u32 = 0x25B0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_mul_z_zi_field_size_3_max_c000_25f0c000() {
    // Encoding: 0x25F0C000
    // Test MUL_Z.ZI__ field size = 3 (Max)
    // Fields: Zdn=0, size=3, imm8=0
    let encoding: u32 = 0x25F0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_mul_z_zi_field_imm8_0_zero_c000_2530c000() {
    // Encoding: 0x2530C000
    // Test MUL_Z.ZI__ field imm8 = 0 (Zero)
    // Fields: size=0, Zdn=0, imm8=0
    let encoding: u32 = 0x2530C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_mul_z_zi_field_imm8_1_poweroftwo_c000_2530c020() {
    // Encoding: 0x2530C020
    // Test MUL_Z.ZI__ field imm8 = 1 (PowerOfTwo)
    // Fields: Zdn=0, imm8=1, size=0
    let encoding: u32 = 0x2530C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_mul_z_zi_field_imm8_3_poweroftwominusone_c000_2530c060() {
    // Encoding: 0x2530C060
    // Test MUL_Z.ZI__ field imm8 = 3 (PowerOfTwoMinusOne)
    // Fields: imm8=3, size=0, Zdn=0
    let encoding: u32 = 0x2530C060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_mul_z_zi_field_imm8_4_poweroftwo_c000_2530c080() {
    // Encoding: 0x2530C080
    // Test MUL_Z.ZI__ field imm8 = 4 (PowerOfTwo)
    // Fields: Zdn=0, imm8=4, size=0
    let encoding: u32 = 0x2530C080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_mul_z_zi_field_imm8_7_poweroftwominusone_c000_2530c0e0() {
    // Encoding: 0x2530C0E0
    // Test MUL_Z.ZI__ field imm8 = 7 (PowerOfTwoMinusOne)
    // Fields: size=0, Zdn=0, imm8=7
    let encoding: u32 = 0x2530C0E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_mul_z_zi_field_imm8_8_poweroftwo_c000_2530c100() {
    // Encoding: 0x2530C100
    // Test MUL_Z.ZI__ field imm8 = 8 (PowerOfTwo)
    // Fields: size=0, Zdn=0, imm8=8
    let encoding: u32 = 0x2530C100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_mul_z_zi_field_imm8_15_poweroftwominusone_c000_2530c1e0() {
    // Encoding: 0x2530C1E0
    // Test MUL_Z.ZI__ field imm8 = 15 (PowerOfTwoMinusOne)
    // Fields: imm8=15, size=0, Zdn=0
    let encoding: u32 = 0x2530C1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_mul_z_zi_field_imm8_16_poweroftwo_c000_2530c200() {
    // Encoding: 0x2530C200
    // Test MUL_Z.ZI__ field imm8 = 16 (PowerOfTwo)
    // Fields: imm8=16, Zdn=0, size=0
    let encoding: u32 = 0x2530C200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_mul_z_zi_field_imm8_31_poweroftwominusone_c000_2530c3e0() {
    // Encoding: 0x2530C3E0
    // Test MUL_Z.ZI__ field imm8 = 31 (PowerOfTwoMinusOne)
    // Fields: imm8=31, Zdn=0, size=0
    let encoding: u32 = 0x2530C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_mul_z_zi_field_imm8_32_poweroftwo_c000_2530c400() {
    // Encoding: 0x2530C400
    // Test MUL_Z.ZI__ field imm8 = 32 (PowerOfTwo)
    // Fields: imm8=32, Zdn=0, size=0
    let encoding: u32 = 0x2530C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_mul_z_zi_field_imm8_63_poweroftwominusone_c000_2530c7e0() {
    // Encoding: 0x2530C7E0
    // Test MUL_Z.ZI__ field imm8 = 63 (PowerOfTwoMinusOne)
    // Fields: Zdn=0, imm8=63, size=0
    let encoding: u32 = 0x2530C7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_mul_z_zi_field_imm8_64_poweroftwo_c000_2530c800() {
    // Encoding: 0x2530C800
    // Test MUL_Z.ZI__ field imm8 = 64 (PowerOfTwo)
    // Fields: imm8=64, Zdn=0, size=0
    let encoding: u32 = 0x2530C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_mul_z_zi_field_imm8_127_poweroftwominusone_c000_2530cfe0() {
    // Encoding: 0x2530CFE0
    // Test MUL_Z.ZI__ field imm8 = 127 (PowerOfTwoMinusOne)
    // Fields: size=0, Zdn=0, imm8=127
    let encoding: u32 = 0x2530CFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_mul_z_zi_field_imm8_128_poweroftwo_c000_2530d000() {
    // Encoding: 0x2530D000
    // Test MUL_Z.ZI__ field imm8 = 128 (PowerOfTwo)
    // Fields: Zdn=0, imm8=128, size=0
    let encoding: u32 = 0x2530D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_mul_z_zi_field_imm8_255_max_c000_2530dfe0() {
    // Encoding: 0x2530DFE0
    // Test MUL_Z.ZI__ field imm8 = 255 (Max)
    // Fields: size=0, Zdn=0, imm8=255
    let encoding: u32 = 0x2530DFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_mul_z_zi_field_zdn_0_min_c000_2530c000() {
    // Encoding: 0x2530C000
    // Test MUL_Z.ZI__ field Zdn = 0 (Min)
    // Fields: imm8=0, Zdn=0, size=0
    let encoding: u32 = 0x2530C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_mul_z_zi_field_zdn_1_poweroftwo_c000_2530c001() {
    // Encoding: 0x2530C001
    // Test MUL_Z.ZI__ field Zdn = 1 (PowerOfTwo)
    // Fields: Zdn=1, size=0, imm8=0
    let encoding: u32 = 0x2530C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_mul_z_zi_field_zdn_15_poweroftwominusone_c000_2530c00f() {
    // Encoding: 0x2530C00F
    // Test MUL_Z.ZI__ field Zdn = 15 (PowerOfTwoMinusOne)
    // Fields: size=0, Zdn=15, imm8=0
    let encoding: u32 = 0x2530C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_mul_z_zi_field_zdn_31_max_c000_2530c01f() {
    // Encoding: 0x2530C01F
    // Test MUL_Z.ZI__ field Zdn = 31 (Max)
    // Fields: size=0, imm8=0, Zdn=31
    let encoding: u32 = 0x2530C01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_mul_z_zi_combo_0_c000_2530c000() {
    // Encoding: 0x2530C000
    // Test MUL_Z.ZI__ field combination: size=0, imm8=0, Zdn=0
    // Fields: imm8=0, size=0, Zdn=0
    let encoding: u32 = 0x2530C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_mul_z_zi_combo_1_c000_2570c000() {
    // Encoding: 0x2570C000
    // Test MUL_Z.ZI__ field combination: size=1, imm8=0, Zdn=0
    // Fields: size=1, imm8=0, Zdn=0
    let encoding: u32 = 0x2570C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_mul_z_zi_combo_2_c000_25b0c000() {
    // Encoding: 0x25B0C000
    // Test MUL_Z.ZI__ field combination: size=2, imm8=0, Zdn=0
    // Fields: Zdn=0, imm8=0, size=2
    let encoding: u32 = 0x25B0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_mul_z_zi_combo_3_c000_25f0c000() {
    // Encoding: 0x25F0C000
    // Test MUL_Z.ZI__ field combination: size=3, imm8=0, Zdn=0
    // Fields: size=3, Zdn=0, imm8=0
    let encoding: u32 = 0x25F0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=0 (immediate value 0)
#[test]
fn test_mul_z_zi_combo_4_c000_2530c000() {
    // Encoding: 0x2530C000
    // Test MUL_Z.ZI__ field combination: size=0, imm8=0, Zdn=0
    // Fields: size=0, imm8=0, Zdn=0
    let encoding: u32 = 0x2530C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=1 (immediate value 1)
#[test]
fn test_mul_z_zi_combo_5_c000_2530c020() {
    // Encoding: 0x2530C020
    // Test MUL_Z.ZI__ field combination: size=0, imm8=1, Zdn=0
    // Fields: Zdn=0, size=0, imm8=1
    let encoding: u32 = 0x2530C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=3 (2^2 - 1 = 3)
#[test]
fn test_mul_z_zi_combo_6_c000_2530c060() {
    // Encoding: 0x2530C060
    // Test MUL_Z.ZI__ field combination: size=0, imm8=3, Zdn=0
    // Fields: size=0, imm8=3, Zdn=0
    let encoding: u32 = 0x2530C060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=4 (power of 2 (2^2 = 4))
#[test]
fn test_mul_z_zi_combo_7_c000_2530c080() {
    // Encoding: 0x2530C080
    // Test MUL_Z.ZI__ field combination: size=0, imm8=4, Zdn=0
    // Fields: imm8=4, Zdn=0, size=0
    let encoding: u32 = 0x2530C080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=7 (2^3 - 1 = 7)
#[test]
fn test_mul_z_zi_combo_8_c000_2530c0e0() {
    // Encoding: 0x2530C0E0
    // Test MUL_Z.ZI__ field combination: size=0, imm8=7, Zdn=0
    // Fields: Zdn=0, size=0, imm8=7
    let encoding: u32 = 0x2530C0E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=8 (power of 2 (2^3 = 8))
#[test]
fn test_mul_z_zi_combo_9_c000_2530c100() {
    // Encoding: 0x2530C100
    // Test MUL_Z.ZI__ field combination: size=0, imm8=8, Zdn=0
    // Fields: size=0, Zdn=0, imm8=8
    let encoding: u32 = 0x2530C100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=15 (2^4 - 1 = 15)
#[test]
fn test_mul_z_zi_combo_10_c000_2530c1e0() {
    // Encoding: 0x2530C1E0
    // Test MUL_Z.ZI__ field combination: size=0, imm8=15, Zdn=0
    // Fields: Zdn=0, imm8=15, size=0
    let encoding: u32 = 0x2530C1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=16 (power of 2 (2^4 = 16))
#[test]
fn test_mul_z_zi_combo_11_c000_2530c200() {
    // Encoding: 0x2530C200
    // Test MUL_Z.ZI__ field combination: size=0, imm8=16, Zdn=0
    // Fields: size=0, imm8=16, Zdn=0
    let encoding: u32 = 0x2530C200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=31 (2^5 - 1 = 31)
#[test]
fn test_mul_z_zi_combo_12_c000_2530c3e0() {
    // Encoding: 0x2530C3E0
    // Test MUL_Z.ZI__ field combination: size=0, imm8=31, Zdn=0
    // Fields: size=0, Zdn=0, imm8=31
    let encoding: u32 = 0x2530C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=32 (power of 2 (2^5 = 32))
#[test]
fn test_mul_z_zi_combo_13_c000_2530c400() {
    // Encoding: 0x2530C400
    // Test MUL_Z.ZI__ field combination: size=0, imm8=32, Zdn=0
    // Fields: size=0, imm8=32, Zdn=0
    let encoding: u32 = 0x2530C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=63 (2^6 - 1 = 63)
#[test]
fn test_mul_z_zi_combo_14_c000_2530c7e0() {
    // Encoding: 0x2530C7E0
    // Test MUL_Z.ZI__ field combination: size=0, imm8=63, Zdn=0
    // Fields: Zdn=0, size=0, imm8=63
    let encoding: u32 = 0x2530C7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=64 (power of 2 (2^6 = 64))
#[test]
fn test_mul_z_zi_combo_15_c000_2530c800() {
    // Encoding: 0x2530C800
    // Test MUL_Z.ZI__ field combination: size=0, imm8=64, Zdn=0
    // Fields: imm8=64, Zdn=0, size=0
    let encoding: u32 = 0x2530C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=127 (immediate midpoint (127))
#[test]
fn test_mul_z_zi_combo_16_c000_2530cfe0() {
    // Encoding: 0x2530CFE0
    // Test MUL_Z.ZI__ field combination: size=0, imm8=127, Zdn=0
    // Fields: Zdn=0, size=0, imm8=127
    let encoding: u32 = 0x2530CFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=128 (power of 2 (2^7 = 128))
#[test]
fn test_mul_z_zi_combo_17_c000_2530d000() {
    // Encoding: 0x2530D000
    // Test MUL_Z.ZI__ field combination: size=0, imm8=128, Zdn=0
    // Fields: imm8=128, Zdn=0, size=0
    let encoding: u32 = 0x2530D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=255 (maximum immediate (255))
#[test]
fn test_mul_z_zi_combo_18_c000_2530dfe0() {
    // Encoding: 0x2530DFE0
    // Test MUL_Z.ZI__ field combination: size=0, imm8=255, Zdn=0
    // Fields: Zdn=0, imm8=255, size=0
    let encoding: u32 = 0x2530DFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=0 (minimum value)
#[test]
fn test_mul_z_zi_combo_19_c000_2530c000() {
    // Encoding: 0x2530C000
    // Test MUL_Z.ZI__ field combination: size=0, imm8=0, Zdn=0
    // Fields: Zdn=0, size=0, imm8=0
    let encoding: u32 = 0x2530C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=1 (value 1)
#[test]
fn test_mul_z_zi_combo_20_c000_2530c001() {
    // Encoding: 0x2530C001
    // Test MUL_Z.ZI__ field combination: size=0, imm8=0, Zdn=1
    // Fields: imm8=0, size=0, Zdn=1
    let encoding: u32 = 0x2530C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=15 (midpoint (15))
#[test]
fn test_mul_z_zi_combo_21_c000_2530c00f() {
    // Encoding: 0x2530C00F
    // Test MUL_Z.ZI__ field combination: size=0, imm8=0, Zdn=15
    // Fields: size=0, imm8=0, Zdn=15
    let encoding: u32 = 0x2530C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=31 (maximum value (31))
#[test]
fn test_mul_z_zi_combo_22_c000_2530c01f() {
    // Encoding: 0x2530C01F
    // Test MUL_Z.ZI__ field combination: size=0, imm8=0, Zdn=31
    // Fields: size=0, imm8=0, Zdn=31
    let encoding: u32 = 0x2530C01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_mul_z_zi_special_size_0_size_variant_0_49152_2530c020() {
    // Encoding: 0x2530C020
    // Test MUL_Z.ZI__ special value size = 0 (Size variant 0)
    // Fields: imm8=1, Zdn=0, size=0
    let encoding: u32 = 0x2530C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_mul_z_zi_special_size_1_size_variant_1_49152_2570c020() {
    // Encoding: 0x2570C020
    // Test MUL_Z.ZI__ special value size = 1 (Size variant 1)
    // Fields: size=1, imm8=1, Zdn=0
    let encoding: u32 = 0x2570C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_mul_z_zi_special_size_2_size_variant_2_49152_25b0c020() {
    // Encoding: 0x25B0C020
    // Test MUL_Z.ZI__ special value size = 2 (Size variant 2)
    // Fields: imm8=1, Zdn=0, size=2
    let encoding: u32 = 0x25B0C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: MUL_Z.ZI__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_mul_z_zi_special_size_3_size_variant_3_49152_25f0c020() {
    // Encoding: 0x25F0C020
    // Test MUL_Z.ZI__ special value size = 3 (Size variant 3)
    // Fields: imm8=1, Zdn=0, size=3
    let encoding: u32 = 0x25F0C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

// ============================================================================
// SUB_Z.ZZ__ Tests
// ============================================================================

/// Provenance: SUB_Z.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_sub_z_zz_field_size_0_min_400_04200400() {
    // Encoding: 0x04200400
    // Test SUB_Z.ZZ__ field size = 0 (Min)
    // Fields: Zn=0, Zm=0, size=0, Zd=0
    let encoding: u32 = 0x04200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_sub_z_zz_field_size_1_poweroftwo_400_04600400() {
    // Encoding: 0x04600400
    // Test SUB_Z.ZZ__ field size = 1 (PowerOfTwo)
    // Fields: Zn=0, Zm=0, Zd=0, size=1
    let encoding: u32 = 0x04600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.h, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_sub_z_zz_field_size_2_poweroftwo_400_04a00400() {
    // Encoding: 0x04A00400
    // Test SUB_Z.ZZ__ field size = 2 (PowerOfTwo)
    // Fields: size=2, Zm=0, Zn=0, Zd=0
    let encoding: u32 = 0x04A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.s, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_sub_z_zz_field_size_3_max_400_04e00400() {
    // Encoding: 0x04E00400
    // Test SUB_Z.ZZ__ field size = 3 (Max)
    // Fields: Zd=0, Zn=0, Zm=0, size=3
    let encoding: u32 = 0x04E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.d, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_sub_z_zz_field_zm_0_min_400_04200400() {
    // Encoding: 0x04200400
    // Test SUB_Z.ZZ__ field Zm = 0 (Min)
    // Fields: size=0, Zn=0, Zm=0, Zd=0
    let encoding: u32 = 0x04200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_sub_z_zz_field_zm_1_poweroftwo_400_04210400() {
    // Encoding: 0x04210400
    // Test SUB_Z.ZZ__ field Zm = 1 (PowerOfTwo)
    // Fields: Zm=1, Zn=0, size=0, Zd=0
    let encoding: u32 = 0x04210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_sub_z_zz_field_zm_30_poweroftwominusone_400_043e0400() {
    // Encoding: 0x043E0400
    // Test SUB_Z.ZZ__ field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: Zm=30, Zn=0, size=0, Zd=0
    let encoding: u32 = 0x043E0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_sub_z_zz_field_zm_31_max_400_043f0400() {
    // Encoding: 0x043F0400
    // Test SUB_Z.ZZ__ field Zm = 31 (Max)
    // Fields: Zn=0, Zd=0, Zm=31, size=0
    let encoding: u32 = 0x043F0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_sub_z_zz_field_zn_0_min_400_04200400() {
    // Encoding: 0x04200400
    // Test SUB_Z.ZZ__ field Zn = 0 (Min)
    // Fields: Zm=0, Zd=0, size=0, Zn=0
    let encoding: u32 = 0x04200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_sub_z_zz_field_zn_1_poweroftwo_400_04200420() {
    // Encoding: 0x04200420
    // Test SUB_Z.ZZ__ field Zn = 1 (PowerOfTwo)
    // Fields: size=0, Zm=0, Zn=1, Zd=0
    let encoding: u32 = 0x04200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_sub_z_zz_field_zn_30_poweroftwominusone_400_042007c0() {
    // Encoding: 0x042007C0
    // Test SUB_Z.ZZ__ field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Zn=30, size=0, Zd=0, Zm=0
    let encoding: u32 = 0x042007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z30.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_sub_z_zz_field_zn_31_max_400_042007e0() {
    // Encoding: 0x042007E0
    // Test SUB_Z.ZZ__ field Zn = 31 (Max)
    // Fields: Zm=0, Zd=0, Zn=31, size=0
    let encoding: u32 = 0x042007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_sub_z_zz_field_zd_0_min_400_04200400() {
    // Encoding: 0x04200400
    // Test SUB_Z.ZZ__ field Zd = 0 (Min)
    // Fields: Zm=0, Zn=0, Zd=0, size=0
    let encoding: u32 = 0x04200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_sub_z_zz_field_zd_1_poweroftwo_400_04200401() {
    // Encoding: 0x04200401
    // Test SUB_Z.ZZ__ field Zd = 1 (PowerOfTwo)
    // Fields: size=0, Zm=0, Zn=0, Zd=1
    let encoding: u32 = 0x04200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z1.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_sub_z_zz_field_zd_30_poweroftwominusone_400_0420041e() {
    // Encoding: 0x0420041E
    // Test SUB_Z.ZZ__ field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Zd=30, size=0, Zm=0, Zn=0
    let encoding: u32 = 0x0420041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z30.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_sub_z_zz_field_zd_31_max_400_0420041f() {
    // Encoding: 0x0420041F
    // Test SUB_Z.ZZ__ field Zd = 31 (Max)
    // Fields: Zm=0, Zd=31, Zn=0, size=0
    let encoding: u32 = 0x0420041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z31.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_sub_z_zz_combo_0_400_04200400() {
    // Encoding: 0x04200400
    // Test SUB_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=0
    // Fields: Zm=0, Zn=0, Zd=0, size=0
    let encoding: u32 = 0x04200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_sub_z_zz_combo_1_400_04600400() {
    // Encoding: 0x04600400
    // Test SUB_Z.ZZ__ field combination: size=1, Zm=0, Zn=0, Zd=0
    // Fields: Zm=0, Zn=0, size=1, Zd=0
    let encoding: u32 = 0x04600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.h, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_sub_z_zz_combo_2_400_04a00400() {
    // Encoding: 0x04A00400
    // Test SUB_Z.ZZ__ field combination: size=2, Zm=0, Zn=0, Zd=0
    // Fields: Zd=0, Zn=0, size=2, Zm=0
    let encoding: u32 = 0x04A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.s, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_sub_z_zz_combo_3_400_04e00400() {
    // Encoding: 0x04E00400
    // Test SUB_Z.ZZ__ field combination: size=3, Zm=0, Zn=0, Zd=0
    // Fields: Zm=0, Zd=0, size=3, Zn=0
    let encoding: u32 = 0x04E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.d, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_sub_z_zz_combo_4_400_04200400() {
    // Encoding: 0x04200400
    // Test SUB_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=0
    // Fields: Zm=0, size=0, Zn=0, Zd=0
    let encoding: u32 = 0x04200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_sub_z_zz_combo_5_400_04210400() {
    // Encoding: 0x04210400
    // Test SUB_Z.ZZ__ field combination: size=0, Zm=1, Zn=0, Zd=0
    // Fields: Zd=0, size=0, Zm=1, Zn=0
    let encoding: u32 = 0x04210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_sub_z_zz_combo_6_400_043e0400() {
    // Encoding: 0x043E0400
    // Test SUB_Z.ZZ__ field combination: size=0, Zm=30, Zn=0, Zd=0
    // Fields: Zd=0, size=0, Zm=30, Zn=0
    let encoding: u32 = 0x043E0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_sub_z_zz_combo_7_400_043f0400() {
    // Encoding: 0x043F0400
    // Test SUB_Z.ZZ__ field combination: size=0, Zm=31, Zn=0, Zd=0
    // Fields: Zn=0, Zm=31, size=0, Zd=0
    let encoding: u32 = 0x043F0400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_sub_z_zz_combo_8_400_04200400() {
    // Encoding: 0x04200400
    // Test SUB_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=0
    // Fields: Zd=0, Zm=0, size=0, Zn=0
    let encoding: u32 = 0x04200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_sub_z_zz_combo_9_400_04200420() {
    // Encoding: 0x04200420
    // Test SUB_Z.ZZ__ field combination: size=0, Zm=0, Zn=1, Zd=0
    // Fields: size=0, Zd=0, Zm=0, Zn=1
    let encoding: u32 = 0x04200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_sub_z_zz_combo_10_400_042007c0() {
    // Encoding: 0x042007C0
    // Test SUB_Z.ZZ__ field combination: size=0, Zm=0, Zn=30, Zd=0
    // Fields: Zm=0, size=0, Zn=30, Zd=0
    let encoding: u32 = 0x042007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z30.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_sub_z_zz_combo_11_400_042007e0() {
    // Encoding: 0x042007E0
    // Test SUB_Z.ZZ__ field combination: size=0, Zm=0, Zn=31, Zd=0
    // Fields: Zm=0, Zd=0, Zn=31, size=0
    let encoding: u32 = 0x042007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_sub_z_zz_combo_12_400_04200400() {
    // Encoding: 0x04200400
    // Test SUB_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=0
    // Fields: Zd=0, size=0, Zm=0, Zn=0
    let encoding: u32 = 0x04200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_sub_z_zz_combo_13_400_04200401() {
    // Encoding: 0x04200401
    // Test SUB_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=1
    // Fields: Zm=0, size=0, Zn=0, Zd=1
    let encoding: u32 = 0x04200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z1.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_sub_z_zz_combo_14_400_0420041e() {
    // Encoding: 0x0420041E
    // Test SUB_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=30
    // Fields: Zn=0, Zm=0, Zd=30, size=0
    let encoding: u32 = 0x0420041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z30.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_sub_z_zz_combo_15_400_0420041f() {
    // Encoding: 0x0420041F
    // Test SUB_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=31
    // Fields: size=0, Zm=0, Zd=31, Zn=0
    let encoding: u32 = 0x0420041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z31.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_sub_z_zz_special_size_0_size_variant_0_1024_04200400() {
    // Encoding: 0x04200400
    // Test SUB_Z.ZZ__ special value size = 0 (Size variant 0)
    // Fields: Zd=0, Zn=0, Zm=0, size=0
    let encoding: u32 = 0x04200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_sub_z_zz_special_size_1_size_variant_1_1024_04600400() {
    // Encoding: 0x04600400
    // Test SUB_Z.ZZ__ special value size = 1 (Size variant 1)
    // Fields: Zm=0, Zd=0, size=1, Zn=0
    let encoding: u32 = 0x04600400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.h, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_sub_z_zz_special_size_2_size_variant_2_1024_04a00400() {
    // Encoding: 0x04A00400
    // Test SUB_Z.ZZ__ special value size = 2 (Size variant 2)
    // Fields: size=2, Zn=0, Zm=0, Zd=0
    let encoding: u32 = 0x04A00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.s, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZZ__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_sub_z_zz_special_size_3_size_variant_3_1024_04e00400() {
    // Encoding: 0x04E00400
    // Test SUB_Z.ZZ__ special value size = 3 (Size variant 3)
    // Fields: Zd=0, size=3, Zm=0, Zn=0
    let encoding: u32 = 0x04E00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.d, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// MLA_Z.P.ZZZ__ Tests
// ============================================================================

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_mla_z_p_zzz_field_size_0_min_4000_04004000() {
    // Encoding: 0x04004000
    // Test MLA_Z.P.ZZZ__ field size = 0 (Min)
    // Fields: Zm=0, size=0, Zn=0, Zda=0, Pg=0
    let encoding: u32 = 0x04004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_mla_z_p_zzz_field_size_1_poweroftwo_4000_04404000() {
    // Encoding: 0x04404000
    // Test MLA_Z.P.ZZZ__ field size = 1 (PowerOfTwo)
    // Fields: Zn=0, Zm=0, size=1, Pg=0, Zda=0
    let encoding: u32 = 0x04404000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_mla_z_p_zzz_field_size_2_poweroftwo_4000_04804000() {
    // Encoding: 0x04804000
    // Test MLA_Z.P.ZZZ__ field size = 2 (PowerOfTwo)
    // Fields: Zm=0, size=2, Zn=0, Zda=0, Pg=0
    let encoding: u32 = 0x04804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_mla_z_p_zzz_field_size_3_max_4000_04c04000() {
    // Encoding: 0x04C04000
    // Test MLA_Z.P.ZZZ__ field size = 3 (Max)
    // Fields: size=3, Zda=0, Zm=0, Zn=0, Pg=0
    let encoding: u32 = 0x04C04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_mla_z_p_zzz_field_zm_0_min_4000_04004000() {
    // Encoding: 0x04004000
    // Test MLA_Z.P.ZZZ__ field Zm = 0 (Min)
    // Fields: Zn=0, Pg=0, Zda=0, size=0, Zm=0
    let encoding: u32 = 0x04004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_mla_z_p_zzz_field_zm_1_poweroftwo_4000_04014000() {
    // Encoding: 0x04014000
    // Test MLA_Z.P.ZZZ__ field Zm = 1 (PowerOfTwo)
    // Fields: Pg=0, Zn=0, Zda=0, size=0, Zm=1
    let encoding: u32 = 0x04014000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_mla_z_p_zzz_field_zm_30_poweroftwominusone_4000_041e4000() {
    // Encoding: 0x041E4000
    // Test MLA_Z.P.ZZZ__ field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Pg=0, Zn=0, Zm=30, Zda=0
    let encoding: u32 = 0x041E4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_mla_z_p_zzz_field_zm_31_max_4000_041f4000() {
    // Encoding: 0x041F4000
    // Test MLA_Z.P.ZZZ__ field Zm = 31 (Max)
    // Fields: size=0, Zn=0, Zda=0, Zm=31, Pg=0
    let encoding: u32 = 0x041F4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_mla_z_p_zzz_field_pg_0_min_4000_04004000() {
    // Encoding: 0x04004000
    // Test MLA_Z.P.ZZZ__ field Pg = 0 (Min)
    // Fields: size=0, Zm=0, Zn=0, Pg=0, Zda=0
    let encoding: u32 = 0x04004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_mla_z_p_zzz_field_pg_1_poweroftwo_4000_04004400() {
    // Encoding: 0x04004400
    // Test MLA_Z.P.ZZZ__ field Pg = 1 (PowerOfTwo)
    // Fields: Zda=0, Pg=1, size=0, Zm=0, Zn=0
    let encoding: u32 = 0x04004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p1/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_mla_z_p_zzz_field_zn_0_min_4000_04004000() {
    // Encoding: 0x04004000
    // Test MLA_Z.P.ZZZ__ field Zn = 0 (Min)
    // Fields: Zm=0, Zda=0, Zn=0, size=0, Pg=0
    let encoding: u32 = 0x04004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_mla_z_p_zzz_field_zn_1_poweroftwo_4000_04004020() {
    // Encoding: 0x04004020
    // Test MLA_Z.P.ZZZ__ field Zn = 1 (PowerOfTwo)
    // Fields: Zm=0, size=0, Zda=0, Pg=0, Zn=1
    let encoding: u32 = 0x04004020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_mla_z_p_zzz_field_zn_30_poweroftwominusone_4000_040043c0() {
    // Encoding: 0x040043C0
    // Test MLA_Z.P.ZZZ__ field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Zm=0, Zda=0, Zn=30, Pg=0
    let encoding: u32 = 0x040043C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z30.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_mla_z_p_zzz_field_zn_31_max_4000_040043e0() {
    // Encoding: 0x040043E0
    // Test MLA_Z.P.ZZZ__ field Zn = 31 (Max)
    // Fields: Pg=0, Zm=0, size=0, Zda=0, Zn=31
    let encoding: u32 = 0x040043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Zda 0 +: 5`
/// Requirement: FieldBoundary { field: "Zda", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_mla_z_p_zzz_field_zda_0_min_4000_04004000() {
    // Encoding: 0x04004000
    // Test MLA_Z.P.ZZZ__ field Zda = 0 (Min)
    // Fields: size=0, Zn=0, Pg=0, Zda=0, Zm=0
    let encoding: u32 = 0x04004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Zda 0 +: 5`
/// Requirement: FieldBoundary { field: "Zda", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_mla_z_p_zzz_field_zda_1_poweroftwo_4000_04004001() {
    // Encoding: 0x04004001
    // Test MLA_Z.P.ZZZ__ field Zda = 1 (PowerOfTwo)
    // Fields: Zda=1, Zm=0, Pg=0, Zn=0, size=0
    let encoding: u32 = 0x04004001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z1.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Zda 0 +: 5`
/// Requirement: FieldBoundary { field: "Zda", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_mla_z_p_zzz_field_zda_15_poweroftwominusone_4000_0400400f() {
    // Encoding: 0x0400400F
    // Test MLA_Z.P.ZZZ__ field Zda = 15 (PowerOfTwoMinusOne)
    // Fields: size=0, Zm=0, Pg=0, Zda=15, Zn=0
    let encoding: u32 = 0x0400400F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z15.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field Zda 0 +: 5`
/// Requirement: FieldBoundary { field: "Zda", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_mla_z_p_zzz_field_zda_31_max_4000_0400401f() {
    // Encoding: 0x0400401F
    // Test MLA_Z.P.ZZZ__ field Zda = 31 (Max)
    // Fields: size=0, Zm=0, Zn=0, Zda=31, Pg=0
    let encoding: u32 = 0x0400401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z31.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_mla_z_p_zzz_combo_0_4000_04004000() {
    // Encoding: 0x04004000
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: Zm=0, Zn=0, Zda=0, Pg=0, size=0
    let encoding: u32 = 0x04004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_mla_z_p_zzz_combo_1_4000_04404000() {
    // Encoding: 0x04404000
    // Test MLA_Z.P.ZZZ__ field combination: size=1, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: Zda=0, size=1, Zm=0, Pg=0, Zn=0
    let encoding: u32 = 0x04404000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_mla_z_p_zzz_combo_2_4000_04804000() {
    // Encoding: 0x04804000
    // Test MLA_Z.P.ZZZ__ field combination: size=2, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: Zm=0, size=2, Pg=0, Zn=0, Zda=0
    let encoding: u32 = 0x04804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_mla_z_p_zzz_combo_3_4000_04c04000() {
    // Encoding: 0x04C04000
    // Test MLA_Z.P.ZZZ__ field combination: size=3, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: Zn=0, size=3, Zm=0, Pg=0, Zda=0
    let encoding: u32 = 0x04C04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_mla_z_p_zzz_combo_4_4000_04004000() {
    // Encoding: 0x04004000
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: Pg=0, size=0, Zn=0, Zda=0, Zm=0
    let encoding: u32 = 0x04004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_mla_z_p_zzz_combo_5_4000_04014000() {
    // Encoding: 0x04014000
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=1, Pg=0, Zn=0, Zda=0
    // Fields: Zm=1, size=0, Zda=0, Zn=0, Pg=0
    let encoding: u32 = 0x04014000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_mla_z_p_zzz_combo_6_4000_041e4000() {
    // Encoding: 0x041E4000
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=30, Pg=0, Zn=0, Zda=0
    // Fields: Zn=0, Zda=0, Zm=30, size=0, Pg=0
    let encoding: u32 = 0x041E4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_mla_z_p_zzz_combo_7_4000_041f4000() {
    // Encoding: 0x041F4000
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=31, Pg=0, Zn=0, Zda=0
    // Fields: Zn=0, size=0, Pg=0, Zda=0, Zm=31
    let encoding: u32 = 0x041F4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_mla_z_p_zzz_combo_8_4000_04004000() {
    // Encoding: 0x04004000
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: Zm=0, Zn=0, Pg=0, Zda=0, size=0
    let encoding: u32 = 0x04004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_mla_z_p_zzz_combo_9_4000_04004400() {
    // Encoding: 0x04004400
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=1, Zn=0, Zda=0
    // Fields: Pg=1, Zm=0, size=0, Zn=0, Zda=0
    let encoding: u32 = 0x04004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p1/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_mla_z_p_zzz_combo_10_4000_04004000() {
    // Encoding: 0x04004000
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: Zm=0, size=0, Zn=0, Zda=0, Pg=0
    let encoding: u32 = 0x04004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_mla_z_p_zzz_combo_11_4000_04004020() {
    // Encoding: 0x04004020
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=1, Zda=0
    // Fields: Zn=1, Zm=0, Pg=0, Zda=0, size=0
    let encoding: u32 = 0x04004020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_mla_z_p_zzz_combo_12_4000_040043c0() {
    // Encoding: 0x040043C0
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=30, Zda=0
    // Fields: Zda=0, size=0, Zn=30, Zm=0, Pg=0
    let encoding: u32 = 0x040043C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z30.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_mla_z_p_zzz_combo_13_4000_040043e0() {
    // Encoding: 0x040043E0
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=31, Zda=0
    // Fields: size=0, Pg=0, Zn=31, Zda=0, Zm=0
    let encoding: u32 = 0x040043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zda=0 (minimum value)
#[test]
fn test_mla_z_p_zzz_combo_14_4000_04004000() {
    // Encoding: 0x04004000
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=0
    // Fields: Zda=0, size=0, Pg=0, Zn=0, Zm=0
    let encoding: u32 = 0x04004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zda=1 (value 1)
#[test]
fn test_mla_z_p_zzz_combo_15_4000_04004001() {
    // Encoding: 0x04004001
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=1
    // Fields: Zm=0, Pg=0, Zn=0, Zda=1, size=0
    let encoding: u32 = 0x04004001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z1.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zda=15 (midpoint (15))
#[test]
fn test_mla_z_p_zzz_combo_16_4000_0400400f() {
    // Encoding: 0x0400400F
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=15
    // Fields: Zda=15, size=0, Zm=0, Pg=0, Zn=0
    let encoding: u32 = 0x0400400F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z15.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zda=31 (maximum value (31))
#[test]
fn test_mla_z_p_zzz_combo_17_4000_0400401f() {
    // Encoding: 0x0400401F
    // Test MLA_Z.P.ZZZ__ field combination: size=0, Zm=0, Pg=0, Zn=0, Zda=31
    // Fields: Zda=31, Zm=0, Zn=0, Pg=0, size=0
    let encoding: u32 = 0x0400401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z31.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_mla_z_p_zzz_special_size_0_size_variant_0_16384_04004000() {
    // Encoding: 0x04004000
    // Test MLA_Z.P.ZZZ__ special value size = 0 (Size variant 0)
    // Fields: Zm=0, Pg=0, size=0, Zn=0, Zda=0
    let encoding: u32 = 0x04004000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_mla_z_p_zzz_special_size_1_size_variant_1_16384_04404000() {
    // Encoding: 0x04404000
    // Test MLA_Z.P.ZZZ__ special value size = 1 (Size variant 1)
    // Fields: Zm=0, Zda=0, Zn=0, size=1, Pg=0
    let encoding: u32 = 0x04404000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_mla_z_p_zzz_special_size_2_size_variant_2_16384_04804000() {
    // Encoding: 0x04804000
    // Test MLA_Z.P.ZZZ__ special value size = 2 (Size variant 2)
    // Fields: size=2, Pg=0, Zm=0, Zn=0, Zda=0
    let encoding: u32 = 0x04804000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MLA_Z.P.ZZZ__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_mla_z_p_zzz_special_size_3_size_variant_3_16384_04c04000() {
    // Encoding: 0x04C04000
    // Test MLA_Z.P.ZZZ__ special value size = 3 (Size variant 3)
    // Fields: Zn=0, size=3, Zda=0, Pg=0, Zm=0
    let encoding: u32 = 0x04C04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mla	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// MUL_Z.P.ZZ__ Tests
// ============================================================================

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_mul_z_p_zz_field_size_0_min_0_04100000() {
    // Encoding: 0x04100000
    // Test MUL_Z.P.ZZ__ field size = 0 (Min)
    // Fields: size=0, Zdn=0, Zm=0, Pg=0
    let encoding: u32 = 0x04100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_mul_z_p_zz_field_size_1_poweroftwo_0_04500000() {
    // Encoding: 0x04500000
    // Test MUL_Z.P.ZZ__ field size = 1 (PowerOfTwo)
    // Fields: Pg=0, size=1, Zm=0, Zdn=0
    let encoding: u32 = 0x04500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_mul_z_p_zz_field_size_2_poweroftwo_0_04900000() {
    // Encoding: 0x04900000
    // Test MUL_Z.P.ZZ__ field size = 2 (PowerOfTwo)
    // Fields: Zm=0, size=2, Pg=0, Zdn=0
    let encoding: u32 = 0x04900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_mul_z_p_zz_field_size_3_max_0_04d00000() {
    // Encoding: 0x04D00000
    // Test MUL_Z.P.ZZ__ field size = 3 (Max)
    // Fields: size=3, Zdn=0, Zm=0, Pg=0
    let encoding: u32 = 0x04D00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_mul_z_p_zz_field_pg_0_min_0_04100000() {
    // Encoding: 0x04100000
    // Test MUL_Z.P.ZZ__ field Pg = 0 (Min)
    // Fields: Zdn=0, Zm=0, size=0, Pg=0
    let encoding: u32 = 0x04100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_mul_z_p_zz_field_pg_1_poweroftwo_0_04100400() {
    // Encoding: 0x04100400
    // Test MUL_Z.P.ZZ__ field Pg = 1 (PowerOfTwo)
    // Fields: Zm=0, size=0, Pg=1, Zdn=0
    let encoding: u32 = 0x04100400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p1/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_mul_z_p_zz_field_zm_0_min_0_04100000() {
    // Encoding: 0x04100000
    // Test MUL_Z.P.ZZ__ field Zm = 0 (Min)
    // Fields: size=0, Zdn=0, Zm=0, Pg=0
    let encoding: u32 = 0x04100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_mul_z_p_zz_field_zm_1_poweroftwo_0_04100020() {
    // Encoding: 0x04100020
    // Test MUL_Z.P.ZZ__ field Zm = 1 (PowerOfTwo)
    // Fields: size=0, Pg=0, Zm=1, Zdn=0
    let encoding: u32 = 0x04100020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_mul_z_p_zz_field_zm_30_poweroftwominusone_0_041003c0() {
    // Encoding: 0x041003C0
    // Test MUL_Z.P.ZZ__ field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, size=0, Zdn=0, Zm=30
    let encoding: u32 = 0x041003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field Zm 5 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_mul_z_p_zz_field_zm_31_max_0_041003e0() {
    // Encoding: 0x041003E0
    // Test MUL_Z.P.ZZ__ field Zm = 31 (Max)
    // Fields: size=0, Zm=31, Zdn=0, Pg=0
    let encoding: u32 = 0x041003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_mul_z_p_zz_field_zdn_0_min_0_04100000() {
    // Encoding: 0x04100000
    // Test MUL_Z.P.ZZ__ field Zdn = 0 (Min)
    // Fields: Zm=0, Pg=0, size=0, Zdn=0
    let encoding: u32 = 0x04100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_mul_z_p_zz_field_zdn_1_poweroftwo_0_04100001() {
    // Encoding: 0x04100001
    // Test MUL_Z.P.ZZ__ field Zdn = 1 (PowerOfTwo)
    // Fields: Zdn=1, Zm=0, Pg=0, size=0
    let encoding: u32 = 0x04100001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z1.b, p0/m, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_mul_z_p_zz_field_zdn_15_poweroftwominusone_0_0410000f() {
    // Encoding: 0x0410000F
    // Test MUL_Z.P.ZZ__ field Zdn = 15 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zm=0, Zdn=15, size=0
    let encoding: u32 = 0x0410000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z15.b, p0/m, z15.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_mul_z_p_zz_field_zdn_31_max_0_0410001f() {
    // Encoding: 0x0410001F
    // Test MUL_Z.P.ZZ__ field Zdn = 31 (Max)
    // Fields: Pg=0, Zm=0, size=0, Zdn=31
    let encoding: u32 = 0x0410001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z31.b, p0/m, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_mul_z_p_zz_combo_0_0_04100000() {
    // Encoding: 0x04100000
    // Test MUL_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: Zm=0, Zdn=0, Pg=0, size=0
    let encoding: u32 = 0x04100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_mul_z_p_zz_combo_1_0_04500000() {
    // Encoding: 0x04500000
    // Test MUL_Z.P.ZZ__ field combination: size=1, Pg=0, Zm=0, Zdn=0
    // Fields: Zdn=0, Zm=0, Pg=0, size=1
    let encoding: u32 = 0x04500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_mul_z_p_zz_combo_2_0_04900000() {
    // Encoding: 0x04900000
    // Test MUL_Z.P.ZZ__ field combination: size=2, Pg=0, Zm=0, Zdn=0
    // Fields: Zm=0, size=2, Pg=0, Zdn=0
    let encoding: u32 = 0x04900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_mul_z_p_zz_combo_3_0_04d00000() {
    // Encoding: 0x04D00000
    // Test MUL_Z.P.ZZ__ field combination: size=3, Pg=0, Zm=0, Zdn=0
    // Fields: size=3, Zm=0, Zdn=0, Pg=0
    let encoding: u32 = 0x04D00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_mul_z_p_zz_combo_4_0_04100000() {
    // Encoding: 0x04100000
    // Test MUL_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: size=0, Pg=0, Zdn=0, Zm=0
    let encoding: u32 = 0x04100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_mul_z_p_zz_combo_5_0_04100400() {
    // Encoding: 0x04100400
    // Test MUL_Z.P.ZZ__ field combination: size=0, Pg=1, Zm=0, Zdn=0
    // Fields: size=0, Zdn=0, Pg=1, Zm=0
    let encoding: u32 = 0x04100400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p1/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_mul_z_p_zz_combo_6_0_04100000() {
    // Encoding: 0x04100000
    // Test MUL_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: size=0, Zm=0, Zdn=0, Pg=0
    let encoding: u32 = 0x04100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_mul_z_p_zz_combo_7_0_04100020() {
    // Encoding: 0x04100020
    // Test MUL_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=1, Zdn=0
    // Fields: size=0, Zm=1, Zdn=0, Pg=0
    let encoding: u32 = 0x04100020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_mul_z_p_zz_combo_8_0_041003c0() {
    // Encoding: 0x041003C0
    // Test MUL_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=30, Zdn=0
    // Fields: Zm=30, Pg=0, size=0, Zdn=0
    let encoding: u32 = 0x041003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_mul_z_p_zz_combo_9_0_041003e0() {
    // Encoding: 0x041003E0
    // Test MUL_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=31, Zdn=0
    // Fields: Zm=31, size=0, Pg=0, Zdn=0
    let encoding: u32 = 0x041003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=0 (minimum value)
#[test]
fn test_mul_z_p_zz_combo_10_0_04100000() {
    // Encoding: 0x04100000
    // Test MUL_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=0
    // Fields: Zdn=0, Zm=0, size=0, Pg=0
    let encoding: u32 = 0x04100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=1 (value 1)
#[test]
fn test_mul_z_p_zz_combo_11_0_04100001() {
    // Encoding: 0x04100001
    // Test MUL_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=1
    // Fields: Zm=0, size=0, Pg=0, Zdn=1
    let encoding: u32 = 0x04100001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z1.b, p0/m, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=15 (midpoint (15))
#[test]
fn test_mul_z_p_zz_combo_12_0_0410000f() {
    // Encoding: 0x0410000F
    // Test MUL_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=15
    // Fields: Zm=0, Zdn=15, size=0, Pg=0
    let encoding: u32 = 0x0410000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z15.b, p0/m, z15.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=31 (maximum value (31))
#[test]
fn test_mul_z_p_zz_combo_13_0_0410001f() {
    // Encoding: 0x0410001F
    // Test MUL_Z.P.ZZ__ field combination: size=0, Pg=0, Zm=0, Zdn=31
    // Fields: Zdn=31, Zm=0, size=0, Pg=0
    let encoding: u32 = 0x0410001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z31.b, p0/m, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_mul_z_p_zz_special_size_0_size_variant_0_0_04100000() {
    // Encoding: 0x04100000
    // Test MUL_Z.P.ZZ__ special value size = 0 (Size variant 0)
    // Fields: size=0, Zm=0, Pg=0, Zdn=0
    let encoding: u32 = 0x04100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.b, p0/m, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_mul_z_p_zz_special_size_1_size_variant_1_0_04500000() {
    // Encoding: 0x04500000
    // Test MUL_Z.P.ZZ__ special value size = 1 (Size variant 1)
    // Fields: Zm=0, size=1, Zdn=0, Pg=0
    let encoding: u32 = 0x04500000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.h, p0/m, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_mul_z_p_zz_special_size_2_size_variant_2_0_04900000() {
    // Encoding: 0x04900000
    // Test MUL_Z.P.ZZ__ special value size = 2 (Size variant 2)
    // Fields: Zdn=0, Zm=0, size=2, Pg=0
    let encoding: u32 = 0x04900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.s, p0/m, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: MUL_Z.P.ZZ__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_mul_z_p_zz_special_size_3_size_variant_3_0_04d00000() {
    // Encoding: 0x04D00000
    // Test MUL_Z.P.ZZ__ special value size = 3 (Size variant 3)
    // Fields: Zm=0, Pg=0, size=3, Zdn=0
    let encoding: u32 = 0x04D00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: mul	z0.d, p0/m, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// SUB_Z.ZI__ Tests
// ============================================================================

/// Provenance: SUB_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_sub_z_zi_field_size_0_min_c000_2521c000() {
    // Encoding: 0x2521C000
    // Test SUB_Z.ZI__ field size = 0 (Min)
    // Fields: sh=0, Zdn=0, size=0, imm8=0
    let encoding: u32 = 0x2521C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_sub_z_zi_field_size_1_poweroftwo_c000_2561c000() {
    // Encoding: 0x2561C000
    // Test SUB_Z.ZI__ field size = 1 (PowerOfTwo)
    // Fields: size=1, imm8=0, Zdn=0, sh=0
    let encoding: u32 = 0x2561C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_sub_z_zi_field_size_2_poweroftwo_c000_25a1c000() {
    // Encoding: 0x25A1C000
    // Test SUB_Z.ZI__ field size = 2 (PowerOfTwo)
    // Fields: Zdn=0, imm8=0, sh=0, size=2
    let encoding: u32 = 0x25A1C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_sub_z_zi_field_size_3_max_c000_25e1c000() {
    // Encoding: 0x25E1C000
    // Test SUB_Z.ZI__ field size = 3 (Max)
    // Fields: size=3, sh=0, Zdn=0, imm8=0
    let encoding: u32 = 0x25E1C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field sh 13 +: 1`
/// Requirement: FieldBoundary { field: "sh", value: 0, boundary: Min }
/// shift type LSL (logical shift left)
#[test]
fn test_sub_z_zi_field_sh_0_min_c000_2521c000() {
    // Encoding: 0x2521C000
    // Test SUB_Z.ZI__ field sh = 0 (Min)
    // Fields: size=0, sh=0, Zdn=0, imm8=0
    let encoding: u32 = 0x2521C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field sh 13 +: 1`
/// Requirement: FieldBoundary { field: "sh", value: 1, boundary: Max }
/// shift type LSR (logical shift right)
#[test]
fn test_sub_z_zi_field_sh_1_max_c000_2521e000() {
    // Encoding: 0x2521E000
    // Test SUB_Z.ZI__ field sh = 1 (Max)
    // Fields: Zdn=0, sh=1, size=0, imm8=0
    let encoding: u32 = 0x2521E000;
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

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_sub_z_zi_field_imm8_0_zero_c000_2521c000() {
    // Encoding: 0x2521C000
    // Test SUB_Z.ZI__ field imm8 = 0 (Zero)
    // Fields: Zdn=0, imm8=0, size=0, sh=0
    let encoding: u32 = 0x2521C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_sub_z_zi_field_imm8_1_poweroftwo_c000_2521c020() {
    // Encoding: 0x2521C020
    // Test SUB_Z.ZI__ field imm8 = 1 (PowerOfTwo)
    // Fields: size=0, Zdn=0, sh=0, imm8=1
    let encoding: u32 = 0x2521C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_sub_z_zi_field_imm8_3_poweroftwominusone_c000_2521c060() {
    // Encoding: 0x2521C060
    // Test SUB_Z.ZI__ field imm8 = 3 (PowerOfTwoMinusOne)
    // Fields: sh=0, Zdn=0, size=0, imm8=3
    let encoding: u32 = 0x2521C060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_sub_z_zi_field_imm8_4_poweroftwo_c000_2521c080() {
    // Encoding: 0x2521C080
    // Test SUB_Z.ZI__ field imm8 = 4 (PowerOfTwo)
    // Fields: Zdn=0, size=0, sh=0, imm8=4
    let encoding: u32 = 0x2521C080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_sub_z_zi_field_imm8_7_poweroftwominusone_c000_2521c0e0() {
    // Encoding: 0x2521C0E0
    // Test SUB_Z.ZI__ field imm8 = 7 (PowerOfTwoMinusOne)
    // Fields: sh=0, size=0, Zdn=0, imm8=7
    let encoding: u32 = 0x2521C0E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_sub_z_zi_field_imm8_8_poweroftwo_c000_2521c100() {
    // Encoding: 0x2521C100
    // Test SUB_Z.ZI__ field imm8 = 8 (PowerOfTwo)
    // Fields: size=0, imm8=8, sh=0, Zdn=0
    let encoding: u32 = 0x2521C100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_sub_z_zi_field_imm8_15_poweroftwominusone_c000_2521c1e0() {
    // Encoding: 0x2521C1E0
    // Test SUB_Z.ZI__ field imm8 = 15 (PowerOfTwoMinusOne)
    // Fields: Zdn=0, imm8=15, sh=0, size=0
    let encoding: u32 = 0x2521C1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_sub_z_zi_field_imm8_16_poweroftwo_c000_2521c200() {
    // Encoding: 0x2521C200
    // Test SUB_Z.ZI__ field imm8 = 16 (PowerOfTwo)
    // Fields: Zdn=0, size=0, sh=0, imm8=16
    let encoding: u32 = 0x2521C200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_sub_z_zi_field_imm8_31_poweroftwominusone_c000_2521c3e0() {
    // Encoding: 0x2521C3E0
    // Test SUB_Z.ZI__ field imm8 = 31 (PowerOfTwoMinusOne)
    // Fields: size=0, Zdn=0, imm8=31, sh=0
    let encoding: u32 = 0x2521C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_sub_z_zi_field_imm8_32_poweroftwo_c000_2521c400() {
    // Encoding: 0x2521C400
    // Test SUB_Z.ZI__ field imm8 = 32 (PowerOfTwo)
    // Fields: imm8=32, Zdn=0, sh=0, size=0
    let encoding: u32 = 0x2521C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_sub_z_zi_field_imm8_63_poweroftwominusone_c000_2521c7e0() {
    // Encoding: 0x2521C7E0
    // Test SUB_Z.ZI__ field imm8 = 63 (PowerOfTwoMinusOne)
    // Fields: size=0, imm8=63, sh=0, Zdn=0
    let encoding: u32 = 0x2521C7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_sub_z_zi_field_imm8_64_poweroftwo_c000_2521c800() {
    // Encoding: 0x2521C800
    // Test SUB_Z.ZI__ field imm8 = 64 (PowerOfTwo)
    // Fields: sh=0, Zdn=0, size=0, imm8=64
    let encoding: u32 = 0x2521C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_sub_z_zi_field_imm8_127_poweroftwominusone_c000_2521cfe0() {
    // Encoding: 0x2521CFE0
    // Test SUB_Z.ZI__ field imm8 = 127 (PowerOfTwoMinusOne)
    // Fields: size=0, imm8=127, sh=0, Zdn=0
    let encoding: u32 = 0x2521CFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_sub_z_zi_field_imm8_128_poweroftwo_c000_2521d000() {
    // Encoding: 0x2521D000
    // Test SUB_Z.ZI__ field imm8 = 128 (PowerOfTwo)
    // Fields: Zdn=0, size=0, sh=0, imm8=128
    let encoding: u32 = 0x2521D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_sub_z_zi_field_imm8_255_max_c000_2521dfe0() {
    // Encoding: 0x2521DFE0
    // Test SUB_Z.ZI__ field imm8 = 255 (Max)
    // Fields: sh=0, size=0, Zdn=0, imm8=255
    let encoding: u32 = 0x2521DFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_sub_z_zi_field_zdn_0_min_c000_2521c000() {
    // Encoding: 0x2521C000
    // Test SUB_Z.ZI__ field Zdn = 0 (Min)
    // Fields: imm8=0, sh=0, Zdn=0, size=0
    let encoding: u32 = 0x2521C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_sub_z_zi_field_zdn_1_poweroftwo_c000_2521c001() {
    // Encoding: 0x2521C001
    // Test SUB_Z.ZI__ field Zdn = 1 (PowerOfTwo)
    // Fields: sh=0, Zdn=1, imm8=0, size=0
    let encoding: u32 = 0x2521C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_sub_z_zi_field_zdn_15_poweroftwominusone_c000_2521c00f() {
    // Encoding: 0x2521C00F
    // Test SUB_Z.ZI__ field Zdn = 15 (PowerOfTwoMinusOne)
    // Fields: imm8=0, Zdn=15, size=0, sh=0
    let encoding: u32 = 0x2521C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_sub_z_zi_field_zdn_31_max_c000_2521c01f() {
    // Encoding: 0x2521C01F
    // Test SUB_Z.ZI__ field Zdn = 31 (Max)
    // Fields: size=0, imm8=0, Zdn=31, sh=0
    let encoding: u32 = 0x2521C01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_sub_z_zi_combo_0_c000_2521c000() {
    // Encoding: 0x2521C000
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=0
    // Fields: Zdn=0, sh=0, imm8=0, size=0
    let encoding: u32 = 0x2521C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_sub_z_zi_combo_1_c000_2561c000() {
    // Encoding: 0x2561C000
    // Test SUB_Z.ZI__ field combination: size=1, sh=0, imm8=0, Zdn=0
    // Fields: size=1, sh=0, imm8=0, Zdn=0
    let encoding: u32 = 0x2561C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_sub_z_zi_combo_2_c000_25a1c000() {
    // Encoding: 0x25A1C000
    // Test SUB_Z.ZI__ field combination: size=2, sh=0, imm8=0, Zdn=0
    // Fields: size=2, sh=0, imm8=0, Zdn=0
    let encoding: u32 = 0x25A1C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_sub_z_zi_combo_3_c000_25e1c000() {
    // Encoding: 0x25E1C000
    // Test SUB_Z.ZI__ field combination: size=3, sh=0, imm8=0, Zdn=0
    // Fields: size=3, Zdn=0, sh=0, imm8=0
    let encoding: u32 = 0x25E1C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sh=0 (shift type LSL (logical shift left))
#[test]
fn test_sub_z_zi_combo_4_c000_2521c000() {
    // Encoding: 0x2521C000
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=0
    // Fields: imm8=0, Zdn=0, sh=0, size=0
    let encoding: u32 = 0x2521C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sh=1 (shift type LSR (logical shift right))
#[test]
fn test_sub_z_zi_combo_5_c000_2521e000() {
    // Encoding: 0x2521E000
    // Test SUB_Z.ZI__ field combination: size=0, sh=1, imm8=0, Zdn=0
    // Fields: Zdn=0, imm8=0, sh=1, size=0
    let encoding: u32 = 0x2521E000;
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

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=0 (immediate value 0)
#[test]
fn test_sub_z_zi_combo_6_c000_2521c000() {
    // Encoding: 0x2521C000
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=0
    // Fields: Zdn=0, imm8=0, size=0, sh=0
    let encoding: u32 = 0x2521C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=1 (immediate value 1)
#[test]
fn test_sub_z_zi_combo_7_c000_2521c020() {
    // Encoding: 0x2521C020
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=1, Zdn=0
    // Fields: imm8=1, sh=0, size=0, Zdn=0
    let encoding: u32 = 0x2521C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=3 (2^2 - 1 = 3)
#[test]
fn test_sub_z_zi_combo_8_c000_2521c060() {
    // Encoding: 0x2521C060
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=3, Zdn=0
    // Fields: size=0, sh=0, Zdn=0, imm8=3
    let encoding: u32 = 0x2521C060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=4 (power of 2 (2^2 = 4))
#[test]
fn test_sub_z_zi_combo_9_c000_2521c080() {
    // Encoding: 0x2521C080
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=4, Zdn=0
    // Fields: sh=0, Zdn=0, size=0, imm8=4
    let encoding: u32 = 0x2521C080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=7 (2^3 - 1 = 7)
#[test]
fn test_sub_z_zi_combo_10_c000_2521c0e0() {
    // Encoding: 0x2521C0E0
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=7, Zdn=0
    // Fields: size=0, imm8=7, sh=0, Zdn=0
    let encoding: u32 = 0x2521C0E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=8 (power of 2 (2^3 = 8))
#[test]
fn test_sub_z_zi_combo_11_c000_2521c100() {
    // Encoding: 0x2521C100
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=8, Zdn=0
    // Fields: Zdn=0, size=0, sh=0, imm8=8
    let encoding: u32 = 0x2521C100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=15 (2^4 - 1 = 15)
#[test]
fn test_sub_z_zi_combo_12_c000_2521c1e0() {
    // Encoding: 0x2521C1E0
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=15, Zdn=0
    // Fields: size=0, sh=0, Zdn=0, imm8=15
    let encoding: u32 = 0x2521C1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=16 (power of 2 (2^4 = 16))
#[test]
fn test_sub_z_zi_combo_13_c000_2521c200() {
    // Encoding: 0x2521C200
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=16, Zdn=0
    // Fields: size=0, imm8=16, sh=0, Zdn=0
    let encoding: u32 = 0x2521C200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=31 (2^5 - 1 = 31)
#[test]
fn test_sub_z_zi_combo_14_c000_2521c3e0() {
    // Encoding: 0x2521C3E0
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=31, Zdn=0
    // Fields: size=0, sh=0, imm8=31, Zdn=0
    let encoding: u32 = 0x2521C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=32 (power of 2 (2^5 = 32))
#[test]
fn test_sub_z_zi_combo_15_c000_2521c400() {
    // Encoding: 0x2521C400
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=32, Zdn=0
    // Fields: size=0, sh=0, imm8=32, Zdn=0
    let encoding: u32 = 0x2521C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=63 (2^6 - 1 = 63)
#[test]
fn test_sub_z_zi_combo_16_c000_2521c7e0() {
    // Encoding: 0x2521C7E0
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=63, Zdn=0
    // Fields: sh=0, size=0, imm8=63, Zdn=0
    let encoding: u32 = 0x2521C7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=64 (power of 2 (2^6 = 64))
#[test]
fn test_sub_z_zi_combo_17_c000_2521c800() {
    // Encoding: 0x2521C800
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=64, Zdn=0
    // Fields: imm8=64, Zdn=0, size=0, sh=0
    let encoding: u32 = 0x2521C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=127 (immediate midpoint (127))
#[test]
fn test_sub_z_zi_combo_18_c000_2521cfe0() {
    // Encoding: 0x2521CFE0
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=127, Zdn=0
    // Fields: size=0, sh=0, imm8=127, Zdn=0
    let encoding: u32 = 0x2521CFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=128 (power of 2 (2^7 = 128))
#[test]
fn test_sub_z_zi_combo_19_c000_2521d000() {
    // Encoding: 0x2521D000
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=128, Zdn=0
    // Fields: size=0, imm8=128, Zdn=0, sh=0
    let encoding: u32 = 0x2521D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=255 (maximum immediate (255))
#[test]
fn test_sub_z_zi_combo_20_c000_2521dfe0() {
    // Encoding: 0x2521DFE0
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=255, Zdn=0
    // Fields: Zdn=0, size=0, sh=0, imm8=255
    let encoding: u32 = 0x2521DFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=0 (minimum value)
#[test]
fn test_sub_z_zi_combo_21_c000_2521c000() {
    // Encoding: 0x2521C000
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=0
    // Fields: sh=0, Zdn=0, imm8=0, size=0
    let encoding: u32 = 0x2521C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=1 (value 1)
#[test]
fn test_sub_z_zi_combo_22_c000_2521c001() {
    // Encoding: 0x2521C001
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=1
    // Fields: Zdn=1, sh=0, imm8=0, size=0
    let encoding: u32 = 0x2521C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=15 (midpoint (15))
#[test]
fn test_sub_z_zi_combo_23_c000_2521c00f() {
    // Encoding: 0x2521C00F
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=15
    // Fields: size=0, sh=0, imm8=0, Zdn=15
    let encoding: u32 = 0x2521C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=31 (maximum value (31))
#[test]
fn test_sub_z_zi_combo_24_c000_2521c01f() {
    // Encoding: 0x2521C01F
    // Test SUB_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=31
    // Fields: imm8=0, Zdn=31, size=0, sh=0
    let encoding: u32 = 0x2521C01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_sub_z_zi_special_size_0_size_variant_0_49152_2521c020() {
    // Encoding: 0x2521C020
    // Test SUB_Z.ZI__ special value size = 0 (Size variant 0)
    // Fields: size=0, sh=0, Zdn=0, imm8=1
    let encoding: u32 = 0x2521C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_sub_z_zi_special_size_1_size_variant_1_49152_2561c020() {
    // Encoding: 0x2561C020
    // Test SUB_Z.ZI__ special value size = 1 (Size variant 1)
    // Fields: imm8=1, sh=0, size=1, Zdn=0
    let encoding: u32 = 0x2561C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_sub_z_zi_special_size_2_size_variant_2_49152_25a1c020() {
    // Encoding: 0x25A1C020
    // Test SUB_Z.ZI__ special value size = 2 (Size variant 2)
    // Fields: size=2, sh=0, Zdn=0, imm8=1
    let encoding: u32 = 0x25A1C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_sub_z_zi_special_size_3_size_variant_3_49152_25e1c020() {
    // Encoding: 0x25E1C020
    // Test SUB_Z.ZI__ special value size = 3 (Size variant 3)
    // Fields: Zdn=0, imm8=1, sh=0, size=3
    let encoding: u32 = 0x25E1C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field sh = 0 (Shift type LSL)`
/// Requirement: FieldSpecial { field: "sh", value: 0, meaning: "Shift type LSL" }
/// Shift type LSL
#[test]
fn test_sub_z_zi_special_sh_0_shift_type_lsl_49152_2561c020() {
    // Encoding: 0x2561C020
    // Test SUB_Z.ZI__ special value sh = 0 (Shift type LSL)
    // Fields: imm8=1, Zdn=0, sh=0, size=1
    let encoding: u32 = 0x2561C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field sh = 1 (Shift type LSR)`
/// Requirement: FieldSpecial { field: "sh", value: 1, meaning: "Shift type LSR" }
/// Shift type LSR
#[test]
fn test_sub_z_zi_special_sh_1_shift_type_lsr_49152_2561e020() {
    // Encoding: 0x2561E020
    // Test SUB_Z.ZI__ special value sh = 1 (Shift type LSR)
    // Fields: imm8=1, size=1, sh=1, Zdn=0
    let encoding: u32 = 0x2561E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.h, z0.h, #256                // =0x100
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field sh = 2 (Shift type ASR)`
/// Requirement: FieldSpecial { field: "sh", value: 2, meaning: "Shift type ASR" }
/// Shift type ASR
#[test]
fn test_sub_z_zi_special_sh_2_shift_type_asr_49152_2561c020() {
    // Encoding: 0x2561C020
    // Test SUB_Z.ZI__ special value sh = 2 (Shift type ASR)
    // Fields: Zdn=0, size=1, imm8=1, sh=2
    let encoding: u32 = 0x2561C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUB_Z.ZI__
/// ASL: `field sh = 3 (Shift type ROR)`
/// Requirement: FieldSpecial { field: "sh", value: 3, meaning: "Shift type ROR" }
/// Shift type ROR
#[test]
fn test_sub_z_zi_special_sh_3_shift_type_ror_49152_2561e020() {
    // Encoding: 0x2561E020
    // Test SUB_Z.ZI__ special value sh = 3 (Shift type ROR)
    // Fields: size=1, imm8=1, sh=3, Zdn=0
    let encoding: u32 = 0x2561E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: sub	z0.h, z0.h, #256                // =0x100
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// ADD_Z.ZI__ Tests
// ============================================================================

/// Provenance: ADD_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_add_z_zi_field_size_0_min_c000_2520c000() {
    // Encoding: 0x2520C000
    // Test ADD_Z.ZI__ field size = 0 (Min)
    // Fields: imm8=0, sh=0, Zdn=0, size=0
    let encoding: u32 = 0x2520C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_add_z_zi_field_size_1_poweroftwo_c000_2560c000() {
    // Encoding: 0x2560C000
    // Test ADD_Z.ZI__ field size = 1 (PowerOfTwo)
    // Fields: imm8=0, Zdn=0, size=1, sh=0
    let encoding: u32 = 0x2560C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_add_z_zi_field_size_2_poweroftwo_c000_25a0c000() {
    // Encoding: 0x25A0C000
    // Test ADD_Z.ZI__ field size = 2 (PowerOfTwo)
    // Fields: sh=0, imm8=0, size=2, Zdn=0
    let encoding: u32 = 0x25A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_add_z_zi_field_size_3_max_c000_25e0c000() {
    // Encoding: 0x25E0C000
    // Test ADD_Z.ZI__ field size = 3 (Max)
    // Fields: sh=0, size=3, Zdn=0, imm8=0
    let encoding: u32 = 0x25E0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field sh 13 +: 1`
/// Requirement: FieldBoundary { field: "sh", value: 0, boundary: Min }
/// shift type LSL (logical shift left)
#[test]
fn test_add_z_zi_field_sh_0_min_c000_2520c000() {
    // Encoding: 0x2520C000
    // Test ADD_Z.ZI__ field sh = 0 (Min)
    // Fields: size=0, sh=0, Zdn=0, imm8=0
    let encoding: u32 = 0x2520C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field sh 13 +: 1`
/// Requirement: FieldBoundary { field: "sh", value: 1, boundary: Max }
/// shift type LSR (logical shift right)
#[test]
fn test_add_z_zi_field_sh_1_max_c000_2520e000() {
    // Encoding: 0x2520E000
    // Test ADD_Z.ZI__ field sh = 1 (Max)
    // Fields: size=0, imm8=0, sh=1, Zdn=0
    let encoding: u32 = 0x2520E000;
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

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_add_z_zi_field_imm8_0_zero_c000_2520c000() {
    // Encoding: 0x2520C000
    // Test ADD_Z.ZI__ field imm8 = 0 (Zero)
    // Fields: imm8=0, Zdn=0, size=0, sh=0
    let encoding: u32 = 0x2520C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_add_z_zi_field_imm8_1_poweroftwo_c000_2520c020() {
    // Encoding: 0x2520C020
    // Test ADD_Z.ZI__ field imm8 = 1 (PowerOfTwo)
    // Fields: imm8=1, size=0, Zdn=0, sh=0
    let encoding: u32 = 0x2520C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_add_z_zi_field_imm8_3_poweroftwominusone_c000_2520c060() {
    // Encoding: 0x2520C060
    // Test ADD_Z.ZI__ field imm8 = 3 (PowerOfTwoMinusOne)
    // Fields: imm8=3, sh=0, size=0, Zdn=0
    let encoding: u32 = 0x2520C060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_add_z_zi_field_imm8_4_poweroftwo_c000_2520c080() {
    // Encoding: 0x2520C080
    // Test ADD_Z.ZI__ field imm8 = 4 (PowerOfTwo)
    // Fields: size=0, imm8=4, sh=0, Zdn=0
    let encoding: u32 = 0x2520C080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_add_z_zi_field_imm8_7_poweroftwominusone_c000_2520c0e0() {
    // Encoding: 0x2520C0E0
    // Test ADD_Z.ZI__ field imm8 = 7 (PowerOfTwoMinusOne)
    // Fields: Zdn=0, size=0, imm8=7, sh=0
    let encoding: u32 = 0x2520C0E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_add_z_zi_field_imm8_8_poweroftwo_c000_2520c100() {
    // Encoding: 0x2520C100
    // Test ADD_Z.ZI__ field imm8 = 8 (PowerOfTwo)
    // Fields: size=0, imm8=8, Zdn=0, sh=0
    let encoding: u32 = 0x2520C100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_add_z_zi_field_imm8_15_poweroftwominusone_c000_2520c1e0() {
    // Encoding: 0x2520C1E0
    // Test ADD_Z.ZI__ field imm8 = 15 (PowerOfTwoMinusOne)
    // Fields: size=0, sh=0, imm8=15, Zdn=0
    let encoding: u32 = 0x2520C1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_add_z_zi_field_imm8_16_poweroftwo_c000_2520c200() {
    // Encoding: 0x2520C200
    // Test ADD_Z.ZI__ field imm8 = 16 (PowerOfTwo)
    // Fields: size=0, Zdn=0, sh=0, imm8=16
    let encoding: u32 = 0x2520C200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_add_z_zi_field_imm8_31_poweroftwominusone_c000_2520c3e0() {
    // Encoding: 0x2520C3E0
    // Test ADD_Z.ZI__ field imm8 = 31 (PowerOfTwoMinusOne)
    // Fields: Zdn=0, size=0, imm8=31, sh=0
    let encoding: u32 = 0x2520C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_add_z_zi_field_imm8_32_poweroftwo_c000_2520c400() {
    // Encoding: 0x2520C400
    // Test ADD_Z.ZI__ field imm8 = 32 (PowerOfTwo)
    // Fields: sh=0, size=0, imm8=32, Zdn=0
    let encoding: u32 = 0x2520C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_add_z_zi_field_imm8_63_poweroftwominusone_c000_2520c7e0() {
    // Encoding: 0x2520C7E0
    // Test ADD_Z.ZI__ field imm8 = 63 (PowerOfTwoMinusOne)
    // Fields: size=0, Zdn=0, sh=0, imm8=63
    let encoding: u32 = 0x2520C7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_add_z_zi_field_imm8_64_poweroftwo_c000_2520c800() {
    // Encoding: 0x2520C800
    // Test ADD_Z.ZI__ field imm8 = 64 (PowerOfTwo)
    // Fields: imm8=64, sh=0, Zdn=0, size=0
    let encoding: u32 = 0x2520C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_add_z_zi_field_imm8_127_poweroftwominusone_c000_2520cfe0() {
    // Encoding: 0x2520CFE0
    // Test ADD_Z.ZI__ field imm8 = 127 (PowerOfTwoMinusOne)
    // Fields: Zdn=0, size=0, sh=0, imm8=127
    let encoding: u32 = 0x2520CFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_add_z_zi_field_imm8_128_poweroftwo_c000_2520d000() {
    // Encoding: 0x2520D000
    // Test ADD_Z.ZI__ field imm8 = 128 (PowerOfTwo)
    // Fields: sh=0, imm8=128, Zdn=0, size=0
    let encoding: u32 = 0x2520D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_add_z_zi_field_imm8_255_max_c000_2520dfe0() {
    // Encoding: 0x2520DFE0
    // Test ADD_Z.ZI__ field imm8 = 255 (Max)
    // Fields: imm8=255, Zdn=0, sh=0, size=0
    let encoding: u32 = 0x2520DFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_add_z_zi_field_zdn_0_min_c000_2520c000() {
    // Encoding: 0x2520C000
    // Test ADD_Z.ZI__ field Zdn = 0 (Min)
    // Fields: Zdn=0, sh=0, size=0, imm8=0
    let encoding: u32 = 0x2520C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_add_z_zi_field_zdn_1_poweroftwo_c000_2520c001() {
    // Encoding: 0x2520C001
    // Test ADD_Z.ZI__ field Zdn = 1 (PowerOfTwo)
    // Fields: size=0, sh=0, Zdn=1, imm8=0
    let encoding: u32 = 0x2520C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_add_z_zi_field_zdn_15_poweroftwominusone_c000_2520c00f() {
    // Encoding: 0x2520C00F
    // Test ADD_Z.ZI__ field Zdn = 15 (PowerOfTwoMinusOne)
    // Fields: Zdn=15, size=0, imm8=0, sh=0
    let encoding: u32 = 0x2520C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_add_z_zi_field_zdn_31_max_c000_2520c01f() {
    // Encoding: 0x2520C01F
    // Test ADD_Z.ZI__ field Zdn = 31 (Max)
    // Fields: size=0, sh=0, Zdn=31, imm8=0
    let encoding: u32 = 0x2520C01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_add_z_zi_combo_0_c000_2520c000() {
    // Encoding: 0x2520C000
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=0
    // Fields: size=0, Zdn=0, sh=0, imm8=0
    let encoding: u32 = 0x2520C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_add_z_zi_combo_1_c000_2560c000() {
    // Encoding: 0x2560C000
    // Test ADD_Z.ZI__ field combination: size=1, sh=0, imm8=0, Zdn=0
    // Fields: Zdn=0, imm8=0, sh=0, size=1
    let encoding: u32 = 0x2560C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_add_z_zi_combo_2_c000_25a0c000() {
    // Encoding: 0x25A0C000
    // Test ADD_Z.ZI__ field combination: size=2, sh=0, imm8=0, Zdn=0
    // Fields: size=2, Zdn=0, sh=0, imm8=0
    let encoding: u32 = 0x25A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_add_z_zi_combo_3_c000_25e0c000() {
    // Encoding: 0x25E0C000
    // Test ADD_Z.ZI__ field combination: size=3, sh=0, imm8=0, Zdn=0
    // Fields: Zdn=0, sh=0, size=3, imm8=0
    let encoding: u32 = 0x25E0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sh=0 (shift type LSL (logical shift left))
#[test]
fn test_add_z_zi_combo_4_c000_2520c000() {
    // Encoding: 0x2520C000
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=0
    // Fields: Zdn=0, size=0, sh=0, imm8=0
    let encoding: u32 = 0x2520C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sh=1 (shift type LSR (logical shift right))
#[test]
fn test_add_z_zi_combo_5_c000_2520e000() {
    // Encoding: 0x2520E000
    // Test ADD_Z.ZI__ field combination: size=0, sh=1, imm8=0, Zdn=0
    // Fields: imm8=0, size=0, sh=1, Zdn=0
    let encoding: u32 = 0x2520E000;
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

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=0 (immediate value 0)
#[test]
fn test_add_z_zi_combo_6_c000_2520c000() {
    // Encoding: 0x2520C000
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=0
    // Fields: sh=0, imm8=0, size=0, Zdn=0
    let encoding: u32 = 0x2520C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=1 (immediate value 1)
#[test]
fn test_add_z_zi_combo_7_c000_2520c020() {
    // Encoding: 0x2520C020
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=1, Zdn=0
    // Fields: Zdn=0, sh=0, size=0, imm8=1
    let encoding: u32 = 0x2520C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=3 (2^2 - 1 = 3)
#[test]
fn test_add_z_zi_combo_8_c000_2520c060() {
    // Encoding: 0x2520C060
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=3, Zdn=0
    // Fields: imm8=3, size=0, Zdn=0, sh=0
    let encoding: u32 = 0x2520C060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=4 (power of 2 (2^2 = 4))
#[test]
fn test_add_z_zi_combo_9_c000_2520c080() {
    // Encoding: 0x2520C080
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=4, Zdn=0
    // Fields: sh=0, imm8=4, Zdn=0, size=0
    let encoding: u32 = 0x2520C080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=7 (2^3 - 1 = 7)
#[test]
fn test_add_z_zi_combo_10_c000_2520c0e0() {
    // Encoding: 0x2520C0E0
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=7, Zdn=0
    // Fields: Zdn=0, sh=0, size=0, imm8=7
    let encoding: u32 = 0x2520C0E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=8 (power of 2 (2^3 = 8))
#[test]
fn test_add_z_zi_combo_11_c000_2520c100() {
    // Encoding: 0x2520C100
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=8, Zdn=0
    // Fields: imm8=8, Zdn=0, sh=0, size=0
    let encoding: u32 = 0x2520C100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=15 (2^4 - 1 = 15)
#[test]
fn test_add_z_zi_combo_12_c000_2520c1e0() {
    // Encoding: 0x2520C1E0
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=15, Zdn=0
    // Fields: sh=0, size=0, Zdn=0, imm8=15
    let encoding: u32 = 0x2520C1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=16 (power of 2 (2^4 = 16))
#[test]
fn test_add_z_zi_combo_13_c000_2520c200() {
    // Encoding: 0x2520C200
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=16, Zdn=0
    // Fields: sh=0, size=0, imm8=16, Zdn=0
    let encoding: u32 = 0x2520C200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=31 (2^5 - 1 = 31)
#[test]
fn test_add_z_zi_combo_14_c000_2520c3e0() {
    // Encoding: 0x2520C3E0
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=31, Zdn=0
    // Fields: size=0, Zdn=0, sh=0, imm8=31
    let encoding: u32 = 0x2520C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=32 (power of 2 (2^5 = 32))
#[test]
fn test_add_z_zi_combo_15_c000_2520c400() {
    // Encoding: 0x2520C400
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=32, Zdn=0
    // Fields: imm8=32, size=0, sh=0, Zdn=0
    let encoding: u32 = 0x2520C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=63 (2^6 - 1 = 63)
#[test]
fn test_add_z_zi_combo_16_c000_2520c7e0() {
    // Encoding: 0x2520C7E0
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=63, Zdn=0
    // Fields: imm8=63, size=0, sh=0, Zdn=0
    let encoding: u32 = 0x2520C7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=64 (power of 2 (2^6 = 64))
#[test]
fn test_add_z_zi_combo_17_c000_2520c800() {
    // Encoding: 0x2520C800
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=64, Zdn=0
    // Fields: imm8=64, Zdn=0, size=0, sh=0
    let encoding: u32 = 0x2520C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=127 (immediate midpoint (127))
#[test]
fn test_add_z_zi_combo_18_c000_2520cfe0() {
    // Encoding: 0x2520CFE0
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=127, Zdn=0
    // Fields: imm8=127, Zdn=0, size=0, sh=0
    let encoding: u32 = 0x2520CFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=128 (power of 2 (2^7 = 128))
#[test]
fn test_add_z_zi_combo_19_c000_2520d000() {
    // Encoding: 0x2520D000
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=128, Zdn=0
    // Fields: imm8=128, size=0, Zdn=0, sh=0
    let encoding: u32 = 0x2520D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=255 (maximum immediate (255))
#[test]
fn test_add_z_zi_combo_20_c000_2520dfe0() {
    // Encoding: 0x2520DFE0
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=255, Zdn=0
    // Fields: sh=0, imm8=255, size=0, Zdn=0
    let encoding: u32 = 0x2520DFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=0 (minimum value)
#[test]
fn test_add_z_zi_combo_21_c000_2520c000() {
    // Encoding: 0x2520C000
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=0
    // Fields: size=0, Zdn=0, sh=0, imm8=0
    let encoding: u32 = 0x2520C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=1 (value 1)
#[test]
fn test_add_z_zi_combo_22_c000_2520c001() {
    // Encoding: 0x2520C001
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=1
    // Fields: sh=0, imm8=0, Zdn=1, size=0
    let encoding: u32 = 0x2520C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=15 (midpoint (15))
#[test]
fn test_add_z_zi_combo_23_c000_2520c00f() {
    // Encoding: 0x2520C00F
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=15
    // Fields: sh=0, imm8=0, size=0, Zdn=15
    let encoding: u32 = 0x2520C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=31 (maximum value (31))
#[test]
fn test_add_z_zi_combo_24_c000_2520c01f() {
    // Encoding: 0x2520C01F
    // Test ADD_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=31
    // Fields: imm8=0, sh=0, size=0, Zdn=31
    let encoding: u32 = 0x2520C01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_add_z_zi_special_size_0_size_variant_0_49152_2520c020() {
    // Encoding: 0x2520C020
    // Test ADD_Z.ZI__ special value size = 0 (Size variant 0)
    // Fields: sh=0, Zdn=0, size=0, imm8=1
    let encoding: u32 = 0x2520C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_add_z_zi_special_size_1_size_variant_1_49152_2560c020() {
    // Encoding: 0x2560C020
    // Test ADD_Z.ZI__ special value size = 1 (Size variant 1)
    // Fields: Zdn=0, sh=0, size=1, imm8=1
    let encoding: u32 = 0x2560C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_add_z_zi_special_size_2_size_variant_2_49152_25a0c020() {
    // Encoding: 0x25A0C020
    // Test ADD_Z.ZI__ special value size = 2 (Size variant 2)
    // Fields: Zdn=0, size=2, sh=0, imm8=1
    let encoding: u32 = 0x25A0C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_add_z_zi_special_size_3_size_variant_3_49152_25e0c020() {
    // Encoding: 0x25E0C020
    // Test ADD_Z.ZI__ special value size = 3 (Size variant 3)
    // Fields: sh=0, size=3, Zdn=0, imm8=1
    let encoding: u32 = 0x25E0C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field sh = 0 (Shift type LSL)`
/// Requirement: FieldSpecial { field: "sh", value: 0, meaning: "Shift type LSL" }
/// Shift type LSL
#[test]
fn test_add_z_zi_special_sh_0_shift_type_lsl_49152_2560c020() {
    // Encoding: 0x2560C020
    // Test ADD_Z.ZI__ special value sh = 0 (Shift type LSL)
    // Fields: sh=0, imm8=1, size=1, Zdn=0
    let encoding: u32 = 0x2560C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field sh = 1 (Shift type LSR)`
/// Requirement: FieldSpecial { field: "sh", value: 1, meaning: "Shift type LSR" }
/// Shift type LSR
#[test]
fn test_add_z_zi_special_sh_1_shift_type_lsr_49152_2560e020() {
    // Encoding: 0x2560E020
    // Test ADD_Z.ZI__ special value sh = 1 (Shift type LSR)
    // Fields: size=1, imm8=1, Zdn=0, sh=1
    let encoding: u32 = 0x2560E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.h, z0.h, #256                // =0x100
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field sh = 2 (Shift type ASR)`
/// Requirement: FieldSpecial { field: "sh", value: 2, meaning: "Shift type ASR" }
/// Shift type ASR
#[test]
fn test_add_z_zi_special_sh_2_shift_type_asr_49152_2560c020() {
    // Encoding: 0x2560C020
    // Test ADD_Z.ZI__ special value sh = 2 (Shift type ASR)
    // Fields: Zdn=0, sh=2, size=1, imm8=1
    let encoding: u32 = 0x2560C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: ADD_Z.ZI__
/// ASL: `field sh = 3 (Shift type ROR)`
/// Requirement: FieldSpecial { field: "sh", value: 3, meaning: "Shift type ROR" }
/// Shift type ROR
#[test]
fn test_add_z_zi_special_sh_3_shift_type_ror_49152_2560e020() {
    // Encoding: 0x2560E020
    // Test ADD_Z.ZI__ special value sh = 3 (Shift type ROR)
    // Fields: sh=3, Zdn=0, imm8=1, size=1
    let encoding: u32 = 0x2560E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.h, z0.h, #256                // =0x100
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// SUBR_Z.ZI__ Tests
// ============================================================================

/// Provenance: SUBR_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_subr_z_zi_field_size_0_min_c000_2523c000() {
    // Encoding: 0x2523C000
    // Test SUBR_Z.ZI__ field size = 0 (Min)
    // Fields: Zdn=0, size=0, sh=0, imm8=0
    let encoding: u32 = 0x2523C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_subr_z_zi_field_size_1_poweroftwo_c000_2563c000() {
    // Encoding: 0x2563C000
    // Test SUBR_Z.ZI__ field size = 1 (PowerOfTwo)
    // Fields: size=1, sh=0, Zdn=0, imm8=0
    let encoding: u32 = 0x2563C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_subr_z_zi_field_size_2_poweroftwo_c000_25a3c000() {
    // Encoding: 0x25A3C000
    // Test SUBR_Z.ZI__ field size = 2 (PowerOfTwo)
    // Fields: imm8=0, Zdn=0, size=2, sh=0
    let encoding: u32 = 0x25A3C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_subr_z_zi_field_size_3_max_c000_25e3c000() {
    // Encoding: 0x25E3C000
    // Test SUBR_Z.ZI__ field size = 3 (Max)
    // Fields: size=3, sh=0, Zdn=0, imm8=0
    let encoding: u32 = 0x25E3C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field sh 13 +: 1`
/// Requirement: FieldBoundary { field: "sh", value: 0, boundary: Min }
/// shift type LSL (logical shift left)
#[test]
fn test_subr_z_zi_field_sh_0_min_c000_2523c000() {
    // Encoding: 0x2523C000
    // Test SUBR_Z.ZI__ field sh = 0 (Min)
    // Fields: size=0, Zdn=0, imm8=0, sh=0
    let encoding: u32 = 0x2523C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field sh 13 +: 1`
/// Requirement: FieldBoundary { field: "sh", value: 1, boundary: Max }
/// shift type LSR (logical shift right)
#[test]
fn test_subr_z_zi_field_sh_1_max_c000_2523e000() {
    // Encoding: 0x2523E000
    // Test SUBR_Z.ZI__ field sh = 1 (Max)
    // Fields: Zdn=0, imm8=0, size=0, sh=1
    let encoding: u32 = 0x2523E000;
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

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_subr_z_zi_field_imm8_0_zero_c000_2523c000() {
    // Encoding: 0x2523C000
    // Test SUBR_Z.ZI__ field imm8 = 0 (Zero)
    // Fields: sh=0, size=0, imm8=0, Zdn=0
    let encoding: u32 = 0x2523C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_subr_z_zi_field_imm8_1_poweroftwo_c000_2523c020() {
    // Encoding: 0x2523C020
    // Test SUBR_Z.ZI__ field imm8 = 1 (PowerOfTwo)
    // Fields: imm8=1, Zdn=0, size=0, sh=0
    let encoding: u32 = 0x2523C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_subr_z_zi_field_imm8_3_poweroftwominusone_c000_2523c060() {
    // Encoding: 0x2523C060
    // Test SUBR_Z.ZI__ field imm8 = 3 (PowerOfTwoMinusOne)
    // Fields: imm8=3, sh=0, Zdn=0, size=0
    let encoding: u32 = 0x2523C060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_subr_z_zi_field_imm8_4_poweroftwo_c000_2523c080() {
    // Encoding: 0x2523C080
    // Test SUBR_Z.ZI__ field imm8 = 4 (PowerOfTwo)
    // Fields: sh=0, imm8=4, size=0, Zdn=0
    let encoding: u32 = 0x2523C080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_subr_z_zi_field_imm8_7_poweroftwominusone_c000_2523c0e0() {
    // Encoding: 0x2523C0E0
    // Test SUBR_Z.ZI__ field imm8 = 7 (PowerOfTwoMinusOne)
    // Fields: sh=0, size=0, imm8=7, Zdn=0
    let encoding: u32 = 0x2523C0E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_subr_z_zi_field_imm8_8_poweroftwo_c000_2523c100() {
    // Encoding: 0x2523C100
    // Test SUBR_Z.ZI__ field imm8 = 8 (PowerOfTwo)
    // Fields: imm8=8, sh=0, size=0, Zdn=0
    let encoding: u32 = 0x2523C100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_subr_z_zi_field_imm8_15_poweroftwominusone_c000_2523c1e0() {
    // Encoding: 0x2523C1E0
    // Test SUBR_Z.ZI__ field imm8 = 15 (PowerOfTwoMinusOne)
    // Fields: imm8=15, size=0, Zdn=0, sh=0
    let encoding: u32 = 0x2523C1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_subr_z_zi_field_imm8_16_poweroftwo_c000_2523c200() {
    // Encoding: 0x2523C200
    // Test SUBR_Z.ZI__ field imm8 = 16 (PowerOfTwo)
    // Fields: imm8=16, sh=0, size=0, Zdn=0
    let encoding: u32 = 0x2523C200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_subr_z_zi_field_imm8_31_poweroftwominusone_c000_2523c3e0() {
    // Encoding: 0x2523C3E0
    // Test SUBR_Z.ZI__ field imm8 = 31 (PowerOfTwoMinusOne)
    // Fields: Zdn=0, imm8=31, sh=0, size=0
    let encoding: u32 = 0x2523C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_subr_z_zi_field_imm8_32_poweroftwo_c000_2523c400() {
    // Encoding: 0x2523C400
    // Test SUBR_Z.ZI__ field imm8 = 32 (PowerOfTwo)
    // Fields: Zdn=0, size=0, imm8=32, sh=0
    let encoding: u32 = 0x2523C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_subr_z_zi_field_imm8_63_poweroftwominusone_c000_2523c7e0() {
    // Encoding: 0x2523C7E0
    // Test SUBR_Z.ZI__ field imm8 = 63 (PowerOfTwoMinusOne)
    // Fields: sh=0, Zdn=0, imm8=63, size=0
    let encoding: u32 = 0x2523C7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_subr_z_zi_field_imm8_64_poweroftwo_c000_2523c800() {
    // Encoding: 0x2523C800
    // Test SUBR_Z.ZI__ field imm8 = 64 (PowerOfTwo)
    // Fields: size=0, imm8=64, Zdn=0, sh=0
    let encoding: u32 = 0x2523C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_subr_z_zi_field_imm8_127_poweroftwominusone_c000_2523cfe0() {
    // Encoding: 0x2523CFE0
    // Test SUBR_Z.ZI__ field imm8 = 127 (PowerOfTwoMinusOne)
    // Fields: sh=0, imm8=127, size=0, Zdn=0
    let encoding: u32 = 0x2523CFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_subr_z_zi_field_imm8_128_poweroftwo_c000_2523d000() {
    // Encoding: 0x2523D000
    // Test SUBR_Z.ZI__ field imm8 = 128 (PowerOfTwo)
    // Fields: imm8=128, size=0, Zdn=0, sh=0
    let encoding: u32 = 0x2523D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field imm8 5 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_subr_z_zi_field_imm8_255_max_c000_2523dfe0() {
    // Encoding: 0x2523DFE0
    // Test SUBR_Z.ZI__ field imm8 = 255 (Max)
    // Fields: size=0, imm8=255, sh=0, Zdn=0
    let encoding: u32 = 0x2523DFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_subr_z_zi_field_zdn_0_min_c000_2523c000() {
    // Encoding: 0x2523C000
    // Test SUBR_Z.ZI__ field Zdn = 0 (Min)
    // Fields: Zdn=0, imm8=0, size=0, sh=0
    let encoding: u32 = 0x2523C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_subr_z_zi_field_zdn_1_poweroftwo_c000_2523c001() {
    // Encoding: 0x2523C001
    // Test SUBR_Z.ZI__ field Zdn = 1 (PowerOfTwo)
    // Fields: imm8=0, size=0, Zdn=1, sh=0
    let encoding: u32 = 0x2523C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 15, boundary: PowerOfTwoMinusOne }
/// midpoint (15)
#[test]
fn test_subr_z_zi_field_zdn_15_poweroftwominusone_c000_2523c00f() {
    // Encoding: 0x2523C00F
    // Test SUBR_Z.ZI__ field Zdn = 15 (PowerOfTwoMinusOne)
    // Fields: size=0, sh=0, Zdn=15, imm8=0
    let encoding: u32 = 0x2523C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field Zdn 0 +: 5`
/// Requirement: FieldBoundary { field: "Zdn", value: 31, boundary: Max }
/// maximum value (31)
#[test]
fn test_subr_z_zi_field_zdn_31_max_c000_2523c01f() {
    // Encoding: 0x2523C01F
    // Test SUBR_Z.ZI__ field Zdn = 31 (Max)
    // Fields: imm8=0, Zdn=31, sh=0, size=0
    let encoding: u32 = 0x2523C01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_subr_z_zi_combo_0_c000_2523c000() {
    // Encoding: 0x2523C000
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=0
    // Fields: imm8=0, Zdn=0, size=0, sh=0
    let encoding: u32 = 0x2523C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_subr_z_zi_combo_1_c000_2563c000() {
    // Encoding: 0x2563C000
    // Test SUBR_Z.ZI__ field combination: size=1, sh=0, imm8=0, Zdn=0
    // Fields: Zdn=0, imm8=0, sh=0, size=1
    let encoding: u32 = 0x2563C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_subr_z_zi_combo_2_c000_25a3c000() {
    // Encoding: 0x25A3C000
    // Test SUBR_Z.ZI__ field combination: size=2, sh=0, imm8=0, Zdn=0
    // Fields: sh=0, imm8=0, Zdn=0, size=2
    let encoding: u32 = 0x25A3C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_subr_z_zi_combo_3_c000_25e3c000() {
    // Encoding: 0x25E3C000
    // Test SUBR_Z.ZI__ field combination: size=3, sh=0, imm8=0, Zdn=0
    // Fields: imm8=0, sh=0, size=3, Zdn=0
    let encoding: u32 = 0x25E3C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sh=0 (shift type LSL (logical shift left))
#[test]
fn test_subr_z_zi_combo_4_c000_2523c000() {
    // Encoding: 0x2523C000
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=0
    // Fields: sh=0, size=0, imm8=0, Zdn=0
    let encoding: u32 = 0x2523C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sh=1 (shift type LSR (logical shift right))
#[test]
fn test_subr_z_zi_combo_5_c000_2523e000() {
    // Encoding: 0x2523E000
    // Test SUBR_Z.ZI__ field combination: size=0, sh=1, imm8=0, Zdn=0
    // Fields: Zdn=0, imm8=0, sh=1, size=0
    let encoding: u32 = 0x2523E000;
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

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=0 (immediate value 0)
#[test]
fn test_subr_z_zi_combo_6_c000_2523c000() {
    // Encoding: 0x2523C000
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=0
    // Fields: size=0, sh=0, imm8=0, Zdn=0
    let encoding: u32 = 0x2523C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=1 (immediate value 1)
#[test]
fn test_subr_z_zi_combo_7_c000_2523c020() {
    // Encoding: 0x2523C020
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=1, Zdn=0
    // Fields: size=0, sh=0, imm8=1, Zdn=0
    let encoding: u32 = 0x2523C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=3 (2^2 - 1 = 3)
#[test]
fn test_subr_z_zi_combo_8_c000_2523c060() {
    // Encoding: 0x2523C060
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=3, Zdn=0
    // Fields: imm8=3, size=0, Zdn=0, sh=0
    let encoding: u32 = 0x2523C060;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=4 (power of 2 (2^2 = 4))
#[test]
fn test_subr_z_zi_combo_9_c000_2523c080() {
    // Encoding: 0x2523C080
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=4, Zdn=0
    // Fields: Zdn=0, imm8=4, size=0, sh=0
    let encoding: u32 = 0x2523C080;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=7 (2^3 - 1 = 7)
#[test]
fn test_subr_z_zi_combo_10_c000_2523c0e0() {
    // Encoding: 0x2523C0E0
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=7, Zdn=0
    // Fields: imm8=7, Zdn=0, size=0, sh=0
    let encoding: u32 = 0x2523C0E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=8 (power of 2 (2^3 = 8))
#[test]
fn test_subr_z_zi_combo_11_c000_2523c100() {
    // Encoding: 0x2523C100
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=8, Zdn=0
    // Fields: size=0, sh=0, Zdn=0, imm8=8
    let encoding: u32 = 0x2523C100;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=15 (2^4 - 1 = 15)
#[test]
fn test_subr_z_zi_combo_12_c000_2523c1e0() {
    // Encoding: 0x2523C1E0
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=15, Zdn=0
    // Fields: size=0, sh=0, imm8=15, Zdn=0
    let encoding: u32 = 0x2523C1E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=16 (power of 2 (2^4 = 16))
#[test]
fn test_subr_z_zi_combo_13_c000_2523c200() {
    // Encoding: 0x2523C200
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=16, Zdn=0
    // Fields: Zdn=0, size=0, sh=0, imm8=16
    let encoding: u32 = 0x2523C200;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=31 (2^5 - 1 = 31)
#[test]
fn test_subr_z_zi_combo_14_c000_2523c3e0() {
    // Encoding: 0x2523C3E0
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=31, Zdn=0
    // Fields: size=0, sh=0, Zdn=0, imm8=31
    let encoding: u32 = 0x2523C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=32 (power of 2 (2^5 = 32))
#[test]
fn test_subr_z_zi_combo_15_c000_2523c400() {
    // Encoding: 0x2523C400
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=32, Zdn=0
    // Fields: sh=0, imm8=32, size=0, Zdn=0
    let encoding: u32 = 0x2523C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=63 (2^6 - 1 = 63)
#[test]
fn test_subr_z_zi_combo_16_c000_2523c7e0() {
    // Encoding: 0x2523C7E0
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=63, Zdn=0
    // Fields: size=0, imm8=63, sh=0, Zdn=0
    let encoding: u32 = 0x2523C7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=64 (power of 2 (2^6 = 64))
#[test]
fn test_subr_z_zi_combo_17_c000_2523c800() {
    // Encoding: 0x2523C800
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=64, Zdn=0
    // Fields: size=0, imm8=64, Zdn=0, sh=0
    let encoding: u32 = 0x2523C800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=127 (immediate midpoint (127))
#[test]
fn test_subr_z_zi_combo_18_c000_2523cfe0() {
    // Encoding: 0x2523CFE0
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=127, Zdn=0
    // Fields: sh=0, Zdn=0, size=0, imm8=127
    let encoding: u32 = 0x2523CFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=128 (power of 2 (2^7 = 128))
#[test]
fn test_subr_z_zi_combo_19_c000_2523d000() {
    // Encoding: 0x2523D000
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=128, Zdn=0
    // Fields: size=0, imm8=128, Zdn=0, sh=0
    let encoding: u32 = 0x2523D000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=255 (maximum immediate (255))
#[test]
fn test_subr_z_zi_combo_20_c000_2523dfe0() {
    // Encoding: 0x2523DFE0
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=255, Zdn=0
    // Fields: sh=0, imm8=255, size=0, Zdn=0
    let encoding: u32 = 0x2523DFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=0 (minimum value)
#[test]
fn test_subr_z_zi_combo_21_c000_2523c000() {
    // Encoding: 0x2523C000
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=0
    // Fields: imm8=0, sh=0, size=0, Zdn=0
    let encoding: u32 = 0x2523C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=1 (value 1)
#[test]
fn test_subr_z_zi_combo_22_c000_2523c001() {
    // Encoding: 0x2523C001
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=1
    // Fields: size=0, sh=0, Zdn=1, imm8=0
    let encoding: u32 = 0x2523C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=15 (midpoint (15))
#[test]
fn test_subr_z_zi_combo_23_c000_2523c00f() {
    // Encoding: 0x2523C00F
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=15
    // Fields: sh=0, imm8=0, size=0, Zdn=15
    let encoding: u32 = 0x2523C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zdn=31 (maximum value (31))
#[test]
fn test_subr_z_zi_combo_24_c000_2523c01f() {
    // Encoding: 0x2523C01F
    // Test SUBR_Z.ZI__ field combination: size=0, sh=0, imm8=0, Zdn=31
    // Fields: sh=0, Zdn=31, size=0, imm8=0
    let encoding: u32 = 0x2523C01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_subr_z_zi_special_size_0_size_variant_0_49152_2523c020() {
    // Encoding: 0x2523C020
    // Test SUBR_Z.ZI__ special value size = 0 (Size variant 0)
    // Fields: Zdn=0, sh=0, size=0, imm8=1
    let encoding: u32 = 0x2523C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_subr_z_zi_special_size_1_size_variant_1_49152_2563c020() {
    // Encoding: 0x2563C020
    // Test SUBR_Z.ZI__ special value size = 1 (Size variant 1)
    // Fields: Zdn=0, sh=0, imm8=1, size=1
    let encoding: u32 = 0x2563C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_subr_z_zi_special_size_2_size_variant_2_49152_25a3c020() {
    // Encoding: 0x25A3C020
    // Test SUBR_Z.ZI__ special value size = 2 (Size variant 2)
    // Fields: imm8=1, Zdn=0, sh=0, size=2
    let encoding: u32 = 0x25A3C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_subr_z_zi_special_size_3_size_variant_3_49152_25e3c020() {
    // Encoding: 0x25E3C020
    // Test SUBR_Z.ZI__ special value size = 3 (Size variant 3)
    // Fields: size=3, imm8=1, Zdn=0, sh=0
    let encoding: u32 = 0x25E3C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field sh = 0 (Shift type LSL)`
/// Requirement: FieldSpecial { field: "sh", value: 0, meaning: "Shift type LSL" }
/// Shift type LSL
#[test]
fn test_subr_z_zi_special_sh_0_shift_type_lsl_49152_2563c020() {
    // Encoding: 0x2563C020
    // Test SUBR_Z.ZI__ special value sh = 0 (Shift type LSL)
    // Fields: sh=0, size=1, imm8=1, Zdn=0
    let encoding: u32 = 0x2563C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field sh = 1 (Shift type LSR)`
/// Requirement: FieldSpecial { field: "sh", value: 1, meaning: "Shift type LSR" }
/// Shift type LSR
#[test]
fn test_subr_z_zi_special_sh_1_shift_type_lsr_49152_2563e020() {
    // Encoding: 0x2563E020
    // Test SUBR_Z.ZI__ special value sh = 1 (Shift type LSR)
    // Fields: Zdn=0, size=1, sh=1, imm8=1
    let encoding: u32 = 0x2563E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.h, z0.h, #256                // =0x100
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field sh = 2 (Shift type ASR)`
/// Requirement: FieldSpecial { field: "sh", value: 2, meaning: "Shift type ASR" }
/// Shift type ASR
#[test]
fn test_subr_z_zi_special_sh_2_shift_type_asr_49152_2563c020() {
    // Encoding: 0x2563C020
    // Test SUBR_Z.ZI__ special value sh = 2 (Shift type ASR)
    // Fields: imm8=1, sh=2, Zdn=0, size=1
    let encoding: u32 = 0x2563C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: SUBR_Z.ZI__
/// ASL: `field sh = 3 (Shift type ROR)`
/// Requirement: FieldSpecial { field: "sh", value: 3, meaning: "Shift type ROR" }
/// Shift type ROR
#[test]
fn test_subr_z_zi_special_sh_3_shift_type_ror_49152_2563e020() {
    // Encoding: 0x2563E020
    // Test SUBR_Z.ZI__ special value sh = 3 (Shift type ROR)
    // Fields: imm8=1, size=1, Zdn=0, sh=3
    let encoding: u32 = 0x2563E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: subr	z0.h, z0.h, #256                // =0x100
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// ADD_Z.ZZ__ Tests
// ============================================================================

/// Provenance: ADD_Z.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_add_z_zz_field_size_0_min_0_04200000() {
    // Encoding: 0x04200000
    // Test ADD_Z.ZZ__ field size = 0 (Min)
    // Fields: Zd=0, Zn=0, Zm=0, size=0
    let encoding: u32 = 0x04200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_add_z_zz_field_size_1_poweroftwo_0_04600000() {
    // Encoding: 0x04600000
    // Test ADD_Z.ZZ__ field size = 1 (PowerOfTwo)
    // Fields: Zm=0, Zn=0, size=1, Zd=0
    let encoding: u32 = 0x04600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.h, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_add_z_zz_field_size_2_poweroftwo_0_04a00000() {
    // Encoding: 0x04A00000
    // Test ADD_Z.ZZ__ field size = 2 (PowerOfTwo)
    // Fields: Zd=0, Zn=0, size=2, Zm=0
    let encoding: u32 = 0x04A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.s, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field size 22 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_add_z_zz_field_size_3_max_0_04e00000() {
    // Encoding: 0x04E00000
    // Test ADD_Z.ZZ__ field size = 3 (Max)
    // Fields: size=3, Zm=0, Zn=0, Zd=0
    let encoding: u32 = 0x04E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.d, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_add_z_zz_field_zm_0_min_0_04200000() {
    // Encoding: 0x04200000
    // Test ADD_Z.ZZ__ field Zm = 0 (Min)
    // Fields: size=0, Zn=0, Zd=0, Zm=0
    let encoding: u32 = 0x04200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_add_z_zz_field_zm_1_poweroftwo_0_04210000() {
    // Encoding: 0x04210000
    // Test ADD_Z.ZZ__ field Zm = 1 (PowerOfTwo)
    // Fields: Zd=0, Zm=1, size=0, Zn=0
    let encoding: u32 = 0x04210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_add_z_zz_field_zm_30_poweroftwominusone_0_043e0000() {
    // Encoding: 0x043E0000
    // Test ADD_Z.ZZ__ field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: Zn=0, Zd=0, Zm=30, size=0
    let encoding: u32 = 0x043E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_add_z_zz_field_zm_31_max_0_043f0000() {
    // Encoding: 0x043F0000
    // Test ADD_Z.ZZ__ field Zm = 31 (Max)
    // Fields: Zm=31, Zn=0, size=0, Zd=0
    let encoding: u32 = 0x043F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_add_z_zz_field_zn_0_min_0_04200000() {
    // Encoding: 0x04200000
    // Test ADD_Z.ZZ__ field Zn = 0 (Min)
    // Fields: Zm=0, Zn=0, Zd=0, size=0
    let encoding: u32 = 0x04200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_add_z_zz_field_zn_1_poweroftwo_0_04200020() {
    // Encoding: 0x04200020
    // Test ADD_Z.ZZ__ field Zn = 1 (PowerOfTwo)
    // Fields: Zd=0, Zm=0, Zn=1, size=0
    let encoding: u32 = 0x04200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_add_z_zz_field_zn_30_poweroftwominusone_0_042003c0() {
    // Encoding: 0x042003C0
    // Test ADD_Z.ZZ__ field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Zm=0, Zn=30, size=0, Zd=0
    let encoding: u32 = 0x042003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z30.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_add_z_zz_field_zn_31_max_0_042003e0() {
    // Encoding: 0x042003E0
    // Test ADD_Z.ZZ__ field Zn = 31 (Max)
    // Fields: Zm=0, Zn=31, Zd=0, size=0
    let encoding: u32 = 0x042003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_add_z_zz_field_zd_0_min_0_04200000() {
    // Encoding: 0x04200000
    // Test ADD_Z.ZZ__ field Zd = 0 (Min)
    // Fields: Zd=0, Zm=0, size=0, Zn=0
    let encoding: u32 = 0x04200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_add_z_zz_field_zd_1_poweroftwo_0_04200001() {
    // Encoding: 0x04200001
    // Test ADD_Z.ZZ__ field Zd = 1 (PowerOfTwo)
    // Fields: Zd=1, Zn=0, size=0, Zm=0
    let encoding: u32 = 0x04200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z1.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_add_z_zz_field_zd_30_poweroftwominusone_0_0420001e() {
    // Encoding: 0x0420001E
    // Test ADD_Z.ZZ__ field Zd = 30 (PowerOfTwoMinusOne)
    // Fields: Zm=0, Zn=0, size=0, Zd=30
    let encoding: u32 = 0x0420001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z30.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field Zd 0 +: 5`
/// Requirement: FieldBoundary { field: "Zd", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_add_z_zz_field_zd_31_max_0_0420001f() {
    // Encoding: 0x0420001F
    // Test ADD_Z.ZZ__ field Zd = 31 (Max)
    // Fields: Zd=31, Zn=0, size=0, Zm=0
    let encoding: u32 = 0x0420001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z31.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_add_z_zz_combo_0_0_04200000() {
    // Encoding: 0x04200000
    // Test ADD_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=0
    // Fields: size=0, Zm=0, Zn=0, Zd=0
    let encoding: u32 = 0x04200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_add_z_zz_combo_1_0_04600000() {
    // Encoding: 0x04600000
    // Test ADD_Z.ZZ__ field combination: size=1, Zm=0, Zn=0, Zd=0
    // Fields: Zm=0, Zd=0, size=1, Zn=0
    let encoding: u32 = 0x04600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.h, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_add_z_zz_combo_2_0_04a00000() {
    // Encoding: 0x04A00000
    // Test ADD_Z.ZZ__ field combination: size=2, Zm=0, Zn=0, Zd=0
    // Fields: Zn=0, Zm=0, size=2, Zd=0
    let encoding: u32 = 0x04A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.s, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_add_z_zz_combo_3_0_04e00000() {
    // Encoding: 0x04E00000
    // Test ADD_Z.ZZ__ field combination: size=3, Zm=0, Zn=0, Zd=0
    // Fields: size=3, Zd=0, Zm=0, Zn=0
    let encoding: u32 = 0x04E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.d, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_add_z_zz_combo_4_0_04200000() {
    // Encoding: 0x04200000
    // Test ADD_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=0
    // Fields: Zd=0, Zn=0, size=0, Zm=0
    let encoding: u32 = 0x04200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_add_z_zz_combo_5_0_04210000() {
    // Encoding: 0x04210000
    // Test ADD_Z.ZZ__ field combination: size=0, Zm=1, Zn=0, Zd=0
    // Fields: Zn=0, Zd=0, Zm=1, size=0
    let encoding: u32 = 0x04210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z1.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_add_z_zz_combo_6_0_043e0000() {
    // Encoding: 0x043E0000
    // Test ADD_Z.ZZ__ field combination: size=0, Zm=30, Zn=0, Zd=0
    // Fields: Zd=0, Zn=0, size=0, Zm=30
    let encoding: u32 = 0x043E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z30.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_add_z_zz_combo_7_0_043f0000() {
    // Encoding: 0x043F0000
    // Test ADD_Z.ZZ__ field combination: size=0, Zm=31, Zn=0, Zd=0
    // Fields: Zn=0, size=0, Zd=0, Zm=31
    let encoding: u32 = 0x043F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z31.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_add_z_zz_combo_8_0_04200000() {
    // Encoding: 0x04200000
    // Test ADD_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=0
    // Fields: Zm=0, size=0, Zd=0, Zn=0
    let encoding: u32 = 0x04200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_add_z_zz_combo_9_0_04200020() {
    // Encoding: 0x04200020
    // Test ADD_Z.ZZ__ field combination: size=0, Zm=0, Zn=1, Zd=0
    // Fields: Zn=1, Zm=0, size=0, Zd=0
    let encoding: u32 = 0x04200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z1.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_add_z_zz_combo_10_0_042003c0() {
    // Encoding: 0x042003C0
    // Test ADD_Z.ZZ__ field combination: size=0, Zm=0, Zn=30, Zd=0
    // Fields: Zd=0, Zm=0, Zn=30, size=0
    let encoding: u32 = 0x042003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z30.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_add_z_zz_combo_11_0_042003e0() {
    // Encoding: 0x042003E0
    // Test ADD_Z.ZZ__ field combination: size=0, Zm=0, Zn=31, Zd=0
    // Fields: Zd=0, size=0, Zn=31, Zm=0
    let encoding: u32 = 0x042003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z31.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=0 (SIMD register V0)
#[test]
fn test_add_z_zz_combo_12_0_04200000() {
    // Encoding: 0x04200000
    // Test ADD_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=0
    // Fields: Zn=0, Zd=0, Zm=0, size=0
    let encoding: u32 = 0x04200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=1 (SIMD register V1)
#[test]
fn test_add_z_zz_combo_13_0_04200001() {
    // Encoding: 0x04200001
    // Test ADD_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=1
    // Fields: Zn=0, Zm=0, Zd=1, size=0
    let encoding: u32 = 0x04200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z1.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=30 (SIMD register V30)
#[test]
fn test_add_z_zz_combo_14_0_0420001e() {
    // Encoding: 0x0420001E
    // Test ADD_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=30
    // Fields: Zd=30, Zn=0, size=0, Zm=0
    let encoding: u32 = 0x0420001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z30.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zd=31 (SIMD register V31)
#[test]
fn test_add_z_zz_combo_15_0_0420001f() {
    // Encoding: 0x0420001F
    // Test ADD_Z.ZZ__ field combination: size=0, Zm=0, Zn=0, Zd=31
    // Fields: size=0, Zd=31, Zm=0, Zn=0
    let encoding: u32 = 0x0420001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z31.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_add_z_zz_special_size_0_size_variant_0_0_04200000() {
    // Encoding: 0x04200000
    // Test ADD_Z.ZZ__ special value size = 0 (Size variant 0)
    // Fields: size=0, Zm=0, Zd=0, Zn=0
    let encoding: u32 = 0x04200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.b, z0.b, z0.b
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_add_z_zz_special_size_1_size_variant_1_0_04600000() {
    // Encoding: 0x04600000
    // Test ADD_Z.ZZ__ special value size = 1 (Size variant 1)
    // Fields: Zm=0, Zd=0, Zn=0, size=1
    let encoding: u32 = 0x04600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.h, z0.h, z0.h
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_add_z_zz_special_size_2_size_variant_2_0_04a00000() {
    // Encoding: 0x04A00000
    // Test ADD_Z.ZZ__ special value size = 2 (Size variant 2)
    // Fields: Zn=0, Zm=0, Zd=0, size=2
    let encoding: u32 = 0x04A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.s, z0.s, z0.s
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: ADD_Z.ZZ__
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_add_z_zz_special_size_3_size_variant_3_0_04e00000() {
    // Encoding: 0x04E00000
    // Test ADD_Z.ZZ__ special value size = 3 (Size variant 3)
    // Fields: Zd=0, Zm=0, size=3, Zn=0
    let encoding: u32 = 0x04E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: add	z0.d, z0.d, z0.d
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}
