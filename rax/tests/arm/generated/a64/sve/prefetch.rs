//! A64 sve prefetch tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// PRFH_I.P.BZ_S.x32.scaled Tests
// ============================================================================

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_xs_0_min_2000_84202000() {
    // Encoding: 0x84202000
    // Test PRFH_I.P.BZ_S.x32.scaled field xs = 0 (Min)
    // Fields: xs=0, prfop=0, Zm=0, Pg=0, Rn=0
    let encoding: u32 = 0x84202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_xs_1_max_2000_84602000() {
    // Encoding: 0x84602000
    // Test PRFH_I.P.BZ_S.x32.scaled field xs = 1 (Max)
    // Fields: Pg=0, xs=1, Rn=0, Zm=0, prfop=0
    let encoding: u32 = 0x84602000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_zm_0_min_2000_84202000() {
    // Encoding: 0x84202000
    // Test PRFH_I.P.BZ_S.x32.scaled field Zm = 0 (Min)
    // Fields: Pg=0, Rn=0, prfop=0, Zm=0, xs=0
    let encoding: u32 = 0x84202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_zm_1_poweroftwo_2000_84212000() {
    // Encoding: 0x84212000
    // Test PRFH_I.P.BZ_S.x32.scaled field Zm = 1 (PowerOfTwo)
    // Fields: Pg=0, Zm=1, xs=0, prfop=0, Rn=0
    let encoding: u32 = 0x84212000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_zm_30_poweroftwominusone_2000_843e2000() {
    // Encoding: 0x843E2000
    // Test PRFH_I.P.BZ_S.x32.scaled field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Pg=0, xs=0, Zm=30, Rn=0
    let encoding: u32 = 0x843E2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_zm_31_max_2000_843f2000() {
    // Encoding: 0x843F2000
    // Test PRFH_I.P.BZ_S.x32.scaled field Zm = 31 (Max)
    // Fields: xs=0, Rn=0, prfop=0, Zm=31, Pg=0
    let encoding: u32 = 0x843F2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_pg_0_min_2000_84202000() {
    // Encoding: 0x84202000
    // Test PRFH_I.P.BZ_S.x32.scaled field Pg = 0 (Min)
    // Fields: Rn=0, prfop=0, xs=0, Pg=0, Zm=0
    let encoding: u32 = 0x84202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_pg_1_poweroftwo_2000_84202400() {
    // Encoding: 0x84202400
    // Test PRFH_I.P.BZ_S.x32.scaled field Pg = 1 (PowerOfTwo)
    // Fields: Rn=0, Zm=0, Pg=1, xs=0, prfop=0
    let encoding: u32 = 0x84202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_rn_0_min_2000_84202000() {
    // Encoding: 0x84202000
    // Test PRFH_I.P.BZ_S.x32.scaled field Rn = 0 (Min)
    // Fields: Rn=0, Zm=0, Pg=0, xs=0, prfop=0
    let encoding: u32 = 0x84202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_rn_1_poweroftwo_2000_84202020() {
    // Encoding: 0x84202020
    // Test PRFH_I.P.BZ_S.x32.scaled field Rn = 1 (PowerOfTwo)
    // Fields: Pg=0, Rn=1, prfop=0, xs=0, Zm=0
    let encoding: u32 = 0x84202020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_rn_30_poweroftwominusone_2000_842023c0() {
    // Encoding: 0x842023C0
    // Test PRFH_I.P.BZ_S.x32.scaled field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, xs=0, Rn=30, prfop=0, Zm=0
    let encoding: u32 = 0x842023C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_rn_31_max_2000_842023e0() {
    // Encoding: 0x842023E0
    // Test PRFH_I.P.BZ_S.x32.scaled field Rn = 31 (Max)
    // Fields: Pg=0, xs=0, Zm=0, Rn=31, prfop=0
    let encoding: u32 = 0x842023E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_prfop_0_min_2000_84202000() {
    // Encoding: 0x84202000
    // Test PRFH_I.P.BZ_S.x32.scaled field prfop = 0 (Min)
    // Fields: Rn=0, Zm=0, prfop=0, xs=0, Pg=0
    let encoding: u32 = 0x84202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_prfop_1_poweroftwo_2000_84202001() {
    // Encoding: 0x84202001
    // Test PRFH_I.P.BZ_S.x32.scaled field prfop = 1 (PowerOfTwo)
    // Fields: prfop=1, Zm=0, xs=0, Rn=0, Pg=0
    let encoding: u32 = 0x84202001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_prfop_7_poweroftwominusone_2000_84202007() {
    // Encoding: 0x84202007
    // Test PRFH_I.P.BZ_S.x32.scaled field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: xs=0, Rn=0, prfop=7, Zm=0, Pg=0
    let encoding: u32 = 0x84202007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_field_prfop_15_max_2000_8420200f() {
    // Encoding: 0x8420200F
    // Test PRFH_I.P.BZ_S.x32.scaled field prfop = 15 (Max)
    // Fields: Zm=0, prfop=15, Pg=0, Rn=0, xs=0
    let encoding: u32 = 0x8420200F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=0 (minimum value)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_0_2000_84202000() {
    // Encoding: 0x84202000
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Zm=0, Pg=0, prfop=0, xs=0
    let encoding: u32 = 0x84202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=1 (maximum value (1))
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_1_2000_84602000() {
    // Encoding: 0x84602000
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=1, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: xs=1, Rn=0, prfop=0, Zm=0, Pg=0
    let encoding: u32 = 0x84602000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_2_2000_84202000() {
    // Encoding: 0x84202000
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Pg=0, xs=0, prfop=0, Rn=0
    let encoding: u32 = 0x84202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_3_2000_84212000() {
    // Encoding: 0x84212000
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=1, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Pg=0, xs=0, Rn=0, Zm=1
    let encoding: u32 = 0x84212000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_4_2000_843e2000() {
    // Encoding: 0x843E2000
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=30, Pg=0, Rn=0, prfop=0
    // Fields: Zm=30, xs=0, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0x843E2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_5_2000_843f2000() {
    // Encoding: 0x843F2000
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=31, Pg=0, Rn=0, prfop=0
    // Fields: Zm=31, Rn=0, prfop=0, Pg=0, xs=0
    let encoding: u32 = 0x843F2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_6_2000_84202000() {
    // Encoding: 0x84202000
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, prfop=0, xs=0, Zm=0, Pg=0
    let encoding: u32 = 0x84202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_7_2000_84202400() {
    // Encoding: 0x84202400
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=0, prfop=0
    // Fields: Pg=1, xs=0, Zm=0, Rn=0, prfop=0
    let encoding: u32 = 0x84202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_8_2000_84202000() {
    // Encoding: 0x84202000
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Pg=0, Rn=0, prfop=0, xs=0
    let encoding: u32 = 0x84202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_9_2000_84202020() {
    // Encoding: 0x84202020
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=1, prfop=0
    // Fields: Rn=1, xs=0, Zm=0, Pg=0, prfop=0
    let encoding: u32 = 0x84202020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_10_2000_842023c0() {
    // Encoding: 0x842023C0
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=30, prfop=0
    // Fields: xs=0, Rn=30, prfop=0, Pg=0, Zm=0
    let encoding: u32 = 0x842023C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_11_2000_842023e0() {
    // Encoding: 0x842023E0
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=31, prfop=0
    // Fields: xs=0, Pg=0, Zm=0, Rn=31, prfop=0
    let encoding: u32 = 0x842023E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_12_2000_84202000() {
    // Encoding: 0x84202000
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Zm=0, prfop=0, Rn=0, xs=0
    let encoding: u32 = 0x84202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_13_2000_84202001() {
    // Encoding: 0x84202001
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=1
    // Fields: Rn=0, Pg=0, prfop=1, xs=0, Zm=0
    let encoding: u32 = 0x84202001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_14_2000_84202007() {
    // Encoding: 0x84202007
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=7
    // Fields: Zm=0, prfop=7, Pg=0, Rn=0, xs=0
    let encoding: u32 = 0x84202007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_15_2000_8420200f() {
    // Encoding: 0x8420200F
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=15
    // Fields: Rn=0, xs=0, Zm=0, Pg=0, prfop=15
    let encoding: u32 = 0x8420200F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_16_2000_84202420() {
    // Encoding: 0x84202420
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=1, prfop=0
    // Fields: Rn=1, Zm=0, prfop=0, xs=0, Pg=1
    let encoding: u32 = 0x84202420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_combo_17_2000_84203fe0() {
    // Encoding: 0x84203FE0
    // Test PRFH_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=31, Rn=31, prfop=0
    // Fields: xs=0, Zm=0, prfop=0, Pg=31, Rn=31
    let encoding: u32 = 0x84203FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_special_rn_31_stack_pointer_sp_may_require_alignment_8192_842023e0()
 {
    // Encoding: 0x842023E0
    // Test PRFH_I.P.BZ_S.x32.scaled special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, xs=0, Pg=0, Zm=0, prfop=0
    let encoding: u32 = 0x842023E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_xs_0_min_2000_c4202000() {
    // Encoding: 0xC4202000
    // Test PRFH_I.P.BZ_D.x32.scaled field xs = 0 (Min)
    // Fields: Pg=0, Rn=0, xs=0, prfop=0, Zm=0
    let encoding: u32 = 0xC4202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_xs_1_max_2000_c4602000() {
    // Encoding: 0xC4602000
    // Test PRFH_I.P.BZ_D.x32.scaled field xs = 1 (Max)
    // Fields: Pg=0, Zm=0, prfop=0, Rn=0, xs=1
    let encoding: u32 = 0xC4602000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_zm_0_min_2000_c4202000() {
    // Encoding: 0xC4202000
    // Test PRFH_I.P.BZ_D.x32.scaled field Zm = 0 (Min)
    // Fields: Zm=0, xs=0, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0xC4202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_zm_1_poweroftwo_2000_c4212000() {
    // Encoding: 0xC4212000
    // Test PRFH_I.P.BZ_D.x32.scaled field Zm = 1 (PowerOfTwo)
    // Fields: Pg=0, xs=0, Rn=0, prfop=0, Zm=1
    let encoding: u32 = 0xC4212000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_zm_30_poweroftwominusone_2000_c43e2000() {
    // Encoding: 0xC43E2000
    // Test PRFH_I.P.BZ_D.x32.scaled field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: xs=0, Zm=30, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0xC43E2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_zm_31_max_2000_c43f2000() {
    // Encoding: 0xC43F2000
    // Test PRFH_I.P.BZ_D.x32.scaled field Zm = 31 (Max)
    // Fields: Zm=31, Pg=0, xs=0, prfop=0, Rn=0
    let encoding: u32 = 0xC43F2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_pg_0_min_2000_c4202000() {
    // Encoding: 0xC4202000
    // Test PRFH_I.P.BZ_D.x32.scaled field Pg = 0 (Min)
    // Fields: Rn=0, prfop=0, Zm=0, Pg=0, xs=0
    let encoding: u32 = 0xC4202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_pg_1_poweroftwo_2000_c4202400() {
    // Encoding: 0xC4202400
    // Test PRFH_I.P.BZ_D.x32.scaled field Pg = 1 (PowerOfTwo)
    // Fields: Rn=0, xs=0, Pg=1, prfop=0, Zm=0
    let encoding: u32 = 0xC4202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_rn_0_min_2000_c4202000() {
    // Encoding: 0xC4202000
    // Test PRFH_I.P.BZ_D.x32.scaled field Rn = 0 (Min)
    // Fields: prfop=0, Pg=0, Rn=0, Zm=0, xs=0
    let encoding: u32 = 0xC4202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_rn_1_poweroftwo_2000_c4202020() {
    // Encoding: 0xC4202020
    // Test PRFH_I.P.BZ_D.x32.scaled field Rn = 1 (PowerOfTwo)
    // Fields: prfop=0, xs=0, Pg=0, Zm=0, Rn=1
    let encoding: u32 = 0xC4202020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_rn_30_poweroftwominusone_2000_c42023c0() {
    // Encoding: 0xC42023C0
    // Test PRFH_I.P.BZ_D.x32.scaled field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, xs=0, Zm=0, Pg=0, prfop=0
    let encoding: u32 = 0xC42023C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_rn_31_max_2000_c42023e0() {
    // Encoding: 0xC42023E0
    // Test PRFH_I.P.BZ_D.x32.scaled field Rn = 31 (Max)
    // Fields: xs=0, Pg=0, Zm=0, prfop=0, Rn=31
    let encoding: u32 = 0xC42023E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_prfop_0_min_2000_c4202000() {
    // Encoding: 0xC4202000
    // Test PRFH_I.P.BZ_D.x32.scaled field prfop = 0 (Min)
    // Fields: Pg=0, Rn=0, xs=0, Zm=0, prfop=0
    let encoding: u32 = 0xC4202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_prfop_1_poweroftwo_2000_c4202001() {
    // Encoding: 0xC4202001
    // Test PRFH_I.P.BZ_D.x32.scaled field prfop = 1 (PowerOfTwo)
    // Fields: Zm=0, Pg=0, Rn=0, xs=0, prfop=1
    let encoding: u32 = 0xC4202001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_prfop_7_poweroftwominusone_2000_c4202007() {
    // Encoding: 0xC4202007
    // Test PRFH_I.P.BZ_D.x32.scaled field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Rn=0, prfop=7, Zm=0, xs=0
    let encoding: u32 = 0xC4202007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_field_prfop_15_max_2000_c420200f() {
    // Encoding: 0xC420200F
    // Test PRFH_I.P.BZ_D.x32.scaled field prfop = 15 (Max)
    // Fields: Rn=0, prfop=15, xs=0, Zm=0, Pg=0
    let encoding: u32 = 0xC420200F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=0 (minimum value)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_0_2000_c4202000() {
    // Encoding: 0xC4202000
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Zm=0, Rn=0, prfop=0, xs=0
    let encoding: u32 = 0xC4202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=1 (maximum value (1))
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_1_2000_c4602000() {
    // Encoding: 0xC4602000
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=1, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: xs=1, prfop=0, Zm=0, Pg=0, Rn=0
    let encoding: u32 = 0xC4602000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_2_2000_c4202000() {
    // Encoding: 0xC4202000
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, xs=0, Pg=0, Rn=0, Zm=0
    let encoding: u32 = 0xC4202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_3_2000_c4212000() {
    // Encoding: 0xC4212000
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=1, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, Zm=1, Pg=0, xs=0
    let encoding: u32 = 0xC4212000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_4_2000_c43e2000() {
    // Encoding: 0xC43E2000
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=30, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Zm=30, prfop=0, xs=0, Rn=0
    let encoding: u32 = 0xC43E2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_5_2000_c43f2000() {
    // Encoding: 0xC43F2000
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=31, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, Pg=0, xs=0, Zm=31
    let encoding: u32 = 0xC43F2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_6_2000_c4202000() {
    // Encoding: 0xC4202000
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, xs=0, Zm=0, prfop=0
    let encoding: u32 = 0xC4202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_7_2000_c4202400() {
    // Encoding: 0xC4202400
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=0, prfop=0
    // Fields: xs=0, Zm=0, prfop=0, Pg=1, Rn=0
    let encoding: u32 = 0xC4202400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_8_2000_c4202000() {
    // Encoding: 0xC4202000
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, prfop=0, Rn=0, xs=0, Zm=0
    let encoding: u32 = 0xC4202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_9_2000_c4202020() {
    // Encoding: 0xC4202020
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=1, prfop=0
    // Fields: Zm=0, Rn=1, prfop=0, xs=0, Pg=0
    let encoding: u32 = 0xC4202020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_10_2000_c42023c0() {
    // Encoding: 0xC42023C0
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=30, prfop=0
    // Fields: prfop=0, Pg=0, Rn=30, Zm=0, xs=0
    let encoding: u32 = 0xC42023C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_11_2000_c42023e0() {
    // Encoding: 0xC42023E0
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=31, prfop=0
    // Fields: Zm=0, Rn=31, Pg=0, prfop=0, xs=0
    let encoding: u32 = 0xC42023E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_12_2000_c4202000() {
    // Encoding: 0xC4202000
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, prfop=0, xs=0, Pg=0, Rn=0
    let encoding: u32 = 0xC4202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_13_2000_c4202001() {
    // Encoding: 0xC4202001
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=1
    // Fields: Zm=0, Rn=0, xs=0, Pg=0, prfop=1
    let encoding: u32 = 0xC4202001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_14_2000_c4202007() {
    // Encoding: 0xC4202007
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=7
    // Fields: Zm=0, xs=0, Pg=0, prfop=7, Rn=0
    let encoding: u32 = 0xC4202007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_15_2000_c420200f() {
    // Encoding: 0xC420200F
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=15
    // Fields: Zm=0, xs=0, prfop=15, Pg=0, Rn=0
    let encoding: u32 = 0xC420200F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_16_2000_c4202420() {
    // Encoding: 0xC4202420
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=1, prfop=0
    // Fields: xs=0, Zm=0, Pg=1, Rn=1, prfop=0
    let encoding: u32 = 0xC4202420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_combo_17_2000_c4203fe0() {
    // Encoding: 0xC4203FE0
    // Test PRFH_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=31, Rn=31, prfop=0
    // Fields: Zm=0, Pg=31, Rn=31, xs=0, prfop=0
    let encoding: u32 = 0xC4203FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_special_rn_31_stack_pointer_sp_may_require_alignment_8192_c42023e0()
 {
    // Encoding: 0xC42023E0
    // Test PRFH_I.P.BZ_D.x32.scaled special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: xs=0, prfop=0, Zm=0, Rn=31, Pg=0
    let encoding: u32 = 0xC42023E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_zm_0_min_a000_c460a000() {
    // Encoding: 0xC460A000
    // Test PRFH_I.P.BZ_D.64.scaled field Zm = 0 (Min)
    // Fields: prfop=0, Zm=0, Rn=0, Pg=0
    let encoding: u32 = 0xC460A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_zm_1_poweroftwo_a000_c461a000() {
    // Encoding: 0xC461A000
    // Test PRFH_I.P.BZ_D.64.scaled field Zm = 1 (PowerOfTwo)
    // Fields: Rn=0, prfop=0, Pg=0, Zm=1
    let encoding: u32 = 0xC461A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_zm_30_poweroftwominusone_a000_c47ea000() {
    // Encoding: 0xC47EA000
    // Test PRFH_I.P.BZ_D.64.scaled field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zm=30, prfop=0, Rn=0
    let encoding: u32 = 0xC47EA000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_zm_31_max_a000_c47fa000() {
    // Encoding: 0xC47FA000
    // Test PRFH_I.P.BZ_D.64.scaled field Zm = 31 (Max)
    // Fields: Pg=0, Rn=0, Zm=31, prfop=0
    let encoding: u32 = 0xC47FA000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_pg_0_min_a000_c460a000() {
    // Encoding: 0xC460A000
    // Test PRFH_I.P.BZ_D.64.scaled field Pg = 0 (Min)
    // Fields: Zm=0, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0xC460A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_pg_1_poweroftwo_a000_c460a400() {
    // Encoding: 0xC460A400
    // Test PRFH_I.P.BZ_D.64.scaled field Pg = 1 (PowerOfTwo)
    // Fields: Zm=0, prfop=0, Pg=1, Rn=0
    let encoding: u32 = 0xC460A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_rn_0_min_a000_c460a000() {
    // Encoding: 0xC460A000
    // Test PRFH_I.P.BZ_D.64.scaled field Rn = 0 (Min)
    // Fields: Rn=0, prfop=0, Zm=0, Pg=0
    let encoding: u32 = 0xC460A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_rn_1_poweroftwo_a000_c460a020() {
    // Encoding: 0xC460A020
    // Test PRFH_I.P.BZ_D.64.scaled field Rn = 1 (PowerOfTwo)
    // Fields: Zm=0, prfop=0, Pg=0, Rn=1
    let encoding: u32 = 0xC460A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_rn_30_poweroftwominusone_a000_c460a3c0() {
    // Encoding: 0xC460A3C0
    // Test PRFH_I.P.BZ_D.64.scaled field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Pg=0, Zm=0, prfop=0
    let encoding: u32 = 0xC460A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_rn_31_max_a000_c460a3e0() {
    // Encoding: 0xC460A3E0
    // Test PRFH_I.P.BZ_D.64.scaled field Rn = 31 (Max)
    // Fields: Zm=0, Pg=0, prfop=0, Rn=31
    let encoding: u32 = 0xC460A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_prfop_0_min_a000_c460a000() {
    // Encoding: 0xC460A000
    // Test PRFH_I.P.BZ_D.64.scaled field prfop = 0 (Min)
    // Fields: Zm=0, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0xC460A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_prfop_1_poweroftwo_a000_c460a001() {
    // Encoding: 0xC460A001
    // Test PRFH_I.P.BZ_D.64.scaled field prfop = 1 (PowerOfTwo)
    // Fields: Rn=0, Zm=0, prfop=1, Pg=0
    let encoding: u32 = 0xC460A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_prfop_7_poweroftwominusone_a000_c460a007() {
    // Encoding: 0xC460A007
    // Test PRFH_I.P.BZ_D.64.scaled field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zm=0, Rn=0, prfop=7
    let encoding: u32 = 0xC460A007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_field_prfop_15_max_a000_c460a00f() {
    // Encoding: 0xC460A00F
    // Test PRFH_I.P.BZ_D.64.scaled field prfop = 15 (Max)
    // Fields: prfop=15, Pg=0, Rn=0, Zm=0
    let encoding: u32 = 0xC460A00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_0_a000_c460a000() {
    // Encoding: 0xC460A000
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Zm=0, Rn=0, prfop=0
    let encoding: u32 = 0xC460A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_1_a000_c461a000() {
    // Encoding: 0xC461A000
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=1, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, Zm=1, prfop=0
    let encoding: u32 = 0xC461A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_2_a000_c47ea000() {
    // Encoding: 0xC47EA000
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=30, Pg=0, Rn=0, prfop=0
    // Fields: Zm=30, prfop=0, Rn=0, Pg=0
    let encoding: u32 = 0xC47EA000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_3_a000_c47fa000() {
    // Encoding: 0xC47FA000
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=31, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Zm=31, prfop=0, Rn=0
    let encoding: u32 = 0xC47FA000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_4_a000_c460a000() {
    // Encoding: 0xC460A000
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0xC460A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_5_a000_c460a400() {
    // Encoding: 0xC460A400
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=1, Rn=0, prfop=0
    // Fields: Rn=0, Zm=0, Pg=1, prfop=0
    let encoding: u32 = 0xC460A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_6_a000_c460a000() {
    // Encoding: 0xC460A000
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rn=0, Zm=0, prfop=0
    let encoding: u32 = 0xC460A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_7_a000_c460a020() {
    // Encoding: 0xC460A020
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=1, prfop=0
    // Fields: Pg=0, prfop=0, Zm=0, Rn=1
    let encoding: u32 = 0xC460A020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_8_a000_c460a3c0() {
    // Encoding: 0xC460A3C0
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=30, prfop=0
    // Fields: Zm=0, Pg=0, prfop=0, Rn=30
    let encoding: u32 = 0xC460A3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_9_a000_c460a3e0() {
    // Encoding: 0xC460A3E0
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=31, prfop=0
    // Fields: Zm=0, Pg=0, prfop=0, Rn=31
    let encoding: u32 = 0xC460A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_10_a000_c460a000() {
    // Encoding: 0xC460A000
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, prfop=0, Rn=0, Zm=0
    let encoding: u32 = 0xC460A000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_11_a000_c460a001() {
    // Encoding: 0xC460A001
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=1
    // Fields: prfop=1, Zm=0, Rn=0, Pg=0
    let encoding: u32 = 0xC460A001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_12_a000_c460a007() {
    // Encoding: 0xC460A007
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=7
    // Fields: Rn=0, Zm=0, prfop=7, Pg=0
    let encoding: u32 = 0xC460A007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_13_a000_c460a00f() {
    // Encoding: 0xC460A00F
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=15
    // Fields: prfop=15, Rn=0, Zm=0, Pg=0
    let encoding: u32 = 0xC460A00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_14_a000_c460a420() {
    // Encoding: 0xC460A420
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=1, Rn=1, prfop=0
    // Fields: Rn=1, Pg=1, prfop=0, Zm=0
    let encoding: u32 = 0xC460A420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfh_i_p_bz_d_64_scaled_combo_15_a000_c460bfe0() {
    // Encoding: 0xC460BFE0
    // Test PRFH_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=31, Rn=31, prfop=0
    // Fields: Zm=0, Pg=31, prfop=0, Rn=31
    let encoding: u32 = 0xC460BFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfh_i_p_bz_d_64_scaled_special_rn_31_stack_pointer_sp_may_require_alignment_40960_c460a3e0()
 {
    // Encoding: 0xC460A3E0
    // Test PRFH_I.P.BZ_D.64.scaled special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: prfop=0, Zm=0, Rn=31, Pg=0
    let encoding: u32 = 0xC460A3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BZ_S.x32.scaled
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfh_i_p_bz_s_x32_scaled_sp_rn_842023e0() {
    // Test PRFH_I.P.BZ_S.x32.scaled with Rn = SP (31)
    // Encoding: 0x842023E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x842023E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PRFH_I.P.BZ_D.x32.scaled
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfh_i_p_bz_d_x32_scaled_sp_rn_c42023e0() {
    // Test PRFH_I.P.BZ_D.x32.scaled with Rn = SP (31)
    // Encoding: 0xC42023E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xC42023E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PRFH_I.P.BZ_D.64.scaled
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfh_i_p_bz_d_64_scaled_sp_rn_c460a3e0() {
    // Test PRFH_I.P.BZ_D.64.scaled with Rn = SP (31)
    // Encoding: 0xC460A3E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xC460A3E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PRFH_I.P.BI_S Tests
// ============================================================================

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_prfh_i_p_bi_s_field_imm6_0_zero_2000_85c02000() {
    // Encoding: 0x85C02000
    // Test PRFH_I.P.BI_S field imm6 = 0 (Zero)
    // Fields: imm6=0, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0x85C02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_prfh_i_p_bi_s_field_imm6_1_poweroftwo_2000_85c12000() {
    // Encoding: 0x85C12000
    // Test PRFH_I.P.BI_S field imm6 = 1 (PowerOfTwo)
    // Fields: imm6=1, Rn=0, Pg=0, prfop=0
    let encoding: u32 = 0x85C12000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_prfh_i_p_bi_s_field_imm6_3_poweroftwominusone_2000_85c32000() {
    // Encoding: 0x85C32000
    // Test PRFH_I.P.BI_S field imm6 = 3 (PowerOfTwoMinusOne)
    // Fields: Pg=0, imm6=3, Rn=0, prfop=0
    let encoding: u32 = 0x85C32000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_prfh_i_p_bi_s_field_imm6_4_poweroftwo_2000_85c42000() {
    // Encoding: 0x85C42000
    // Test PRFH_I.P.BI_S field imm6 = 4 (PowerOfTwo)
    // Fields: imm6=4, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0x85C42000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_prfh_i_p_bi_s_field_imm6_7_poweroftwominusone_2000_85c72000() {
    // Encoding: 0x85C72000
    // Test PRFH_I.P.BI_S field imm6 = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, imm6=7, prfop=0, Pg=0
    let encoding: u32 = 0x85C72000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_prfh_i_p_bi_s_field_imm6_8_poweroftwo_2000_85c82000() {
    // Encoding: 0x85C82000
    // Test PRFH_I.P.BI_S field imm6 = 8 (PowerOfTwo)
    // Fields: Rn=0, Pg=0, imm6=8, prfop=0
    let encoding: u32 = 0x85C82000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_prfh_i_p_bi_s_field_imm6_15_poweroftwominusone_2000_85cf2000() {
    // Encoding: 0x85CF2000
    // Test PRFH_I.P.BI_S field imm6 = 15 (PowerOfTwoMinusOne)
    // Fields: Pg=0, imm6=15, Rn=0, prfop=0
    let encoding: u32 = 0x85CF2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_prfh_i_p_bi_s_field_imm6_16_poweroftwo_2000_85d02000() {
    // Encoding: 0x85D02000
    // Test PRFH_I.P.BI_S field imm6 = 16 (PowerOfTwo)
    // Fields: Rn=0, prfop=0, Pg=0, imm6=16
    let encoding: u32 = 0x85D02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_prfh_i_p_bi_s_field_imm6_31_poweroftwominusone_2000_85df2000() {
    // Encoding: 0x85DF2000
    // Test PRFH_I.P.BI_S field imm6 = 31 (PowerOfTwoMinusOne)
    // Fields: Pg=0, prfop=0, imm6=31, Rn=0
    let encoding: u32 = 0x85DF2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_prfh_i_p_bi_s_field_imm6_32_poweroftwo_2000_85e02000() {
    // Encoding: 0x85E02000
    // Test PRFH_I.P.BI_S field imm6 = 32 (PowerOfTwo)
    // Fields: Rn=0, imm6=32, prfop=0, Pg=0
    let encoding: u32 = 0x85E02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_prfh_i_p_bi_s_field_imm6_63_max_2000_85ff2000() {
    // Encoding: 0x85FF2000
    // Test PRFH_I.P.BI_S field imm6 = 63 (Max)
    // Fields: imm6=63, Rn=0, prfop=0, Pg=0
    let encoding: u32 = 0x85FF2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfh_i_p_bi_s_field_pg_0_min_2000_85c02000() {
    // Encoding: 0x85C02000
    // Test PRFH_I.P.BI_S field Pg = 0 (Min)
    // Fields: Pg=0, prfop=0, Rn=0, imm6=0
    let encoding: u32 = 0x85C02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfh_i_p_bi_s_field_pg_1_poweroftwo_2000_85c02400() {
    // Encoding: 0x85C02400
    // Test PRFH_I.P.BI_S field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, prfop=0, Rn=0, imm6=0
    let encoding: u32 = 0x85C02400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfh_i_p_bi_s_field_rn_0_min_2000_85c02000() {
    // Encoding: 0x85C02000
    // Test PRFH_I.P.BI_S field Rn = 0 (Min)
    // Fields: Pg=0, imm6=0, Rn=0, prfop=0
    let encoding: u32 = 0x85C02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfh_i_p_bi_s_field_rn_1_poweroftwo_2000_85c02020() {
    // Encoding: 0x85C02020
    // Test PRFH_I.P.BI_S field Rn = 1 (PowerOfTwo)
    // Fields: imm6=0, Pg=0, prfop=0, Rn=1
    let encoding: u32 = 0x85C02020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfh_i_p_bi_s_field_rn_30_poweroftwominusone_2000_85c023c0() {
    // Encoding: 0x85C023C0
    // Test PRFH_I.P.BI_S field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, prfop=0, imm6=0, Rn=30
    let encoding: u32 = 0x85C023C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfh_i_p_bi_s_field_rn_31_max_2000_85c023e0() {
    // Encoding: 0x85C023E0
    // Test PRFH_I.P.BI_S field Rn = 31 (Max)
    // Fields: Rn=31, imm6=0, Pg=0, prfop=0
    let encoding: u32 = 0x85C023E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfh_i_p_bi_s_field_prfop_0_min_2000_85c02000() {
    // Encoding: 0x85C02000
    // Test PRFH_I.P.BI_S field prfop = 0 (Min)
    // Fields: Rn=0, Pg=0, prfop=0, imm6=0
    let encoding: u32 = 0x85C02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfh_i_p_bi_s_field_prfop_1_poweroftwo_2000_85c02001() {
    // Encoding: 0x85C02001
    // Test PRFH_I.P.BI_S field prfop = 1 (PowerOfTwo)
    // Fields: Pg=0, prfop=1, imm6=0, Rn=0
    let encoding: u32 = 0x85C02001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfh_i_p_bi_s_field_prfop_7_poweroftwominusone_2000_85c02007() {
    // Encoding: 0x85C02007
    // Test PRFH_I.P.BI_S field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: imm6=0, Rn=0, prfop=7, Pg=0
    let encoding: u32 = 0x85C02007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfh_i_p_bi_s_field_prfop_15_max_2000_85c0200f() {
    // Encoding: 0x85C0200F
    // Test PRFH_I.P.BI_S field prfop = 15 (Max)
    // Fields: Rn=0, prfop=15, Pg=0, imm6=0
    let encoding: u32 = 0x85C0200F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=0 (immediate value 0)
#[test]
fn test_prfh_i_p_bi_s_combo_0_2000_85c02000() {
    // Encoding: 0x85C02000
    // Test PRFH_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, imm6=0, prfop=0, Pg=0
    let encoding: u32 = 0x85C02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=1 (immediate value 1)
#[test]
fn test_prfh_i_p_bi_s_combo_1_2000_85c12000() {
    // Encoding: 0x85C12000
    // Test PRFH_I.P.BI_S field combination: imm6=1, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, imm6=1, Pg=0, Rn=0
    let encoding: u32 = 0x85C12000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=3 (2^2 - 1 = 3)
#[test]
fn test_prfh_i_p_bi_s_combo_2_2000_85c32000() {
    // Encoding: 0x85C32000
    // Test PRFH_I.P.BI_S field combination: imm6=3, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, imm6=3, Rn=0, Pg=0
    let encoding: u32 = 0x85C32000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=4 (power of 2 (2^2 = 4))
#[test]
fn test_prfh_i_p_bi_s_combo_3_2000_85c42000() {
    // Encoding: 0x85C42000
    // Test PRFH_I.P.BI_S field combination: imm6=4, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rn=0, imm6=4, prfop=0
    let encoding: u32 = 0x85C42000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=7 (2^3 - 1 = 7)
#[test]
fn test_prfh_i_p_bi_s_combo_4_2000_85c72000() {
    // Encoding: 0x85C72000
    // Test PRFH_I.P.BI_S field combination: imm6=7, Pg=0, Rn=0, prfop=0
    // Fields: imm6=7, Rn=0, prfop=0, Pg=0
    let encoding: u32 = 0x85C72000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=8 (power of 2 (2^3 = 8))
#[test]
fn test_prfh_i_p_bi_s_combo_5_2000_85c82000() {
    // Encoding: 0x85C82000
    // Test PRFH_I.P.BI_S field combination: imm6=8, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, imm6=8, Pg=0, prfop=0
    let encoding: u32 = 0x85C82000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=15 (2^4 - 1 = 15)
#[test]
fn test_prfh_i_p_bi_s_combo_6_2000_85cf2000() {
    // Encoding: 0x85CF2000
    // Test PRFH_I.P.BI_S field combination: imm6=15, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, imm6=15, prfop=0
    let encoding: u32 = 0x85CF2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=16 (power of 2 (2^4 = 16))
#[test]
fn test_prfh_i_p_bi_s_combo_7_2000_85d02000() {
    // Encoding: 0x85D02000
    // Test PRFH_I.P.BI_S field combination: imm6=16, Pg=0, Rn=0, prfop=0
    // Fields: imm6=16, Rn=0, Pg=0, prfop=0
    let encoding: u32 = 0x85D02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=31 (immediate midpoint (31))
#[test]
fn test_prfh_i_p_bi_s_combo_8_2000_85df2000() {
    // Encoding: 0x85DF2000
    // Test PRFH_I.P.BI_S field combination: imm6=31, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, imm6=31, Pg=0, prfop=0
    let encoding: u32 = 0x85DF2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=32 (power of 2 (2^5 = 32))
#[test]
fn test_prfh_i_p_bi_s_combo_9_2000_85e02000() {
    // Encoding: 0x85E02000
    // Test PRFH_I.P.BI_S field combination: imm6=32, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, imm6=32, Rn=0, Pg=0
    let encoding: u32 = 0x85E02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=63 (maximum immediate (63))
#[test]
fn test_prfh_i_p_bi_s_combo_10_2000_85ff2000() {
    // Encoding: 0x85FF2000
    // Test PRFH_I.P.BI_S field combination: imm6=63, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, prfop=0, Rn=0, imm6=63
    let encoding: u32 = 0x85FF2000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfh_i_p_bi_s_combo_11_2000_85c02000() {
    // Encoding: 0x85C02000
    // Test PRFH_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: imm6=0, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0x85C02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfh_i_p_bi_s_combo_12_2000_85c02400() {
    // Encoding: 0x85C02400
    // Test PRFH_I.P.BI_S field combination: imm6=0, Pg=1, Rn=0, prfop=0
    // Fields: Rn=0, Pg=1, prfop=0, imm6=0
    let encoding: u32 = 0x85C02400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfh_i_p_bi_s_combo_13_2000_85c02000() {
    // Encoding: 0x85C02000
    // Test PRFH_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, prfop=0, imm6=0
    let encoding: u32 = 0x85C02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfh_i_p_bi_s_combo_14_2000_85c02020() {
    // Encoding: 0x85C02020
    // Test PRFH_I.P.BI_S field combination: imm6=0, Pg=0, Rn=1, prfop=0
    // Fields: imm6=0, Pg=0, prfop=0, Rn=1
    let encoding: u32 = 0x85C02020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfh_i_p_bi_s_combo_15_2000_85c023c0() {
    // Encoding: 0x85C023C0
    // Test PRFH_I.P.BI_S field combination: imm6=0, Pg=0, Rn=30, prfop=0
    // Fields: prfop=0, Pg=0, Rn=30, imm6=0
    let encoding: u32 = 0x85C023C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfh_i_p_bi_s_combo_16_2000_85c023e0() {
    // Encoding: 0x85C023E0
    // Test PRFH_I.P.BI_S field combination: imm6=0, Pg=0, Rn=31, prfop=0
    // Fields: Pg=0, imm6=0, Rn=31, prfop=0
    let encoding: u32 = 0x85C023E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfh_i_p_bi_s_combo_17_2000_85c02000() {
    // Encoding: 0x85C02000
    // Test PRFH_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, prfop=0, Rn=0, imm6=0
    let encoding: u32 = 0x85C02000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfh_i_p_bi_s_combo_18_2000_85c02001() {
    // Encoding: 0x85C02001
    // Test PRFH_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=1
    // Fields: Rn=0, Pg=0, imm6=0, prfop=1
    let encoding: u32 = 0x85C02001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfh_i_p_bi_s_combo_19_2000_85c02007() {
    // Encoding: 0x85C02007
    // Test PRFH_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=7
    // Fields: imm6=0, Pg=0, prfop=7, Rn=0
    let encoding: u32 = 0x85C02007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfh_i_p_bi_s_combo_20_2000_85c0200f() {
    // Encoding: 0x85C0200F
    // Test PRFH_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=15
    // Fields: imm6=0, prfop=15, Rn=0, Pg=0
    let encoding: u32 = 0x85C0200F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfh_i_p_bi_s_combo_21_2000_85c02420() {
    // Encoding: 0x85C02420
    // Test PRFH_I.P.BI_S field combination: imm6=0, Pg=1, Rn=1, prfop=0
    // Fields: Pg=1, Rn=1, imm6=0, prfop=0
    let encoding: u32 = 0x85C02420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfh_i_p_bi_s_combo_22_2000_85c03fe0() {
    // Encoding: 0x85C03FE0
    // Test PRFH_I.P.BI_S field combination: imm6=0, Pg=31, Rn=31, prfop=0
    // Fields: Pg=31, imm6=0, prfop=0, Rn=31
    let encoding: u32 = 0x85C03FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfh_i_p_bi_s_special_rn_31_stack_pointer_sp_may_require_alignment_8192_85c123e0() {
    // Encoding: 0x85C123E0
    // Test PRFH_I.P.BI_S special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Pg=0, Rn=31, prfop=0, imm6=1
    let encoding: u32 = 0x85C123E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BI_S
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfh_i_p_bi_s_sp_rn_85c023e0() {
    // Test PRFH_I.P.BI_S with Rn = SP (31)
    // Encoding: 0x85C023E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x85C023E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PRFB_I.P.BI_S Tests
// ============================================================================

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_prfb_i_p_bi_s_field_imm6_0_zero_0_85c00000() {
    // Encoding: 0x85C00000
    // Test PRFB_I.P.BI_S field imm6 = 0 (Zero)
    // Fields: prfop=0, Rn=0, Pg=0, imm6=0
    let encoding: u32 = 0x85C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_prfb_i_p_bi_s_field_imm6_1_poweroftwo_0_85c10000() {
    // Encoding: 0x85C10000
    // Test PRFB_I.P.BI_S field imm6 = 1 (PowerOfTwo)
    // Fields: imm6=1, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0x85C10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_prfb_i_p_bi_s_field_imm6_3_poweroftwominusone_0_85c30000() {
    // Encoding: 0x85C30000
    // Test PRFB_I.P.BI_S field imm6 = 3 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Rn=0, prfop=0, imm6=3
    let encoding: u32 = 0x85C30000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_prfb_i_p_bi_s_field_imm6_4_poweroftwo_0_85c40000() {
    // Encoding: 0x85C40000
    // Test PRFB_I.P.BI_S field imm6 = 4 (PowerOfTwo)
    // Fields: Pg=0, prfop=0, Rn=0, imm6=4
    let encoding: u32 = 0x85C40000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_prfb_i_p_bi_s_field_imm6_7_poweroftwominusone_0_85c70000() {
    // Encoding: 0x85C70000
    // Test PRFB_I.P.BI_S field imm6 = 7 (PowerOfTwoMinusOne)
    // Fields: imm6=7, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0x85C70000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_prfb_i_p_bi_s_field_imm6_8_poweroftwo_0_85c80000() {
    // Encoding: 0x85C80000
    // Test PRFB_I.P.BI_S field imm6 = 8 (PowerOfTwo)
    // Fields: Pg=0, Rn=0, imm6=8, prfop=0
    let encoding: u32 = 0x85C80000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_prfb_i_p_bi_s_field_imm6_15_poweroftwominusone_0_85cf0000() {
    // Encoding: 0x85CF0000
    // Test PRFB_I.P.BI_S field imm6 = 15 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Pg=0, imm6=15, Rn=0
    let encoding: u32 = 0x85CF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_prfb_i_p_bi_s_field_imm6_16_poweroftwo_0_85d00000() {
    // Encoding: 0x85D00000
    // Test PRFB_I.P.BI_S field imm6 = 16 (PowerOfTwo)
    // Fields: Rn=0, imm6=16, prfop=0, Pg=0
    let encoding: u32 = 0x85D00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_prfb_i_p_bi_s_field_imm6_31_poweroftwominusone_0_85df0000() {
    // Encoding: 0x85DF0000
    // Test PRFB_I.P.BI_S field imm6 = 31 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Pg=0, imm6=31, prfop=0
    let encoding: u32 = 0x85DF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_prfb_i_p_bi_s_field_imm6_32_poweroftwo_0_85e00000() {
    // Encoding: 0x85E00000
    // Test PRFB_I.P.BI_S field imm6 = 32 (PowerOfTwo)
    // Fields: prfop=0, Pg=0, Rn=0, imm6=32
    let encoding: u32 = 0x85E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_prfb_i_p_bi_s_field_imm6_63_max_0_85ff0000() {
    // Encoding: 0x85FF0000
    // Test PRFB_I.P.BI_S field imm6 = 63 (Max)
    // Fields: Pg=0, Rn=0, imm6=63, prfop=0
    let encoding: u32 = 0x85FF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfb_i_p_bi_s_field_pg_0_min_0_85c00000() {
    // Encoding: 0x85C00000
    // Test PRFB_I.P.BI_S field Pg = 0 (Min)
    // Fields: Pg=0, imm6=0, Rn=0, prfop=0
    let encoding: u32 = 0x85C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfb_i_p_bi_s_field_pg_1_poweroftwo_0_85c00400() {
    // Encoding: 0x85C00400
    // Test PRFB_I.P.BI_S field Pg = 1 (PowerOfTwo)
    // Fields: imm6=0, prfop=0, Pg=1, Rn=0
    let encoding: u32 = 0x85C00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfb_i_p_bi_s_field_rn_0_min_0_85c00000() {
    // Encoding: 0x85C00000
    // Test PRFB_I.P.BI_S field Rn = 0 (Min)
    // Fields: Rn=0, imm6=0, prfop=0, Pg=0
    let encoding: u32 = 0x85C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfb_i_p_bi_s_field_rn_1_poweroftwo_0_85c00020() {
    // Encoding: 0x85C00020
    // Test PRFB_I.P.BI_S field Rn = 1 (PowerOfTwo)
    // Fields: imm6=0, Pg=0, Rn=1, prfop=0
    let encoding: u32 = 0x85C00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfb_i_p_bi_s_field_rn_30_poweroftwominusone_0_85c003c0() {
    // Encoding: 0x85C003C0
    // Test PRFB_I.P.BI_S field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, prfop=0, Rn=30, imm6=0
    let encoding: u32 = 0x85C003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfb_i_p_bi_s_field_rn_31_max_0_85c003e0() {
    // Encoding: 0x85C003E0
    // Test PRFB_I.P.BI_S field Rn = 31 (Max)
    // Fields: imm6=0, prfop=0, Pg=0, Rn=31
    let encoding: u32 = 0x85C003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfb_i_p_bi_s_field_prfop_0_min_0_85c00000() {
    // Encoding: 0x85C00000
    // Test PRFB_I.P.BI_S field prfop = 0 (Min)
    // Fields: Rn=0, Pg=0, imm6=0, prfop=0
    let encoding: u32 = 0x85C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfb_i_p_bi_s_field_prfop_1_poweroftwo_0_85c00001() {
    // Encoding: 0x85C00001
    // Test PRFB_I.P.BI_S field prfop = 1 (PowerOfTwo)
    // Fields: Pg=0, prfop=1, Rn=0, imm6=0
    let encoding: u32 = 0x85C00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfb_i_p_bi_s_field_prfop_7_poweroftwominusone_0_85c00007() {
    // Encoding: 0x85C00007
    // Test PRFB_I.P.BI_S field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Pg=0, imm6=0, prfop=7
    let encoding: u32 = 0x85C00007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfb_i_p_bi_s_field_prfop_15_max_0_85c0000f() {
    // Encoding: 0x85C0000F
    // Test PRFB_I.P.BI_S field prfop = 15 (Max)
    // Fields: Rn=0, prfop=15, Pg=0, imm6=0
    let encoding: u32 = 0x85C0000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=0 (immediate value 0)
#[test]
fn test_prfb_i_p_bi_s_combo_0_0_85c00000() {
    // Encoding: 0x85C00000
    // Test PRFB_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: imm6=0, prfop=0, Rn=0, Pg=0
    let encoding: u32 = 0x85C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=1 (immediate value 1)
#[test]
fn test_prfb_i_p_bi_s_combo_1_0_85c10000() {
    // Encoding: 0x85C10000
    // Test PRFB_I.P.BI_S field combination: imm6=1, Pg=0, Rn=0, prfop=0
    // Fields: imm6=1, Rn=0, Pg=0, prfop=0
    let encoding: u32 = 0x85C10000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=3 (2^2 - 1 = 3)
#[test]
fn test_prfb_i_p_bi_s_combo_2_0_85c30000() {
    // Encoding: 0x85C30000
    // Test PRFB_I.P.BI_S field combination: imm6=3, Pg=0, Rn=0, prfop=0
    // Fields: imm6=3, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0x85C30000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=4 (power of 2 (2^2 = 4))
#[test]
fn test_prfb_i_p_bi_s_combo_3_0_85c40000() {
    // Encoding: 0x85C40000
    // Test PRFB_I.P.BI_S field combination: imm6=4, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, imm6=4, Pg=0, prfop=0
    let encoding: u32 = 0x85C40000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=7 (2^3 - 1 = 7)
#[test]
fn test_prfb_i_p_bi_s_combo_4_0_85c70000() {
    // Encoding: 0x85C70000
    // Test PRFB_I.P.BI_S field combination: imm6=7, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rn=0, prfop=0, imm6=7
    let encoding: u32 = 0x85C70000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=8 (power of 2 (2^3 = 8))
#[test]
fn test_prfb_i_p_bi_s_combo_5_0_85c80000() {
    // Encoding: 0x85C80000
    // Test PRFB_I.P.BI_S field combination: imm6=8, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Pg=0, Rn=0, imm6=8
    let encoding: u32 = 0x85C80000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=15 (2^4 - 1 = 15)
#[test]
fn test_prfb_i_p_bi_s_combo_6_0_85cf0000() {
    // Encoding: 0x85CF0000
    // Test PRFB_I.P.BI_S field combination: imm6=15, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, prfop=0, imm6=15, Pg=0
    let encoding: u32 = 0x85CF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=16 (power of 2 (2^4 = 16))
#[test]
fn test_prfb_i_p_bi_s_combo_7_0_85d00000() {
    // Encoding: 0x85D00000
    // Test PRFB_I.P.BI_S field combination: imm6=16, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, prfop=0, Pg=0, imm6=16
    let encoding: u32 = 0x85D00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=31 (immediate midpoint (31))
#[test]
fn test_prfb_i_p_bi_s_combo_8_0_85df0000() {
    // Encoding: 0x85DF0000
    // Test PRFB_I.P.BI_S field combination: imm6=31, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, imm6=31, prfop=0, Rn=0
    let encoding: u32 = 0x85DF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=32 (power of 2 (2^5 = 32))
#[test]
fn test_prfb_i_p_bi_s_combo_9_0_85e00000() {
    // Encoding: 0x85E00000
    // Test PRFB_I.P.BI_S field combination: imm6=32, Pg=0, Rn=0, prfop=0
    // Fields: imm6=32, Rn=0, Pg=0, prfop=0
    let encoding: u32 = 0x85E00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=63 (maximum immediate (63))
#[test]
fn test_prfb_i_p_bi_s_combo_10_0_85ff0000() {
    // Encoding: 0x85FF0000
    // Test PRFB_I.P.BI_S field combination: imm6=63, Pg=0, Rn=0, prfop=0
    // Fields: imm6=63, prfop=0, Rn=0, Pg=0
    let encoding: u32 = 0x85FF0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfb_i_p_bi_s_combo_11_0_85c00000() {
    // Encoding: 0x85C00000
    // Test PRFB_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rn=0, prfop=0, imm6=0
    let encoding: u32 = 0x85C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfb_i_p_bi_s_combo_12_0_85c00400() {
    // Encoding: 0x85C00400
    // Test PRFB_I.P.BI_S field combination: imm6=0, Pg=1, Rn=0, prfop=0
    // Fields: prfop=0, imm6=0, Pg=1, Rn=0
    let encoding: u32 = 0x85C00400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfb_i_p_bi_s_combo_13_0_85c00000() {
    // Encoding: 0x85C00000
    // Test PRFB_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: imm6=0, prfop=0, Rn=0, Pg=0
    let encoding: u32 = 0x85C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfb_i_p_bi_s_combo_14_0_85c00020() {
    // Encoding: 0x85C00020
    // Test PRFB_I.P.BI_S field combination: imm6=0, Pg=0, Rn=1, prfop=0
    // Fields: Rn=1, Pg=0, imm6=0, prfop=0
    let encoding: u32 = 0x85C00020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfb_i_p_bi_s_combo_15_0_85c003c0() {
    // Encoding: 0x85C003C0
    // Test PRFB_I.P.BI_S field combination: imm6=0, Pg=0, Rn=30, prfop=0
    // Fields: Pg=0, prfop=0, Rn=30, imm6=0
    let encoding: u32 = 0x85C003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfb_i_p_bi_s_combo_16_0_85c003e0() {
    // Encoding: 0x85C003E0
    // Test PRFB_I.P.BI_S field combination: imm6=0, Pg=0, Rn=31, prfop=0
    // Fields: Pg=0, Rn=31, prfop=0, imm6=0
    let encoding: u32 = 0x85C003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfb_i_p_bi_s_combo_17_0_85c00000() {
    // Encoding: 0x85C00000
    // Test PRFB_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, prfop=0, imm6=0
    let encoding: u32 = 0x85C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfb_i_p_bi_s_combo_18_0_85c00001() {
    // Encoding: 0x85C00001
    // Test PRFB_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=1
    // Fields: Pg=0, imm6=0, Rn=0, prfop=1
    let encoding: u32 = 0x85C00001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfb_i_p_bi_s_combo_19_0_85c00007() {
    // Encoding: 0x85C00007
    // Test PRFB_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=7
    // Fields: Pg=0, imm6=0, prfop=7, Rn=0
    let encoding: u32 = 0x85C00007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfb_i_p_bi_s_combo_20_0_85c0000f() {
    // Encoding: 0x85C0000F
    // Test PRFB_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=15
    // Fields: Rn=0, prfop=15, Pg=0, imm6=0
    let encoding: u32 = 0x85C0000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfb_i_p_bi_s_combo_21_0_85c00420() {
    // Encoding: 0x85C00420
    // Test PRFB_I.P.BI_S field combination: imm6=0, Pg=1, Rn=1, prfop=0
    // Fields: prfop=0, Pg=1, Rn=1, imm6=0
    let encoding: u32 = 0x85C00420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfb_i_p_bi_s_combo_22_0_85c01fe0() {
    // Encoding: 0x85C01FE0
    // Test PRFB_I.P.BI_S field combination: imm6=0, Pg=31, Rn=31, prfop=0
    // Fields: imm6=0, prfop=0, Pg=31, Rn=31
    let encoding: u32 = 0x85C01FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfb_i_p_bi_s_special_rn_31_stack_pointer_sp_may_require_alignment_0_85c103e0() {
    // Encoding: 0x85C103E0
    // Test PRFB_I.P.BI_S special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: prfop=0, Pg=0, Rn=31, imm6=1
    let encoding: u32 = 0x85C103E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BI_S
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfb_i_p_bi_s_sp_rn_85c003e0() {
    // Test PRFB_I.P.BI_S with Rn = SP (31)
    // Encoding: 0x85C003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x85C003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PRFD_I.P.BR_S Tests
// ============================================================================

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfd_i_p_br_s_field_rm_0_min_c000_8580c000() {
    // Encoding: 0x8580C000
    // Test PRFD_I.P.BR_S field Rm = 0 (Min)
    // Fields: Rn=0, Pg=0, Rm=0, prfop=0
    let encoding: u32 = 0x8580C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfd_i_p_br_s_field_rm_1_poweroftwo_c000_8581c000() {
    // Encoding: 0x8581C000
    // Test PRFD_I.P.BR_S field Rm = 1 (PowerOfTwo)
    // Fields: prfop=0, Rm=1, Rn=0, Pg=0
    let encoding: u32 = 0x8581C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfd_i_p_br_s_field_rm_30_poweroftwominusone_c000_859ec000() {
    // Encoding: 0x859EC000
    // Test PRFD_I.P.BR_S field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Rn=0, Rm=30, prfop=0
    let encoding: u32 = 0x859EC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_prfd_i_p_br_s_field_rm_31_max_c000_859fc000() {
    // Encoding: 0x859FC000
    // Test PRFD_I.P.BR_S field Rm = 31 (Max)
    // Fields: Pg=0, Rm=31, Rn=0, prfop=0
    let encoding: u32 = 0x859FC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfd_i_p_br_s_field_pg_0_min_c000_8580c000() {
    // Encoding: 0x8580C000
    // Test PRFD_I.P.BR_S field Pg = 0 (Min)
    // Fields: prfop=0, Rn=0, Rm=0, Pg=0
    let encoding: u32 = 0x8580C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfd_i_p_br_s_field_pg_1_poweroftwo_c000_8580c400() {
    // Encoding: 0x8580C400
    // Test PRFD_I.P.BR_S field Pg = 1 (PowerOfTwo)
    // Fields: Rn=0, Rm=0, prfop=0, Pg=1
    let encoding: u32 = 0x8580C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfd_i_p_br_s_field_rn_0_min_c000_8580c000() {
    // Encoding: 0x8580C000
    // Test PRFD_I.P.BR_S field Rn = 0 (Min)
    // Fields: Pg=0, Rn=0, prfop=0, Rm=0
    let encoding: u32 = 0x8580C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfd_i_p_br_s_field_rn_1_poweroftwo_c000_8580c020() {
    // Encoding: 0x8580C020
    // Test PRFD_I.P.BR_S field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Rm=0, Pg=0, prfop=0
    let encoding: u32 = 0x8580C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfd_i_p_br_s_field_rn_30_poweroftwominusone_c000_8580c3c0() {
    // Encoding: 0x8580C3C0
    // Test PRFD_I.P.BR_S field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, Pg=0, prfop=0, Rn=30
    let encoding: u32 = 0x8580C3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfd_i_p_br_s_field_rn_31_max_c000_8580c3e0() {
    // Encoding: 0x8580C3E0
    // Test PRFD_I.P.BR_S field Rn = 31 (Max)
    // Fields: Rm=0, Pg=0, Rn=31, prfop=0
    let encoding: u32 = 0x8580C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfd_i_p_br_s_field_prfop_0_min_c000_8580c000() {
    // Encoding: 0x8580C000
    // Test PRFD_I.P.BR_S field prfop = 0 (Min)
    // Fields: Rm=0, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0x8580C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfd_i_p_br_s_field_prfop_1_poweroftwo_c000_8580c001() {
    // Encoding: 0x8580C001
    // Test PRFD_I.P.BR_S field prfop = 1 (PowerOfTwo)
    // Fields: Rm=0, prfop=1, Rn=0, Pg=0
    let encoding: u32 = 0x8580C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfd_i_p_br_s_field_prfop_7_poweroftwominusone_c000_8580c007() {
    // Encoding: 0x8580C007
    // Test PRFD_I.P.BR_S field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Rm=0, prfop=7, Rn=0
    let encoding: u32 = 0x8580C007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfd_i_p_br_s_field_prfop_15_max_c000_8580c00f() {
    // Encoding: 0x8580C00F
    // Test PRFD_I.P.BR_S field prfop = 15 (Max)
    // Fields: prfop=15, Rn=0, Pg=0, Rm=0
    let encoding: u32 = 0x8580C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_prfd_i_p_br_s_combo_0_c000_8580c000() {
    // Encoding: 0x8580C000
    // Test PRFD_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rm=0, prfop=0, Rn=0
    let encoding: u32 = 0x8580C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_prfd_i_p_br_s_combo_1_c000_8581c000() {
    // Encoding: 0x8581C000
    // Test PRFD_I.P.BR_S field combination: Rm=1, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rm=1, Rn=0, prfop=0
    let encoding: u32 = 0x8581C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfd_i_p_br_s_combo_2_c000_859ec000() {
    // Encoding: 0x859EC000
    // Test PRFD_I.P.BR_S field combination: Rm=30, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rm=30, Rn=0, prfop=0
    let encoding: u32 = 0x859EC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_prfd_i_p_br_s_combo_3_c000_859fc000() {
    // Encoding: 0x859FC000
    // Test PRFD_I.P.BR_S field combination: Rm=31, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Pg=0, Rn=0, Rm=31
    let encoding: u32 = 0x859FC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfd_i_p_br_s_combo_4_c000_8580c000() {
    // Encoding: 0x8580C000
    // Test PRFD_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rn=0, prfop=0, Rm=0
    let encoding: u32 = 0x8580C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfd_i_p_br_s_combo_5_c000_8580c400() {
    // Encoding: 0x8580C400
    // Test PRFD_I.P.BR_S field combination: Rm=0, Pg=1, Rn=0, prfop=0
    // Fields: Rn=0, Rm=0, Pg=1, prfop=0
    let encoding: u32 = 0x8580C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfd_i_p_br_s_combo_6_c000_8580c000() {
    // Encoding: 0x8580C000
    // Test PRFD_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Rm=0, prfop=0, Pg=0
    let encoding: u32 = 0x8580C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfd_i_p_br_s_combo_7_c000_8580c020() {
    // Encoding: 0x8580C020
    // Test PRFD_I.P.BR_S field combination: Rm=0, Pg=0, Rn=1, prfop=0
    // Fields: Rm=0, prfop=0, Rn=1, Pg=0
    let encoding: u32 = 0x8580C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfd_i_p_br_s_combo_8_c000_8580c3c0() {
    // Encoding: 0x8580C3C0
    // Test PRFD_I.P.BR_S field combination: Rm=0, Pg=0, Rn=30, prfop=0
    // Fields: Rm=0, prfop=0, Rn=30, Pg=0
    let encoding: u32 = 0x8580C3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfd_i_p_br_s_combo_9_c000_8580c3e0() {
    // Encoding: 0x8580C3E0
    // Test PRFD_I.P.BR_S field combination: Rm=0, Pg=0, Rn=31, prfop=0
    // Fields: Rm=0, Pg=0, Rn=31, prfop=0
    let encoding: u32 = 0x8580C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfd_i_p_br_s_combo_10_c000_8580c000() {
    // Encoding: 0x8580C000
    // Test PRFD_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rm=0, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0x8580C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfd_i_p_br_s_combo_11_c000_8580c001() {
    // Encoding: 0x8580C001
    // Test PRFD_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=1
    // Fields: Pg=0, Rn=0, Rm=0, prfop=1
    let encoding: u32 = 0x8580C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfd_i_p_br_s_combo_12_c000_8580c007() {
    // Encoding: 0x8580C007
    // Test PRFD_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=7
    // Fields: Pg=0, Rn=0, Rm=0, prfop=7
    let encoding: u32 = 0x8580C007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfd_i_p_br_s_combo_13_c000_8580c00f() {
    // Encoding: 0x8580C00F
    // Test PRFD_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=15
    // Fields: Rn=0, prfop=15, Pg=0, Rm=0
    let encoding: u32 = 0x8580C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_prfd_i_p_br_s_combo_14_c000_8581c400() {
    // Encoding: 0x8581C400
    // Test PRFD_I.P.BR_S field combination: Rm=1, Pg=1, Rn=0, prfop=0
    // Fields: Rm=1, Rn=0, prfop=0, Pg=1
    let encoding: u32 = 0x8581C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_prfd_i_p_br_s_combo_15_c000_859fdc00() {
    // Encoding: 0x859FDC00
    // Test PRFD_I.P.BR_S field combination: Rm=31, Pg=31, Rn=0, prfop=0
    // Fields: Rn=0, Pg=31, prfop=0, Rm=31
    let encoding: u32 = 0x859FDC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfd_i_p_br_s_combo_16_c000_8581c020() {
    // Encoding: 0x8581C020
    // Test PRFD_I.P.BR_S field combination: Rm=1, Pg=0, Rn=1, prfop=0
    // Fields: prfop=0, Rm=1, Rn=1, Pg=0
    let encoding: u32 = 0x8581C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfd_i_p_br_s_combo_17_c000_859fc3e0() {
    // Encoding: 0x859FC3E0
    // Test PRFD_I.P.BR_S field combination: Rm=31, Pg=0, Rn=31, prfop=0
    // Fields: Pg=0, Rn=31, Rm=31, prfop=0
    let encoding: u32 = 0x859FC3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfd_i_p_br_s_combo_18_c000_8580c420() {
    // Encoding: 0x8580C420
    // Test PRFD_I.P.BR_S field combination: Rm=0, Pg=1, Rn=1, prfop=0
    // Fields: Rn=1, Rm=0, Pg=1, prfop=0
    let encoding: u32 = 0x8580C420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfd_i_p_br_s_combo_19_c000_8580dfe0() {
    // Encoding: 0x8580DFE0
    // Test PRFD_I.P.BR_S field combination: Rm=0, Pg=31, Rn=31, prfop=0
    // Fields: Rm=0, prfop=0, Rn=31, Pg=31
    let encoding: u32 = 0x8580DFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfd_i_p_br_s_special_rn_31_stack_pointer_sp_may_require_alignment_49152_8580c3e0() {
    // Encoding: 0x8580C3E0
    // Test PRFD_I.P.BR_S special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Pg=0, prfop=0, Rm=0
    let encoding: u32 = 0x8580C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BR_S
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfd_i_p_br_s_sp_rn_8580c3e0() {
    // Test PRFD_I.P.BR_S with Rn = SP (31)
    // Encoding: 0x8580C3E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x8580C3E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PRFD_I.P.AI_S Tests
// ============================================================================

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_prfd_i_p_ai_s_field_imm5_0_zero_e000_8580e000() {
    // Encoding: 0x8580E000
    // Test PRFD_I.P.AI_S field imm5 = 0 (Zero)
    // Fields: imm5=0, Pg=0, prfop=0, Zn=0
    let encoding: u32 = 0x8580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_prfd_i_p_ai_s_field_imm5_1_poweroftwo_e000_8581e000() {
    // Encoding: 0x8581E000
    // Test PRFD_I.P.AI_S field imm5 = 1 (PowerOfTwo)
    // Fields: Pg=0, imm5=1, Zn=0, prfop=0
    let encoding: u32 = 0x8581E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_prfd_i_p_ai_s_field_imm5_3_poweroftwominusone_e000_8583e000() {
    // Encoding: 0x8583E000
    // Test PRFD_I.P.AI_S field imm5 = 3 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Zn=0, imm5=3, Pg=0
    let encoding: u32 = 0x8583E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_prfd_i_p_ai_s_field_imm5_4_poweroftwo_e000_8584e000() {
    // Encoding: 0x8584E000
    // Test PRFD_I.P.AI_S field imm5 = 4 (PowerOfTwo)
    // Fields: Pg=0, prfop=0, Zn=0, imm5=4
    let encoding: u32 = 0x8584E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_prfd_i_p_ai_s_field_imm5_7_poweroftwominusone_e000_8587e000() {
    // Encoding: 0x8587E000
    // Test PRFD_I.P.AI_S field imm5 = 7 (PowerOfTwoMinusOne)
    // Fields: prfop=0, imm5=7, Zn=0, Pg=0
    let encoding: u32 = 0x8587E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_prfd_i_p_ai_s_field_imm5_8_poweroftwo_e000_8588e000() {
    // Encoding: 0x8588E000
    // Test PRFD_I.P.AI_S field imm5 = 8 (PowerOfTwo)
    // Fields: imm5=8, Pg=0, Zn=0, prfop=0
    let encoding: u32 = 0x8588E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_prfd_i_p_ai_s_field_imm5_15_poweroftwominusone_e000_858fe000() {
    // Encoding: 0x858FE000
    // Test PRFD_I.P.AI_S field imm5 = 15 (PowerOfTwoMinusOne)
    // Fields: imm5=15, Zn=0, Pg=0, prfop=0
    let encoding: u32 = 0x858FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_prfd_i_p_ai_s_field_imm5_16_poweroftwo_e000_8590e000() {
    // Encoding: 0x8590E000
    // Test PRFD_I.P.AI_S field imm5 = 16 (PowerOfTwo)
    // Fields: Zn=0, prfop=0, imm5=16, Pg=0
    let encoding: u32 = 0x8590E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_prfd_i_p_ai_s_field_imm5_31_max_e000_859fe000() {
    // Encoding: 0x859FE000
    // Test PRFD_I.P.AI_S field imm5 = 31 (Max)
    // Fields: Pg=0, prfop=0, Zn=0, imm5=31
    let encoding: u32 = 0x859FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfd_i_p_ai_s_field_pg_0_min_e000_8580e000() {
    // Encoding: 0x8580E000
    // Test PRFD_I.P.AI_S field Pg = 0 (Min)
    // Fields: Zn=0, prfop=0, imm5=0, Pg=0
    let encoding: u32 = 0x8580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfd_i_p_ai_s_field_pg_1_poweroftwo_e000_8580e400() {
    // Encoding: 0x8580E400
    // Test PRFD_I.P.AI_S field Pg = 1 (PowerOfTwo)
    // Fields: prfop=0, Zn=0, imm5=0, Pg=1
    let encoding: u32 = 0x8580E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfd_i_p_ai_s_field_zn_0_min_e000_8580e000() {
    // Encoding: 0x8580E000
    // Test PRFD_I.P.AI_S field Zn = 0 (Min)
    // Fields: Zn=0, imm5=0, Pg=0, prfop=0
    let encoding: u32 = 0x8580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfd_i_p_ai_s_field_zn_1_poweroftwo_e000_8580e020() {
    // Encoding: 0x8580E020
    // Test PRFD_I.P.AI_S field Zn = 1 (PowerOfTwo)
    // Fields: imm5=0, Pg=0, Zn=1, prfop=0
    let encoding: u32 = 0x8580E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfd_i_p_ai_s_field_zn_30_poweroftwominusone_e000_8580e3c0() {
    // Encoding: 0x8580E3C0
    // Test PRFD_I.P.AI_S field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Zn=30, imm5=0, prfop=0, Pg=0
    let encoding: u32 = 0x8580E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfd_i_p_ai_s_field_zn_31_max_e000_8580e3e0() {
    // Encoding: 0x8580E3E0
    // Test PRFD_I.P.AI_S field Zn = 31 (Max)
    // Fields: Zn=31, Pg=0, imm5=0, prfop=0
    let encoding: u32 = 0x8580E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfd_i_p_ai_s_field_prfop_0_min_e000_8580e000() {
    // Encoding: 0x8580E000
    // Test PRFD_I.P.AI_S field prfop = 0 (Min)
    // Fields: imm5=0, prfop=0, Pg=0, Zn=0
    let encoding: u32 = 0x8580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfd_i_p_ai_s_field_prfop_1_poweroftwo_e000_8580e001() {
    // Encoding: 0x8580E001
    // Test PRFD_I.P.AI_S field prfop = 1 (PowerOfTwo)
    // Fields: imm5=0, prfop=1, Pg=0, Zn=0
    let encoding: u32 = 0x8580E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfd_i_p_ai_s_field_prfop_7_poweroftwominusone_e000_8580e007() {
    // Encoding: 0x8580E007
    // Test PRFD_I.P.AI_S field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: imm5=0, Pg=0, Zn=0, prfop=7
    let encoding: u32 = 0x8580E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfd_i_p_ai_s_field_prfop_15_max_e000_8580e00f() {
    // Encoding: 0x8580E00F
    // Test PRFD_I.P.AI_S field prfop = 15 (Max)
    // Fields: Zn=0, Pg=0, imm5=0, prfop=15
    let encoding: u32 = 0x8580E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=0 (immediate value 0)
#[test]
fn test_prfd_i_p_ai_s_combo_0_e000_8580e000() {
    // Encoding: 0x8580E000
    // Test PRFD_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, imm5=0, Zn=0, prfop=0
    let encoding: u32 = 0x8580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=1 (immediate value 1)
#[test]
fn test_prfd_i_p_ai_s_combo_1_e000_8581e000() {
    // Encoding: 0x8581E000
    // Test PRFD_I.P.AI_S field combination: imm5=1, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, prfop=0, imm5=1, Zn=0
    let encoding: u32 = 0x8581E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=3 (2^2 - 1 = 3)
#[test]
fn test_prfd_i_p_ai_s_combo_2_e000_8583e000() {
    // Encoding: 0x8583E000
    // Test PRFD_I.P.AI_S field combination: imm5=3, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, Pg=0, prfop=0, imm5=3
    let encoding: u32 = 0x8583E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=4 (power of 2 (2^2 = 4))
#[test]
fn test_prfd_i_p_ai_s_combo_3_e000_8584e000() {
    // Encoding: 0x8584E000
    // Test PRFD_I.P.AI_S field combination: imm5=4, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, prfop=0, Pg=0, imm5=4
    let encoding: u32 = 0x8584E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=7 (2^3 - 1 = 7)
#[test]
fn test_prfd_i_p_ai_s_combo_4_e000_8587e000() {
    // Encoding: 0x8587E000
    // Test PRFD_I.P.AI_S field combination: imm5=7, Pg=0, Zn=0, prfop=0
    // Fields: imm5=7, Pg=0, Zn=0, prfop=0
    let encoding: u32 = 0x8587E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=8 (power of 2 (2^3 = 8))
#[test]
fn test_prfd_i_p_ai_s_combo_5_e000_8588e000() {
    // Encoding: 0x8588E000
    // Test PRFD_I.P.AI_S field combination: imm5=8, Pg=0, Zn=0, prfop=0
    // Fields: imm5=8, Pg=0, prfop=0, Zn=0
    let encoding: u32 = 0x8588E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=15 (immediate midpoint (15))
#[test]
fn test_prfd_i_p_ai_s_combo_6_e000_858fe000() {
    // Encoding: 0x858FE000
    // Test PRFD_I.P.AI_S field combination: imm5=15, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, Zn=0, imm5=15, Pg=0
    let encoding: u32 = 0x858FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=16 (power of 2 (2^4 = 16))
#[test]
fn test_prfd_i_p_ai_s_combo_7_e000_8590e000() {
    // Encoding: 0x8590E000
    // Test PRFD_I.P.AI_S field combination: imm5=16, Pg=0, Zn=0, prfop=0
    // Fields: imm5=16, Zn=0, Pg=0, prfop=0
    let encoding: u32 = 0x8590E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=31 (maximum immediate (31))
#[test]
fn test_prfd_i_p_ai_s_combo_8_e000_859fe000() {
    // Encoding: 0x859FE000
    // Test PRFD_I.P.AI_S field combination: imm5=31, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, Zn=0, imm5=31, prfop=0
    let encoding: u32 = 0x859FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfd_i_p_ai_s_combo_9_e000_8580e000() {
    // Encoding: 0x8580E000
    // Test PRFD_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, imm5=0, Zn=0, prfop=0
    let encoding: u32 = 0x8580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfd_i_p_ai_s_combo_10_e000_8580e400() {
    // Encoding: 0x8580E400
    // Test PRFD_I.P.AI_S field combination: imm5=0, Pg=1, Zn=0, prfop=0
    // Fields: prfop=0, Zn=0, imm5=0, Pg=1
    let encoding: u32 = 0x8580E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_prfd_i_p_ai_s_combo_11_e000_8580e000() {
    // Encoding: 0x8580E000
    // Test PRFD_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, prfop=0, Zn=0, imm5=0
    let encoding: u32 = 0x8580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_prfd_i_p_ai_s_combo_12_e000_8580e020() {
    // Encoding: 0x8580E020
    // Test PRFD_I.P.AI_S field combination: imm5=0, Pg=0, Zn=1, prfop=0
    // Fields: Pg=0, prfop=0, Zn=1, imm5=0
    let encoding: u32 = 0x8580E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_prfd_i_p_ai_s_combo_13_e000_8580e3c0() {
    // Encoding: 0x8580E3C0
    // Test PRFD_I.P.AI_S field combination: imm5=0, Pg=0, Zn=30, prfop=0
    // Fields: Zn=30, Pg=0, imm5=0, prfop=0
    let encoding: u32 = 0x8580E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_prfd_i_p_ai_s_combo_14_e000_8580e3e0() {
    // Encoding: 0x8580E3E0
    // Test PRFD_I.P.AI_S field combination: imm5=0, Pg=0, Zn=31, prfop=0
    // Fields: imm5=0, Pg=0, prfop=0, Zn=31
    let encoding: u32 = 0x8580E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfd_i_p_ai_s_combo_15_e000_8580e000() {
    // Encoding: 0x8580E000
    // Test PRFD_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, imm5=0, prfop=0, Zn=0
    let encoding: u32 = 0x8580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfd_i_p_ai_s_combo_16_e000_8580e001() {
    // Encoding: 0x8580E001
    // Test PRFD_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=1
    // Fields: Zn=0, imm5=0, Pg=0, prfop=1
    let encoding: u32 = 0x8580E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfd_i_p_ai_s_combo_17_e000_8580e007() {
    // Encoding: 0x8580E007
    // Test PRFD_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=7
    // Fields: imm5=0, Pg=0, Zn=0, prfop=7
    let encoding: u32 = 0x8580E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_S
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfd_i_p_ai_s_combo_18_e000_8580e00f() {
    // Encoding: 0x8580E00F
    // Test PRFD_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=15
    // Fields: Zn=0, Pg=0, imm5=0, prfop=15
    let encoding: u32 = 0x8580E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_prfd_i_p_ai_d_field_imm5_0_zero_e000_c580e000() {
    // Encoding: 0xC580E000
    // Test PRFD_I.P.AI_D field imm5 = 0 (Zero)
    // Fields: Zn=0, imm5=0, prfop=0, Pg=0
    let encoding: u32 = 0xC580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_prfd_i_p_ai_d_field_imm5_1_poweroftwo_e000_c581e000() {
    // Encoding: 0xC581E000
    // Test PRFD_I.P.AI_D field imm5 = 1 (PowerOfTwo)
    // Fields: prfop=0, imm5=1, Pg=0, Zn=0
    let encoding: u32 = 0xC581E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_prfd_i_p_ai_d_field_imm5_3_poweroftwominusone_e000_c583e000() {
    // Encoding: 0xC583E000
    // Test PRFD_I.P.AI_D field imm5 = 3 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Zn=0, Pg=0, imm5=3
    let encoding: u32 = 0xC583E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_prfd_i_p_ai_d_field_imm5_4_poweroftwo_e000_c584e000() {
    // Encoding: 0xC584E000
    // Test PRFD_I.P.AI_D field imm5 = 4 (PowerOfTwo)
    // Fields: Pg=0, Zn=0, imm5=4, prfop=0
    let encoding: u32 = 0xC584E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_prfd_i_p_ai_d_field_imm5_7_poweroftwominusone_e000_c587e000() {
    // Encoding: 0xC587E000
    // Test PRFD_I.P.AI_D field imm5 = 7 (PowerOfTwoMinusOne)
    // Fields: Pg=0, prfop=0, Zn=0, imm5=7
    let encoding: u32 = 0xC587E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_prfd_i_p_ai_d_field_imm5_8_poweroftwo_e000_c588e000() {
    // Encoding: 0xC588E000
    // Test PRFD_I.P.AI_D field imm5 = 8 (PowerOfTwo)
    // Fields: Zn=0, Pg=0, imm5=8, prfop=0
    let encoding: u32 = 0xC588E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_prfd_i_p_ai_d_field_imm5_15_poweroftwominusone_e000_c58fe000() {
    // Encoding: 0xC58FE000
    // Test PRFD_I.P.AI_D field imm5 = 15 (PowerOfTwoMinusOne)
    // Fields: Pg=0, imm5=15, Zn=0, prfop=0
    let encoding: u32 = 0xC58FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_prfd_i_p_ai_d_field_imm5_16_poweroftwo_e000_c590e000() {
    // Encoding: 0xC590E000
    // Test PRFD_I.P.AI_D field imm5 = 16 (PowerOfTwo)
    // Fields: imm5=16, Pg=0, prfop=0, Zn=0
    let encoding: u32 = 0xC590E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_prfd_i_p_ai_d_field_imm5_31_max_e000_c59fe000() {
    // Encoding: 0xC59FE000
    // Test PRFD_I.P.AI_D field imm5 = 31 (Max)
    // Fields: Zn=0, imm5=31, prfop=0, Pg=0
    let encoding: u32 = 0xC59FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfd_i_p_ai_d_field_pg_0_min_e000_c580e000() {
    // Encoding: 0xC580E000
    // Test PRFD_I.P.AI_D field Pg = 0 (Min)
    // Fields: Pg=0, Zn=0, prfop=0, imm5=0
    let encoding: u32 = 0xC580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfd_i_p_ai_d_field_pg_1_poweroftwo_e000_c580e400() {
    // Encoding: 0xC580E400
    // Test PRFD_I.P.AI_D field Pg = 1 (PowerOfTwo)
    // Fields: prfop=0, imm5=0, Zn=0, Pg=1
    let encoding: u32 = 0xC580E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfd_i_p_ai_d_field_zn_0_min_e000_c580e000() {
    // Encoding: 0xC580E000
    // Test PRFD_I.P.AI_D field Zn = 0 (Min)
    // Fields: imm5=0, Pg=0, Zn=0, prfop=0
    let encoding: u32 = 0xC580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfd_i_p_ai_d_field_zn_1_poweroftwo_e000_c580e020() {
    // Encoding: 0xC580E020
    // Test PRFD_I.P.AI_D field Zn = 1 (PowerOfTwo)
    // Fields: Zn=1, prfop=0, imm5=0, Pg=0
    let encoding: u32 = 0xC580E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfd_i_p_ai_d_field_zn_30_poweroftwominusone_e000_c580e3c0() {
    // Encoding: 0xC580E3C0
    // Test PRFD_I.P.AI_D field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Pg=0, imm5=0, Zn=30
    let encoding: u32 = 0xC580E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfd_i_p_ai_d_field_zn_31_max_e000_c580e3e0() {
    // Encoding: 0xC580E3E0
    // Test PRFD_I.P.AI_D field Zn = 31 (Max)
    // Fields: Zn=31, imm5=0, Pg=0, prfop=0
    let encoding: u32 = 0xC580E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfd_i_p_ai_d_field_prfop_0_min_e000_c580e000() {
    // Encoding: 0xC580E000
    // Test PRFD_I.P.AI_D field prfop = 0 (Min)
    // Fields: Pg=0, prfop=0, imm5=0, Zn=0
    let encoding: u32 = 0xC580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfd_i_p_ai_d_field_prfop_1_poweroftwo_e000_c580e001() {
    // Encoding: 0xC580E001
    // Test PRFD_I.P.AI_D field prfop = 1 (PowerOfTwo)
    // Fields: prfop=1, imm5=0, Pg=0, Zn=0
    let encoding: u32 = 0xC580E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfd_i_p_ai_d_field_prfop_7_poweroftwominusone_e000_c580e007() {
    // Encoding: 0xC580E007
    // Test PRFD_I.P.AI_D field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: prfop=7, Zn=0, imm5=0, Pg=0
    let encoding: u32 = 0xC580E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfd_i_p_ai_d_field_prfop_15_max_e000_c580e00f() {
    // Encoding: 0xC580E00F
    // Test PRFD_I.P.AI_D field prfop = 15 (Max)
    // Fields: Zn=0, imm5=0, Pg=0, prfop=15
    let encoding: u32 = 0xC580E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=0 (immediate value 0)
#[test]
fn test_prfd_i_p_ai_d_combo_0_e000_c580e000() {
    // Encoding: 0xC580E000
    // Test PRFD_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, imm5=0, Zn=0, prfop=0
    let encoding: u32 = 0xC580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=1 (immediate value 1)
#[test]
fn test_prfd_i_p_ai_d_combo_1_e000_c581e000() {
    // Encoding: 0xC581E000
    // Test PRFD_I.P.AI_D field combination: imm5=1, Pg=0, Zn=0, prfop=0
    // Fields: imm5=1, Zn=0, prfop=0, Pg=0
    let encoding: u32 = 0xC581E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=3 (2^2 - 1 = 3)
#[test]
fn test_prfd_i_p_ai_d_combo_2_e000_c583e000() {
    // Encoding: 0xC583E000
    // Test PRFD_I.P.AI_D field combination: imm5=3, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, Pg=0, prfop=0, imm5=3
    let encoding: u32 = 0xC583E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=4 (power of 2 (2^2 = 4))
#[test]
fn test_prfd_i_p_ai_d_combo_3_e000_c584e000() {
    // Encoding: 0xC584E000
    // Test PRFD_I.P.AI_D field combination: imm5=4, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, imm5=4, prfop=0, Pg=0
    let encoding: u32 = 0xC584E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=7 (2^3 - 1 = 7)
#[test]
fn test_prfd_i_p_ai_d_combo_4_e000_c587e000() {
    // Encoding: 0xC587E000
    // Test PRFD_I.P.AI_D field combination: imm5=7, Pg=0, Zn=0, prfop=0
    // Fields: imm5=7, prfop=0, Zn=0, Pg=0
    let encoding: u32 = 0xC587E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=8 (power of 2 (2^3 = 8))
#[test]
fn test_prfd_i_p_ai_d_combo_5_e000_c588e000() {
    // Encoding: 0xC588E000
    // Test PRFD_I.P.AI_D field combination: imm5=8, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, Pg=0, imm5=8, Zn=0
    let encoding: u32 = 0xC588E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=15 (immediate midpoint (15))
#[test]
fn test_prfd_i_p_ai_d_combo_6_e000_c58fe000() {
    // Encoding: 0xC58FE000
    // Test PRFD_I.P.AI_D field combination: imm5=15, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, imm5=15, Pg=0, prfop=0
    let encoding: u32 = 0xC58FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=16 (power of 2 (2^4 = 16))
#[test]
fn test_prfd_i_p_ai_d_combo_7_e000_c590e000() {
    // Encoding: 0xC590E000
    // Test PRFD_I.P.AI_D field combination: imm5=16, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, Pg=0, prfop=0, imm5=16
    let encoding: u32 = 0xC590E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=31 (maximum immediate (31))
#[test]
fn test_prfd_i_p_ai_d_combo_8_e000_c59fe000() {
    // Encoding: 0xC59FE000
    // Test PRFD_I.P.AI_D field combination: imm5=31, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, Pg=0, Zn=0, imm5=31
    let encoding: u32 = 0xC59FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfd_i_p_ai_d_combo_9_e000_c580e000() {
    // Encoding: 0xC580E000
    // Test PRFD_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: imm5=0, Zn=0, Pg=0, prfop=0
    let encoding: u32 = 0xC580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfd_i_p_ai_d_combo_10_e000_c580e400() {
    // Encoding: 0xC580E400
    // Test PRFD_I.P.AI_D field combination: imm5=0, Pg=1, Zn=0, prfop=0
    // Fields: Zn=0, Pg=1, imm5=0, prfop=0
    let encoding: u32 = 0xC580E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_prfd_i_p_ai_d_combo_11_e000_c580e000() {
    // Encoding: 0xC580E000
    // Test PRFD_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: imm5=0, Zn=0, Pg=0, prfop=0
    let encoding: u32 = 0xC580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_prfd_i_p_ai_d_combo_12_e000_c580e020() {
    // Encoding: 0xC580E020
    // Test PRFD_I.P.AI_D field combination: imm5=0, Pg=0, Zn=1, prfop=0
    // Fields: Pg=0, Zn=1, imm5=0, prfop=0
    let encoding: u32 = 0xC580E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_prfd_i_p_ai_d_combo_13_e000_c580e3c0() {
    // Encoding: 0xC580E3C0
    // Test PRFD_I.P.AI_D field combination: imm5=0, Pg=0, Zn=30, prfop=0
    // Fields: Pg=0, Zn=30, prfop=0, imm5=0
    let encoding: u32 = 0xC580E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_prfd_i_p_ai_d_combo_14_e000_c580e3e0() {
    // Encoding: 0xC580E3E0
    // Test PRFD_I.P.AI_D field combination: imm5=0, Pg=0, Zn=31, prfop=0
    // Fields: Pg=0, imm5=0, Zn=31, prfop=0
    let encoding: u32 = 0xC580E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfd_i_p_ai_d_combo_15_e000_c580e000() {
    // Encoding: 0xC580E000
    // Test PRFD_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, prfop=0, imm5=0, Zn=0
    let encoding: u32 = 0xC580E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfd_i_p_ai_d_combo_16_e000_c580e001() {
    // Encoding: 0xC580E001
    // Test PRFD_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=1
    // Fields: prfop=1, Zn=0, Pg=0, imm5=0
    let encoding: u32 = 0xC580E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfd_i_p_ai_d_combo_17_e000_c580e007() {
    // Encoding: 0xC580E007
    // Test PRFD_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=7
    // Fields: prfop=7, imm5=0, Pg=0, Zn=0
    let encoding: u32 = 0xC580E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.AI_D
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfd_i_p_ai_d_combo_18_e000_c580e00f() {
    // Encoding: 0xC580E00F
    // Test PRFD_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=15
    // Fields: Zn=0, imm5=0, Pg=0, prfop=15
    let encoding: u32 = 0xC580E00F;
    let mut cpu = create_test_cpu();
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
// PRFW_I.P.BR_S Tests
// ============================================================================

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfw_i_p_br_s_field_rm_0_min_c000_8500c000() {
    // Encoding: 0x8500C000
    // Test PRFW_I.P.BR_S field Rm = 0 (Min)
    // Fields: prfop=0, Rm=0, Rn=0, Pg=0
    let encoding: u32 = 0x8500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfw_i_p_br_s_field_rm_1_poweroftwo_c000_8501c000() {
    // Encoding: 0x8501C000
    // Test PRFW_I.P.BR_S field Rm = 1 (PowerOfTwo)
    // Fields: Rm=1, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0x8501C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfw_i_p_br_s_field_rm_30_poweroftwominusone_c000_851ec000() {
    // Encoding: 0x851EC000
    // Test PRFW_I.P.BR_S field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Rm=30, Rn=0, Pg=0
    let encoding: u32 = 0x851EC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_prfw_i_p_br_s_field_rm_31_max_c000_851fc000() {
    // Encoding: 0x851FC000
    // Test PRFW_I.P.BR_S field Rm = 31 (Max)
    // Fields: Rn=0, Rm=31, Pg=0, prfop=0
    let encoding: u32 = 0x851FC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfw_i_p_br_s_field_pg_0_min_c000_8500c000() {
    // Encoding: 0x8500C000
    // Test PRFW_I.P.BR_S field Pg = 0 (Min)
    // Fields: Pg=0, Rn=0, prfop=0, Rm=0
    let encoding: u32 = 0x8500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfw_i_p_br_s_field_pg_1_poweroftwo_c000_8500c400() {
    // Encoding: 0x8500C400
    // Test PRFW_I.P.BR_S field Pg = 1 (PowerOfTwo)
    // Fields: Rm=0, Pg=1, prfop=0, Rn=0
    let encoding: u32 = 0x8500C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfw_i_p_br_s_field_rn_0_min_c000_8500c000() {
    // Encoding: 0x8500C000
    // Test PRFW_I.P.BR_S field Rn = 0 (Min)
    // Fields: Pg=0, Rm=0, Rn=0, prfop=0
    let encoding: u32 = 0x8500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfw_i_p_br_s_field_rn_1_poweroftwo_c000_8500c020() {
    // Encoding: 0x8500C020
    // Test PRFW_I.P.BR_S field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Rm=0, Pg=0, prfop=0
    let encoding: u32 = 0x8500C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfw_i_p_br_s_field_rn_30_poweroftwominusone_c000_8500c3c0() {
    // Encoding: 0x8500C3C0
    // Test PRFW_I.P.BR_S field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, prfop=0, Rm=0, Rn=30
    let encoding: u32 = 0x8500C3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfw_i_p_br_s_field_rn_31_max_c000_8500c3e0() {
    // Encoding: 0x8500C3E0
    // Test PRFW_I.P.BR_S field Rn = 31 (Max)
    // Fields: Rm=0, Rn=31, prfop=0, Pg=0
    let encoding: u32 = 0x8500C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfw_i_p_br_s_field_prfop_0_min_c000_8500c000() {
    // Encoding: 0x8500C000
    // Test PRFW_I.P.BR_S field prfop = 0 (Min)
    // Fields: Rm=0, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0x8500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfw_i_p_br_s_field_prfop_1_poweroftwo_c000_8500c001() {
    // Encoding: 0x8500C001
    // Test PRFW_I.P.BR_S field prfop = 1 (PowerOfTwo)
    // Fields: Pg=0, Rm=0, Rn=0, prfop=1
    let encoding: u32 = 0x8500C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfw_i_p_br_s_field_prfop_7_poweroftwominusone_c000_8500c007() {
    // Encoding: 0x8500C007
    // Test PRFW_I.P.BR_S field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Rm=0, prfop=7, Rn=0, Pg=0
    let encoding: u32 = 0x8500C007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfw_i_p_br_s_field_prfop_15_max_c000_8500c00f() {
    // Encoding: 0x8500C00F
    // Test PRFW_I.P.BR_S field prfop = 15 (Max)
    // Fields: Rm=0, prfop=15, Pg=0, Rn=0
    let encoding: u32 = 0x8500C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_prfw_i_p_br_s_combo_0_c000_8500c000() {
    // Encoding: 0x8500C000
    // Test PRFW_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, Rm=0, Pg=0
    let encoding: u32 = 0x8500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_prfw_i_p_br_s_combo_1_c000_8501c000() {
    // Encoding: 0x8501C000
    // Test PRFW_I.P.BR_S field combination: Rm=1, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, prfop=0, Pg=0, Rm=1
    let encoding: u32 = 0x8501C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfw_i_p_br_s_combo_2_c000_851ec000() {
    // Encoding: 0x851EC000
    // Test PRFW_I.P.BR_S field combination: Rm=30, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rm=30, Rn=0, Pg=0
    let encoding: u32 = 0x851EC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_prfw_i_p_br_s_combo_3_c000_851fc000() {
    // Encoding: 0x851FC000
    // Test PRFW_I.P.BR_S field combination: Rm=31, Pg=0, Rn=0, prfop=0
    // Fields: Rm=31, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0x851FC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfw_i_p_br_s_combo_4_c000_8500c000() {
    // Encoding: 0x8500C000
    // Test PRFW_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rn=0, Rm=0, prfop=0
    let encoding: u32 = 0x8500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfw_i_p_br_s_combo_5_c000_8500c400() {
    // Encoding: 0x8500C400
    // Test PRFW_I.P.BR_S field combination: Rm=0, Pg=1, Rn=0, prfop=0
    // Fields: prfop=0, Pg=1, Rm=0, Rn=0
    let encoding: u32 = 0x8500C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfw_i_p_br_s_combo_6_c000_8500c000() {
    // Encoding: 0x8500C000
    // Test PRFW_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rm=0, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0x8500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfw_i_p_br_s_combo_7_c000_8500c020() {
    // Encoding: 0x8500C020
    // Test PRFW_I.P.BR_S field combination: Rm=0, Pg=0, Rn=1, prfop=0
    // Fields: Pg=0, Rn=1, Rm=0, prfop=0
    let encoding: u32 = 0x8500C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfw_i_p_br_s_combo_8_c000_8500c3c0() {
    // Encoding: 0x8500C3C0
    // Test PRFW_I.P.BR_S field combination: Rm=0, Pg=0, Rn=30, prfop=0
    // Fields: prfop=0, Rn=30, Rm=0, Pg=0
    let encoding: u32 = 0x8500C3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfw_i_p_br_s_combo_9_c000_8500c3e0() {
    // Encoding: 0x8500C3E0
    // Test PRFW_I.P.BR_S field combination: Rm=0, Pg=0, Rn=31, prfop=0
    // Fields: Rm=0, Rn=31, prfop=0, Pg=0
    let encoding: u32 = 0x8500C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfw_i_p_br_s_combo_10_c000_8500c000() {
    // Encoding: 0x8500C000
    // Test PRFW_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, Rm=0, Pg=0
    let encoding: u32 = 0x8500C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfw_i_p_br_s_combo_11_c000_8500c001() {
    // Encoding: 0x8500C001
    // Test PRFW_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=1
    // Fields: Rn=0, prfop=1, Pg=0, Rm=0
    let encoding: u32 = 0x8500C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfw_i_p_br_s_combo_12_c000_8500c007() {
    // Encoding: 0x8500C007
    // Test PRFW_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=7
    // Fields: Rm=0, Pg=0, prfop=7, Rn=0
    let encoding: u32 = 0x8500C007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfw_i_p_br_s_combo_13_c000_8500c00f() {
    // Encoding: 0x8500C00F
    // Test PRFW_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=15
    // Fields: Pg=0, Rm=0, Rn=0, prfop=15
    let encoding: u32 = 0x8500C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_prfw_i_p_br_s_combo_14_c000_8501c400() {
    // Encoding: 0x8501C400
    // Test PRFW_I.P.BR_S field combination: Rm=1, Pg=1, Rn=0, prfop=0
    // Fields: Rn=0, prfop=0, Rm=1, Pg=1
    let encoding: u32 = 0x8501C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_prfw_i_p_br_s_combo_15_c000_851fdc00() {
    // Encoding: 0x851FDC00
    // Test PRFW_I.P.BR_S field combination: Rm=31, Pg=31, Rn=0, prfop=0
    // Fields: Rm=31, prfop=0, Pg=31, Rn=0
    let encoding: u32 = 0x851FDC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfw_i_p_br_s_combo_16_c000_8501c020() {
    // Encoding: 0x8501C020
    // Test PRFW_I.P.BR_S field combination: Rm=1, Pg=0, Rn=1, prfop=0
    // Fields: Pg=0, prfop=0, Rn=1, Rm=1
    let encoding: u32 = 0x8501C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfw_i_p_br_s_combo_17_c000_851fc3e0() {
    // Encoding: 0x851FC3E0
    // Test PRFW_I.P.BR_S field combination: Rm=31, Pg=0, Rn=31, prfop=0
    // Fields: Pg=0, Rm=31, Rn=31, prfop=0
    let encoding: u32 = 0x851FC3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfw_i_p_br_s_combo_18_c000_8500c420() {
    // Encoding: 0x8500C420
    // Test PRFW_I.P.BR_S field combination: Rm=0, Pg=1, Rn=1, prfop=0
    // Fields: Rn=1, Rm=0, Pg=1, prfop=0
    let encoding: u32 = 0x8500C420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfw_i_p_br_s_combo_19_c000_8500dfe0() {
    // Encoding: 0x8500DFE0
    // Test PRFW_I.P.BR_S field combination: Rm=0, Pg=31, Rn=31, prfop=0
    // Fields: prfop=0, Rn=31, Pg=31, Rm=0
    let encoding: u32 = 0x8500DFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfw_i_p_br_s_special_rn_31_stack_pointer_sp_may_require_alignment_49152_8500c3e0() {
    // Encoding: 0x8500C3E0
    // Test PRFW_I.P.BR_S special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: prfop=0, Rm=0, Pg=0, Rn=31
    let encoding: u32 = 0x8500C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BR_S
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfw_i_p_br_s_sp_rn_8500c3e0() {
    // Test PRFW_I.P.BR_S with Rn = SP (31)
    // Encoding: 0x8500C3E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x8500C3E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PRFW_I.P.AI_S Tests
// ============================================================================

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_prfw_i_p_ai_s_field_imm5_0_zero_e000_8500e000() {
    // Encoding: 0x8500E000
    // Test PRFW_I.P.AI_S field imm5 = 0 (Zero)
    // Fields: imm5=0, Zn=0, Pg=0, prfop=0
    let encoding: u32 = 0x8500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_prfw_i_p_ai_s_field_imm5_1_poweroftwo_e000_8501e000() {
    // Encoding: 0x8501E000
    // Test PRFW_I.P.AI_S field imm5 = 1 (PowerOfTwo)
    // Fields: prfop=0, Pg=0, imm5=1, Zn=0
    let encoding: u32 = 0x8501E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_prfw_i_p_ai_s_field_imm5_3_poweroftwominusone_e000_8503e000() {
    // Encoding: 0x8503E000
    // Test PRFW_I.P.AI_S field imm5 = 3 (PowerOfTwoMinusOne)
    // Fields: imm5=3, prfop=0, Pg=0, Zn=0
    let encoding: u32 = 0x8503E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_prfw_i_p_ai_s_field_imm5_4_poweroftwo_e000_8504e000() {
    // Encoding: 0x8504E000
    // Test PRFW_I.P.AI_S field imm5 = 4 (PowerOfTwo)
    // Fields: prfop=0, Pg=0, imm5=4, Zn=0
    let encoding: u32 = 0x8504E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_prfw_i_p_ai_s_field_imm5_7_poweroftwominusone_e000_8507e000() {
    // Encoding: 0x8507E000
    // Test PRFW_I.P.AI_S field imm5 = 7 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Pg=0, Zn=0, imm5=7
    let encoding: u32 = 0x8507E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_prfw_i_p_ai_s_field_imm5_8_poweroftwo_e000_8508e000() {
    // Encoding: 0x8508E000
    // Test PRFW_I.P.AI_S field imm5 = 8 (PowerOfTwo)
    // Fields: imm5=8, Zn=0, Pg=0, prfop=0
    let encoding: u32 = 0x8508E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_prfw_i_p_ai_s_field_imm5_15_poweroftwominusone_e000_850fe000() {
    // Encoding: 0x850FE000
    // Test PRFW_I.P.AI_S field imm5 = 15 (PowerOfTwoMinusOne)
    // Fields: imm5=15, prfop=0, Zn=0, Pg=0
    let encoding: u32 = 0x850FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_prfw_i_p_ai_s_field_imm5_16_poweroftwo_e000_8510e000() {
    // Encoding: 0x8510E000
    // Test PRFW_I.P.AI_S field imm5 = 16 (PowerOfTwo)
    // Fields: Zn=0, imm5=16, Pg=0, prfop=0
    let encoding: u32 = 0x8510E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_prfw_i_p_ai_s_field_imm5_31_max_e000_851fe000() {
    // Encoding: 0x851FE000
    // Test PRFW_I.P.AI_S field imm5 = 31 (Max)
    // Fields: imm5=31, Zn=0, prfop=0, Pg=0
    let encoding: u32 = 0x851FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfw_i_p_ai_s_field_pg_0_min_e000_8500e000() {
    // Encoding: 0x8500E000
    // Test PRFW_I.P.AI_S field Pg = 0 (Min)
    // Fields: Pg=0, Zn=0, imm5=0, prfop=0
    let encoding: u32 = 0x8500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfw_i_p_ai_s_field_pg_1_poweroftwo_e000_8500e400() {
    // Encoding: 0x8500E400
    // Test PRFW_I.P.AI_S field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, prfop=0, Zn=0, imm5=0
    let encoding: u32 = 0x8500E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfw_i_p_ai_s_field_zn_0_min_e000_8500e000() {
    // Encoding: 0x8500E000
    // Test PRFW_I.P.AI_S field Zn = 0 (Min)
    // Fields: Zn=0, imm5=0, Pg=0, prfop=0
    let encoding: u32 = 0x8500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfw_i_p_ai_s_field_zn_1_poweroftwo_e000_8500e020() {
    // Encoding: 0x8500E020
    // Test PRFW_I.P.AI_S field Zn = 1 (PowerOfTwo)
    // Fields: Pg=0, prfop=0, imm5=0, Zn=1
    let encoding: u32 = 0x8500E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfw_i_p_ai_s_field_zn_30_poweroftwominusone_e000_8500e3c0() {
    // Encoding: 0x8500E3C0
    // Test PRFW_I.P.AI_S field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, imm5=0, Zn=30, prfop=0
    let encoding: u32 = 0x8500E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfw_i_p_ai_s_field_zn_31_max_e000_8500e3e0() {
    // Encoding: 0x8500E3E0
    // Test PRFW_I.P.AI_S field Zn = 31 (Max)
    // Fields: imm5=0, Pg=0, Zn=31, prfop=0
    let encoding: u32 = 0x8500E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfw_i_p_ai_s_field_prfop_0_min_e000_8500e000() {
    // Encoding: 0x8500E000
    // Test PRFW_I.P.AI_S field prfop = 0 (Min)
    // Fields: prfop=0, imm5=0, Zn=0, Pg=0
    let encoding: u32 = 0x8500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfw_i_p_ai_s_field_prfop_1_poweroftwo_e000_8500e001() {
    // Encoding: 0x8500E001
    // Test PRFW_I.P.AI_S field prfop = 1 (PowerOfTwo)
    // Fields: Pg=0, prfop=1, Zn=0, imm5=0
    let encoding: u32 = 0x8500E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfw_i_p_ai_s_field_prfop_7_poweroftwominusone_e000_8500e007() {
    // Encoding: 0x8500E007
    // Test PRFW_I.P.AI_S field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: prfop=7, Zn=0, Pg=0, imm5=0
    let encoding: u32 = 0x8500E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfw_i_p_ai_s_field_prfop_15_max_e000_8500e00f() {
    // Encoding: 0x8500E00F
    // Test PRFW_I.P.AI_S field prfop = 15 (Max)
    // Fields: imm5=0, Zn=0, Pg=0, prfop=15
    let encoding: u32 = 0x8500E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=0 (immediate value 0)
#[test]
fn test_prfw_i_p_ai_s_combo_0_e000_8500e000() {
    // Encoding: 0x8500E000
    // Test PRFW_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, Zn=0, imm5=0, prfop=0
    let encoding: u32 = 0x8500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=1 (immediate value 1)
#[test]
fn test_prfw_i_p_ai_s_combo_1_e000_8501e000() {
    // Encoding: 0x8501E000
    // Test PRFW_I.P.AI_S field combination: imm5=1, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, imm5=1, Pg=0, prfop=0
    let encoding: u32 = 0x8501E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=3 (2^2 - 1 = 3)
#[test]
fn test_prfw_i_p_ai_s_combo_2_e000_8503e000() {
    // Encoding: 0x8503E000
    // Test PRFW_I.P.AI_S field combination: imm5=3, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, imm5=3, Zn=0, prfop=0
    let encoding: u32 = 0x8503E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=4 (power of 2 (2^2 = 4))
#[test]
fn test_prfw_i_p_ai_s_combo_3_e000_8504e000() {
    // Encoding: 0x8504E000
    // Test PRFW_I.P.AI_S field combination: imm5=4, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, Pg=0, Zn=0, imm5=4
    let encoding: u32 = 0x8504E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=7 (2^3 - 1 = 7)
#[test]
fn test_prfw_i_p_ai_s_combo_4_e000_8507e000() {
    // Encoding: 0x8507E000
    // Test PRFW_I.P.AI_S field combination: imm5=7, Pg=0, Zn=0, prfop=0
    // Fields: imm5=7, prfop=0, Zn=0, Pg=0
    let encoding: u32 = 0x8507E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=8 (power of 2 (2^3 = 8))
#[test]
fn test_prfw_i_p_ai_s_combo_5_e000_8508e000() {
    // Encoding: 0x8508E000
    // Test PRFW_I.P.AI_S field combination: imm5=8, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, prfop=0, Zn=0, imm5=8
    let encoding: u32 = 0x8508E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=15 (immediate midpoint (15))
#[test]
fn test_prfw_i_p_ai_s_combo_6_e000_850fe000() {
    // Encoding: 0x850FE000
    // Test PRFW_I.P.AI_S field combination: imm5=15, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, imm5=15, Pg=0, prfop=0
    let encoding: u32 = 0x850FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=16 (power of 2 (2^4 = 16))
#[test]
fn test_prfw_i_p_ai_s_combo_7_e000_8510e000() {
    // Encoding: 0x8510E000
    // Test PRFW_I.P.AI_S field combination: imm5=16, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, imm5=16, Pg=0, prfop=0
    let encoding: u32 = 0x8510E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=31 (maximum immediate (31))
#[test]
fn test_prfw_i_p_ai_s_combo_8_e000_851fe000() {
    // Encoding: 0x851FE000
    // Test PRFW_I.P.AI_S field combination: imm5=31, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, imm5=31, Zn=0, prfop=0
    let encoding: u32 = 0x851FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfw_i_p_ai_s_combo_9_e000_8500e000() {
    // Encoding: 0x8500E000
    // Test PRFW_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, Pg=0, imm5=0, Zn=0
    let encoding: u32 = 0x8500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfw_i_p_ai_s_combo_10_e000_8500e400() {
    // Encoding: 0x8500E400
    // Test PRFW_I.P.AI_S field combination: imm5=0, Pg=1, Zn=0, prfop=0
    // Fields: prfop=0, Zn=0, imm5=0, Pg=1
    let encoding: u32 = 0x8500E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_prfw_i_p_ai_s_combo_11_e000_8500e000() {
    // Encoding: 0x8500E000
    // Test PRFW_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, imm5=0, Zn=0, Pg=0
    let encoding: u32 = 0x8500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_prfw_i_p_ai_s_combo_12_e000_8500e020() {
    // Encoding: 0x8500E020
    // Test PRFW_I.P.AI_S field combination: imm5=0, Pg=0, Zn=1, prfop=0
    // Fields: Pg=0, Zn=1, imm5=0, prfop=0
    let encoding: u32 = 0x8500E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_prfw_i_p_ai_s_combo_13_e000_8500e3c0() {
    // Encoding: 0x8500E3C0
    // Test PRFW_I.P.AI_S field combination: imm5=0, Pg=0, Zn=30, prfop=0
    // Fields: imm5=0, Zn=30, prfop=0, Pg=0
    let encoding: u32 = 0x8500E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_prfw_i_p_ai_s_combo_14_e000_8500e3e0() {
    // Encoding: 0x8500E3E0
    // Test PRFW_I.P.AI_S field combination: imm5=0, Pg=0, Zn=31, prfop=0
    // Fields: prfop=0, Pg=0, Zn=31, imm5=0
    let encoding: u32 = 0x8500E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfw_i_p_ai_s_combo_15_e000_8500e000() {
    // Encoding: 0x8500E000
    // Test PRFW_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, imm5=0, prfop=0, Zn=0
    let encoding: u32 = 0x8500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfw_i_p_ai_s_combo_16_e000_8500e001() {
    // Encoding: 0x8500E001
    // Test PRFW_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=1
    // Fields: imm5=0, Zn=0, prfop=1, Pg=0
    let encoding: u32 = 0x8500E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfw_i_p_ai_s_combo_17_e000_8500e007() {
    // Encoding: 0x8500E007
    // Test PRFW_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=7
    // Fields: prfop=7, imm5=0, Pg=0, Zn=0
    let encoding: u32 = 0x8500E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_S
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfw_i_p_ai_s_combo_18_e000_8500e00f() {
    // Encoding: 0x8500E00F
    // Test PRFW_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=15
    // Fields: imm5=0, Pg=0, prfop=15, Zn=0
    let encoding: u32 = 0x8500E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_prfw_i_p_ai_d_field_imm5_0_zero_e000_c500e000() {
    // Encoding: 0xC500E000
    // Test PRFW_I.P.AI_D field imm5 = 0 (Zero)
    // Fields: Pg=0, imm5=0, prfop=0, Zn=0
    let encoding: u32 = 0xC500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_prfw_i_p_ai_d_field_imm5_1_poweroftwo_e000_c501e000() {
    // Encoding: 0xC501E000
    // Test PRFW_I.P.AI_D field imm5 = 1 (PowerOfTwo)
    // Fields: imm5=1, Zn=0, Pg=0, prfop=0
    let encoding: u32 = 0xC501E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_prfw_i_p_ai_d_field_imm5_3_poweroftwominusone_e000_c503e000() {
    // Encoding: 0xC503E000
    // Test PRFW_I.P.AI_D field imm5 = 3 (PowerOfTwoMinusOne)
    // Fields: prfop=0, imm5=3, Pg=0, Zn=0
    let encoding: u32 = 0xC503E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_prfw_i_p_ai_d_field_imm5_4_poweroftwo_e000_c504e000() {
    // Encoding: 0xC504E000
    // Test PRFW_I.P.AI_D field imm5 = 4 (PowerOfTwo)
    // Fields: Zn=0, prfop=0, imm5=4, Pg=0
    let encoding: u32 = 0xC504E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_prfw_i_p_ai_d_field_imm5_7_poweroftwominusone_e000_c507e000() {
    // Encoding: 0xC507E000
    // Test PRFW_I.P.AI_D field imm5 = 7 (PowerOfTwoMinusOne)
    // Fields: Zn=0, imm5=7, Pg=0, prfop=0
    let encoding: u32 = 0xC507E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_prfw_i_p_ai_d_field_imm5_8_poweroftwo_e000_c508e000() {
    // Encoding: 0xC508E000
    // Test PRFW_I.P.AI_D field imm5 = 8 (PowerOfTwo)
    // Fields: Pg=0, Zn=0, imm5=8, prfop=0
    let encoding: u32 = 0xC508E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_prfw_i_p_ai_d_field_imm5_15_poweroftwominusone_e000_c50fe000() {
    // Encoding: 0xC50FE000
    // Test PRFW_I.P.AI_D field imm5 = 15 (PowerOfTwoMinusOne)
    // Fields: Zn=0, prfop=0, Pg=0, imm5=15
    let encoding: u32 = 0xC50FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_prfw_i_p_ai_d_field_imm5_16_poweroftwo_e000_c510e000() {
    // Encoding: 0xC510E000
    // Test PRFW_I.P.AI_D field imm5 = 16 (PowerOfTwo)
    // Fields: prfop=0, Pg=0, imm5=16, Zn=0
    let encoding: u32 = 0xC510E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_prfw_i_p_ai_d_field_imm5_31_max_e000_c51fe000() {
    // Encoding: 0xC51FE000
    // Test PRFW_I.P.AI_D field imm5 = 31 (Max)
    // Fields: prfop=0, Pg=0, Zn=0, imm5=31
    let encoding: u32 = 0xC51FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfw_i_p_ai_d_field_pg_0_min_e000_c500e000() {
    // Encoding: 0xC500E000
    // Test PRFW_I.P.AI_D field Pg = 0 (Min)
    // Fields: Zn=0, imm5=0, Pg=0, prfop=0
    let encoding: u32 = 0xC500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfw_i_p_ai_d_field_pg_1_poweroftwo_e000_c500e400() {
    // Encoding: 0xC500E400
    // Test PRFW_I.P.AI_D field Pg = 1 (PowerOfTwo)
    // Fields: imm5=0, prfop=0, Zn=0, Pg=1
    let encoding: u32 = 0xC500E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfw_i_p_ai_d_field_zn_0_min_e000_c500e000() {
    // Encoding: 0xC500E000
    // Test PRFW_I.P.AI_D field Zn = 0 (Min)
    // Fields: Zn=0, Pg=0, prfop=0, imm5=0
    let encoding: u32 = 0xC500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfw_i_p_ai_d_field_zn_1_poweroftwo_e000_c500e020() {
    // Encoding: 0xC500E020
    // Test PRFW_I.P.AI_D field Zn = 1 (PowerOfTwo)
    // Fields: prfop=0, Zn=1, Pg=0, imm5=0
    let encoding: u32 = 0xC500E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfw_i_p_ai_d_field_zn_30_poweroftwominusone_e000_c500e3c0() {
    // Encoding: 0xC500E3C0
    // Test PRFW_I.P.AI_D field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: imm5=0, Pg=0, prfop=0, Zn=30
    let encoding: u32 = 0xC500E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfw_i_p_ai_d_field_zn_31_max_e000_c500e3e0() {
    // Encoding: 0xC500E3E0
    // Test PRFW_I.P.AI_D field Zn = 31 (Max)
    // Fields: Pg=0, imm5=0, Zn=31, prfop=0
    let encoding: u32 = 0xC500E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfw_i_p_ai_d_field_prfop_0_min_e000_c500e000() {
    // Encoding: 0xC500E000
    // Test PRFW_I.P.AI_D field prfop = 0 (Min)
    // Fields: Pg=0, Zn=0, prfop=0, imm5=0
    let encoding: u32 = 0xC500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfw_i_p_ai_d_field_prfop_1_poweroftwo_e000_c500e001() {
    // Encoding: 0xC500E001
    // Test PRFW_I.P.AI_D field prfop = 1 (PowerOfTwo)
    // Fields: prfop=1, Pg=0, Zn=0, imm5=0
    let encoding: u32 = 0xC500E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfw_i_p_ai_d_field_prfop_7_poweroftwominusone_e000_c500e007() {
    // Encoding: 0xC500E007
    // Test PRFW_I.P.AI_D field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: imm5=0, Pg=0, prfop=7, Zn=0
    let encoding: u32 = 0xC500E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfw_i_p_ai_d_field_prfop_15_max_e000_c500e00f() {
    // Encoding: 0xC500E00F
    // Test PRFW_I.P.AI_D field prfop = 15 (Max)
    // Fields: prfop=15, Zn=0, Pg=0, imm5=0
    let encoding: u32 = 0xC500E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=0 (immediate value 0)
#[test]
fn test_prfw_i_p_ai_d_combo_0_e000_c500e000() {
    // Encoding: 0xC500E000
    // Test PRFW_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: imm5=0, Pg=0, prfop=0, Zn=0
    let encoding: u32 = 0xC500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=1 (immediate value 1)
#[test]
fn test_prfw_i_p_ai_d_combo_1_e000_c501e000() {
    // Encoding: 0xC501E000
    // Test PRFW_I.P.AI_D field combination: imm5=1, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, prfop=0, imm5=1, Pg=0
    let encoding: u32 = 0xC501E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=3 (2^2 - 1 = 3)
#[test]
fn test_prfw_i_p_ai_d_combo_2_e000_c503e000() {
    // Encoding: 0xC503E000
    // Test PRFW_I.P.AI_D field combination: imm5=3, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, Zn=0, Pg=0, imm5=3
    let encoding: u32 = 0xC503E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=4 (power of 2 (2^2 = 4))
#[test]
fn test_prfw_i_p_ai_d_combo_3_e000_c504e000() {
    // Encoding: 0xC504E000
    // Test PRFW_I.P.AI_D field combination: imm5=4, Pg=0, Zn=0, prfop=0
    // Fields: imm5=4, prfop=0, Pg=0, Zn=0
    let encoding: u32 = 0xC504E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=7 (2^3 - 1 = 7)
#[test]
fn test_prfw_i_p_ai_d_combo_4_e000_c507e000() {
    // Encoding: 0xC507E000
    // Test PRFW_I.P.AI_D field combination: imm5=7, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, Pg=0, imm5=7, Zn=0
    let encoding: u32 = 0xC507E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=8 (power of 2 (2^3 = 8))
#[test]
fn test_prfw_i_p_ai_d_combo_5_e000_c508e000() {
    // Encoding: 0xC508E000
    // Test PRFW_I.P.AI_D field combination: imm5=8, Pg=0, Zn=0, prfop=0
    // Fields: imm5=8, prfop=0, Pg=0, Zn=0
    let encoding: u32 = 0xC508E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=15 (immediate midpoint (15))
#[test]
fn test_prfw_i_p_ai_d_combo_6_e000_c50fe000() {
    // Encoding: 0xC50FE000
    // Test PRFW_I.P.AI_D field combination: imm5=15, Pg=0, Zn=0, prfop=0
    // Fields: imm5=15, prfop=0, Pg=0, Zn=0
    let encoding: u32 = 0xC50FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=16 (power of 2 (2^4 = 16))
#[test]
fn test_prfw_i_p_ai_d_combo_7_e000_c510e000() {
    // Encoding: 0xC510E000
    // Test PRFW_I.P.AI_D field combination: imm5=16, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, imm5=16, prfop=0, Zn=0
    let encoding: u32 = 0xC510E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=31 (maximum immediate (31))
#[test]
fn test_prfw_i_p_ai_d_combo_8_e000_c51fe000() {
    // Encoding: 0xC51FE000
    // Test PRFW_I.P.AI_D field combination: imm5=31, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, prfop=0, imm5=31, Zn=0
    let encoding: u32 = 0xC51FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfw_i_p_ai_d_combo_9_e000_c500e000() {
    // Encoding: 0xC500E000
    // Test PRFW_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, imm5=0, prfop=0, Pg=0
    let encoding: u32 = 0xC500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfw_i_p_ai_d_combo_10_e000_c500e400() {
    // Encoding: 0xC500E400
    // Test PRFW_I.P.AI_D field combination: imm5=0, Pg=1, Zn=0, prfop=0
    // Fields: prfop=0, Zn=0, Pg=1, imm5=0
    let encoding: u32 = 0xC500E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_prfw_i_p_ai_d_combo_11_e000_c500e000() {
    // Encoding: 0xC500E000
    // Test PRFW_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, imm5=0, prfop=0, Pg=0
    let encoding: u32 = 0xC500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_prfw_i_p_ai_d_combo_12_e000_c500e020() {
    // Encoding: 0xC500E020
    // Test PRFW_I.P.AI_D field combination: imm5=0, Pg=0, Zn=1, prfop=0
    // Fields: Zn=1, imm5=0, Pg=0, prfop=0
    let encoding: u32 = 0xC500E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_prfw_i_p_ai_d_combo_13_e000_c500e3c0() {
    // Encoding: 0xC500E3C0
    // Test PRFW_I.P.AI_D field combination: imm5=0, Pg=0, Zn=30, prfop=0
    // Fields: prfop=0, Pg=0, Zn=30, imm5=0
    let encoding: u32 = 0xC500E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_prfw_i_p_ai_d_combo_14_e000_c500e3e0() {
    // Encoding: 0xC500E3E0
    // Test PRFW_I.P.AI_D field combination: imm5=0, Pg=0, Zn=31, prfop=0
    // Fields: prfop=0, imm5=0, Pg=0, Zn=31
    let encoding: u32 = 0xC500E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfw_i_p_ai_d_combo_15_e000_c500e000() {
    // Encoding: 0xC500E000
    // Test PRFW_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, imm5=0, Zn=0, Pg=0
    let encoding: u32 = 0xC500E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfw_i_p_ai_d_combo_16_e000_c500e001() {
    // Encoding: 0xC500E001
    // Test PRFW_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=1
    // Fields: imm5=0, Zn=0, Pg=0, prfop=1
    let encoding: u32 = 0xC500E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfw_i_p_ai_d_combo_17_e000_c500e007() {
    // Encoding: 0xC500E007
    // Test PRFW_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=7
    // Fields: Pg=0, prfop=7, imm5=0, Zn=0
    let encoding: u32 = 0xC500E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.AI_D
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfw_i_p_ai_d_combo_18_e000_c500e00f() {
    // Encoding: 0xC500E00F
    // Test PRFW_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=15
    // Fields: Zn=0, Pg=0, imm5=0, prfop=15
    let encoding: u32 = 0xC500E00F;
    let mut cpu = create_test_cpu();
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
// PRFB_I.P.BZ_S.x32.scaled Tests
// ============================================================================

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_xs_0_min_0_84200000() {
    // Encoding: 0x84200000
    // Test PRFB_I.P.BZ_S.x32.scaled field xs = 0 (Min)
    // Fields: prfop=0, Zm=0, Rn=0, xs=0, Pg=0
    let encoding: u32 = 0x84200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_xs_1_max_0_84600000() {
    // Encoding: 0x84600000
    // Test PRFB_I.P.BZ_S.x32.scaled field xs = 1 (Max)
    // Fields: xs=1, Pg=0, Zm=0, prfop=0, Rn=0
    let encoding: u32 = 0x84600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_zm_0_min_0_84200000() {
    // Encoding: 0x84200000
    // Test PRFB_I.P.BZ_S.x32.scaled field Zm = 0 (Min)
    // Fields: prfop=0, xs=0, Pg=0, Rn=0, Zm=0
    let encoding: u32 = 0x84200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_zm_1_poweroftwo_0_84210000() {
    // Encoding: 0x84210000
    // Test PRFB_I.P.BZ_S.x32.scaled field Zm = 1 (PowerOfTwo)
    // Fields: xs=0, Rn=0, Pg=0, Zm=1, prfop=0
    let encoding: u32 = 0x84210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_zm_30_poweroftwominusone_0_843e0000() {
    // Encoding: 0x843E0000
    // Test PRFB_I.P.BZ_S.x32.scaled field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, prfop=0, Zm=30, xs=0, Pg=0
    let encoding: u32 = 0x843E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_zm_31_max_0_843f0000() {
    // Encoding: 0x843F0000
    // Test PRFB_I.P.BZ_S.x32.scaled field Zm = 31 (Max)
    // Fields: Pg=0, prfop=0, xs=0, Zm=31, Rn=0
    let encoding: u32 = 0x843F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_pg_0_min_0_84200000() {
    // Encoding: 0x84200000
    // Test PRFB_I.P.BZ_S.x32.scaled field Pg = 0 (Min)
    // Fields: Pg=0, prfop=0, xs=0, Zm=0, Rn=0
    let encoding: u32 = 0x84200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_pg_1_poweroftwo_0_84200400() {
    // Encoding: 0x84200400
    // Test PRFB_I.P.BZ_S.x32.scaled field Pg = 1 (PowerOfTwo)
    // Fields: Zm=0, Rn=0, xs=0, Pg=1, prfop=0
    let encoding: u32 = 0x84200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_rn_0_min_0_84200000() {
    // Encoding: 0x84200000
    // Test PRFB_I.P.BZ_S.x32.scaled field Rn = 0 (Min)
    // Fields: Zm=0, prfop=0, Rn=0, xs=0, Pg=0
    let encoding: u32 = 0x84200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_rn_1_poweroftwo_0_84200020() {
    // Encoding: 0x84200020
    // Test PRFB_I.P.BZ_S.x32.scaled field Rn = 1 (PowerOfTwo)
    // Fields: Pg=0, Rn=1, Zm=0, xs=0, prfop=0
    let encoding: u32 = 0x84200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_rn_30_poweroftwominusone_0_842003c0() {
    // Encoding: 0x842003C0
    // Test PRFB_I.P.BZ_S.x32.scaled field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: xs=0, prfop=0, Pg=0, Rn=30, Zm=0
    let encoding: u32 = 0x842003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_rn_31_max_0_842003e0() {
    // Encoding: 0x842003E0
    // Test PRFB_I.P.BZ_S.x32.scaled field Rn = 31 (Max)
    // Fields: Zm=0, Rn=31, prfop=0, Pg=0, xs=0
    let encoding: u32 = 0x842003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_prfop_0_min_0_84200000() {
    // Encoding: 0x84200000
    // Test PRFB_I.P.BZ_S.x32.scaled field prfop = 0 (Min)
    // Fields: Pg=0, xs=0, Rn=0, prfop=0, Zm=0
    let encoding: u32 = 0x84200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_prfop_1_poweroftwo_0_84200001() {
    // Encoding: 0x84200001
    // Test PRFB_I.P.BZ_S.x32.scaled field prfop = 1 (PowerOfTwo)
    // Fields: Rn=0, Pg=0, xs=0, prfop=1, Zm=0
    let encoding: u32 = 0x84200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_prfop_7_poweroftwominusone_0_84200007() {
    // Encoding: 0x84200007
    // Test PRFB_I.P.BZ_S.x32.scaled field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Zm=0, prfop=7, Rn=0, xs=0, Pg=0
    let encoding: u32 = 0x84200007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_field_prfop_15_max_0_8420000f() {
    // Encoding: 0x8420000F
    // Test PRFB_I.P.BZ_S.x32.scaled field prfop = 15 (Max)
    // Fields: Pg=0, Rn=0, prfop=15, xs=0, Zm=0
    let encoding: u32 = 0x8420000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=0 (minimum value)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_0_0_84200000() {
    // Encoding: 0x84200000
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Pg=0, Rn=0, xs=0, prfop=0
    let encoding: u32 = 0x84200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=1 (maximum value (1))
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_1_0_84600000() {
    // Encoding: 0x84600000
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=1, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: xs=1, Zm=0, Rn=0, prfop=0, Pg=0
    let encoding: u32 = 0x84600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_2_0_84200000() {
    // Encoding: 0x84200000
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0x84200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_3_0_84210000() {
    // Encoding: 0x84210000
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=1, Pg=0, Rn=0, prfop=0
    // Fields: Zm=1, Rn=0, Pg=0, prfop=0, xs=0
    let encoding: u32 = 0x84210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_4_0_843e0000() {
    // Encoding: 0x843E0000
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=30, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Zm=30, prfop=0, xs=0, Rn=0
    let encoding: u32 = 0x843E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_5_0_843f0000() {
    // Encoding: 0x843F0000
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=31, Pg=0, Rn=0, prfop=0
    // Fields: xs=0, Rn=0, prfop=0, Zm=31, Pg=0
    let encoding: u32 = 0x843F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_6_0_84200000() {
    // Encoding: 0x84200000
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Pg=0, xs=0, Zm=0, Rn=0
    let encoding: u32 = 0x84200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_7_0_84200400() {
    // Encoding: 0x84200400
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=0, prfop=0
    // Fields: xs=0, prfop=0, Zm=0, Pg=1, Rn=0
    let encoding: u32 = 0x84200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_8_0_84200000() {
    // Encoding: 0x84200000
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: xs=0, Zm=0, Rn=0, Pg=0, prfop=0
    let encoding: u32 = 0x84200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_9_0_84200020() {
    // Encoding: 0x84200020
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=1, prfop=0
    // Fields: Zm=0, prfop=0, Pg=0, Rn=1, xs=0
    let encoding: u32 = 0x84200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_10_0_842003c0() {
    // Encoding: 0x842003C0
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=30, prfop=0
    // Fields: Pg=0, xs=0, Rn=30, prfop=0, Zm=0
    let encoding: u32 = 0x842003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_11_0_842003e0() {
    // Encoding: 0x842003E0
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=31, prfop=0
    // Fields: xs=0, Zm=0, Rn=31, Pg=0, prfop=0
    let encoding: u32 = 0x842003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_12_0_84200000() {
    // Encoding: 0x84200000
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0x84200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_13_0_84200001() {
    // Encoding: 0x84200001
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=1
    // Fields: prfop=1, Zm=0, Pg=0, xs=0, Rn=0
    let encoding: u32 = 0x84200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_14_0_84200007() {
    // Encoding: 0x84200007
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=7
    // Fields: xs=0, Zm=0, Pg=0, prfop=7, Rn=0
    let encoding: u32 = 0x84200007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_15_0_8420000f() {
    // Encoding: 0x8420000F
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=15
    // Fields: Zm=0, Pg=0, xs=0, prfop=15, Rn=0
    let encoding: u32 = 0x8420000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_16_0_84200420() {
    // Encoding: 0x84200420
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=1, prfop=0
    // Fields: prfop=0, Rn=1, xs=0, Zm=0, Pg=1
    let encoding: u32 = 0x84200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_combo_17_0_84201fe0() {
    // Encoding: 0x84201FE0
    // Test PRFB_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=31, Rn=31, prfop=0
    // Fields: Zm=0, Pg=31, Rn=31, xs=0, prfop=0
    let encoding: u32 = 0x84201FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_special_rn_31_stack_pointer_sp_may_require_alignment_0_842003e0() {
    // Encoding: 0x842003E0
    // Test PRFB_I.P.BZ_S.x32.scaled special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Zm=0, xs=0, Pg=0, prfop=0
    let encoding: u32 = 0x842003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_xs_0_min_0_c4200000() {
    // Encoding: 0xC4200000
    // Test PRFB_I.P.BZ_D.x32.scaled field xs = 0 (Min)
    // Fields: Pg=0, xs=0, Zm=0, Rn=0, prfop=0
    let encoding: u32 = 0xC4200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_xs_1_max_0_c4600000() {
    // Encoding: 0xC4600000
    // Test PRFB_I.P.BZ_D.x32.scaled field xs = 1 (Max)
    // Fields: xs=1, Zm=0, prfop=0, Rn=0, Pg=0
    let encoding: u32 = 0xC4600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_zm_0_min_0_c4200000() {
    // Encoding: 0xC4200000
    // Test PRFB_I.P.BZ_D.x32.scaled field Zm = 0 (Min)
    // Fields: prfop=0, Rn=0, Zm=0, xs=0, Pg=0
    let encoding: u32 = 0xC4200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_zm_1_poweroftwo_0_c4210000() {
    // Encoding: 0xC4210000
    // Test PRFB_I.P.BZ_D.x32.scaled field Zm = 1 (PowerOfTwo)
    // Fields: xs=0, Pg=0, Zm=1, Rn=0, prfop=0
    let encoding: u32 = 0xC4210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_zm_30_poweroftwominusone_0_c43e0000() {
    // Encoding: 0xC43E0000
    // Test PRFB_I.P.BZ_D.x32.scaled field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, xs=0, Zm=30, Pg=0, prfop=0
    let encoding: u32 = 0xC43E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_zm_31_max_0_c43f0000() {
    // Encoding: 0xC43F0000
    // Test PRFB_I.P.BZ_D.x32.scaled field Zm = 31 (Max)
    // Fields: xs=0, Pg=0, Zm=31, Rn=0, prfop=0
    let encoding: u32 = 0xC43F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_pg_0_min_0_c4200000() {
    // Encoding: 0xC4200000
    // Test PRFB_I.P.BZ_D.x32.scaled field Pg = 0 (Min)
    // Fields: Zm=0, Pg=0, prfop=0, xs=0, Rn=0
    let encoding: u32 = 0xC4200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_pg_1_poweroftwo_0_c4200400() {
    // Encoding: 0xC4200400
    // Test PRFB_I.P.BZ_D.x32.scaled field Pg = 1 (PowerOfTwo)
    // Fields: prfop=0, Pg=1, xs=0, Rn=0, Zm=0
    let encoding: u32 = 0xC4200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_rn_0_min_0_c4200000() {
    // Encoding: 0xC4200000
    // Test PRFB_I.P.BZ_D.x32.scaled field Rn = 0 (Min)
    // Fields: Zm=0, xs=0, Rn=0, prfop=0, Pg=0
    let encoding: u32 = 0xC4200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_rn_1_poweroftwo_0_c4200020() {
    // Encoding: 0xC4200020
    // Test PRFB_I.P.BZ_D.x32.scaled field Rn = 1 (PowerOfTwo)
    // Fields: Pg=0, Rn=1, xs=0, Zm=0, prfop=0
    let encoding: u32 = 0xC4200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_rn_30_poweroftwominusone_0_c42003c0() {
    // Encoding: 0xC42003C0
    // Test PRFB_I.P.BZ_D.x32.scaled field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: xs=0, Zm=0, Rn=30, Pg=0, prfop=0
    let encoding: u32 = 0xC42003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_rn_31_max_0_c42003e0() {
    // Encoding: 0xC42003E0
    // Test PRFB_I.P.BZ_D.x32.scaled field Rn = 31 (Max)
    // Fields: Rn=31, Zm=0, prfop=0, xs=0, Pg=0
    let encoding: u32 = 0xC42003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_prfop_0_min_0_c4200000() {
    // Encoding: 0xC4200000
    // Test PRFB_I.P.BZ_D.x32.scaled field prfop = 0 (Min)
    // Fields: xs=0, prfop=0, Pg=0, Zm=0, Rn=0
    let encoding: u32 = 0xC4200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_prfop_1_poweroftwo_0_c4200001() {
    // Encoding: 0xC4200001
    // Test PRFB_I.P.BZ_D.x32.scaled field prfop = 1 (PowerOfTwo)
    // Fields: xs=0, Pg=0, prfop=1, Rn=0, Zm=0
    let encoding: u32 = 0xC4200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_prfop_7_poweroftwominusone_0_c4200007() {
    // Encoding: 0xC4200007
    // Test PRFB_I.P.BZ_D.x32.scaled field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, xs=0, Zm=0, prfop=7, Pg=0
    let encoding: u32 = 0xC4200007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_field_prfop_15_max_0_c420000f() {
    // Encoding: 0xC420000F
    // Test PRFB_I.P.BZ_D.x32.scaled field prfop = 15 (Max)
    // Fields: Zm=0, Pg=0, xs=0, prfop=15, Rn=0
    let encoding: u32 = 0xC420000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=0 (minimum value)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_0_0_c4200000() {
    // Encoding: 0xC4200000
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, xs=0, prfop=0, Zm=0, Pg=0
    let encoding: u32 = 0xC4200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=1 (maximum value (1))
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_1_0_c4600000() {
    // Encoding: 0xC4600000
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=1, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Rn=0, prfop=0, Pg=0, xs=1
    let encoding: u32 = 0xC4600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_2_0_c4200000() {
    // Encoding: 0xC4200000
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, Zm=0, prfop=0, xs=0
    let encoding: u32 = 0xC4200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_3_0_c4210000() {
    // Encoding: 0xC4210000
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=1, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, Zm=1, xs=0, Pg=0
    let encoding: u32 = 0xC4210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_4_0_c43e0000() {
    // Encoding: 0xC43E0000
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=30, Pg=0, Rn=0, prfop=0
    // Fields: xs=0, Rn=0, Zm=30, Pg=0, prfop=0
    let encoding: u32 = 0xC43E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_5_0_c43f0000() {
    // Encoding: 0xC43F0000
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=31, Pg=0, Rn=0, prfop=0
    // Fields: xs=0, prfop=0, Zm=31, Rn=0, Pg=0
    let encoding: u32 = 0xC43F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_6_0_c4200000() {
    // Encoding: 0xC4200000
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Rn=0, Pg=0, xs=0, prfop=0
    let encoding: u32 = 0xC4200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_7_0_c4200400() {
    // Encoding: 0xC4200400
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=0, prfop=0
    // Fields: Zm=0, Pg=1, xs=0, prfop=0, Rn=0
    let encoding: u32 = 0xC4200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_8_0_c4200000() {
    // Encoding: 0xC4200000
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, xs=0, Pg=0, Zm=0
    let encoding: u32 = 0xC4200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_9_0_c4200020() {
    // Encoding: 0xC4200020
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=1, prfop=0
    // Fields: Rn=1, Pg=0, prfop=0, xs=0, Zm=0
    let encoding: u32 = 0xC4200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_10_0_c42003c0() {
    // Encoding: 0xC42003C0
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=30, prfop=0
    // Fields: Rn=30, xs=0, Pg=0, Zm=0, prfop=0
    let encoding: u32 = 0xC42003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_11_0_c42003e0() {
    // Encoding: 0xC42003E0
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=31, prfop=0
    // Fields: xs=0, Rn=31, prfop=0, Pg=0, Zm=0
    let encoding: u32 = 0xC42003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_12_0_c4200000() {
    // Encoding: 0xC4200000
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Pg=0, Rn=0, prfop=0, xs=0
    let encoding: u32 = 0xC4200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_13_0_c4200001() {
    // Encoding: 0xC4200001
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=1
    // Fields: Zm=0, prfop=1, Pg=0, xs=0, Rn=0
    let encoding: u32 = 0xC4200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_14_0_c4200007() {
    // Encoding: 0xC4200007
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=7
    // Fields: xs=0, prfop=7, Zm=0, Pg=0, Rn=0
    let encoding: u32 = 0xC4200007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_15_0_c420000f() {
    // Encoding: 0xC420000F
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=15
    // Fields: Zm=0, Pg=0, prfop=15, xs=0, Rn=0
    let encoding: u32 = 0xC420000F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_16_0_c4200420() {
    // Encoding: 0xC4200420
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=1, prfop=0
    // Fields: xs=0, Pg=1, Rn=1, prfop=0, Zm=0
    let encoding: u32 = 0xC4200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_combo_17_0_c4201fe0() {
    // Encoding: 0xC4201FE0
    // Test PRFB_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=31, Rn=31, prfop=0
    // Fields: Zm=0, Rn=31, xs=0, prfop=0, Pg=31
    let encoding: u32 = 0xC4201FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_special_rn_31_stack_pointer_sp_may_require_alignment_0_c42003e0() {
    // Encoding: 0xC42003E0
    // Test PRFB_I.P.BZ_D.x32.scaled special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Pg=0, Rn=31, xs=0, Zm=0, prfop=0
    let encoding: u32 = 0xC42003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_zm_0_min_8000_c4608000() {
    // Encoding: 0xC4608000
    // Test PRFB_I.P.BZ_D.64.scaled field Zm = 0 (Min)
    // Fields: Pg=0, prfop=0, Zm=0, Rn=0
    let encoding: u32 = 0xC4608000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_zm_1_poweroftwo_8000_c4618000() {
    // Encoding: 0xC4618000
    // Test PRFB_I.P.BZ_D.64.scaled field Zm = 1 (PowerOfTwo)
    // Fields: prfop=0, Rn=0, Pg=0, Zm=1
    let encoding: u32 = 0xC4618000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_zm_30_poweroftwominusone_8000_c47e8000() {
    // Encoding: 0xC47E8000
    // Test PRFB_I.P.BZ_D.64.scaled field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, prfop=0, Zm=30, Rn=0
    let encoding: u32 = 0xC47E8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_zm_31_max_8000_c47f8000() {
    // Encoding: 0xC47F8000
    // Test PRFB_I.P.BZ_D.64.scaled field Zm = 31 (Max)
    // Fields: prfop=0, Rn=0, Zm=31, Pg=0
    let encoding: u32 = 0xC47F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_pg_0_min_8000_c4608000() {
    // Encoding: 0xC4608000
    // Test PRFB_I.P.BZ_D.64.scaled field Pg = 0 (Min)
    // Fields: prfop=0, Pg=0, Rn=0, Zm=0
    let encoding: u32 = 0xC4608000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_pg_1_poweroftwo_8000_c4608400() {
    // Encoding: 0xC4608400
    // Test PRFB_I.P.BZ_D.64.scaled field Pg = 1 (PowerOfTwo)
    // Fields: prfop=0, Rn=0, Pg=1, Zm=0
    let encoding: u32 = 0xC4608400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_rn_0_min_8000_c4608000() {
    // Encoding: 0xC4608000
    // Test PRFB_I.P.BZ_D.64.scaled field Rn = 0 (Min)
    // Fields: Zm=0, Rn=0, prfop=0, Pg=0
    let encoding: u32 = 0xC4608000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_rn_1_poweroftwo_8000_c4608020() {
    // Encoding: 0xC4608020
    // Test PRFB_I.P.BZ_D.64.scaled field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Zm=0, prfop=0, Pg=0
    let encoding: u32 = 0xC4608020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_rn_30_poweroftwominusone_8000_c46083c0() {
    // Encoding: 0xC46083C0
    // Test PRFB_I.P.BZ_D.64.scaled field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Zm=0, prfop=0, Rn=30, Pg=0
    let encoding: u32 = 0xC46083C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_rn_31_max_8000_c46083e0() {
    // Encoding: 0xC46083E0
    // Test PRFB_I.P.BZ_D.64.scaled field Rn = 31 (Max)
    // Fields: prfop=0, Pg=0, Rn=31, Zm=0
    let encoding: u32 = 0xC46083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_prfop_0_min_8000_c4608000() {
    // Encoding: 0xC4608000
    // Test PRFB_I.P.BZ_D.64.scaled field prfop = 0 (Min)
    // Fields: Rn=0, Zm=0, Pg=0, prfop=0
    let encoding: u32 = 0xC4608000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_prfop_1_poweroftwo_8000_c4608001() {
    // Encoding: 0xC4608001
    // Test PRFB_I.P.BZ_D.64.scaled field prfop = 1 (PowerOfTwo)
    // Fields: Pg=0, prfop=1, Zm=0, Rn=0
    let encoding: u32 = 0xC4608001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_prfop_7_poweroftwominusone_8000_c4608007() {
    // Encoding: 0xC4608007
    // Test PRFB_I.P.BZ_D.64.scaled field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Zm=0, prfop=7, Pg=0, Rn=0
    let encoding: u32 = 0xC4608007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_field_prfop_15_max_8000_c460800f() {
    // Encoding: 0xC460800F
    // Test PRFB_I.P.BZ_D.64.scaled field prfop = 15 (Max)
    // Fields: Zm=0, Pg=0, Rn=0, prfop=15
    let encoding: u32 = 0xC460800F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_0_8000_c4608000() {
    // Encoding: 0xC4608000
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Pg=0, Zm=0, Rn=0
    let encoding: u32 = 0xC4608000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_1_8000_c4618000() {
    // Encoding: 0xC4618000
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=1, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Zm=1, prfop=0, Pg=0
    let encoding: u32 = 0xC4618000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_2_8000_c47e8000() {
    // Encoding: 0xC47E8000
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=30, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Zm=30, Rn=0, Pg=0
    let encoding: u32 = 0xC47E8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_3_8000_c47f8000() {
    // Encoding: 0xC47F8000
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=31, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, prfop=0, Zm=31
    let encoding: u32 = 0xC47F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_4_8000_c4608000() {
    // Encoding: 0xC4608000
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Zm=0, Pg=0, prfop=0
    let encoding: u32 = 0xC4608000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_5_8000_c4608400() {
    // Encoding: 0xC4608400
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=1, Rn=0, prfop=0
    // Fields: Pg=1, Zm=0, Rn=0, prfop=0
    let encoding: u32 = 0xC4608400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_6_8000_c4608000() {
    // Encoding: 0xC4608000
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rn=0, prfop=0, Zm=0
    let encoding: u32 = 0xC4608000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_7_8000_c4608020() {
    // Encoding: 0xC4608020
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=1, prfop=0
    // Fields: prfop=0, Pg=0, Rn=1, Zm=0
    let encoding: u32 = 0xC4608020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_8_8000_c46083c0() {
    // Encoding: 0xC46083C0
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=30, prfop=0
    // Fields: Rn=30, Zm=0, Pg=0, prfop=0
    let encoding: u32 = 0xC46083C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_9_8000_c46083e0() {
    // Encoding: 0xC46083E0
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=31, prfop=0
    // Fields: Zm=0, Pg=0, prfop=0, Rn=31
    let encoding: u32 = 0xC46083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_10_8000_c4608000() {
    // Encoding: 0xC4608000
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0xC4608000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_11_8000_c4608001() {
    // Encoding: 0xC4608001
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=1
    // Fields: Rn=0, Zm=0, prfop=1, Pg=0
    let encoding: u32 = 0xC4608001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_12_8000_c4608007() {
    // Encoding: 0xC4608007
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=7
    // Fields: Pg=0, Rn=0, prfop=7, Zm=0
    let encoding: u32 = 0xC4608007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_13_8000_c460800f() {
    // Encoding: 0xC460800F
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=15
    // Fields: Pg=0, Zm=0, Rn=0, prfop=15
    let encoding: u32 = 0xC460800F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_14_8000_c4608420() {
    // Encoding: 0xC4608420
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=1, Rn=1, prfop=0
    // Fields: Zm=0, Rn=1, Pg=1, prfop=0
    let encoding: u32 = 0xC4608420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfb_i_p_bz_d_64_scaled_combo_15_8000_c4609fe0() {
    // Encoding: 0xC4609FE0
    // Test PRFB_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=31, Rn=31, prfop=0
    // Fields: Pg=31, Rn=31, prfop=0, Zm=0
    let encoding: u32 = 0xC4609FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfb_i_p_bz_d_64_scaled_special_rn_31_stack_pointer_sp_may_require_alignment_32768_c46083e0()
 {
    // Encoding: 0xC46083E0
    // Test PRFB_I.P.BZ_D.64.scaled special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Zm=0, Pg=0, Rn=31, prfop=0
    let encoding: u32 = 0xC46083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BZ_S.x32.scaled
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfb_i_p_bz_s_x32_scaled_sp_rn_842003e0() {
    // Test PRFB_I.P.BZ_S.x32.scaled with Rn = SP (31)
    // Encoding: 0x842003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x842003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PRFB_I.P.BZ_D.x32.scaled
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfb_i_p_bz_d_x32_scaled_sp_rn_c42003e0() {
    // Test PRFB_I.P.BZ_D.x32.scaled with Rn = SP (31)
    // Encoding: 0xC42003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xC42003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PRFB_I.P.BZ_D.64.scaled
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfb_i_p_bz_d_64_scaled_sp_rn_c46083e0() {
    // Test PRFB_I.P.BZ_D.64.scaled with Rn = SP (31)
    // Encoding: 0xC46083E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xC46083E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PRFW_I.P.BI_S Tests
// ============================================================================

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_prfw_i_p_bi_s_field_imm6_0_zero_4000_85c04000() {
    // Encoding: 0x85C04000
    // Test PRFW_I.P.BI_S field imm6 = 0 (Zero)
    // Fields: imm6=0, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0x85C04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_prfw_i_p_bi_s_field_imm6_1_poweroftwo_4000_85c14000() {
    // Encoding: 0x85C14000
    // Test PRFW_I.P.BI_S field imm6 = 1 (PowerOfTwo)
    // Fields: imm6=1, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0x85C14000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_prfw_i_p_bi_s_field_imm6_3_poweroftwominusone_4000_85c34000() {
    // Encoding: 0x85C34000
    // Test PRFW_I.P.BI_S field imm6 = 3 (PowerOfTwoMinusOne)
    // Fields: prfop=0, imm6=3, Rn=0, Pg=0
    let encoding: u32 = 0x85C34000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_prfw_i_p_bi_s_field_imm6_4_poweroftwo_4000_85c44000() {
    // Encoding: 0x85C44000
    // Test PRFW_I.P.BI_S field imm6 = 4 (PowerOfTwo)
    // Fields: prfop=0, imm6=4, Pg=0, Rn=0
    let encoding: u32 = 0x85C44000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_prfw_i_p_bi_s_field_imm6_7_poweroftwominusone_4000_85c74000() {
    // Encoding: 0x85C74000
    // Test PRFW_I.P.BI_S field imm6 = 7 (PowerOfTwoMinusOne)
    // Fields: imm6=7, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0x85C74000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_prfw_i_p_bi_s_field_imm6_8_poweroftwo_4000_85c84000() {
    // Encoding: 0x85C84000
    // Test PRFW_I.P.BI_S field imm6 = 8 (PowerOfTwo)
    // Fields: prfop=0, imm6=8, Rn=0, Pg=0
    let encoding: u32 = 0x85C84000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_prfw_i_p_bi_s_field_imm6_15_poweroftwominusone_4000_85cf4000() {
    // Encoding: 0x85CF4000
    // Test PRFW_I.P.BI_S field imm6 = 15 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Rn=0, Pg=0, imm6=15
    let encoding: u32 = 0x85CF4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_prfw_i_p_bi_s_field_imm6_16_poweroftwo_4000_85d04000() {
    // Encoding: 0x85D04000
    // Test PRFW_I.P.BI_S field imm6 = 16 (PowerOfTwo)
    // Fields: prfop=0, imm6=16, Pg=0, Rn=0
    let encoding: u32 = 0x85D04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_prfw_i_p_bi_s_field_imm6_31_poweroftwominusone_4000_85df4000() {
    // Encoding: 0x85DF4000
    // Test PRFW_I.P.BI_S field imm6 = 31 (PowerOfTwoMinusOne)
    // Fields: Rn=0, imm6=31, prfop=0, Pg=0
    let encoding: u32 = 0x85DF4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_prfw_i_p_bi_s_field_imm6_32_poweroftwo_4000_85e04000() {
    // Encoding: 0x85E04000
    // Test PRFW_I.P.BI_S field imm6 = 32 (PowerOfTwo)
    // Fields: imm6=32, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0x85E04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_prfw_i_p_bi_s_field_imm6_63_max_4000_85ff4000() {
    // Encoding: 0x85FF4000
    // Test PRFW_I.P.BI_S field imm6 = 63 (Max)
    // Fields: Pg=0, prfop=0, Rn=0, imm6=63
    let encoding: u32 = 0x85FF4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfw_i_p_bi_s_field_pg_0_min_4000_85c04000() {
    // Encoding: 0x85C04000
    // Test PRFW_I.P.BI_S field Pg = 0 (Min)
    // Fields: Rn=0, imm6=0, Pg=0, prfop=0
    let encoding: u32 = 0x85C04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfw_i_p_bi_s_field_pg_1_poweroftwo_4000_85c04400() {
    // Encoding: 0x85C04400
    // Test PRFW_I.P.BI_S field Pg = 1 (PowerOfTwo)
    // Fields: imm6=0, Pg=1, prfop=0, Rn=0
    let encoding: u32 = 0x85C04400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfw_i_p_bi_s_field_rn_0_min_4000_85c04000() {
    // Encoding: 0x85C04000
    // Test PRFW_I.P.BI_S field Rn = 0 (Min)
    // Fields: imm6=0, prfop=0, Rn=0, Pg=0
    let encoding: u32 = 0x85C04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfw_i_p_bi_s_field_rn_1_poweroftwo_4000_85c04020() {
    // Encoding: 0x85C04020
    // Test PRFW_I.P.BI_S field Rn = 1 (PowerOfTwo)
    // Fields: imm6=0, Pg=0, Rn=1, prfop=0
    let encoding: u32 = 0x85C04020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfw_i_p_bi_s_field_rn_30_poweroftwominusone_4000_85c043c0() {
    // Encoding: 0x85C043C0
    // Test PRFW_I.P.BI_S field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, imm6=0, prfop=0, Pg=0
    let encoding: u32 = 0x85C043C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfw_i_p_bi_s_field_rn_31_max_4000_85c043e0() {
    // Encoding: 0x85C043E0
    // Test PRFW_I.P.BI_S field Rn = 31 (Max)
    // Fields: Pg=0, Rn=31, prfop=0, imm6=0
    let encoding: u32 = 0x85C043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfw_i_p_bi_s_field_prfop_0_min_4000_85c04000() {
    // Encoding: 0x85C04000
    // Test PRFW_I.P.BI_S field prfop = 0 (Min)
    // Fields: prfop=0, Rn=0, imm6=0, Pg=0
    let encoding: u32 = 0x85C04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfw_i_p_bi_s_field_prfop_1_poweroftwo_4000_85c04001() {
    // Encoding: 0x85C04001
    // Test PRFW_I.P.BI_S field prfop = 1 (PowerOfTwo)
    // Fields: prfop=1, Rn=0, imm6=0, Pg=0
    let encoding: u32 = 0x85C04001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfw_i_p_bi_s_field_prfop_7_poweroftwominusone_4000_85c04007() {
    // Encoding: 0x85C04007
    // Test PRFW_I.P.BI_S field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: imm6=0, Rn=0, prfop=7, Pg=0
    let encoding: u32 = 0x85C04007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfw_i_p_bi_s_field_prfop_15_max_4000_85c0400f() {
    // Encoding: 0x85C0400F
    // Test PRFW_I.P.BI_S field prfop = 15 (Max)
    // Fields: prfop=15, Pg=0, imm6=0, Rn=0
    let encoding: u32 = 0x85C0400F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=0 (immediate value 0)
#[test]
fn test_prfw_i_p_bi_s_combo_0_4000_85c04000() {
    // Encoding: 0x85C04000
    // Test PRFW_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, imm6=0, Rn=0, Pg=0
    let encoding: u32 = 0x85C04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=1 (immediate value 1)
#[test]
fn test_prfw_i_p_bi_s_combo_1_4000_85c14000() {
    // Encoding: 0x85C14000
    // Test PRFW_I.P.BI_S field combination: imm6=1, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, imm6=1, Pg=0, prfop=0
    let encoding: u32 = 0x85C14000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=3 (2^2 - 1 = 3)
#[test]
fn test_prfw_i_p_bi_s_combo_2_4000_85c34000() {
    // Encoding: 0x85C34000
    // Test PRFW_I.P.BI_S field combination: imm6=3, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, imm6=3, prfop=0
    let encoding: u32 = 0x85C34000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=4 (power of 2 (2^2 = 4))
#[test]
fn test_prfw_i_p_bi_s_combo_3_4000_85c44000() {
    // Encoding: 0x85C44000
    // Test PRFW_I.P.BI_S field combination: imm6=4, Pg=0, Rn=0, prfop=0
    // Fields: imm6=4, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0x85C44000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=7 (2^3 - 1 = 7)
#[test]
fn test_prfw_i_p_bi_s_combo_4_4000_85c74000() {
    // Encoding: 0x85C74000
    // Test PRFW_I.P.BI_S field combination: imm6=7, Pg=0, Rn=0, prfop=0
    // Fields: imm6=7, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0x85C74000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=8 (power of 2 (2^3 = 8))
#[test]
fn test_prfw_i_p_bi_s_combo_5_4000_85c84000() {
    // Encoding: 0x85C84000
    // Test PRFW_I.P.BI_S field combination: imm6=8, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, imm6=8, Pg=0, prfop=0
    let encoding: u32 = 0x85C84000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=15 (2^4 - 1 = 15)
#[test]
fn test_prfw_i_p_bi_s_combo_6_4000_85cf4000() {
    // Encoding: 0x85CF4000
    // Test PRFW_I.P.BI_S field combination: imm6=15, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, prfop=0, imm6=15, Rn=0
    let encoding: u32 = 0x85CF4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=16 (power of 2 (2^4 = 16))
#[test]
fn test_prfw_i_p_bi_s_combo_7_4000_85d04000() {
    // Encoding: 0x85D04000
    // Test PRFW_I.P.BI_S field combination: imm6=16, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, prfop=0, Pg=0, imm6=16
    let encoding: u32 = 0x85D04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=31 (immediate midpoint (31))
#[test]
fn test_prfw_i_p_bi_s_combo_8_4000_85df4000() {
    // Encoding: 0x85DF4000
    // Test PRFW_I.P.BI_S field combination: imm6=31, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, imm6=31, prfop=0
    let encoding: u32 = 0x85DF4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=32 (power of 2 (2^5 = 32))
#[test]
fn test_prfw_i_p_bi_s_combo_9_4000_85e04000() {
    // Encoding: 0x85E04000
    // Test PRFW_I.P.BI_S field combination: imm6=32, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, imm6=32, Pg=0, Rn=0
    let encoding: u32 = 0x85E04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=63 (maximum immediate (63))
#[test]
fn test_prfw_i_p_bi_s_combo_10_4000_85ff4000() {
    // Encoding: 0x85FF4000
    // Test PRFW_I.P.BI_S field combination: imm6=63, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rn=0, prfop=0, imm6=63
    let encoding: u32 = 0x85FF4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfw_i_p_bi_s_combo_11_4000_85c04000() {
    // Encoding: 0x85C04000
    // Test PRFW_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, imm6=0, Pg=0, Rn=0
    let encoding: u32 = 0x85C04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfw_i_p_bi_s_combo_12_4000_85c04400() {
    // Encoding: 0x85C04400
    // Test PRFW_I.P.BI_S field combination: imm6=0, Pg=1, Rn=0, prfop=0
    // Fields: prfop=0, imm6=0, Rn=0, Pg=1
    let encoding: u32 = 0x85C04400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfw_i_p_bi_s_combo_13_4000_85c04000() {
    // Encoding: 0x85C04000
    // Test PRFW_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, imm6=0, Rn=0, prfop=0
    let encoding: u32 = 0x85C04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfw_i_p_bi_s_combo_14_4000_85c04020() {
    // Encoding: 0x85C04020
    // Test PRFW_I.P.BI_S field combination: imm6=0, Pg=0, Rn=1, prfop=0
    // Fields: imm6=0, prfop=0, Rn=1, Pg=0
    let encoding: u32 = 0x85C04020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfw_i_p_bi_s_combo_15_4000_85c043c0() {
    // Encoding: 0x85C043C0
    // Test PRFW_I.P.BI_S field combination: imm6=0, Pg=0, Rn=30, prfop=0
    // Fields: imm6=0, Rn=30, Pg=0, prfop=0
    let encoding: u32 = 0x85C043C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfw_i_p_bi_s_combo_16_4000_85c043e0() {
    // Encoding: 0x85C043E0
    // Test PRFW_I.P.BI_S field combination: imm6=0, Pg=0, Rn=31, prfop=0
    // Fields: prfop=0, Rn=31, imm6=0, Pg=0
    let encoding: u32 = 0x85C043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfw_i_p_bi_s_combo_17_4000_85c04000() {
    // Encoding: 0x85C04000
    // Test PRFW_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, Pg=0, imm6=0
    let encoding: u32 = 0x85C04000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfw_i_p_bi_s_combo_18_4000_85c04001() {
    // Encoding: 0x85C04001
    // Test PRFW_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=1
    // Fields: imm6=0, prfop=1, Pg=0, Rn=0
    let encoding: u32 = 0x85C04001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfw_i_p_bi_s_combo_19_4000_85c04007() {
    // Encoding: 0x85C04007
    // Test PRFW_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=7
    // Fields: prfop=7, Pg=0, imm6=0, Rn=0
    let encoding: u32 = 0x85C04007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfw_i_p_bi_s_combo_20_4000_85c0400f() {
    // Encoding: 0x85C0400F
    // Test PRFW_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=15
    // Fields: Pg=0, Rn=0, imm6=0, prfop=15
    let encoding: u32 = 0x85C0400F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfw_i_p_bi_s_combo_21_4000_85c04420() {
    // Encoding: 0x85C04420
    // Test PRFW_I.P.BI_S field combination: imm6=0, Pg=1, Rn=1, prfop=0
    // Fields: Pg=1, Rn=1, prfop=0, imm6=0
    let encoding: u32 = 0x85C04420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfw_i_p_bi_s_combo_22_4000_85c05fe0() {
    // Encoding: 0x85C05FE0
    // Test PRFW_I.P.BI_S field combination: imm6=0, Pg=31, Rn=31, prfop=0
    // Fields: Pg=31, Rn=31, imm6=0, prfop=0
    let encoding: u32 = 0x85C05FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfw_i_p_bi_s_special_rn_31_stack_pointer_sp_may_require_alignment_16384_85c143e0() {
    // Encoding: 0x85C143E0
    // Test PRFW_I.P.BI_S special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: prfop=0, Rn=31, Pg=0, imm6=1
    let encoding: u32 = 0x85C143E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BI_S
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfw_i_p_bi_s_sp_rn_85c043e0() {
    // Test PRFW_I.P.BI_S with Rn = SP (31)
    // Encoding: 0x85C043E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x85C043E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PRFD_I.P.BI_S Tests
// ============================================================================

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_prfd_i_p_bi_s_field_imm6_0_zero_6000_85c06000() {
    // Encoding: 0x85C06000
    // Test PRFD_I.P.BI_S field imm6 = 0 (Zero)
    // Fields: Pg=0, imm6=0, Rn=0, prfop=0
    let encoding: u32 = 0x85C06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_prfd_i_p_bi_s_field_imm6_1_poweroftwo_6000_85c16000() {
    // Encoding: 0x85C16000
    // Test PRFD_I.P.BI_S field imm6 = 1 (PowerOfTwo)
    // Fields: prfop=0, Pg=0, imm6=1, Rn=0
    let encoding: u32 = 0x85C16000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_prfd_i_p_bi_s_field_imm6_3_poweroftwominusone_6000_85c36000() {
    // Encoding: 0x85C36000
    // Test PRFD_I.P.BI_S field imm6 = 3 (PowerOfTwoMinusOne)
    // Fields: Rn=0, prfop=0, Pg=0, imm6=3
    let encoding: u32 = 0x85C36000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_prfd_i_p_bi_s_field_imm6_4_poweroftwo_6000_85c46000() {
    // Encoding: 0x85C46000
    // Test PRFD_I.P.BI_S field imm6 = 4 (PowerOfTwo)
    // Fields: imm6=4, prfop=0, Rn=0, Pg=0
    let encoding: u32 = 0x85C46000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_prfd_i_p_bi_s_field_imm6_7_poweroftwominusone_6000_85c76000() {
    // Encoding: 0x85C76000
    // Test PRFD_I.P.BI_S field imm6 = 7 (PowerOfTwoMinusOne)
    // Fields: imm6=7, Rn=0, Pg=0, prfop=0
    let encoding: u32 = 0x85C76000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_prfd_i_p_bi_s_field_imm6_8_poweroftwo_6000_85c86000() {
    // Encoding: 0x85C86000
    // Test PRFD_I.P.BI_S field imm6 = 8 (PowerOfTwo)
    // Fields: Rn=0, Pg=0, prfop=0, imm6=8
    let encoding: u32 = 0x85C86000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_prfd_i_p_bi_s_field_imm6_15_poweroftwominusone_6000_85cf6000() {
    // Encoding: 0x85CF6000
    // Test PRFD_I.P.BI_S field imm6 = 15 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Pg=0, imm6=15, Rn=0
    let encoding: u32 = 0x85CF6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_prfd_i_p_bi_s_field_imm6_16_poweroftwo_6000_85d06000() {
    // Encoding: 0x85D06000
    // Test PRFD_I.P.BI_S field imm6 = 16 (PowerOfTwo)
    // Fields: Rn=0, imm6=16, prfop=0, Pg=0
    let encoding: u32 = 0x85D06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_prfd_i_p_bi_s_field_imm6_31_poweroftwominusone_6000_85df6000() {
    // Encoding: 0x85DF6000
    // Test PRFD_I.P.BI_S field imm6 = 31 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Rn=0, Pg=0, imm6=31
    let encoding: u32 = 0x85DF6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_prfd_i_p_bi_s_field_imm6_32_poweroftwo_6000_85e06000() {
    // Encoding: 0x85E06000
    // Test PRFD_I.P.BI_S field imm6 = 32 (PowerOfTwo)
    // Fields: prfop=0, Pg=0, Rn=0, imm6=32
    let encoding: u32 = 0x85E06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_prfd_i_p_bi_s_field_imm6_63_max_6000_85ff6000() {
    // Encoding: 0x85FF6000
    // Test PRFD_I.P.BI_S field imm6 = 63 (Max)
    // Fields: imm6=63, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0x85FF6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfd_i_p_bi_s_field_pg_0_min_6000_85c06000() {
    // Encoding: 0x85C06000
    // Test PRFD_I.P.BI_S field Pg = 0 (Min)
    // Fields: Rn=0, Pg=0, prfop=0, imm6=0
    let encoding: u32 = 0x85C06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfd_i_p_bi_s_field_pg_1_poweroftwo_6000_85c06400() {
    // Encoding: 0x85C06400
    // Test PRFD_I.P.BI_S field Pg = 1 (PowerOfTwo)
    // Fields: prfop=0, imm6=0, Pg=1, Rn=0
    let encoding: u32 = 0x85C06400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfd_i_p_bi_s_field_rn_0_min_6000_85c06000() {
    // Encoding: 0x85C06000
    // Test PRFD_I.P.BI_S field Rn = 0 (Min)
    // Fields: Pg=0, prfop=0, imm6=0, Rn=0
    let encoding: u32 = 0x85C06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfd_i_p_bi_s_field_rn_1_poweroftwo_6000_85c06020() {
    // Encoding: 0x85C06020
    // Test PRFD_I.P.BI_S field Rn = 1 (PowerOfTwo)
    // Fields: prfop=0, imm6=0, Pg=0, Rn=1
    let encoding: u32 = 0x85C06020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfd_i_p_bi_s_field_rn_30_poweroftwominusone_6000_85c063c0() {
    // Encoding: 0x85C063C0
    // Test PRFD_I.P.BI_S field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Rn=30, imm6=0, prfop=0
    let encoding: u32 = 0x85C063C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfd_i_p_bi_s_field_rn_31_max_6000_85c063e0() {
    // Encoding: 0x85C063E0
    // Test PRFD_I.P.BI_S field Rn = 31 (Max)
    // Fields: imm6=0, Pg=0, prfop=0, Rn=31
    let encoding: u32 = 0x85C063E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfd_i_p_bi_s_field_prfop_0_min_6000_85c06000() {
    // Encoding: 0x85C06000
    // Test PRFD_I.P.BI_S field prfop = 0 (Min)
    // Fields: imm6=0, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0x85C06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfd_i_p_bi_s_field_prfop_1_poweroftwo_6000_85c06001() {
    // Encoding: 0x85C06001
    // Test PRFD_I.P.BI_S field prfop = 1 (PowerOfTwo)
    // Fields: prfop=1, imm6=0, Rn=0, Pg=0
    let encoding: u32 = 0x85C06001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfd_i_p_bi_s_field_prfop_7_poweroftwominusone_6000_85c06007() {
    // Encoding: 0x85C06007
    // Test PRFD_I.P.BI_S field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Pg=0, imm6=0, Rn=0, prfop=7
    let encoding: u32 = 0x85C06007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfd_i_p_bi_s_field_prfop_15_max_6000_85c0600f() {
    // Encoding: 0x85C0600F
    // Test PRFD_I.P.BI_S field prfop = 15 (Max)
    // Fields: Pg=0, prfop=15, imm6=0, Rn=0
    let encoding: u32 = 0x85C0600F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=0 (immediate value 0)
#[test]
fn test_prfd_i_p_bi_s_combo_0_6000_85c06000() {
    // Encoding: 0x85C06000
    // Test PRFD_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, imm6=0, Pg=0, prfop=0
    let encoding: u32 = 0x85C06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=1 (immediate value 1)
#[test]
fn test_prfd_i_p_bi_s_combo_1_6000_85c16000() {
    // Encoding: 0x85C16000
    // Test PRFD_I.P.BI_S field combination: imm6=1, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, Pg=0, imm6=1
    let encoding: u32 = 0x85C16000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=3 (2^2 - 1 = 3)
#[test]
fn test_prfd_i_p_bi_s_combo_2_6000_85c36000() {
    // Encoding: 0x85C36000
    // Test PRFD_I.P.BI_S field combination: imm6=3, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, Pg=0, imm6=3
    let encoding: u32 = 0x85C36000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=4 (power of 2 (2^2 = 4))
#[test]
fn test_prfd_i_p_bi_s_combo_3_6000_85c46000() {
    // Encoding: 0x85C46000
    // Test PRFD_I.P.BI_S field combination: imm6=4, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rn=0, prfop=0, imm6=4
    let encoding: u32 = 0x85C46000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=7 (2^3 - 1 = 7)
#[test]
fn test_prfd_i_p_bi_s_combo_4_6000_85c76000() {
    // Encoding: 0x85C76000
    // Test PRFD_I.P.BI_S field combination: imm6=7, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, imm6=7, Pg=0, Rn=0
    let encoding: u32 = 0x85C76000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=8 (power of 2 (2^3 = 8))
#[test]
fn test_prfd_i_p_bi_s_combo_5_6000_85c86000() {
    // Encoding: 0x85C86000
    // Test PRFD_I.P.BI_S field combination: imm6=8, Pg=0, Rn=0, prfop=0
    // Fields: imm6=8, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0x85C86000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=15 (2^4 - 1 = 15)
#[test]
fn test_prfd_i_p_bi_s_combo_6_6000_85cf6000() {
    // Encoding: 0x85CF6000
    // Test PRFD_I.P.BI_S field combination: imm6=15, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, imm6=15, Pg=0, Rn=0
    let encoding: u32 = 0x85CF6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=16 (power of 2 (2^4 = 16))
#[test]
fn test_prfd_i_p_bi_s_combo_7_6000_85d06000() {
    // Encoding: 0x85D06000
    // Test PRFD_I.P.BI_S field combination: imm6=16, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, imm6=16, Pg=0
    let encoding: u32 = 0x85D06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=31 (immediate midpoint (31))
#[test]
fn test_prfd_i_p_bi_s_combo_8_6000_85df6000() {
    // Encoding: 0x85DF6000
    // Test PRFD_I.P.BI_S field combination: imm6=31, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, imm6=31, Rn=0, Pg=0
    let encoding: u32 = 0x85DF6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=32 (power of 2 (2^5 = 32))
#[test]
fn test_prfd_i_p_bi_s_combo_9_6000_85e06000() {
    // Encoding: 0x85E06000
    // Test PRFD_I.P.BI_S field combination: imm6=32, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, prfop=0, Rn=0, imm6=32
    let encoding: u32 = 0x85E06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm6=63 (maximum immediate (63))
#[test]
fn test_prfd_i_p_bi_s_combo_10_6000_85ff6000() {
    // Encoding: 0x85FF6000
    // Test PRFD_I.P.BI_S field combination: imm6=63, Pg=0, Rn=0, prfop=0
    // Fields: imm6=63, Rn=0, Pg=0, prfop=0
    let encoding: u32 = 0x85FF6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfd_i_p_bi_s_combo_11_6000_85c06000() {
    // Encoding: 0x85C06000
    // Test PRFD_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, imm6=0, prfop=0
    let encoding: u32 = 0x85C06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfd_i_p_bi_s_combo_12_6000_85c06400() {
    // Encoding: 0x85C06400
    // Test PRFD_I.P.BI_S field combination: imm6=0, Pg=1, Rn=0, prfop=0
    // Fields: prfop=0, Pg=1, Rn=0, imm6=0
    let encoding: u32 = 0x85C06400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfd_i_p_bi_s_combo_13_6000_85c06000() {
    // Encoding: 0x85C06000
    // Test PRFD_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, prfop=0, Rn=0, imm6=0
    let encoding: u32 = 0x85C06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfd_i_p_bi_s_combo_14_6000_85c06020() {
    // Encoding: 0x85C06020
    // Test PRFD_I.P.BI_S field combination: imm6=0, Pg=0, Rn=1, prfop=0
    // Fields: prfop=0, Rn=1, Pg=0, imm6=0
    let encoding: u32 = 0x85C06020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfd_i_p_bi_s_combo_15_6000_85c063c0() {
    // Encoding: 0x85C063C0
    // Test PRFD_I.P.BI_S field combination: imm6=0, Pg=0, Rn=30, prfop=0
    // Fields: imm6=0, Rn=30, Pg=0, prfop=0
    let encoding: u32 = 0x85C063C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfd_i_p_bi_s_combo_16_6000_85c063e0() {
    // Encoding: 0x85C063E0
    // Test PRFD_I.P.BI_S field combination: imm6=0, Pg=0, Rn=31, prfop=0
    // Fields: prfop=0, Rn=31, Pg=0, imm6=0
    let encoding: u32 = 0x85C063E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfd_i_p_bi_s_combo_17_6000_85c06000() {
    // Encoding: 0x85C06000
    // Test PRFD_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, prfop=0, imm6=0, Pg=0
    let encoding: u32 = 0x85C06000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfd_i_p_bi_s_combo_18_6000_85c06001() {
    // Encoding: 0x85C06001
    // Test PRFD_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=1
    // Fields: Pg=0, imm6=0, Rn=0, prfop=1
    let encoding: u32 = 0x85C06001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfd_i_p_bi_s_combo_19_6000_85c06007() {
    // Encoding: 0x85C06007
    // Test PRFD_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=7
    // Fields: Pg=0, prfop=7, Rn=0, imm6=0
    let encoding: u32 = 0x85C06007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfd_i_p_bi_s_combo_20_6000_85c0600f() {
    // Encoding: 0x85C0600F
    // Test PRFD_I.P.BI_S field combination: imm6=0, Pg=0, Rn=0, prfop=15
    // Fields: imm6=0, Pg=0, Rn=0, prfop=15
    let encoding: u32 = 0x85C0600F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfd_i_p_bi_s_combo_21_6000_85c06420() {
    // Encoding: 0x85C06420
    // Test PRFD_I.P.BI_S field combination: imm6=0, Pg=1, Rn=1, prfop=0
    // Fields: Rn=1, prfop=0, imm6=0, Pg=1
    let encoding: u32 = 0x85C06420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfd_i_p_bi_s_combo_22_6000_85c07fe0() {
    // Encoding: 0x85C07FE0
    // Test PRFD_I.P.BI_S field combination: imm6=0, Pg=31, Rn=31, prfop=0
    // Fields: prfop=0, Pg=31, imm6=0, Rn=31
    let encoding: u32 = 0x85C07FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfd_i_p_bi_s_special_rn_31_stack_pointer_sp_may_require_alignment_24576_85c163e0() {
    // Encoding: 0x85C163E0
    // Test PRFD_I.P.BI_S special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Pg=0, prfop=0, Rn=31, imm6=1
    let encoding: u32 = 0x85C163E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BI_S
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfd_i_p_bi_s_sp_rn_85c063e0() {
    // Test PRFD_I.P.BI_S with Rn = SP (31)
    // Encoding: 0x85C063E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x85C063E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PRFD_I.P.BZ_S.x32.scaled Tests
// ============================================================================

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_xs_0_min_6000_84206000() {
    // Encoding: 0x84206000
    // Test PRFD_I.P.BZ_S.x32.scaled field xs = 0 (Min)
    // Fields: xs=0, Pg=0, prfop=0, Rn=0, Zm=0
    let encoding: u32 = 0x84206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_xs_1_max_6000_84606000() {
    // Encoding: 0x84606000
    // Test PRFD_I.P.BZ_S.x32.scaled field xs = 1 (Max)
    // Fields: Rn=0, prfop=0, Pg=0, xs=1, Zm=0
    let encoding: u32 = 0x84606000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_zm_0_min_6000_84206000() {
    // Encoding: 0x84206000
    // Test PRFD_I.P.BZ_S.x32.scaled field Zm = 0 (Min)
    // Fields: xs=0, Pg=0, Rn=0, prfop=0, Zm=0
    let encoding: u32 = 0x84206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_zm_1_poweroftwo_6000_84216000() {
    // Encoding: 0x84216000
    // Test PRFD_I.P.BZ_S.x32.scaled field Zm = 1 (PowerOfTwo)
    // Fields: prfop=0, Zm=1, xs=0, Pg=0, Rn=0
    let encoding: u32 = 0x84216000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_zm_30_poweroftwominusone_6000_843e6000() {
    // Encoding: 0x843E6000
    // Test PRFD_I.P.BZ_S.x32.scaled field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Pg=0, Zm=30, Rn=0, xs=0
    let encoding: u32 = 0x843E6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_zm_31_max_6000_843f6000() {
    // Encoding: 0x843F6000
    // Test PRFD_I.P.BZ_S.x32.scaled field Zm = 31 (Max)
    // Fields: Zm=31, xs=0, Rn=0, prfop=0, Pg=0
    let encoding: u32 = 0x843F6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_pg_0_min_6000_84206000() {
    // Encoding: 0x84206000
    // Test PRFD_I.P.BZ_S.x32.scaled field Pg = 0 (Min)
    // Fields: Pg=0, Rn=0, prfop=0, xs=0, Zm=0
    let encoding: u32 = 0x84206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_pg_1_poweroftwo_6000_84206400() {
    // Encoding: 0x84206400
    // Test PRFD_I.P.BZ_S.x32.scaled field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, xs=0, prfop=0, Rn=0, Zm=0
    let encoding: u32 = 0x84206400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_rn_0_min_6000_84206000() {
    // Encoding: 0x84206000
    // Test PRFD_I.P.BZ_S.x32.scaled field Rn = 0 (Min)
    // Fields: prfop=0, Pg=0, Zm=0, Rn=0, xs=0
    let encoding: u32 = 0x84206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_rn_1_poweroftwo_6000_84206020() {
    // Encoding: 0x84206020
    // Test PRFD_I.P.BZ_S.x32.scaled field Rn = 1 (PowerOfTwo)
    // Fields: Pg=0, xs=0, prfop=0, Zm=0, Rn=1
    let encoding: u32 = 0x84206020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_rn_30_poweroftwominusone_6000_842063c0() {
    // Encoding: 0x842063C0
    // Test PRFD_I.P.BZ_S.x32.scaled field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: xs=0, prfop=0, Rn=30, Zm=0, Pg=0
    let encoding: u32 = 0x842063C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_rn_31_max_6000_842063e0() {
    // Encoding: 0x842063E0
    // Test PRFD_I.P.BZ_S.x32.scaled field Rn = 31 (Max)
    // Fields: Rn=31, prfop=0, xs=0, Zm=0, Pg=0
    let encoding: u32 = 0x842063E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_prfop_0_min_6000_84206000() {
    // Encoding: 0x84206000
    // Test PRFD_I.P.BZ_S.x32.scaled field prfop = 0 (Min)
    // Fields: xs=0, Pg=0, prfop=0, Zm=0, Rn=0
    let encoding: u32 = 0x84206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_prfop_1_poweroftwo_6000_84206001() {
    // Encoding: 0x84206001
    // Test PRFD_I.P.BZ_S.x32.scaled field prfop = 1 (PowerOfTwo)
    // Fields: Zm=0, Pg=0, xs=0, Rn=0, prfop=1
    let encoding: u32 = 0x84206001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_prfop_7_poweroftwominusone_6000_84206007() {
    // Encoding: 0x84206007
    // Test PRFD_I.P.BZ_S.x32.scaled field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Pg=0, xs=0, Rn=0, prfop=7, Zm=0
    let encoding: u32 = 0x84206007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_field_prfop_15_max_6000_8420600f() {
    // Encoding: 0x8420600F
    // Test PRFD_I.P.BZ_S.x32.scaled field prfop = 15 (Max)
    // Fields: prfop=15, xs=0, Zm=0, Pg=0, Rn=0
    let encoding: u32 = 0x8420600F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=0 (minimum value)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_0_6000_84206000() {
    // Encoding: 0x84206000
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, Zm=0, xs=0, prfop=0
    let encoding: u32 = 0x84206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=1 (maximum value (1))
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_1_6000_84606000() {
    // Encoding: 0x84606000
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=1, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Zm=0, Rn=0, xs=1, Pg=0
    let encoding: u32 = 0x84606000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_2_6000_84206000() {
    // Encoding: 0x84206000
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rn=0, Zm=0, xs=0, prfop=0
    let encoding: u32 = 0x84206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_3_6000_84216000() {
    // Encoding: 0x84216000
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=1, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Zm=1, Rn=0, prfop=0, xs=0
    let encoding: u32 = 0x84216000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_4_6000_843e6000() {
    // Encoding: 0x843E6000
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=30, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Pg=0, xs=0, Zm=30, Rn=0
    let encoding: u32 = 0x843E6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_5_6000_843f6000() {
    // Encoding: 0x843F6000
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=31, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Zm=31, Pg=0, xs=0, Rn=0
    let encoding: u32 = 0x843F6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_6_6000_84206000() {
    // Encoding: 0x84206000
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, xs=0, Pg=0, Zm=0, Rn=0
    let encoding: u32 = 0x84206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_7_6000_84206400() {
    // Encoding: 0x84206400
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=0, prfop=0
    // Fields: Rn=0, Pg=1, xs=0, Zm=0, prfop=0
    let encoding: u32 = 0x84206400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_8_6000_84206000() {
    // Encoding: 0x84206000
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, prfop=0, xs=0, Pg=0, Zm=0
    let encoding: u32 = 0x84206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_9_6000_84206020() {
    // Encoding: 0x84206020
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=1, prfop=0
    // Fields: Zm=0, xs=0, Pg=0, Rn=1, prfop=0
    let encoding: u32 = 0x84206020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_10_6000_842063c0() {
    // Encoding: 0x842063C0
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=30, prfop=0
    // Fields: xs=0, Rn=30, prfop=0, Pg=0, Zm=0
    let encoding: u32 = 0x842063C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_11_6000_842063e0() {
    // Encoding: 0x842063E0
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=31, prfop=0
    // Fields: Zm=0, Rn=31, prfop=0, xs=0, Pg=0
    let encoding: u32 = 0x842063E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_12_6000_84206000() {
    // Encoding: 0x84206000
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, xs=0, Rn=0, Zm=0, Pg=0
    let encoding: u32 = 0x84206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_13_6000_84206001() {
    // Encoding: 0x84206001
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=1
    // Fields: xs=0, Rn=0, prfop=1, Zm=0, Pg=0
    let encoding: u32 = 0x84206001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_14_6000_84206007() {
    // Encoding: 0x84206007
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=7
    // Fields: Zm=0, Rn=0, Pg=0, xs=0, prfop=7
    let encoding: u32 = 0x84206007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_15_6000_8420600f() {
    // Encoding: 0x8420600F
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=15
    // Fields: Rn=0, Zm=0, prfop=15, Pg=0, xs=0
    let encoding: u32 = 0x8420600F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_16_6000_84206420() {
    // Encoding: 0x84206420
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=1, prfop=0
    // Fields: prfop=0, Pg=1, xs=0, Zm=0, Rn=1
    let encoding: u32 = 0x84206420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_combo_17_6000_84207fe0() {
    // Encoding: 0x84207FE0
    // Test PRFD_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=31, Rn=31, prfop=0
    // Fields: xs=0, Zm=0, Pg=31, prfop=0, Rn=31
    let encoding: u32 = 0x84207FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_special_rn_31_stack_pointer_sp_may_require_alignment_24576_842063e0()
 {
    // Encoding: 0x842063E0
    // Test PRFD_I.P.BZ_S.x32.scaled special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Pg=0, Rn=31, xs=0, prfop=0, Zm=0
    let encoding: u32 = 0x842063E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_xs_0_min_6000_c4206000() {
    // Encoding: 0xC4206000
    // Test PRFD_I.P.BZ_D.x32.scaled field xs = 0 (Min)
    // Fields: prfop=0, Pg=0, Zm=0, xs=0, Rn=0
    let encoding: u32 = 0xC4206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_xs_1_max_6000_c4606000() {
    // Encoding: 0xC4606000
    // Test PRFD_I.P.BZ_D.x32.scaled field xs = 1 (Max)
    // Fields: Rn=0, Zm=0, prfop=0, Pg=0, xs=1
    let encoding: u32 = 0xC4606000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_zm_0_min_6000_c4206000() {
    // Encoding: 0xC4206000
    // Test PRFD_I.P.BZ_D.x32.scaled field Zm = 0 (Min)
    // Fields: xs=0, Pg=0, Zm=0, Rn=0, prfop=0
    let encoding: u32 = 0xC4206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_zm_1_poweroftwo_6000_c4216000() {
    // Encoding: 0xC4216000
    // Test PRFD_I.P.BZ_D.x32.scaled field Zm = 1 (PowerOfTwo)
    // Fields: Rn=0, Zm=1, prfop=0, Pg=0, xs=0
    let encoding: u32 = 0xC4216000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_zm_30_poweroftwominusone_6000_c43e6000() {
    // Encoding: 0xC43E6000
    // Test PRFD_I.P.BZ_D.x32.scaled field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: xs=0, Zm=30, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0xC43E6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_zm_31_max_6000_c43f6000() {
    // Encoding: 0xC43F6000
    // Test PRFD_I.P.BZ_D.x32.scaled field Zm = 31 (Max)
    // Fields: Rn=0, prfop=0, xs=0, Pg=0, Zm=31
    let encoding: u32 = 0xC43F6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_pg_0_min_6000_c4206000() {
    // Encoding: 0xC4206000
    // Test PRFD_I.P.BZ_D.x32.scaled field Pg = 0 (Min)
    // Fields: prfop=0, xs=0, Pg=0, Rn=0, Zm=0
    let encoding: u32 = 0xC4206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_pg_1_poweroftwo_6000_c4206400() {
    // Encoding: 0xC4206400
    // Test PRFD_I.P.BZ_D.x32.scaled field Pg = 1 (PowerOfTwo)
    // Fields: Zm=0, prfop=0, Pg=1, xs=0, Rn=0
    let encoding: u32 = 0xC4206400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_rn_0_min_6000_c4206000() {
    // Encoding: 0xC4206000
    // Test PRFD_I.P.BZ_D.x32.scaled field Rn = 0 (Min)
    // Fields: xs=0, Pg=0, Zm=0, Rn=0, prfop=0
    let encoding: u32 = 0xC4206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_rn_1_poweroftwo_6000_c4206020() {
    // Encoding: 0xC4206020
    // Test PRFD_I.P.BZ_D.x32.scaled field Rn = 1 (PowerOfTwo)
    // Fields: xs=0, Pg=0, Rn=1, Zm=0, prfop=0
    let encoding: u32 = 0xC4206020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_rn_30_poweroftwominusone_6000_c42063c0() {
    // Encoding: 0xC42063C0
    // Test PRFD_I.P.BZ_D.x32.scaled field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Rn=30, Zm=0, xs=0, Pg=0
    let encoding: u32 = 0xC42063C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_rn_31_max_6000_c42063e0() {
    // Encoding: 0xC42063E0
    // Test PRFD_I.P.BZ_D.x32.scaled field Rn = 31 (Max)
    // Fields: Pg=0, xs=0, Zm=0, prfop=0, Rn=31
    let encoding: u32 = 0xC42063E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_prfop_0_min_6000_c4206000() {
    // Encoding: 0xC4206000
    // Test PRFD_I.P.BZ_D.x32.scaled field prfop = 0 (Min)
    // Fields: Rn=0, xs=0, prfop=0, Zm=0, Pg=0
    let encoding: u32 = 0xC4206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_prfop_1_poweroftwo_6000_c4206001() {
    // Encoding: 0xC4206001
    // Test PRFD_I.P.BZ_D.x32.scaled field prfop = 1 (PowerOfTwo)
    // Fields: prfop=1, xs=0, Pg=0, Zm=0, Rn=0
    let encoding: u32 = 0xC4206001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_prfop_7_poweroftwominusone_6000_c4206007() {
    // Encoding: 0xC4206007
    // Test PRFD_I.P.BZ_D.x32.scaled field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Zm=0, prfop=7, Rn=0, Pg=0, xs=0
    let encoding: u32 = 0xC4206007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_field_prfop_15_max_6000_c420600f() {
    // Encoding: 0xC420600F
    // Test PRFD_I.P.BZ_D.x32.scaled field prfop = 15 (Max)
    // Fields: xs=0, Pg=0, prfop=15, Rn=0, Zm=0
    let encoding: u32 = 0xC420600F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=0 (minimum value)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_0_6000_c4206000() {
    // Encoding: 0xC4206000
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, xs=0, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0xC4206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=1 (maximum value (1))
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_1_6000_c4606000() {
    // Encoding: 0xC4606000
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=1, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Rn=0, prfop=0, Pg=0, xs=1
    let encoding: u32 = 0xC4606000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_2_6000_c4206000() {
    // Encoding: 0xC4206000
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, xs=0, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0xC4206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_3_6000_c4216000() {
    // Encoding: 0xC4216000
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=1, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rn=0, Zm=1, prfop=0, xs=0
    let encoding: u32 = 0xC4216000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_4_6000_c43e6000() {
    // Encoding: 0xC43E6000
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=30, Pg=0, Rn=0, prfop=0
    // Fields: xs=0, prfop=0, Rn=0, Pg=0, Zm=30
    let encoding: u32 = 0xC43E6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_5_6000_c43f6000() {
    // Encoding: 0xC43F6000
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=31, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Zm=31, Rn=0, xs=0, prfop=0
    let encoding: u32 = 0xC43F6000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_6_6000_c4206000() {
    // Encoding: 0xC4206000
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, Zm=0, Pg=0, xs=0
    let encoding: u32 = 0xC4206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_7_6000_c4206400() {
    // Encoding: 0xC4206400
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=0, prfop=0
    // Fields: prfop=0, xs=0, Rn=0, Zm=0, Pg=1
    let encoding: u32 = 0xC4206400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_8_6000_c4206000() {
    // Encoding: 0xC4206000
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Rn=0, xs=0, Pg=0, prfop=0
    let encoding: u32 = 0xC4206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_9_6000_c4206020() {
    // Encoding: 0xC4206020
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=1, prfop=0
    // Fields: Zm=0, Pg=0, prfop=0, xs=0, Rn=1
    let encoding: u32 = 0xC4206020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_10_6000_c42063c0() {
    // Encoding: 0xC42063C0
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=30, prfop=0
    // Fields: Rn=30, prfop=0, xs=0, Zm=0, Pg=0
    let encoding: u32 = 0xC42063C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_11_6000_c42063e0() {
    // Encoding: 0xC42063E0
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=31, prfop=0
    // Fields: Pg=0, xs=0, prfop=0, Zm=0, Rn=31
    let encoding: u32 = 0xC42063E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_12_6000_c4206000() {
    // Encoding: 0xC4206000
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Zm=0, Pg=0, Rn=0, xs=0
    let encoding: u32 = 0xC4206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_13_6000_c4206001() {
    // Encoding: 0xC4206001
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=1
    // Fields: Zm=0, Rn=0, Pg=0, xs=0, prfop=1
    let encoding: u32 = 0xC4206001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_14_6000_c4206007() {
    // Encoding: 0xC4206007
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=7
    // Fields: prfop=7, xs=0, Pg=0, Zm=0, Rn=0
    let encoding: u32 = 0xC4206007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_15_6000_c420600f() {
    // Encoding: 0xC420600F
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=15
    // Fields: xs=0, Zm=0, Pg=0, Rn=0, prfop=15
    let encoding: u32 = 0xC420600F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_16_6000_c4206420() {
    // Encoding: 0xC4206420
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=1, prfop=0
    // Fields: Rn=1, Pg=1, prfop=0, xs=0, Zm=0
    let encoding: u32 = 0xC4206420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_combo_17_6000_c4207fe0() {
    // Encoding: 0xC4207FE0
    // Test PRFD_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=31, Rn=31, prfop=0
    // Fields: Zm=0, Rn=31, prfop=0, Pg=31, xs=0
    let encoding: u32 = 0xC4207FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_special_rn_31_stack_pointer_sp_may_require_alignment_24576_c42063e0()
 {
    // Encoding: 0xC42063E0
    // Test PRFD_I.P.BZ_D.x32.scaled special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Zm=0, xs=0, prfop=0, Rn=31, Pg=0
    let encoding: u32 = 0xC42063E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_zm_0_min_e000_c460e000() {
    // Encoding: 0xC460E000
    // Test PRFD_I.P.BZ_D.64.scaled field Zm = 0 (Min)
    // Fields: Zm=0, prfop=0, Rn=0, Pg=0
    let encoding: u32 = 0xC460E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_zm_1_poweroftwo_e000_c461e000() {
    // Encoding: 0xC461E000
    // Test PRFD_I.P.BZ_D.64.scaled field Zm = 1 (PowerOfTwo)
    // Fields: Pg=0, prfop=0, Zm=1, Rn=0
    let encoding: u32 = 0xC461E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_zm_30_poweroftwominusone_e000_c47ee000() {
    // Encoding: 0xC47EE000
    // Test PRFD_I.P.BZ_D.64.scaled field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Pg=0, Zm=30, prfop=0
    let encoding: u32 = 0xC47EE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_zm_31_max_e000_c47fe000() {
    // Encoding: 0xC47FE000
    // Test PRFD_I.P.BZ_D.64.scaled field Zm = 31 (Max)
    // Fields: Rn=0, Zm=31, Pg=0, prfop=0
    let encoding: u32 = 0xC47FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_pg_0_min_e000_c460e000() {
    // Encoding: 0xC460E000
    // Test PRFD_I.P.BZ_D.64.scaled field Pg = 0 (Min)
    // Fields: Pg=0, Zm=0, Rn=0, prfop=0
    let encoding: u32 = 0xC460E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_pg_1_poweroftwo_e000_c460e400() {
    // Encoding: 0xC460E400
    // Test PRFD_I.P.BZ_D.64.scaled field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, Rn=0, Zm=0, prfop=0
    let encoding: u32 = 0xC460E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_rn_0_min_e000_c460e000() {
    // Encoding: 0xC460E000
    // Test PRFD_I.P.BZ_D.64.scaled field Rn = 0 (Min)
    // Fields: Zm=0, Rn=0, Pg=0, prfop=0
    let encoding: u32 = 0xC460E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_rn_1_poweroftwo_e000_c460e020() {
    // Encoding: 0xC460E020
    // Test PRFD_I.P.BZ_D.64.scaled field Rn = 1 (PowerOfTwo)
    // Fields: prfop=0, Pg=0, Rn=1, Zm=0
    let encoding: u32 = 0xC460E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_rn_30_poweroftwominusone_e000_c460e3c0() {
    // Encoding: 0xC460E3C0
    // Test PRFD_I.P.BZ_D.64.scaled field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Zm=0, Pg=0, prfop=0, Rn=30
    let encoding: u32 = 0xC460E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_rn_31_max_e000_c460e3e0() {
    // Encoding: 0xC460E3E0
    // Test PRFD_I.P.BZ_D.64.scaled field Rn = 31 (Max)
    // Fields: Zm=0, prfop=0, Pg=0, Rn=31
    let encoding: u32 = 0xC460E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_prfop_0_min_e000_c460e000() {
    // Encoding: 0xC460E000
    // Test PRFD_I.P.BZ_D.64.scaled field prfop = 0 (Min)
    // Fields: Rn=0, prfop=0, Pg=0, Zm=0
    let encoding: u32 = 0xC460E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_prfop_1_poweroftwo_e000_c460e001() {
    // Encoding: 0xC460E001
    // Test PRFD_I.P.BZ_D.64.scaled field prfop = 1 (PowerOfTwo)
    // Fields: prfop=1, Pg=0, Rn=0, Zm=0
    let encoding: u32 = 0xC460E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_prfop_7_poweroftwominusone_e000_c460e007() {
    // Encoding: 0xC460E007
    // Test PRFD_I.P.BZ_D.64.scaled field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Zm=0, Pg=0, prfop=7, Rn=0
    let encoding: u32 = 0xC460E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_field_prfop_15_max_e000_c460e00f() {
    // Encoding: 0xC460E00F
    // Test PRFD_I.P.BZ_D.64.scaled field prfop = 15 (Max)
    // Fields: Pg=0, Rn=0, prfop=15, Zm=0
    let encoding: u32 = 0xC460E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_0_e000_c460e000() {
    // Encoding: 0xC460E000
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Zm=0, Rn=0, prfop=0
    let encoding: u32 = 0xC460E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_1_e000_c461e000() {
    // Encoding: 0xC461E000
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=1, Pg=0, Rn=0, prfop=0
    // Fields: Zm=1, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0xC461E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_2_e000_c47ee000() {
    // Encoding: 0xC47EE000
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=30, Pg=0, Rn=0, prfop=0
    // Fields: Zm=30, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0xC47EE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_3_e000_c47fe000() {
    // Encoding: 0xC47FE000
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=31, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Zm=31, Rn=0, prfop=0
    let encoding: u32 = 0xC47FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_4_e000_c460e000() {
    // Encoding: 0xC460E000
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0xC460E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_5_e000_c460e400() {
    // Encoding: 0xC460E400
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=1, Rn=0, prfop=0
    // Fields: Zm=0, Rn=0, prfop=0, Pg=1
    let encoding: u32 = 0xC460E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_6_e000_c460e000() {
    // Encoding: 0xC460E000
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0xC460E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_7_e000_c460e020() {
    // Encoding: 0xC460E020
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=1, prfop=0
    // Fields: prfop=0, Pg=0, Rn=1, Zm=0
    let encoding: u32 = 0xC460E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_8_e000_c460e3c0() {
    // Encoding: 0xC460E3C0
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=30, prfop=0
    // Fields: Zm=0, prfop=0, Rn=30, Pg=0
    let encoding: u32 = 0xC460E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_9_e000_c460e3e0() {
    // Encoding: 0xC460E3E0
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=31, prfop=0
    // Fields: Rn=31, prfop=0, Zm=0, Pg=0
    let encoding: u32 = 0xC460E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_10_e000_c460e000() {
    // Encoding: 0xC460E000
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Rn=0, Pg=0, prfop=0
    let encoding: u32 = 0xC460E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_11_e000_c460e001() {
    // Encoding: 0xC460E001
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=1
    // Fields: Pg=0, Rn=0, prfop=1, Zm=0
    let encoding: u32 = 0xC460E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_12_e000_c460e007() {
    // Encoding: 0xC460E007
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=7
    // Fields: Rn=0, prfop=7, Zm=0, Pg=0
    let encoding: u32 = 0xC460E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_13_e000_c460e00f() {
    // Encoding: 0xC460E00F
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=15
    // Fields: Pg=0, Rn=0, Zm=0, prfop=15
    let encoding: u32 = 0xC460E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_14_e000_c460e420() {
    // Encoding: 0xC460E420
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=1, Rn=1, prfop=0
    // Fields: Pg=1, prfop=0, Zm=0, Rn=1
    let encoding: u32 = 0xC460E420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfd_i_p_bz_d_64_scaled_combo_15_e000_c460ffe0() {
    // Encoding: 0xC460FFE0
    // Test PRFD_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=31, Rn=31, prfop=0
    // Fields: Rn=31, prfop=0, Pg=31, Zm=0
    let encoding: u32 = 0xC460FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfd_i_p_bz_d_64_scaled_special_rn_31_stack_pointer_sp_may_require_alignment_57344_c460e3e0()
 {
    // Encoding: 0xC460E3E0
    // Test PRFD_I.P.BZ_D.64.scaled special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Pg=0, prfop=0, Zm=0, Rn=31
    let encoding: u32 = 0xC460E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFD_I.P.BZ_S.x32.scaled
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfd_i_p_bz_s_x32_scaled_sp_rn_842063e0() {
    // Test PRFD_I.P.BZ_S.x32.scaled with Rn = SP (31)
    // Encoding: 0x842063E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x842063E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PRFD_I.P.BZ_D.x32.scaled
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfd_i_p_bz_d_x32_scaled_sp_rn_c42063e0() {
    // Test PRFD_I.P.BZ_D.x32.scaled with Rn = SP (31)
    // Encoding: 0xC42063E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xC42063E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PRFD_I.P.BZ_D.64.scaled
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfd_i_p_bz_d_64_scaled_sp_rn_c460e3e0() {
    // Test PRFD_I.P.BZ_D.64.scaled with Rn = SP (31)
    // Encoding: 0xC460E3E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xC460E3E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PRFB_I.P.AI_S Tests
// ============================================================================

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_prfb_i_p_ai_s_field_imm5_0_zero_e000_8400e000() {
    // Encoding: 0x8400E000
    // Test PRFB_I.P.AI_S field imm5 = 0 (Zero)
    // Fields: prfop=0, imm5=0, Pg=0, Zn=0
    let encoding: u32 = 0x8400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_prfb_i_p_ai_s_field_imm5_1_poweroftwo_e000_8401e000() {
    // Encoding: 0x8401E000
    // Test PRFB_I.P.AI_S field imm5 = 1 (PowerOfTwo)
    // Fields: imm5=1, Pg=0, prfop=0, Zn=0
    let encoding: u32 = 0x8401E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_prfb_i_p_ai_s_field_imm5_3_poweroftwominusone_e000_8403e000() {
    // Encoding: 0x8403E000
    // Test PRFB_I.P.AI_S field imm5 = 3 (PowerOfTwoMinusOne)
    // Fields: Pg=0, imm5=3, Zn=0, prfop=0
    let encoding: u32 = 0x8403E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_prfb_i_p_ai_s_field_imm5_4_poweroftwo_e000_8404e000() {
    // Encoding: 0x8404E000
    // Test PRFB_I.P.AI_S field imm5 = 4 (PowerOfTwo)
    // Fields: imm5=4, Zn=0, prfop=0, Pg=0
    let encoding: u32 = 0x8404E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_prfb_i_p_ai_s_field_imm5_7_poweroftwominusone_e000_8407e000() {
    // Encoding: 0x8407E000
    // Test PRFB_I.P.AI_S field imm5 = 7 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Pg=0, Zn=0, imm5=7
    let encoding: u32 = 0x8407E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_prfb_i_p_ai_s_field_imm5_8_poweroftwo_e000_8408e000() {
    // Encoding: 0x8408E000
    // Test PRFB_I.P.AI_S field imm5 = 8 (PowerOfTwo)
    // Fields: imm5=8, prfop=0, Zn=0, Pg=0
    let encoding: u32 = 0x8408E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_prfb_i_p_ai_s_field_imm5_15_poweroftwominusone_e000_840fe000() {
    // Encoding: 0x840FE000
    // Test PRFB_I.P.AI_S field imm5 = 15 (PowerOfTwoMinusOne)
    // Fields: Pg=0, imm5=15, prfop=0, Zn=0
    let encoding: u32 = 0x840FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_prfb_i_p_ai_s_field_imm5_16_poweroftwo_e000_8410e000() {
    // Encoding: 0x8410E000
    // Test PRFB_I.P.AI_S field imm5 = 16 (PowerOfTwo)
    // Fields: imm5=16, Pg=0, Zn=0, prfop=0
    let encoding: u32 = 0x8410E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_prfb_i_p_ai_s_field_imm5_31_max_e000_841fe000() {
    // Encoding: 0x841FE000
    // Test PRFB_I.P.AI_S field imm5 = 31 (Max)
    // Fields: Zn=0, imm5=31, Pg=0, prfop=0
    let encoding: u32 = 0x841FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfb_i_p_ai_s_field_pg_0_min_e000_8400e000() {
    // Encoding: 0x8400E000
    // Test PRFB_I.P.AI_S field Pg = 0 (Min)
    // Fields: imm5=0, prfop=0, Zn=0, Pg=0
    let encoding: u32 = 0x8400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfb_i_p_ai_s_field_pg_1_poweroftwo_e000_8400e400() {
    // Encoding: 0x8400E400
    // Test PRFB_I.P.AI_S field Pg = 1 (PowerOfTwo)
    // Fields: imm5=0, Pg=1, Zn=0, prfop=0
    let encoding: u32 = 0x8400E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfb_i_p_ai_s_field_zn_0_min_e000_8400e000() {
    // Encoding: 0x8400E000
    // Test PRFB_I.P.AI_S field Zn = 0 (Min)
    // Fields: prfop=0, Pg=0, Zn=0, imm5=0
    let encoding: u32 = 0x8400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfb_i_p_ai_s_field_zn_1_poweroftwo_e000_8400e020() {
    // Encoding: 0x8400E020
    // Test PRFB_I.P.AI_S field Zn = 1 (PowerOfTwo)
    // Fields: Pg=0, Zn=1, imm5=0, prfop=0
    let encoding: u32 = 0x8400E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfb_i_p_ai_s_field_zn_30_poweroftwominusone_e000_8400e3c0() {
    // Encoding: 0x8400E3C0
    // Test PRFB_I.P.AI_S field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: imm5=0, prfop=0, Zn=30, Pg=0
    let encoding: u32 = 0x8400E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfb_i_p_ai_s_field_zn_31_max_e000_8400e3e0() {
    // Encoding: 0x8400E3E0
    // Test PRFB_I.P.AI_S field Zn = 31 (Max)
    // Fields: Zn=31, imm5=0, prfop=0, Pg=0
    let encoding: u32 = 0x8400E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfb_i_p_ai_s_field_prfop_0_min_e000_8400e000() {
    // Encoding: 0x8400E000
    // Test PRFB_I.P.AI_S field prfop = 0 (Min)
    // Fields: imm5=0, Zn=0, Pg=0, prfop=0
    let encoding: u32 = 0x8400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfb_i_p_ai_s_field_prfop_1_poweroftwo_e000_8400e001() {
    // Encoding: 0x8400E001
    // Test PRFB_I.P.AI_S field prfop = 1 (PowerOfTwo)
    // Fields: Pg=0, imm5=0, Zn=0, prfop=1
    let encoding: u32 = 0x8400E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfb_i_p_ai_s_field_prfop_7_poweroftwominusone_e000_8400e007() {
    // Encoding: 0x8400E007
    // Test PRFB_I.P.AI_S field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Zn=0, imm5=0, prfop=7, Pg=0
    let encoding: u32 = 0x8400E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfb_i_p_ai_s_field_prfop_15_max_e000_8400e00f() {
    // Encoding: 0x8400E00F
    // Test PRFB_I.P.AI_S field prfop = 15 (Max)
    // Fields: imm5=0, prfop=15, Pg=0, Zn=0
    let encoding: u32 = 0x8400E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=0 (immediate value 0)
#[test]
fn test_prfb_i_p_ai_s_combo_0_e000_8400e000() {
    // Encoding: 0x8400E000
    // Test PRFB_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, imm5=0, prfop=0, Pg=0
    let encoding: u32 = 0x8400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=1 (immediate value 1)
#[test]
fn test_prfb_i_p_ai_s_combo_1_e000_8401e000() {
    // Encoding: 0x8401E000
    // Test PRFB_I.P.AI_S field combination: imm5=1, Pg=0, Zn=0, prfop=0
    // Fields: imm5=1, Zn=0, Pg=0, prfop=0
    let encoding: u32 = 0x8401E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=3 (2^2 - 1 = 3)
#[test]
fn test_prfb_i_p_ai_s_combo_2_e000_8403e000() {
    // Encoding: 0x8403E000
    // Test PRFB_I.P.AI_S field combination: imm5=3, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, imm5=3, Pg=0, prfop=0
    let encoding: u32 = 0x8403E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=4 (power of 2 (2^2 = 4))
#[test]
fn test_prfb_i_p_ai_s_combo_3_e000_8404e000() {
    // Encoding: 0x8404E000
    // Test PRFB_I.P.AI_S field combination: imm5=4, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, imm5=4, Pg=0, Zn=0
    let encoding: u32 = 0x8404E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=7 (2^3 - 1 = 7)
#[test]
fn test_prfb_i_p_ai_s_combo_4_e000_8407e000() {
    // Encoding: 0x8407E000
    // Test PRFB_I.P.AI_S field combination: imm5=7, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, prfop=0, imm5=7, Zn=0
    let encoding: u32 = 0x8407E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=8 (power of 2 (2^3 = 8))
#[test]
fn test_prfb_i_p_ai_s_combo_5_e000_8408e000() {
    // Encoding: 0x8408E000
    // Test PRFB_I.P.AI_S field combination: imm5=8, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, prfop=0, imm5=8, Zn=0
    let encoding: u32 = 0x8408E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=15 (immediate midpoint (15))
#[test]
fn test_prfb_i_p_ai_s_combo_6_e000_840fe000() {
    // Encoding: 0x840FE000
    // Test PRFB_I.P.AI_S field combination: imm5=15, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, Pg=0, imm5=15, Zn=0
    let encoding: u32 = 0x840FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=16 (power of 2 (2^4 = 16))
#[test]
fn test_prfb_i_p_ai_s_combo_7_e000_8410e000() {
    // Encoding: 0x8410E000
    // Test PRFB_I.P.AI_S field combination: imm5=16, Pg=0, Zn=0, prfop=0
    // Fields: imm5=16, Pg=0, prfop=0, Zn=0
    let encoding: u32 = 0x8410E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=31 (maximum immediate (31))
#[test]
fn test_prfb_i_p_ai_s_combo_8_e000_841fe000() {
    // Encoding: 0x841FE000
    // Test PRFB_I.P.AI_S field combination: imm5=31, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, imm5=31, prfop=0, Pg=0
    let encoding: u32 = 0x841FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfb_i_p_ai_s_combo_9_e000_8400e000() {
    // Encoding: 0x8400E000
    // Test PRFB_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, Pg=0, imm5=0, Zn=0
    let encoding: u32 = 0x8400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfb_i_p_ai_s_combo_10_e000_8400e400() {
    // Encoding: 0x8400E400
    // Test PRFB_I.P.AI_S field combination: imm5=0, Pg=1, Zn=0, prfop=0
    // Fields: Zn=0, prfop=0, imm5=0, Pg=1
    let encoding: u32 = 0x8400E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_prfb_i_p_ai_s_combo_11_e000_8400e000() {
    // Encoding: 0x8400E000
    // Test PRFB_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, prfop=0, Pg=0, imm5=0
    let encoding: u32 = 0x8400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_prfb_i_p_ai_s_combo_12_e000_8400e020() {
    // Encoding: 0x8400E020
    // Test PRFB_I.P.AI_S field combination: imm5=0, Pg=0, Zn=1, prfop=0
    // Fields: imm5=0, prfop=0, Pg=0, Zn=1
    let encoding: u32 = 0x8400E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_prfb_i_p_ai_s_combo_13_e000_8400e3c0() {
    // Encoding: 0x8400E3C0
    // Test PRFB_I.P.AI_S field combination: imm5=0, Pg=0, Zn=30, prfop=0
    // Fields: Pg=0, Zn=30, imm5=0, prfop=0
    let encoding: u32 = 0x8400E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_prfb_i_p_ai_s_combo_14_e000_8400e3e0() {
    // Encoding: 0x8400E3E0
    // Test PRFB_I.P.AI_S field combination: imm5=0, Pg=0, Zn=31, prfop=0
    // Fields: Zn=31, imm5=0, prfop=0, Pg=0
    let encoding: u32 = 0x8400E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfb_i_p_ai_s_combo_15_e000_8400e000() {
    // Encoding: 0x8400E000
    // Test PRFB_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, imm5=0, Pg=0, Zn=0
    let encoding: u32 = 0x8400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfb_i_p_ai_s_combo_16_e000_8400e001() {
    // Encoding: 0x8400E001
    // Test PRFB_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=1
    // Fields: prfop=1, Zn=0, Pg=0, imm5=0
    let encoding: u32 = 0x8400E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfb_i_p_ai_s_combo_17_e000_8400e007() {
    // Encoding: 0x8400E007
    // Test PRFB_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=7
    // Fields: imm5=0, Pg=0, Zn=0, prfop=7
    let encoding: u32 = 0x8400E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_S
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfb_i_p_ai_s_combo_18_e000_8400e00f() {
    // Encoding: 0x8400E00F
    // Test PRFB_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=15
    // Fields: Zn=0, Pg=0, imm5=0, prfop=15
    let encoding: u32 = 0x8400E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_prfb_i_p_ai_d_field_imm5_0_zero_e000_c400e000() {
    // Encoding: 0xC400E000
    // Test PRFB_I.P.AI_D field imm5 = 0 (Zero)
    // Fields: Pg=0, imm5=0, prfop=0, Zn=0
    let encoding: u32 = 0xC400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_prfb_i_p_ai_d_field_imm5_1_poweroftwo_e000_c401e000() {
    // Encoding: 0xC401E000
    // Test PRFB_I.P.AI_D field imm5 = 1 (PowerOfTwo)
    // Fields: Pg=0, Zn=0, imm5=1, prfop=0
    let encoding: u32 = 0xC401E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_prfb_i_p_ai_d_field_imm5_3_poweroftwominusone_e000_c403e000() {
    // Encoding: 0xC403E000
    // Test PRFB_I.P.AI_D field imm5 = 3 (PowerOfTwoMinusOne)
    // Fields: Zn=0, Pg=0, imm5=3, prfop=0
    let encoding: u32 = 0xC403E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_prfb_i_p_ai_d_field_imm5_4_poweroftwo_e000_c404e000() {
    // Encoding: 0xC404E000
    // Test PRFB_I.P.AI_D field imm5 = 4 (PowerOfTwo)
    // Fields: Pg=0, imm5=4, prfop=0, Zn=0
    let encoding: u32 = 0xC404E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_prfb_i_p_ai_d_field_imm5_7_poweroftwominusone_e000_c407e000() {
    // Encoding: 0xC407E000
    // Test PRFB_I.P.AI_D field imm5 = 7 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Zn=0, imm5=7, prfop=0
    let encoding: u32 = 0xC407E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_prfb_i_p_ai_d_field_imm5_8_poweroftwo_e000_c408e000() {
    // Encoding: 0xC408E000
    // Test PRFB_I.P.AI_D field imm5 = 8 (PowerOfTwo)
    // Fields: Pg=0, prfop=0, imm5=8, Zn=0
    let encoding: u32 = 0xC408E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_prfb_i_p_ai_d_field_imm5_15_poweroftwominusone_e000_c40fe000() {
    // Encoding: 0xC40FE000
    // Test PRFB_I.P.AI_D field imm5 = 15 (PowerOfTwoMinusOne)
    // Fields: imm5=15, prfop=0, Zn=0, Pg=0
    let encoding: u32 = 0xC40FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_prfb_i_p_ai_d_field_imm5_16_poweroftwo_e000_c410e000() {
    // Encoding: 0xC410E000
    // Test PRFB_I.P.AI_D field imm5 = 16 (PowerOfTwo)
    // Fields: prfop=0, Zn=0, imm5=16, Pg=0
    let encoding: u32 = 0xC410E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_prfb_i_p_ai_d_field_imm5_31_max_e000_c41fe000() {
    // Encoding: 0xC41FE000
    // Test PRFB_I.P.AI_D field imm5 = 31 (Max)
    // Fields: prfop=0, imm5=31, Pg=0, Zn=0
    let encoding: u32 = 0xC41FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfb_i_p_ai_d_field_pg_0_min_e000_c400e000() {
    // Encoding: 0xC400E000
    // Test PRFB_I.P.AI_D field Pg = 0 (Min)
    // Fields: Pg=0, imm5=0, prfop=0, Zn=0
    let encoding: u32 = 0xC400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfb_i_p_ai_d_field_pg_1_poweroftwo_e000_c400e400() {
    // Encoding: 0xC400E400
    // Test PRFB_I.P.AI_D field Pg = 1 (PowerOfTwo)
    // Fields: Zn=0, imm5=0, prfop=0, Pg=1
    let encoding: u32 = 0xC400E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfb_i_p_ai_d_field_zn_0_min_e000_c400e000() {
    // Encoding: 0xC400E000
    // Test PRFB_I.P.AI_D field Zn = 0 (Min)
    // Fields: prfop=0, imm5=0, Pg=0, Zn=0
    let encoding: u32 = 0xC400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfb_i_p_ai_d_field_zn_1_poweroftwo_e000_c400e020() {
    // Encoding: 0xC400E020
    // Test PRFB_I.P.AI_D field Zn = 1 (PowerOfTwo)
    // Fields: Pg=0, prfop=0, imm5=0, Zn=1
    let encoding: u32 = 0xC400E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfb_i_p_ai_d_field_zn_30_poweroftwominusone_e000_c400e3c0() {
    // Encoding: 0xC400E3C0
    // Test PRFB_I.P.AI_D field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, imm5=0, prfop=0, Zn=30
    let encoding: u32 = 0xC400E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfb_i_p_ai_d_field_zn_31_max_e000_c400e3e0() {
    // Encoding: 0xC400E3E0
    // Test PRFB_I.P.AI_D field Zn = 31 (Max)
    // Fields: Pg=0, prfop=0, imm5=0, Zn=31
    let encoding: u32 = 0xC400E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfb_i_p_ai_d_field_prfop_0_min_e000_c400e000() {
    // Encoding: 0xC400E000
    // Test PRFB_I.P.AI_D field prfop = 0 (Min)
    // Fields: Pg=0, Zn=0, prfop=0, imm5=0
    let encoding: u32 = 0xC400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfb_i_p_ai_d_field_prfop_1_poweroftwo_e000_c400e001() {
    // Encoding: 0xC400E001
    // Test PRFB_I.P.AI_D field prfop = 1 (PowerOfTwo)
    // Fields: Pg=0, prfop=1, imm5=0, Zn=0
    let encoding: u32 = 0xC400E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfb_i_p_ai_d_field_prfop_7_poweroftwominusone_e000_c400e007() {
    // Encoding: 0xC400E007
    // Test PRFB_I.P.AI_D field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: prfop=7, Zn=0, Pg=0, imm5=0
    let encoding: u32 = 0xC400E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfb_i_p_ai_d_field_prfop_15_max_e000_c400e00f() {
    // Encoding: 0xC400E00F
    // Test PRFB_I.P.AI_D field prfop = 15 (Max)
    // Fields: imm5=0, prfop=15, Zn=0, Pg=0
    let encoding: u32 = 0xC400E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=0 (immediate value 0)
#[test]
fn test_prfb_i_p_ai_d_combo_0_e000_c400e000() {
    // Encoding: 0xC400E000
    // Test PRFB_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, prfop=0, Zn=0, imm5=0
    let encoding: u32 = 0xC400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=1 (immediate value 1)
#[test]
fn test_prfb_i_p_ai_d_combo_1_e000_c401e000() {
    // Encoding: 0xC401E000
    // Test PRFB_I.P.AI_D field combination: imm5=1, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, prfop=0, imm5=1, Zn=0
    let encoding: u32 = 0xC401E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=3 (2^2 - 1 = 3)
#[test]
fn test_prfb_i_p_ai_d_combo_2_e000_c403e000() {
    // Encoding: 0xC403E000
    // Test PRFB_I.P.AI_D field combination: imm5=3, Pg=0, Zn=0, prfop=0
    // Fields: imm5=3, Pg=0, prfop=0, Zn=0
    let encoding: u32 = 0xC403E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=4 (power of 2 (2^2 = 4))
#[test]
fn test_prfb_i_p_ai_d_combo_3_e000_c404e000() {
    // Encoding: 0xC404E000
    // Test PRFB_I.P.AI_D field combination: imm5=4, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, imm5=4, Pg=0, prfop=0
    let encoding: u32 = 0xC404E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=7 (2^3 - 1 = 7)
#[test]
fn test_prfb_i_p_ai_d_combo_4_e000_c407e000() {
    // Encoding: 0xC407E000
    // Test PRFB_I.P.AI_D field combination: imm5=7, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, Zn=0, imm5=7, Pg=0
    let encoding: u32 = 0xC407E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=8 (power of 2 (2^3 = 8))
#[test]
fn test_prfb_i_p_ai_d_combo_5_e000_c408e000() {
    // Encoding: 0xC408E000
    // Test PRFB_I.P.AI_D field combination: imm5=8, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, imm5=8, Zn=0, Pg=0
    let encoding: u32 = 0xC408E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=15 (immediate midpoint (15))
#[test]
fn test_prfb_i_p_ai_d_combo_6_e000_c40fe000() {
    // Encoding: 0xC40FE000
    // Test PRFB_I.P.AI_D field combination: imm5=15, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, Zn=0, imm5=15, prfop=0
    let encoding: u32 = 0xC40FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=16 (power of 2 (2^4 = 16))
#[test]
fn test_prfb_i_p_ai_d_combo_7_e000_c410e000() {
    // Encoding: 0xC410E000
    // Test PRFB_I.P.AI_D field combination: imm5=16, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, Pg=0, imm5=16, Zn=0
    let encoding: u32 = 0xC410E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=31 (maximum immediate (31))
#[test]
fn test_prfb_i_p_ai_d_combo_8_e000_c41fe000() {
    // Encoding: 0xC41FE000
    // Test PRFB_I.P.AI_D field combination: imm5=31, Pg=0, Zn=0, prfop=0
    // Fields: imm5=31, Pg=0, prfop=0, Zn=0
    let encoding: u32 = 0xC41FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfb_i_p_ai_d_combo_9_e000_c400e000() {
    // Encoding: 0xC400E000
    // Test PRFB_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, imm5=0, prfop=0, Pg=0
    let encoding: u32 = 0xC400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfb_i_p_ai_d_combo_10_e000_c400e400() {
    // Encoding: 0xC400E400
    // Test PRFB_I.P.AI_D field combination: imm5=0, Pg=1, Zn=0, prfop=0
    // Fields: Pg=1, prfop=0, imm5=0, Zn=0
    let encoding: u32 = 0xC400E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_prfb_i_p_ai_d_combo_11_e000_c400e000() {
    // Encoding: 0xC400E000
    // Test PRFB_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: imm5=0, prfop=0, Pg=0, Zn=0
    let encoding: u32 = 0xC400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_prfb_i_p_ai_d_combo_12_e000_c400e020() {
    // Encoding: 0xC400E020
    // Test PRFB_I.P.AI_D field combination: imm5=0, Pg=0, Zn=1, prfop=0
    // Fields: Zn=1, prfop=0, Pg=0, imm5=0
    let encoding: u32 = 0xC400E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_prfb_i_p_ai_d_combo_13_e000_c400e3c0() {
    // Encoding: 0xC400E3C0
    // Test PRFB_I.P.AI_D field combination: imm5=0, Pg=0, Zn=30, prfop=0
    // Fields: imm5=0, Pg=0, prfop=0, Zn=30
    let encoding: u32 = 0xC400E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_prfb_i_p_ai_d_combo_14_e000_c400e3e0() {
    // Encoding: 0xC400E3E0
    // Test PRFB_I.P.AI_D field combination: imm5=0, Pg=0, Zn=31, prfop=0
    // Fields: prfop=0, imm5=0, Pg=0, Zn=31
    let encoding: u32 = 0xC400E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfb_i_p_ai_d_combo_15_e000_c400e000() {
    // Encoding: 0xC400E000
    // Test PRFB_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: imm5=0, Pg=0, prfop=0, Zn=0
    let encoding: u32 = 0xC400E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfb_i_p_ai_d_combo_16_e000_c400e001() {
    // Encoding: 0xC400E001
    // Test PRFB_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=1
    // Fields: Pg=0, imm5=0, Zn=0, prfop=1
    let encoding: u32 = 0xC400E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfb_i_p_ai_d_combo_17_e000_c400e007() {
    // Encoding: 0xC400E007
    // Test PRFB_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=7
    // Fields: imm5=0, Pg=0, prfop=7, Zn=0
    let encoding: u32 = 0xC400E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.AI_D
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfb_i_p_ai_d_combo_18_e000_c400e00f() {
    // Encoding: 0xC400E00F
    // Test PRFB_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=15
    // Fields: prfop=15, imm5=0, Zn=0, Pg=0
    let encoding: u32 = 0xC400E00F;
    let mut cpu = create_test_cpu();
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
// PRFB_I.P.BR_S Tests
// ============================================================================

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfb_i_p_br_s_field_rm_0_min_c000_8400c000() {
    // Encoding: 0x8400C000
    // Test PRFB_I.P.BR_S field Rm = 0 (Min)
    // Fields: prfop=0, Pg=0, Rn=0, Rm=0
    let encoding: u32 = 0x8400C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfb_i_p_br_s_field_rm_1_poweroftwo_c000_8401c000() {
    // Encoding: 0x8401C000
    // Test PRFB_I.P.BR_S field Rm = 1 (PowerOfTwo)
    // Fields: Pg=0, Rn=0, prfop=0, Rm=1
    let encoding: u32 = 0x8401C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfb_i_p_br_s_field_rm_30_poweroftwominusone_c000_841ec000() {
    // Encoding: 0x841EC000
    // Test PRFB_I.P.BR_S field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=30, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0x841EC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_prfb_i_p_br_s_field_rm_31_max_c000_841fc000() {
    // Encoding: 0x841FC000
    // Test PRFB_I.P.BR_S field Rm = 31 (Max)
    // Fields: Pg=0, Rn=0, prfop=0, Rm=31
    let encoding: u32 = 0x841FC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfb_i_p_br_s_field_pg_0_min_c000_8400c000() {
    // Encoding: 0x8400C000
    // Test PRFB_I.P.BR_S field Pg = 0 (Min)
    // Fields: Rn=0, Pg=0, Rm=0, prfop=0
    let encoding: u32 = 0x8400C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfb_i_p_br_s_field_pg_1_poweroftwo_c000_8400c400() {
    // Encoding: 0x8400C400
    // Test PRFB_I.P.BR_S field Pg = 1 (PowerOfTwo)
    // Fields: Rn=0, Rm=0, prfop=0, Pg=1
    let encoding: u32 = 0x8400C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfb_i_p_br_s_field_rn_0_min_c000_8400c000() {
    // Encoding: 0x8400C000
    // Test PRFB_I.P.BR_S field Rn = 0 (Min)
    // Fields: Rm=0, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0x8400C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfb_i_p_br_s_field_rn_1_poweroftwo_c000_8400c020() {
    // Encoding: 0x8400C020
    // Test PRFB_I.P.BR_S field Rn = 1 (PowerOfTwo)
    // Fields: prfop=0, Pg=0, Rm=0, Rn=1
    let encoding: u32 = 0x8400C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfb_i_p_br_s_field_rn_30_poweroftwominusone_c000_8400c3c0() {
    // Encoding: 0x8400C3C0
    // Test PRFB_I.P.BR_S field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Pg=0, Rn=30, Rm=0
    let encoding: u32 = 0x8400C3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfb_i_p_br_s_field_rn_31_max_c000_8400c3e0() {
    // Encoding: 0x8400C3E0
    // Test PRFB_I.P.BR_S field Rn = 31 (Max)
    // Fields: Rn=31, Rm=0, Pg=0, prfop=0
    let encoding: u32 = 0x8400C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfb_i_p_br_s_field_prfop_0_min_c000_8400c000() {
    // Encoding: 0x8400C000
    // Test PRFB_I.P.BR_S field prfop = 0 (Min)
    // Fields: Rm=0, prfop=0, Rn=0, Pg=0
    let encoding: u32 = 0x8400C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfb_i_p_br_s_field_prfop_1_poweroftwo_c000_8400c001() {
    // Encoding: 0x8400C001
    // Test PRFB_I.P.BR_S field prfop = 1 (PowerOfTwo)
    // Fields: prfop=1, Rn=0, Rm=0, Pg=0
    let encoding: u32 = 0x8400C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfb_i_p_br_s_field_prfop_7_poweroftwominusone_c000_8400c007() {
    // Encoding: 0x8400C007
    // Test PRFB_I.P.BR_S field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: prfop=7, Pg=0, Rm=0, Rn=0
    let encoding: u32 = 0x8400C007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfb_i_p_br_s_field_prfop_15_max_c000_8400c00f() {
    // Encoding: 0x8400C00F
    // Test PRFB_I.P.BR_S field prfop = 15 (Max)
    // Fields: Pg=0, Rn=0, Rm=0, prfop=15
    let encoding: u32 = 0x8400C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_prfb_i_p_br_s_combo_0_c000_8400c000() {
    // Encoding: 0x8400C000
    // Test PRFB_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rm=0, prfop=0, Rn=0
    let encoding: u32 = 0x8400C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_prfb_i_p_br_s_combo_1_c000_8401c000() {
    // Encoding: 0x8401C000
    // Test PRFB_I.P.BR_S field combination: Rm=1, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Rm=1, Pg=0, prfop=0
    let encoding: u32 = 0x8401C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfb_i_p_br_s_combo_2_c000_841ec000() {
    // Encoding: 0x841EC000
    // Test PRFB_I.P.BR_S field combination: Rm=30, Pg=0, Rn=0, prfop=0
    // Fields: Rm=30, Rn=0, prfop=0, Pg=0
    let encoding: u32 = 0x841EC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_prfb_i_p_br_s_combo_3_c000_841fc000() {
    // Encoding: 0x841FC000
    // Test PRFB_I.P.BR_S field combination: Rm=31, Pg=0, Rn=0, prfop=0
    // Fields: Rm=31, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0x841FC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfb_i_p_br_s_combo_4_c000_8400c000() {
    // Encoding: 0x8400C000
    // Test PRFB_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Rm=0, prfop=0, Pg=0
    let encoding: u32 = 0x8400C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfb_i_p_br_s_combo_5_c000_8400c400() {
    // Encoding: 0x8400C400
    // Test PRFB_I.P.BR_S field combination: Rm=0, Pg=1, Rn=0, prfop=0
    // Fields: Rn=0, Rm=0, Pg=1, prfop=0
    let encoding: u32 = 0x8400C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfb_i_p_br_s_combo_6_c000_8400c000() {
    // Encoding: 0x8400C000
    // Test PRFB_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Rm=0, prfop=0, Pg=0
    let encoding: u32 = 0x8400C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfb_i_p_br_s_combo_7_c000_8400c020() {
    // Encoding: 0x8400C020
    // Test PRFB_I.P.BR_S field combination: Rm=0, Pg=0, Rn=1, prfop=0
    // Fields: Rm=0, Pg=0, prfop=0, Rn=1
    let encoding: u32 = 0x8400C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfb_i_p_br_s_combo_8_c000_8400c3c0() {
    // Encoding: 0x8400C3C0
    // Test PRFB_I.P.BR_S field combination: Rm=0, Pg=0, Rn=30, prfop=0
    // Fields: prfop=0, Rm=0, Pg=0, Rn=30
    let encoding: u32 = 0x8400C3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfb_i_p_br_s_combo_9_c000_8400c3e0() {
    // Encoding: 0x8400C3E0
    // Test PRFB_I.P.BR_S field combination: Rm=0, Pg=0, Rn=31, prfop=0
    // Fields: Rm=0, prfop=0, Pg=0, Rn=31
    let encoding: u32 = 0x8400C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfb_i_p_br_s_combo_10_c000_8400c000() {
    // Encoding: 0x8400C000
    // Test PRFB_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, prfop=0, Rm=0, Rn=0
    let encoding: u32 = 0x8400C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfb_i_p_br_s_combo_11_c000_8400c001() {
    // Encoding: 0x8400C001
    // Test PRFB_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=1
    // Fields: Rm=0, Rn=0, Pg=0, prfop=1
    let encoding: u32 = 0x8400C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfb_i_p_br_s_combo_12_c000_8400c007() {
    // Encoding: 0x8400C007
    // Test PRFB_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=7
    // Fields: Pg=0, Rm=0, Rn=0, prfop=7
    let encoding: u32 = 0x8400C007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfb_i_p_br_s_combo_13_c000_8400c00f() {
    // Encoding: 0x8400C00F
    // Test PRFB_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=15
    // Fields: Rm=0, prfop=15, Pg=0, Rn=0
    let encoding: u32 = 0x8400C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_prfb_i_p_br_s_combo_14_c000_8401c400() {
    // Encoding: 0x8401C400
    // Test PRFB_I.P.BR_S field combination: Rm=1, Pg=1, Rn=0, prfop=0
    // Fields: Rn=0, Pg=1, Rm=1, prfop=0
    let encoding: u32 = 0x8401C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_prfb_i_p_br_s_combo_15_c000_841fdc00() {
    // Encoding: 0x841FDC00
    // Test PRFB_I.P.BR_S field combination: Rm=31, Pg=31, Rn=0, prfop=0
    // Fields: Pg=31, prfop=0, Rn=0, Rm=31
    let encoding: u32 = 0x841FDC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfb_i_p_br_s_combo_16_c000_8401c020() {
    // Encoding: 0x8401C020
    // Test PRFB_I.P.BR_S field combination: Rm=1, Pg=0, Rn=1, prfop=0
    // Fields: Rm=1, Pg=0, Rn=1, prfop=0
    let encoding: u32 = 0x8401C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfb_i_p_br_s_combo_17_c000_841fc3e0() {
    // Encoding: 0x841FC3E0
    // Test PRFB_I.P.BR_S field combination: Rm=31, Pg=0, Rn=31, prfop=0
    // Fields: Rm=31, Pg=0, prfop=0, Rn=31
    let encoding: u32 = 0x841FC3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfb_i_p_br_s_combo_18_c000_8400c420() {
    // Encoding: 0x8400C420
    // Test PRFB_I.P.BR_S field combination: Rm=0, Pg=1, Rn=1, prfop=0
    // Fields: Pg=1, prfop=0, Rm=0, Rn=1
    let encoding: u32 = 0x8400C420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfb_i_p_br_s_combo_19_c000_8400dfe0() {
    // Encoding: 0x8400DFE0
    // Test PRFB_I.P.BR_S field combination: Rm=0, Pg=31, Rn=31, prfop=0
    // Fields: Rn=31, prfop=0, Rm=0, Pg=31
    let encoding: u32 = 0x8400DFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfb_i_p_br_s_special_rn_31_stack_pointer_sp_may_require_alignment_49152_8400c3e0() {
    // Encoding: 0x8400C3E0
    // Test PRFB_I.P.BR_S special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rm=0, Pg=0, Rn=31, prfop=0
    let encoding: u32 = 0x8400C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFB_I.P.BR_S
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfb_i_p_br_s_sp_rn_8400c3e0() {
    // Test PRFB_I.P.BR_S with Rn = SP (31)
    // Encoding: 0x8400C3E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x8400C3E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PRFW_I.P.BZ_S.x32.scaled Tests
// ============================================================================

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_xs_0_min_4000_84204000() {
    // Encoding: 0x84204000
    // Test PRFW_I.P.BZ_S.x32.scaled field xs = 0 (Min)
    // Fields: Zm=0, xs=0, Rn=0, prfop=0, Pg=0
    let encoding: u32 = 0x84204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_xs_1_max_4000_84604000() {
    // Encoding: 0x84604000
    // Test PRFW_I.P.BZ_S.x32.scaled field xs = 1 (Max)
    // Fields: prfop=0, Rn=0, xs=1, Pg=0, Zm=0
    let encoding: u32 = 0x84604000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_zm_0_min_4000_84204000() {
    // Encoding: 0x84204000
    // Test PRFW_I.P.BZ_S.x32.scaled field Zm = 0 (Min)
    // Fields: Zm=0, xs=0, Rn=0, prfop=0, Pg=0
    let encoding: u32 = 0x84204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_zm_1_poweroftwo_4000_84214000() {
    // Encoding: 0x84214000
    // Test PRFW_I.P.BZ_S.x32.scaled field Zm = 1 (PowerOfTwo)
    // Fields: Zm=1, xs=0, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0x84214000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_zm_30_poweroftwominusone_4000_843e4000() {
    // Encoding: 0x843E4000
    // Test PRFW_I.P.BZ_S.x32.scaled field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: Zm=30, Pg=0, xs=0, Rn=0, prfop=0
    let encoding: u32 = 0x843E4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_zm_31_max_4000_843f4000() {
    // Encoding: 0x843F4000
    // Test PRFW_I.P.BZ_S.x32.scaled field Zm = 31 (Max)
    // Fields: Rn=0, Pg=0, prfop=0, xs=0, Zm=31
    let encoding: u32 = 0x843F4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_pg_0_min_4000_84204000() {
    // Encoding: 0x84204000
    // Test PRFW_I.P.BZ_S.x32.scaled field Pg = 0 (Min)
    // Fields: Pg=0, Rn=0, prfop=0, xs=0, Zm=0
    let encoding: u32 = 0x84204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_pg_1_poweroftwo_4000_84204400() {
    // Encoding: 0x84204400
    // Test PRFW_I.P.BZ_S.x32.scaled field Pg = 1 (PowerOfTwo)
    // Fields: xs=0, prfop=0, Pg=1, Rn=0, Zm=0
    let encoding: u32 = 0x84204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_rn_0_min_4000_84204000() {
    // Encoding: 0x84204000
    // Test PRFW_I.P.BZ_S.x32.scaled field Rn = 0 (Min)
    // Fields: Pg=0, Zm=0, xs=0, Rn=0, prfop=0
    let encoding: u32 = 0x84204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_rn_1_poweroftwo_4000_84204020() {
    // Encoding: 0x84204020
    // Test PRFW_I.P.BZ_S.x32.scaled field Rn = 1 (PowerOfTwo)
    // Fields: Pg=0, prfop=0, Rn=1, xs=0, Zm=0
    let encoding: u32 = 0x84204020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_rn_30_poweroftwominusone_4000_842043c0() {
    // Encoding: 0x842043C0
    // Test PRFW_I.P.BZ_S.x32.scaled field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Zm=0, xs=0, Rn=30, Pg=0
    let encoding: u32 = 0x842043C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_rn_31_max_4000_842043e0() {
    // Encoding: 0x842043E0
    // Test PRFW_I.P.BZ_S.x32.scaled field Rn = 31 (Max)
    // Fields: Zm=0, Rn=31, Pg=0, prfop=0, xs=0
    let encoding: u32 = 0x842043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_prfop_0_min_4000_84204000() {
    // Encoding: 0x84204000
    // Test PRFW_I.P.BZ_S.x32.scaled field prfop = 0 (Min)
    // Fields: xs=0, Zm=0, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0x84204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_prfop_1_poweroftwo_4000_84204001() {
    // Encoding: 0x84204001
    // Test PRFW_I.P.BZ_S.x32.scaled field prfop = 1 (PowerOfTwo)
    // Fields: Zm=0, Rn=0, Pg=0, prfop=1, xs=0
    let encoding: u32 = 0x84204001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_prfop_7_poweroftwominusone_4000_84204007() {
    // Encoding: 0x84204007
    // Test PRFW_I.P.BZ_S.x32.scaled field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: prfop=7, Zm=0, Pg=0, xs=0, Rn=0
    let encoding: u32 = 0x84204007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_field_prfop_15_max_4000_8420400f() {
    // Encoding: 0x8420400F
    // Test PRFW_I.P.BZ_S.x32.scaled field prfop = 15 (Max)
    // Fields: Rn=0, prfop=15, Zm=0, Pg=0, xs=0
    let encoding: u32 = 0x8420400F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=0 (minimum value)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_0_4000_84204000() {
    // Encoding: 0x84204000
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: xs=0, Pg=0, Rn=0, prfop=0, Zm=0
    let encoding: u32 = 0x84204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=1 (maximum value (1))
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_1_4000_84604000() {
    // Encoding: 0x84604000
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=1, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Pg=0, Rn=0, Zm=0, xs=1
    let encoding: u32 = 0x84604000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_2_4000_84204000() {
    // Encoding: 0x84204000
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Pg=0, xs=0, Rn=0, prfop=0
    let encoding: u32 = 0x84204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_3_4000_84214000() {
    // Encoding: 0x84214000
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=1, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, prfop=0, Zm=1, xs=0
    let encoding: u32 = 0x84214000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_4_4000_843e4000() {
    // Encoding: 0x843E4000
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=30, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Pg=0, xs=0, Zm=30, Rn=0
    let encoding: u32 = 0x843E4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_5_4000_843f4000() {
    // Encoding: 0x843F4000
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=31, Pg=0, Rn=0, prfop=0
    // Fields: Zm=31, xs=0, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0x843F4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_6_4000_84204000() {
    // Encoding: 0x84204000
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: xs=0, Rn=0, prfop=0, Zm=0, Pg=0
    let encoding: u32 = 0x84204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_7_4000_84204400() {
    // Encoding: 0x84204400
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=0, prfop=0
    // Fields: Zm=0, xs=0, prfop=0, Pg=1, Rn=0
    let encoding: u32 = 0x84204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_8_4000_84204000() {
    // Encoding: 0x84204000
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, xs=0, Zm=0, prfop=0, Rn=0
    let encoding: u32 = 0x84204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_9_4000_84204020() {
    // Encoding: 0x84204020
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=1, prfop=0
    // Fields: Zm=0, Pg=0, Rn=1, prfop=0, xs=0
    let encoding: u32 = 0x84204020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_10_4000_842043c0() {
    // Encoding: 0x842043C0
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=30, prfop=0
    // Fields: xs=0, Zm=0, Rn=30, prfop=0, Pg=0
    let encoding: u32 = 0x842043C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_11_4000_842043e0() {
    // Encoding: 0x842043E0
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=31, prfop=0
    // Fields: Pg=0, xs=0, Zm=0, Rn=31, prfop=0
    let encoding: u32 = 0x842043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_12_4000_84204000() {
    // Encoding: 0x84204000
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Pg=0, xs=0, prfop=0, Rn=0
    let encoding: u32 = 0x84204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_13_4000_84204001() {
    // Encoding: 0x84204001
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=1
    // Fields: Pg=0, Rn=0, prfop=1, Zm=0, xs=0
    let encoding: u32 = 0x84204001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_14_4000_84204007() {
    // Encoding: 0x84204007
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=7
    // Fields: Zm=0, xs=0, Pg=0, prfop=7, Rn=0
    let encoding: u32 = 0x84204007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_15_4000_8420400f() {
    // Encoding: 0x8420400F
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=15
    // Fields: Pg=0, xs=0, prfop=15, Zm=0, Rn=0
    let encoding: u32 = 0x8420400F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_16_4000_84204420() {
    // Encoding: 0x84204420
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=1, prfop=0
    // Fields: prfop=0, Pg=1, Zm=0, Rn=1, xs=0
    let encoding: u32 = 0x84204420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_combo_17_4000_84205fe0() {
    // Encoding: 0x84205FE0
    // Test PRFW_I.P.BZ_S.x32.scaled field combination: xs=0, Zm=0, Pg=31, Rn=31, prfop=0
    // Fields: prfop=0, Rn=31, xs=0, Pg=31, Zm=0
    let encoding: u32 = 0x84205FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_special_rn_31_stack_pointer_sp_may_require_alignment_16384_842043e0()
 {
    // Encoding: 0x842043E0
    // Test PRFW_I.P.BZ_S.x32.scaled special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: prfop=0, Rn=31, xs=0, Pg=0, Zm=0
    let encoding: u32 = 0x842043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_xs_0_min_4000_c4204000() {
    // Encoding: 0xC4204000
    // Test PRFW_I.P.BZ_D.x32.scaled field xs = 0 (Min)
    // Fields: Pg=0, xs=0, Zm=0, prfop=0, Rn=0
    let encoding: u32 = 0xC4204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field xs 22 +: 1`
/// Requirement: FieldBoundary { field: "xs", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_xs_1_max_4000_c4604000() {
    // Encoding: 0xC4604000
    // Test PRFW_I.P.BZ_D.x32.scaled field xs = 1 (Max)
    // Fields: Pg=0, Rn=0, xs=1, Zm=0, prfop=0
    let encoding: u32 = 0xC4604000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_zm_0_min_4000_c4204000() {
    // Encoding: 0xC4204000
    // Test PRFW_I.P.BZ_D.x32.scaled field Zm = 0 (Min)
    // Fields: Rn=0, Zm=0, xs=0, Pg=0, prfop=0
    let encoding: u32 = 0xC4204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_zm_1_poweroftwo_4000_c4214000() {
    // Encoding: 0xC4214000
    // Test PRFW_I.P.BZ_D.x32.scaled field Zm = 1 (PowerOfTwo)
    // Fields: Pg=0, prfop=0, xs=0, Zm=1, Rn=0
    let encoding: u32 = 0xC4214000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_zm_30_poweroftwominusone_4000_c43e4000() {
    // Encoding: 0xC43E4000
    // Test PRFW_I.P.BZ_D.x32.scaled field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: xs=0, Rn=0, Pg=0, prfop=0, Zm=30
    let encoding: u32 = 0xC43E4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_zm_31_max_4000_c43f4000() {
    // Encoding: 0xC43F4000
    // Test PRFW_I.P.BZ_D.x32.scaled field Zm = 31 (Max)
    // Fields: Pg=0, prfop=0, xs=0, Zm=31, Rn=0
    let encoding: u32 = 0xC43F4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_pg_0_min_4000_c4204000() {
    // Encoding: 0xC4204000
    // Test PRFW_I.P.BZ_D.x32.scaled field Pg = 0 (Min)
    // Fields: Pg=0, xs=0, Rn=0, prfop=0, Zm=0
    let encoding: u32 = 0xC4204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_pg_1_poweroftwo_4000_c4204400() {
    // Encoding: 0xC4204400
    // Test PRFW_I.P.BZ_D.x32.scaled field Pg = 1 (PowerOfTwo)
    // Fields: prfop=0, Zm=0, xs=0, Pg=1, Rn=0
    let encoding: u32 = 0xC4204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_rn_0_min_4000_c4204000() {
    // Encoding: 0xC4204000
    // Test PRFW_I.P.BZ_D.x32.scaled field Rn = 0 (Min)
    // Fields: Rn=0, prfop=0, xs=0, Zm=0, Pg=0
    let encoding: u32 = 0xC4204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_rn_1_poweroftwo_4000_c4204020() {
    // Encoding: 0xC4204020
    // Test PRFW_I.P.BZ_D.x32.scaled field Rn = 1 (PowerOfTwo)
    // Fields: xs=0, Zm=0, Rn=1, Pg=0, prfop=0
    let encoding: u32 = 0xC4204020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_rn_30_poweroftwominusone_4000_c42043c0() {
    // Encoding: 0xC42043C0
    // Test PRFW_I.P.BZ_D.x32.scaled field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Zm=0, xs=0, Rn=30, prfop=0, Pg=0
    let encoding: u32 = 0xC42043C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_rn_31_max_4000_c42043e0() {
    // Encoding: 0xC42043E0
    // Test PRFW_I.P.BZ_D.x32.scaled field Rn = 31 (Max)
    // Fields: prfop=0, Rn=31, Zm=0, xs=0, Pg=0
    let encoding: u32 = 0xC42043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_prfop_0_min_4000_c4204000() {
    // Encoding: 0xC4204000
    // Test PRFW_I.P.BZ_D.x32.scaled field prfop = 0 (Min)
    // Fields: Zm=0, prfop=0, xs=0, Rn=0, Pg=0
    let encoding: u32 = 0xC4204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_prfop_1_poweroftwo_4000_c4204001() {
    // Encoding: 0xC4204001
    // Test PRFW_I.P.BZ_D.x32.scaled field prfop = 1 (PowerOfTwo)
    // Fields: Zm=0, Rn=0, Pg=0, prfop=1, xs=0
    let encoding: u32 = 0xC4204001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_prfop_7_poweroftwominusone_4000_c4204007() {
    // Encoding: 0xC4204007
    // Test PRFW_I.P.BZ_D.x32.scaled field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Zm=0, Rn=0, prfop=7, Pg=0, xs=0
    let encoding: u32 = 0xC4204007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_field_prfop_15_max_4000_c420400f() {
    // Encoding: 0xC420400F
    // Test PRFW_I.P.BZ_D.x32.scaled field prfop = 15 (Max)
    // Fields: xs=0, Rn=0, Zm=0, Pg=0, prfop=15
    let encoding: u32 = 0xC420400F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=0 (minimum value)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_0_4000_c4204000() {
    // Encoding: 0xC4204000
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, prfop=0, Rn=0, Pg=0, xs=0
    let encoding: u32 = 0xC4204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// xs=1 (maximum value (1))
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_1_4000_c4604000() {
    // Encoding: 0xC4604000
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=1, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Zm=0, prfop=0, Pg=0, xs=1
    let encoding: u32 = 0xC4604000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_2_4000_c4204000() {
    // Encoding: 0xC4204000
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: xs=0, prfop=0, Pg=0, Zm=0, Rn=0
    let encoding: u32 = 0xC4204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_3_4000_c4214000() {
    // Encoding: 0xC4214000
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=1, Pg=0, Rn=0, prfop=0
    // Fields: xs=0, Rn=0, Zm=1, Pg=0, prfop=0
    let encoding: u32 = 0xC4214000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_4_4000_c43e4000() {
    // Encoding: 0xC43E4000
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=30, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, prfop=0, Zm=30, Pg=0, xs=0
    let encoding: u32 = 0xC43E4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_5_4000_c43f4000() {
    // Encoding: 0xC43F4000
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=31, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, xs=0, Pg=0, Zm=31
    let encoding: u32 = 0xC43F4000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_6_4000_c4204000() {
    // Encoding: 0xC4204000
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: xs=0, Rn=0, prfop=0, Pg=0, Zm=0
    let encoding: u32 = 0xC4204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_7_4000_c4204400() {
    // Encoding: 0xC4204400
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=0, prfop=0
    // Fields: Pg=1, xs=0, Zm=0, Rn=0, prfop=0
    let encoding: u32 = 0xC4204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_8_4000_c4204000() {
    // Encoding: 0xC4204000
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, xs=0, Rn=0, Zm=0, Pg=0
    let encoding: u32 = 0xC4204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_9_4000_c4204020() {
    // Encoding: 0xC4204020
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=1, prfop=0
    // Fields: Pg=0, Rn=1, prfop=0, xs=0, Zm=0
    let encoding: u32 = 0xC4204020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_10_4000_c42043c0() {
    // Encoding: 0xC42043C0
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=30, prfop=0
    // Fields: prfop=0, Zm=0, xs=0, Pg=0, Rn=30
    let encoding: u32 = 0xC42043C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_11_4000_c42043e0() {
    // Encoding: 0xC42043E0
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=31, prfop=0
    // Fields: prfop=0, Rn=31, Zm=0, xs=0, Pg=0
    let encoding: u32 = 0xC42043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_12_4000_c4204000() {
    // Encoding: 0xC4204000
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: xs=0, Pg=0, Rn=0, prfop=0, Zm=0
    let encoding: u32 = 0xC4204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_13_4000_c4204001() {
    // Encoding: 0xC4204001
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=1
    // Fields: Zm=0, Rn=0, xs=0, Pg=0, prfop=1
    let encoding: u32 = 0xC4204001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_14_4000_c4204007() {
    // Encoding: 0xC4204007
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=7
    // Fields: prfop=7, xs=0, Pg=0, Zm=0, Rn=0
    let encoding: u32 = 0xC4204007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_15_4000_c420400f() {
    // Encoding: 0xC420400F
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=0, Rn=0, prfop=15
    // Fields: Zm=0, Pg=0, Rn=0, prfop=15, xs=0
    let encoding: u32 = 0xC420400F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_16_4000_c4204420() {
    // Encoding: 0xC4204420
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=1, Rn=1, prfop=0
    // Fields: Zm=0, prfop=0, Rn=1, xs=0, Pg=1
    let encoding: u32 = 0xC4204420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_combo_17_4000_c4205fe0() {
    // Encoding: 0xC4205FE0
    // Test PRFW_I.P.BZ_D.x32.scaled field combination: xs=0, Zm=0, Pg=31, Rn=31, prfop=0
    // Fields: prfop=0, Pg=31, xs=0, Zm=0, Rn=31
    let encoding: u32 = 0xC4205FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_special_rn_31_stack_pointer_sp_may_require_alignment_16384_c42043e0()
 {
    // Encoding: 0xC42043E0
    // Test PRFW_I.P.BZ_D.x32.scaled special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: xs=0, Rn=31, Pg=0, Zm=0, prfop=0
    let encoding: u32 = 0xC42043E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_zm_0_min_c000_c460c000() {
    // Encoding: 0xC460C000
    // Test PRFW_I.P.BZ_D.64.scaled field Zm = 0 (Min)
    // Fields: Zm=0, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0xC460C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_zm_1_poweroftwo_c000_c461c000() {
    // Encoding: 0xC461C000
    // Test PRFW_I.P.BZ_D.64.scaled field Zm = 1 (PowerOfTwo)
    // Fields: prfop=0, Pg=0, Zm=1, Rn=0
    let encoding: u32 = 0xC461C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_zm_30_poweroftwominusone_c000_c47ec000() {
    // Encoding: 0xC47EC000
    // Test PRFW_I.P.BZ_D.64.scaled field Zm = 30 (PowerOfTwoMinusOne)
    // Fields: Zm=30, Pg=0, prfop=0, Rn=0
    let encoding: u32 = 0xC47EC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field Zm 16 +: 5`
/// Requirement: FieldBoundary { field: "Zm", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_zm_31_max_c000_c47fc000() {
    // Encoding: 0xC47FC000
    // Test PRFW_I.P.BZ_D.64.scaled field Zm = 31 (Max)
    // Fields: prfop=0, Pg=0, Rn=0, Zm=31
    let encoding: u32 = 0xC47FC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_pg_0_min_c000_c460c000() {
    // Encoding: 0xC460C000
    // Test PRFW_I.P.BZ_D.64.scaled field Pg = 0 (Min)
    // Fields: Pg=0, Zm=0, Rn=0, prfop=0
    let encoding: u32 = 0xC460C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_pg_1_poweroftwo_c000_c460c400() {
    // Encoding: 0xC460C400
    // Test PRFW_I.P.BZ_D.64.scaled field Pg = 1 (PowerOfTwo)
    // Fields: Zm=0, prfop=0, Rn=0, Pg=1
    let encoding: u32 = 0xC460C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_rn_0_min_c000_c460c000() {
    // Encoding: 0xC460C000
    // Test PRFW_I.P.BZ_D.64.scaled field Rn = 0 (Min)
    // Fields: Pg=0, Zm=0, Rn=0, prfop=0
    let encoding: u32 = 0xC460C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_rn_1_poweroftwo_c000_c460c020() {
    // Encoding: 0xC460C020
    // Test PRFW_I.P.BZ_D.64.scaled field Rn = 1 (PowerOfTwo)
    // Fields: Zm=0, Pg=0, Rn=1, prfop=0
    let encoding: u32 = 0xC460C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_rn_30_poweroftwominusone_c000_c460c3c0() {
    // Encoding: 0xC460C3C0
    // Test PRFW_I.P.BZ_D.64.scaled field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Pg=0, Rn=30, Zm=0
    let encoding: u32 = 0xC460C3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_rn_31_max_c000_c460c3e0() {
    // Encoding: 0xC460C3E0
    // Test PRFW_I.P.BZ_D.64.scaled field Rn = 31 (Max)
    // Fields: prfop=0, Zm=0, Pg=0, Rn=31
    let encoding: u32 = 0xC460C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_prfop_0_min_c000_c460c000() {
    // Encoding: 0xC460C000
    // Test PRFW_I.P.BZ_D.64.scaled field prfop = 0 (Min)
    // Fields: prfop=0, Pg=0, Zm=0, Rn=0
    let encoding: u32 = 0xC460C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_prfop_1_poweroftwo_c000_c460c001() {
    // Encoding: 0xC460C001
    // Test PRFW_I.P.BZ_D.64.scaled field prfop = 1 (PowerOfTwo)
    // Fields: Rn=0, Pg=0, prfop=1, Zm=0
    let encoding: u32 = 0xC460C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_prfop_7_poweroftwominusone_c000_c460c007() {
    // Encoding: 0xC460C007
    // Test PRFW_I.P.BZ_D.64.scaled field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, prfop=7, Pg=0, Zm=0
    let encoding: u32 = 0xC460C007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_field_prfop_15_max_c000_c460c00f() {
    // Encoding: 0xC460C00F
    // Test PRFW_I.P.BZ_D.64.scaled field prfop = 15 (Max)
    // Fields: Rn=0, Zm=0, Pg=0, prfop=15
    let encoding: u32 = 0xC460C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=0 (SIMD register V0)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_0_c000_c460c000() {
    // Encoding: 0xC460C000
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Zm=0, prfop=0, Pg=0
    let encoding: u32 = 0xC460C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=1 (SIMD register V1)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_1_c000_c461c000() {
    // Encoding: 0xC461C000
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=1, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, Zm=1, Pg=0
    let encoding: u32 = 0xC461C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=30 (SIMD register V30)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_2_c000_c47ec000() {
    // Encoding: 0xC47EC000
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=30, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Zm=30, Rn=0, Pg=0
    let encoding: u32 = 0xC47EC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zm=31 (SIMD register V31)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_3_c000_c47fc000() {
    // Encoding: 0xC47FC000
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=31, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Zm=31, Pg=0, Rn=0
    let encoding: u32 = 0xC47FC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_4_c000_c460c000() {
    // Encoding: 0xC460C000
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, prfop=0, Zm=0, Rn=0
    let encoding: u32 = 0xC460C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_5_c000_c460c400() {
    // Encoding: 0xC460C400
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=1, Rn=0, prfop=0
    // Fields: prfop=0, Zm=0, Rn=0, Pg=1
    let encoding: u32 = 0xC460C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_6_c000_c460c000() {
    // Encoding: 0xC460C000
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Zm=0, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0xC460C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_7_c000_c460c020() {
    // Encoding: 0xC460C020
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=1, prfop=0
    // Fields: Rn=1, Pg=0, Zm=0, prfop=0
    let encoding: u32 = 0xC460C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_8_c000_c460c3c0() {
    // Encoding: 0xC460C3C0
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=30, prfop=0
    // Fields: Rn=30, Zm=0, Pg=0, prfop=0
    let encoding: u32 = 0xC460C3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_9_c000_c460c3e0() {
    // Encoding: 0xC460C3E0
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=31, prfop=0
    // Fields: Pg=0, prfop=0, Zm=0, Rn=31
    let encoding: u32 = 0xC460C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_10_c000_c460c000() {
    // Encoding: 0xC460C000
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, prfop=0, Rn=0, Zm=0
    let encoding: u32 = 0xC460C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_11_c000_c460c001() {
    // Encoding: 0xC460C001
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=1
    // Fields: Zm=0, Pg=0, Rn=0, prfop=1
    let encoding: u32 = 0xC460C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_12_c000_c460c007() {
    // Encoding: 0xC460C007
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=7
    // Fields: Zm=0, Rn=0, Pg=0, prfop=7
    let encoding: u32 = 0xC460C007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_13_c000_c460c00f() {
    // Encoding: 0xC460C00F
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=0, Rn=0, prfop=15
    // Fields: Zm=0, Rn=0, prfop=15, Pg=0
    let encoding: u32 = 0xC460C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_14_c000_c460c420() {
    // Encoding: 0xC460C420
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=1, Rn=1, prfop=0
    // Fields: Rn=1, Zm=0, prfop=0, Pg=1
    let encoding: u32 = 0xC460C420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfw_i_p_bz_d_64_scaled_combo_15_c000_c460dfe0() {
    // Encoding: 0xC460DFE0
    // Test PRFW_I.P.BZ_D.64.scaled field combination: Zm=0, Pg=31, Rn=31, prfop=0
    // Fields: Pg=31, Rn=31, prfop=0, Zm=0
    let encoding: u32 = 0xC460DFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfw_i_p_bz_d_64_scaled_special_rn_31_stack_pointer_sp_may_require_alignment_49152_c460c3e0()
 {
    // Encoding: 0xC460C3E0
    // Test PRFW_I.P.BZ_D.64.scaled special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Zm=0, prfop=0, Pg=0
    let encoding: u32 = 0xC460C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFW_I.P.BZ_S.x32.scaled
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfw_i_p_bz_s_x32_scaled_sp_rn_842043e0() {
    // Test PRFW_I.P.BZ_S.x32.scaled with Rn = SP (31)
    // Encoding: 0x842043E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x842043E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PRFW_I.P.BZ_D.x32.scaled
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfw_i_p_bz_d_x32_scaled_sp_rn_c42043e0() {
    // Test PRFW_I.P.BZ_D.x32.scaled with Rn = SP (31)
    // Encoding: 0xC42043E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xC42043E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: PRFW_I.P.BZ_D.64.scaled
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfw_i_p_bz_d_64_scaled_sp_rn_c460c3e0() {
    // Test PRFW_I.P.BZ_D.64.scaled with Rn = SP (31)
    // Encoding: 0xC460C3E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xC460C3E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PRFH_I.P.BR_S Tests
// ============================================================================

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfh_i_p_br_s_field_rm_0_min_c000_8480c000() {
    // Encoding: 0x8480C000
    // Test PRFH_I.P.BR_S field Rm = 0 (Min)
    // Fields: Pg=0, Rm=0, Rn=0, prfop=0
    let encoding: u32 = 0x8480C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfh_i_p_br_s_field_rm_1_poweroftwo_c000_8481c000() {
    // Encoding: 0x8481C000
    // Test PRFH_I.P.BR_S field Rm = 1 (PowerOfTwo)
    // Fields: Pg=0, prfop=0, Rm=1, Rn=0
    let encoding: u32 = 0x8481C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfh_i_p_br_s_field_rm_30_poweroftwominusone_c000_849ec000() {
    // Encoding: 0x849EC000
    // Test PRFH_I.P.BR_S field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Pg=0, Rn=0, prfop=0, Rm=30
    let encoding: u32 = 0x849EC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_prfh_i_p_br_s_field_rm_31_max_c000_849fc000() {
    // Encoding: 0x849FC000
    // Test PRFH_I.P.BR_S field Rm = 31 (Max)
    // Fields: Rm=31, prfop=0, Pg=0, Rn=0
    let encoding: u32 = 0x849FC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfh_i_p_br_s_field_pg_0_min_c000_8480c000() {
    // Encoding: 0x8480C000
    // Test PRFH_I.P.BR_S field Pg = 0 (Min)
    // Fields: Rm=0, Rn=0, prfop=0, Pg=0
    let encoding: u32 = 0x8480C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfh_i_p_br_s_field_pg_1_poweroftwo_c000_8480c400() {
    // Encoding: 0x8480C400
    // Test PRFH_I.P.BR_S field Pg = 1 (PowerOfTwo)
    // Fields: Pg=1, prfop=0, Rm=0, Rn=0
    let encoding: u32 = 0x8480C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfh_i_p_br_s_field_rn_0_min_c000_8480c000() {
    // Encoding: 0x8480C000
    // Test PRFH_I.P.BR_S field Rn = 0 (Min)
    // Fields: Rm=0, Rn=0, Pg=0, prfop=0
    let encoding: u32 = 0x8480C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfh_i_p_br_s_field_rn_1_poweroftwo_c000_8480c020() {
    // Encoding: 0x8480C020
    // Test PRFH_I.P.BR_S field Rn = 1 (PowerOfTwo)
    // Fields: prfop=0, Rn=1, Pg=0, Rm=0
    let encoding: u32 = 0x8480C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_prfh_i_p_br_s_field_rn_30_poweroftwominusone_c000_8480c3c0() {
    // Encoding: 0x8480C3C0
    // Test PRFH_I.P.BR_S field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Pg=0, Rn=30, Rm=0
    let encoding: u32 = 0x8480C3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_prfh_i_p_br_s_field_rn_31_max_c000_8480c3e0() {
    // Encoding: 0x8480C3E0
    // Test PRFH_I.P.BR_S field Rn = 31 (Max)
    // Fields: Pg=0, Rm=0, prfop=0, Rn=31
    let encoding: u32 = 0x8480C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfh_i_p_br_s_field_prfop_0_min_c000_8480c000() {
    // Encoding: 0x8480C000
    // Test PRFH_I.P.BR_S field prfop = 0 (Min)
    // Fields: Rm=0, Rn=0, prfop=0, Pg=0
    let encoding: u32 = 0x8480C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfh_i_p_br_s_field_prfop_1_poweroftwo_c000_8480c001() {
    // Encoding: 0x8480C001
    // Test PRFH_I.P.BR_S field prfop = 1 (PowerOfTwo)
    // Fields: Rm=0, Pg=0, Rn=0, prfop=1
    let encoding: u32 = 0x8480C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfh_i_p_br_s_field_prfop_7_poweroftwominusone_c000_8480c007() {
    // Encoding: 0x8480C007
    // Test PRFH_I.P.BR_S field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rm=0, Pg=0, prfop=7
    let encoding: u32 = 0x8480C007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfh_i_p_br_s_field_prfop_15_max_c000_8480c00f() {
    // Encoding: 0x8480C00F
    // Test PRFH_I.P.BR_S field prfop = 15 (Max)
    // Fields: Rm=0, Pg=0, prfop=15, Rn=0
    let encoding: u32 = 0x8480C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_prfh_i_p_br_s_combo_0_c000_8480c000() {
    // Encoding: 0x8480C000
    // Test PRFH_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rm=0, Pg=0, Rn=0, prfop=0
    let encoding: u32 = 0x8480C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_prfh_i_p_br_s_combo_1_c000_8481c000() {
    // Encoding: 0x8481C000
    // Test PRFH_I.P.BR_S field combination: Rm=1, Pg=0, Rn=0, prfop=0
    // Fields: Rm=1, Rn=0, prfop=0, Pg=0
    let encoding: u32 = 0x8481C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfh_i_p_br_s_combo_2_c000_849ec000() {
    // Encoding: 0x849EC000
    // Test PRFH_I.P.BR_S field combination: Rm=30, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, Rm=30, prfop=0
    let encoding: u32 = 0x849EC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_prfh_i_p_br_s_combo_3_c000_849fc000() {
    // Encoding: 0x849FC000
    // Test PRFH_I.P.BR_S field combination: Rm=31, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rm=31, Pg=0, Rn=0
    let encoding: u32 = 0x849FC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfh_i_p_br_s_combo_4_c000_8480c000() {
    // Encoding: 0x8480C000
    // Test PRFH_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: Pg=0, Rm=0, Rn=0, prfop=0
    let encoding: u32 = 0x8480C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfh_i_p_br_s_combo_5_c000_8480c400() {
    // Encoding: 0x8480C400
    // Test PRFH_I.P.BR_S field combination: Rm=0, Pg=1, Rn=0, prfop=0
    // Fields: Pg=1, Rm=0, Rn=0, prfop=0
    let encoding: u32 = 0x8480C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_prfh_i_p_br_s_combo_6_c000_8480c000() {
    // Encoding: 0x8480C000
    // Test PRFH_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: prfop=0, Rn=0, Pg=0, Rm=0
    let encoding: u32 = 0x8480C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_prfh_i_p_br_s_combo_7_c000_8480c020() {
    // Encoding: 0x8480C020
    // Test PRFH_I.P.BR_S field combination: Rm=0, Pg=0, Rn=1, prfop=0
    // Fields: Pg=0, prfop=0, Rm=0, Rn=1
    let encoding: u32 = 0x8480C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_prfh_i_p_br_s_combo_8_c000_8480c3c0() {
    // Encoding: 0x8480C3C0
    // Test PRFH_I.P.BR_S field combination: Rm=0, Pg=0, Rn=30, prfop=0
    // Fields: Rn=30, prfop=0, Rm=0, Pg=0
    let encoding: u32 = 0x8480C3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_prfh_i_p_br_s_combo_9_c000_8480c3e0() {
    // Encoding: 0x8480C3E0
    // Test PRFH_I.P.BR_S field combination: Rm=0, Pg=0, Rn=31, prfop=0
    // Fields: Rm=0, Pg=0, Rn=31, prfop=0
    let encoding: u32 = 0x8480C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfh_i_p_br_s_combo_10_c000_8480c000() {
    // Encoding: 0x8480C000
    // Test PRFH_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=0
    // Fields: Rn=0, Pg=0, Rm=0, prfop=0
    let encoding: u32 = 0x8480C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfh_i_p_br_s_combo_11_c000_8480c001() {
    // Encoding: 0x8480C001
    // Test PRFH_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=1
    // Fields: Rm=0, prfop=1, Pg=0, Rn=0
    let encoding: u32 = 0x8480C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfh_i_p_br_s_combo_12_c000_8480c007() {
    // Encoding: 0x8480C007
    // Test PRFH_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=7
    // Fields: Pg=0, Rm=0, Rn=0, prfop=7
    let encoding: u32 = 0x8480C007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfh_i_p_br_s_combo_13_c000_8480c00f() {
    // Encoding: 0x8480C00F
    // Test PRFH_I.P.BR_S field combination: Rm=0, Pg=0, Rn=0, prfop=15
    // Fields: prfop=15, Rm=0, Rn=0, Pg=0
    let encoding: u32 = 0x8480C00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Pg=1 (same register test (reg=1))
#[test]
fn test_prfh_i_p_br_s_combo_14_c000_8481c400() {
    // Encoding: 0x8481C400
    // Test PRFH_I.P.BR_S field combination: Rm=1, Pg=1, Rn=0, prfop=0
    // Fields: Rn=0, Rm=1, Pg=1, prfop=0
    let encoding: u32 = 0x8481C400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Pg=31 (same register test (reg=31))
#[test]
fn test_prfh_i_p_br_s_combo_15_c000_849fdc00() {
    // Encoding: 0x849FDC00
    // Test PRFH_I.P.BR_S field combination: Rm=31, Pg=31, Rn=0, prfop=0
    // Fields: Pg=31, Rn=0, Rm=31, prfop=0
    let encoding: u32 = 0x849FDC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfh_i_p_br_s_combo_16_c000_8481c020() {
    // Encoding: 0x8481C020
    // Test PRFH_I.P.BR_S field combination: Rm=1, Pg=0, Rn=1, prfop=0
    // Fields: Pg=0, Rn=1, Rm=1, prfop=0
    let encoding: u32 = 0x8481C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfh_i_p_br_s_combo_17_c000_849fc3e0() {
    // Encoding: 0x849FC3E0
    // Test PRFH_I.P.BR_S field combination: Rm=31, Pg=0, Rn=31, prfop=0
    // Fields: Rn=31, Rm=31, prfop=0, Pg=0
    let encoding: u32 = 0x849FC3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_prfh_i_p_br_s_combo_18_c000_8480c420() {
    // Encoding: 0x8480C420
    // Test PRFH_I.P.BR_S field combination: Rm=0, Pg=1, Rn=1, prfop=0
    // Fields: Pg=1, prfop=0, Rm=0, Rn=1
    let encoding: u32 = 0x8480C420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_prfh_i_p_br_s_combo_19_c000_8480dfe0() {
    // Encoding: 0x8480DFE0
    // Test PRFH_I.P.BR_S field combination: Rm=0, Pg=31, Rn=31, prfop=0
    // Fields: Pg=31, prfop=0, Rm=0, Rn=31
    let encoding: u32 = 0x8480DFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_prfh_i_p_br_s_special_rn_31_stack_pointer_sp_may_require_alignment_49152_8480c3e0() {
    // Encoding: 0x8480C3E0
    // Test PRFH_I.P.BR_S special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, prfop=0, Rm=0, Pg=0
    let encoding: u32 = 0x8480C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.BR_S
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_prfh_i_p_br_s_sp_rn_8480c3e0() {
    // Test PRFH_I.P.BR_S with Rn = SP (31)
    // Encoding: 0x8480C3E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x8480C3E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// PRFH_I.P.AI_S Tests
// ============================================================================

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_prfh_i_p_ai_s_field_imm5_0_zero_e000_8480e000() {
    // Encoding: 0x8480E000
    // Test PRFH_I.P.AI_S field imm5 = 0 (Zero)
    // Fields: prfop=0, Zn=0, Pg=0, imm5=0
    let encoding: u32 = 0x8480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_prfh_i_p_ai_s_field_imm5_1_poweroftwo_e000_8481e000() {
    // Encoding: 0x8481E000
    // Test PRFH_I.P.AI_S field imm5 = 1 (PowerOfTwo)
    // Fields: imm5=1, Pg=0, Zn=0, prfop=0
    let encoding: u32 = 0x8481E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_prfh_i_p_ai_s_field_imm5_3_poweroftwominusone_e000_8483e000() {
    // Encoding: 0x8483E000
    // Test PRFH_I.P.AI_S field imm5 = 3 (PowerOfTwoMinusOne)
    // Fields: prfop=0, Zn=0, imm5=3, Pg=0
    let encoding: u32 = 0x8483E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_prfh_i_p_ai_s_field_imm5_4_poweroftwo_e000_8484e000() {
    // Encoding: 0x8484E000
    // Test PRFH_I.P.AI_S field imm5 = 4 (PowerOfTwo)
    // Fields: Pg=0, imm5=4, prfop=0, Zn=0
    let encoding: u32 = 0x8484E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_prfh_i_p_ai_s_field_imm5_7_poweroftwominusone_e000_8487e000() {
    // Encoding: 0x8487E000
    // Test PRFH_I.P.AI_S field imm5 = 7 (PowerOfTwoMinusOne)
    // Fields: imm5=7, prfop=0, Zn=0, Pg=0
    let encoding: u32 = 0x8487E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_prfh_i_p_ai_s_field_imm5_8_poweroftwo_e000_8488e000() {
    // Encoding: 0x8488E000
    // Test PRFH_I.P.AI_S field imm5 = 8 (PowerOfTwo)
    // Fields: Pg=0, prfop=0, Zn=0, imm5=8
    let encoding: u32 = 0x8488E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_prfh_i_p_ai_s_field_imm5_15_poweroftwominusone_e000_848fe000() {
    // Encoding: 0x848FE000
    // Test PRFH_I.P.AI_S field imm5 = 15 (PowerOfTwoMinusOne)
    // Fields: imm5=15, prfop=0, Zn=0, Pg=0
    let encoding: u32 = 0x848FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_prfh_i_p_ai_s_field_imm5_16_poweroftwo_e000_8490e000() {
    // Encoding: 0x8490E000
    // Test PRFH_I.P.AI_S field imm5 = 16 (PowerOfTwo)
    // Fields: imm5=16, prfop=0, Pg=0, Zn=0
    let encoding: u32 = 0x8490E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_prfh_i_p_ai_s_field_imm5_31_max_e000_849fe000() {
    // Encoding: 0x849FE000
    // Test PRFH_I.P.AI_S field imm5 = 31 (Max)
    // Fields: Pg=0, imm5=31, Zn=0, prfop=0
    let encoding: u32 = 0x849FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfh_i_p_ai_s_field_pg_0_min_e000_8480e000() {
    // Encoding: 0x8480E000
    // Test PRFH_I.P.AI_S field Pg = 0 (Min)
    // Fields: Zn=0, prfop=0, imm5=0, Pg=0
    let encoding: u32 = 0x8480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfh_i_p_ai_s_field_pg_1_poweroftwo_e000_8480e400() {
    // Encoding: 0x8480E400
    // Test PRFH_I.P.AI_S field Pg = 1 (PowerOfTwo)
    // Fields: imm5=0, Zn=0, Pg=1, prfop=0
    let encoding: u32 = 0x8480E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfh_i_p_ai_s_field_zn_0_min_e000_8480e000() {
    // Encoding: 0x8480E000
    // Test PRFH_I.P.AI_S field Zn = 0 (Min)
    // Fields: Pg=0, prfop=0, Zn=0, imm5=0
    let encoding: u32 = 0x8480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfh_i_p_ai_s_field_zn_1_poweroftwo_e000_8480e020() {
    // Encoding: 0x8480E020
    // Test PRFH_I.P.AI_S field Zn = 1 (PowerOfTwo)
    // Fields: imm5=0, Pg=0, Zn=1, prfop=0
    let encoding: u32 = 0x8480E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfh_i_p_ai_s_field_zn_30_poweroftwominusone_e000_8480e3c0() {
    // Encoding: 0x8480E3C0
    // Test PRFH_I.P.AI_S field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: imm5=0, Pg=0, prfop=0, Zn=30
    let encoding: u32 = 0x8480E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfh_i_p_ai_s_field_zn_31_max_e000_8480e3e0() {
    // Encoding: 0x8480E3E0
    // Test PRFH_I.P.AI_S field Zn = 31 (Max)
    // Fields: imm5=0, Pg=0, prfop=0, Zn=31
    let encoding: u32 = 0x8480E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfh_i_p_ai_s_field_prfop_0_min_e000_8480e000() {
    // Encoding: 0x8480E000
    // Test PRFH_I.P.AI_S field prfop = 0 (Min)
    // Fields: Pg=0, imm5=0, prfop=0, Zn=0
    let encoding: u32 = 0x8480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfh_i_p_ai_s_field_prfop_1_poweroftwo_e000_8480e001() {
    // Encoding: 0x8480E001
    // Test PRFH_I.P.AI_S field prfop = 1 (PowerOfTwo)
    // Fields: Pg=0, Zn=0, prfop=1, imm5=0
    let encoding: u32 = 0x8480E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfh_i_p_ai_s_field_prfop_7_poweroftwominusone_e000_8480e007() {
    // Encoding: 0x8480E007
    // Test PRFH_I.P.AI_S field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: prfop=7, Zn=0, Pg=0, imm5=0
    let encoding: u32 = 0x8480E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfh_i_p_ai_s_field_prfop_15_max_e000_8480e00f() {
    // Encoding: 0x8480E00F
    // Test PRFH_I.P.AI_S field prfop = 15 (Max)
    // Fields: Zn=0, prfop=15, Pg=0, imm5=0
    let encoding: u32 = 0x8480E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=0 (immediate value 0)
#[test]
fn test_prfh_i_p_ai_s_combo_0_e000_8480e000() {
    // Encoding: 0x8480E000
    // Test PRFH_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, imm5=0, Zn=0, prfop=0
    let encoding: u32 = 0x8480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=1 (immediate value 1)
#[test]
fn test_prfh_i_p_ai_s_combo_1_e000_8481e000() {
    // Encoding: 0x8481E000
    // Test PRFH_I.P.AI_S field combination: imm5=1, Pg=0, Zn=0, prfop=0
    // Fields: imm5=1, Pg=0, prfop=0, Zn=0
    let encoding: u32 = 0x8481E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=3 (2^2 - 1 = 3)
#[test]
fn test_prfh_i_p_ai_s_combo_2_e000_8483e000() {
    // Encoding: 0x8483E000
    // Test PRFH_I.P.AI_S field combination: imm5=3, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, Pg=0, imm5=3, prfop=0
    let encoding: u32 = 0x8483E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=4 (power of 2 (2^2 = 4))
#[test]
fn test_prfh_i_p_ai_s_combo_3_e000_8484e000() {
    // Encoding: 0x8484E000
    // Test PRFH_I.P.AI_S field combination: imm5=4, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, imm5=4, Zn=0, prfop=0
    let encoding: u32 = 0x8484E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=7 (2^3 - 1 = 7)
#[test]
fn test_prfh_i_p_ai_s_combo_4_e000_8487e000() {
    // Encoding: 0x8487E000
    // Test PRFH_I.P.AI_S field combination: imm5=7, Pg=0, Zn=0, prfop=0
    // Fields: imm5=7, Zn=0, prfop=0, Pg=0
    let encoding: u32 = 0x8487E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=8 (power of 2 (2^3 = 8))
#[test]
fn test_prfh_i_p_ai_s_combo_5_e000_8488e000() {
    // Encoding: 0x8488E000
    // Test PRFH_I.P.AI_S field combination: imm5=8, Pg=0, Zn=0, prfop=0
    // Fields: imm5=8, Pg=0, prfop=0, Zn=0
    let encoding: u32 = 0x8488E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=15 (immediate midpoint (15))
#[test]
fn test_prfh_i_p_ai_s_combo_6_e000_848fe000() {
    // Encoding: 0x848FE000
    // Test PRFH_I.P.AI_S field combination: imm5=15, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, Zn=0, prfop=0, imm5=15
    let encoding: u32 = 0x848FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=16 (power of 2 (2^4 = 16))
#[test]
fn test_prfh_i_p_ai_s_combo_7_e000_8490e000() {
    // Encoding: 0x8490E000
    // Test PRFH_I.P.AI_S field combination: imm5=16, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, prfop=0, imm5=16, Pg=0
    let encoding: u32 = 0x8490E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=31 (maximum immediate (31))
#[test]
fn test_prfh_i_p_ai_s_combo_8_e000_849fe000() {
    // Encoding: 0x849FE000
    // Test PRFH_I.P.AI_S field combination: imm5=31, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, Pg=0, imm5=31, prfop=0
    let encoding: u32 = 0x849FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfh_i_p_ai_s_combo_9_e000_8480e000() {
    // Encoding: 0x8480E000
    // Test PRFH_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: imm5=0, Pg=0, Zn=0, prfop=0
    let encoding: u32 = 0x8480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfh_i_p_ai_s_combo_10_e000_8480e400() {
    // Encoding: 0x8480E400
    // Test PRFH_I.P.AI_S field combination: imm5=0, Pg=1, Zn=0, prfop=0
    // Fields: Pg=1, Zn=0, imm5=0, prfop=0
    let encoding: u32 = 0x8480E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_prfh_i_p_ai_s_combo_11_e000_8480e000() {
    // Encoding: 0x8480E000
    // Test PRFH_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: imm5=0, Zn=0, prfop=0, Pg=0
    let encoding: u32 = 0x8480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_prfh_i_p_ai_s_combo_12_e000_8480e020() {
    // Encoding: 0x8480E020
    // Test PRFH_I.P.AI_S field combination: imm5=0, Pg=0, Zn=1, prfop=0
    // Fields: prfop=0, imm5=0, Zn=1, Pg=0
    let encoding: u32 = 0x8480E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_prfh_i_p_ai_s_combo_13_e000_8480e3c0() {
    // Encoding: 0x8480E3C0
    // Test PRFH_I.P.AI_S field combination: imm5=0, Pg=0, Zn=30, prfop=0
    // Fields: Zn=30, Pg=0, imm5=0, prfop=0
    let encoding: u32 = 0x8480E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_prfh_i_p_ai_s_combo_14_e000_8480e3e0() {
    // Encoding: 0x8480E3E0
    // Test PRFH_I.P.AI_S field combination: imm5=0, Pg=0, Zn=31, prfop=0
    // Fields: prfop=0, imm5=0, Pg=0, Zn=31
    let encoding: u32 = 0x8480E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfh_i_p_ai_s_combo_15_e000_8480e000() {
    // Encoding: 0x8480E000
    // Test PRFH_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: imm5=0, Pg=0, prfop=0, Zn=0
    let encoding: u32 = 0x8480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfh_i_p_ai_s_combo_16_e000_8480e001() {
    // Encoding: 0x8480E001
    // Test PRFH_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=1
    // Fields: imm5=0, Zn=0, Pg=0, prfop=1
    let encoding: u32 = 0x8480E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfh_i_p_ai_s_combo_17_e000_8480e007() {
    // Encoding: 0x8480E007
    // Test PRFH_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=7
    // Fields: Pg=0, Zn=0, imm5=0, prfop=7
    let encoding: u32 = 0x8480E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_S
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfh_i_p_ai_s_combo_18_e000_8480e00f() {
    // Encoding: 0x8480E00F
    // Test PRFH_I.P.AI_S field combination: imm5=0, Pg=0, Zn=0, prfop=15
    // Fields: Pg=0, Zn=0, imm5=0, prfop=15
    let encoding: u32 = 0x8480E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_prfh_i_p_ai_d_field_imm5_0_zero_e000_c480e000() {
    // Encoding: 0xC480E000
    // Test PRFH_I.P.AI_D field imm5 = 0 (Zero)
    // Fields: prfop=0, Zn=0, Pg=0, imm5=0
    let encoding: u32 = 0xC480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_prfh_i_p_ai_d_field_imm5_1_poweroftwo_e000_c481e000() {
    // Encoding: 0xC481E000
    // Test PRFH_I.P.AI_D field imm5 = 1 (PowerOfTwo)
    // Fields: prfop=0, Pg=0, imm5=1, Zn=0
    let encoding: u32 = 0xC481E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_prfh_i_p_ai_d_field_imm5_3_poweroftwominusone_e000_c483e000() {
    // Encoding: 0xC483E000
    // Test PRFH_I.P.AI_D field imm5 = 3 (PowerOfTwoMinusOne)
    // Fields: imm5=3, Zn=0, Pg=0, prfop=0
    let encoding: u32 = 0xC483E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_prfh_i_p_ai_d_field_imm5_4_poweroftwo_e000_c484e000() {
    // Encoding: 0xC484E000
    // Test PRFH_I.P.AI_D field imm5 = 4 (PowerOfTwo)
    // Fields: prfop=0, imm5=4, Pg=0, Zn=0
    let encoding: u32 = 0xC484E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_prfh_i_p_ai_d_field_imm5_7_poweroftwominusone_e000_c487e000() {
    // Encoding: 0xC487E000
    // Test PRFH_I.P.AI_D field imm5 = 7 (PowerOfTwoMinusOne)
    // Fields: prfop=0, imm5=7, Zn=0, Pg=0
    let encoding: u32 = 0xC487E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_prfh_i_p_ai_d_field_imm5_8_poweroftwo_e000_c488e000() {
    // Encoding: 0xC488E000
    // Test PRFH_I.P.AI_D field imm5 = 8 (PowerOfTwo)
    // Fields: Pg=0, prfop=0, imm5=8, Zn=0
    let encoding: u32 = 0xC488E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 15, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (15)
#[test]
fn test_prfh_i_p_ai_d_field_imm5_15_poweroftwominusone_e000_c48fe000() {
    // Encoding: 0xC48FE000
    // Test PRFH_I.P.AI_D field imm5 = 15 (PowerOfTwoMinusOne)
    // Fields: prfop=0, imm5=15, Zn=0, Pg=0
    let encoding: u32 = 0xC48FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_prfh_i_p_ai_d_field_imm5_16_poweroftwo_e000_c490e000() {
    // Encoding: 0xC490E000
    // Test PRFH_I.P.AI_D field imm5 = 16 (PowerOfTwo)
    // Fields: Pg=0, imm5=16, prfop=0, Zn=0
    let encoding: u32 = 0xC490E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field imm5 16 +: 5`
/// Requirement: FieldBoundary { field: "imm5", value: 31, boundary: Max }
/// maximum immediate (31)
#[test]
fn test_prfh_i_p_ai_d_field_imm5_31_max_e000_c49fe000() {
    // Encoding: 0xC49FE000
    // Test PRFH_I.P.AI_D field imm5 = 31 (Max)
    // Fields: Pg=0, Zn=0, prfop=0, imm5=31
    let encoding: u32 = 0xC49FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_prfh_i_p_ai_d_field_pg_0_min_e000_c480e000() {
    // Encoding: 0xC480E000
    // Test PRFH_I.P.AI_D field Pg = 0 (Min)
    // Fields: imm5=0, prfop=0, Zn=0, Pg=0
    let encoding: u32 = 0xC480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field Pg 10 +: 3`
/// Requirement: FieldBoundary { field: "Pg", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_prfh_i_p_ai_d_field_pg_1_poweroftwo_e000_c480e400() {
    // Encoding: 0xC480E400
    // Test PRFH_I.P.AI_D field Pg = 1 (PowerOfTwo)
    // Fields: imm5=0, Pg=1, Zn=0, prfop=0
    let encoding: u32 = 0xC480E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_prfh_i_p_ai_d_field_zn_0_min_e000_c480e000() {
    // Encoding: 0xC480E000
    // Test PRFH_I.P.AI_D field Zn = 0 (Min)
    // Fields: Zn=0, Pg=0, imm5=0, prfop=0
    let encoding: u32 = 0xC480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_prfh_i_p_ai_d_field_zn_1_poweroftwo_e000_c480e020() {
    // Encoding: 0xC480E020
    // Test PRFH_I.P.AI_D field Zn = 1 (PowerOfTwo)
    // Fields: Zn=1, prfop=0, Pg=0, imm5=0
    let encoding: u32 = 0xC480E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 30, boundary: PowerOfTwoMinusOne }
/// SIMD register V30
#[test]
fn test_prfh_i_p_ai_d_field_zn_30_poweroftwominusone_e000_c480e3c0() {
    // Encoding: 0xC480E3C0
    // Test PRFH_I.P.AI_D field Zn = 30 (PowerOfTwoMinusOne)
    // Fields: prfop=0, imm5=0, Zn=30, Pg=0
    let encoding: u32 = 0xC480E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field Zn 5 +: 5`
/// Requirement: FieldBoundary { field: "Zn", value: 31, boundary: Max }
/// SIMD register V31
#[test]
fn test_prfh_i_p_ai_d_field_zn_31_max_e000_c480e3e0() {
    // Encoding: 0xC480E3E0
    // Test PRFH_I.P.AI_D field Zn = 31 (Max)
    // Fields: imm5=0, Pg=0, Zn=31, prfop=0
    let encoding: u32 = 0xC480E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_prfh_i_p_ai_d_field_prfop_0_min_e000_c480e000() {
    // Encoding: 0xC480E000
    // Test PRFH_I.P.AI_D field prfop = 0 (Min)
    // Fields: imm5=0, Pg=0, Zn=0, prfop=0
    let encoding: u32 = 0xC480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_prfh_i_p_ai_d_field_prfop_1_poweroftwo_e000_c480e001() {
    // Encoding: 0xC480E001
    // Test PRFH_I.P.AI_D field prfop = 1 (PowerOfTwo)
    // Fields: prfop=1, Zn=0, imm5=0, Pg=0
    let encoding: u32 = 0xC480E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_prfh_i_p_ai_d_field_prfop_7_poweroftwominusone_e000_c480e007() {
    // Encoding: 0xC480E007
    // Test PRFH_I.P.AI_D field prfop = 7 (PowerOfTwoMinusOne)
    // Fields: Pg=0, prfop=7, imm5=0, Zn=0
    let encoding: u32 = 0xC480E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field prfop 0 +: 4`
/// Requirement: FieldBoundary { field: "prfop", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_prfh_i_p_ai_d_field_prfop_15_max_e000_c480e00f() {
    // Encoding: 0xC480E00F
    // Test PRFH_I.P.AI_D field prfop = 15 (Max)
    // Fields: Zn=0, imm5=0, Pg=0, prfop=15
    let encoding: u32 = 0xC480E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=0 (immediate value 0)
#[test]
fn test_prfh_i_p_ai_d_combo_0_e000_c480e000() {
    // Encoding: 0xC480E000
    // Test PRFH_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: imm5=0, Pg=0, Zn=0, prfop=0
    let encoding: u32 = 0xC480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=1 (immediate value 1)
#[test]
fn test_prfh_i_p_ai_d_combo_1_e000_c481e000() {
    // Encoding: 0xC481E000
    // Test PRFH_I.P.AI_D field combination: imm5=1, Pg=0, Zn=0, prfop=0
    // Fields: imm5=1, Pg=0, prfop=0, Zn=0
    let encoding: u32 = 0xC481E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=3 (2^2 - 1 = 3)
#[test]
fn test_prfh_i_p_ai_d_combo_2_e000_c483e000() {
    // Encoding: 0xC483E000
    // Test PRFH_I.P.AI_D field combination: imm5=3, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, Zn=0, prfop=0, imm5=3
    let encoding: u32 = 0xC483E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=4 (power of 2 (2^2 = 4))
#[test]
fn test_prfh_i_p_ai_d_combo_3_e000_c484e000() {
    // Encoding: 0xC484E000
    // Test PRFH_I.P.AI_D field combination: imm5=4, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, imm5=4, prfop=0, Zn=0
    let encoding: u32 = 0xC484E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=7 (2^3 - 1 = 7)
#[test]
fn test_prfh_i_p_ai_d_combo_4_e000_c487e000() {
    // Encoding: 0xC487E000
    // Test PRFH_I.P.AI_D field combination: imm5=7, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, imm5=7, Zn=0, prfop=0
    let encoding: u32 = 0xC487E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=8 (power of 2 (2^3 = 8))
#[test]
fn test_prfh_i_p_ai_d_combo_5_e000_c488e000() {
    // Encoding: 0xC488E000
    // Test PRFH_I.P.AI_D field combination: imm5=8, Pg=0, Zn=0, prfop=0
    // Fields: imm5=8, Pg=0, Zn=0, prfop=0
    let encoding: u32 = 0xC488E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=15 (immediate midpoint (15))
#[test]
fn test_prfh_i_p_ai_d_combo_6_e000_c48fe000() {
    // Encoding: 0xC48FE000
    // Test PRFH_I.P.AI_D field combination: imm5=15, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, imm5=15, Zn=0, Pg=0
    let encoding: u32 = 0xC48FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=16 (power of 2 (2^4 = 16))
#[test]
fn test_prfh_i_p_ai_d_combo_7_e000_c490e000() {
    // Encoding: 0xC490E000
    // Test PRFH_I.P.AI_D field combination: imm5=16, Pg=0, Zn=0, prfop=0
    // Fields: Pg=0, prfop=0, imm5=16, Zn=0
    let encoding: u32 = 0xC490E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm5=31 (maximum immediate (31))
#[test]
fn test_prfh_i_p_ai_d_combo_8_e000_c49fe000() {
    // Encoding: 0xC49FE000
    // Test PRFH_I.P.AI_D field combination: imm5=31, Pg=0, Zn=0, prfop=0
    // Fields: prfop=0, Pg=0, Zn=0, imm5=31
    let encoding: u32 = 0xC49FE000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=0 (register index 0 (first register))
#[test]
fn test_prfh_i_p_ai_d_combo_9_e000_c480e000() {
    // Encoding: 0xC480E000
    // Test PRFH_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, Pg=0, prfop=0, imm5=0
    let encoding: u32 = 0xC480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Pg=1 (register index 1 (second register))
#[test]
fn test_prfh_i_p_ai_d_combo_10_e000_c480e400() {
    // Encoding: 0xC480E400
    // Test PRFH_I.P.AI_D field combination: imm5=0, Pg=1, Zn=0, prfop=0
    // Fields: prfop=0, imm5=0, Pg=1, Zn=0
    let encoding: u32 = 0xC480E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=0 (SIMD register V0)
#[test]
fn test_prfh_i_p_ai_d_combo_11_e000_c480e000() {
    // Encoding: 0xC480E000
    // Test PRFH_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: imm5=0, Zn=0, prfop=0, Pg=0
    let encoding: u32 = 0xC480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=1 (SIMD register V1)
#[test]
fn test_prfh_i_p_ai_d_combo_12_e000_c480e020() {
    // Encoding: 0xC480E020
    // Test PRFH_I.P.AI_D field combination: imm5=0, Pg=0, Zn=1, prfop=0
    // Fields: imm5=0, Pg=0, Zn=1, prfop=0
    let encoding: u32 = 0xC480E020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=30 (SIMD register V30)
#[test]
fn test_prfh_i_p_ai_d_combo_13_e000_c480e3c0() {
    // Encoding: 0xC480E3C0
    // Test PRFH_I.P.AI_D field combination: imm5=0, Pg=0, Zn=30, prfop=0
    // Fields: Zn=30, prfop=0, imm5=0, Pg=0
    let encoding: u32 = 0xC480E3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Zn=31 (SIMD register V31)
#[test]
fn test_prfh_i_p_ai_d_combo_14_e000_c480e3e0() {
    // Encoding: 0xC480E3E0
    // Test PRFH_I.P.AI_D field combination: imm5=0, Pg=0, Zn=31, prfop=0
    // Fields: prfop=0, Zn=31, Pg=0, imm5=0
    let encoding: u32 = 0xC480E3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=0 (minimum value)
#[test]
fn test_prfh_i_p_ai_d_combo_15_e000_c480e000() {
    // Encoding: 0xC480E000
    // Test PRFH_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=0
    // Fields: Zn=0, prfop=0, imm5=0, Pg=0
    let encoding: u32 = 0xC480E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=1 (value 1)
#[test]
fn test_prfh_i_p_ai_d_combo_16_e000_c480e001() {
    // Encoding: 0xC480E001
    // Test PRFH_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=1
    // Fields: imm5=0, Pg=0, prfop=1, Zn=0
    let encoding: u32 = 0xC480E001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=7 (midpoint (7))
#[test]
fn test_prfh_i_p_ai_d_combo_17_e000_c480e007() {
    // Encoding: 0xC480E007
    // Test PRFH_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=7
    // Fields: imm5=0, Pg=0, prfop=7, Zn=0
    let encoding: u32 = 0xC480E007;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: PRFH_I.P.AI_D
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// prfop=15 (maximum value (15))
#[test]
fn test_prfh_i_p_ai_d_combo_18_e000_c480e00f() {
    // Encoding: 0xC480E00F
    // Test PRFH_I.P.AI_D field combination: imm5=0, Pg=0, Zn=0, prfop=15
    // Fields: Zn=0, imm5=0, prfop=15, Pg=0
    let encoding: u32 = 0xC480E00F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}
