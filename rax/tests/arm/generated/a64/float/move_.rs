//! A64 float move_ tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_float_move_fp_select Tests
// ============================================================================

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_move_fp_select_field_type1_0_min_c00_1e200c00() {
    // Encoding: 0x1E200C00
    // Test aarch64_float_move_fp_select field type1 = 0 (Min)
    // Fields: Rm=0, Rd=0, cond=0, Rn=0, type1=0
    let encoding: u32 = 0x1E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_float_move_fp_select_field_type1_1_poweroftwo_c00_1e600c00() {
    // Encoding: 0x1E600C00
    // Test aarch64_float_move_fp_select field type1 = 1 (PowerOfTwo)
    // Fields: Rm=0, type1=1, cond=0, Rd=0, Rn=0
    let encoding: u32 = 0x1E600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	d0, d0, d0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch64_float_move_fp_select_field_type1_3_max_c00_1ee00c00() {
    // Encoding: 0x1EE00C00
    // Test aarch64_float_move_fp_select field type1 = 3 (Max)
    // Fields: Rn=0, Rm=0, Rd=0, cond=0, type1=3
    let encoding: u32 = 0x1EE00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	h0, h0, h0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_move_fp_select_field_rm_0_min_c00_1e200c00() {
    // Encoding: 0x1E200C00
    // Test aarch64_float_move_fp_select field Rm = 0 (Min)
    // Fields: Rd=0, Rm=0, type1=0, Rn=0, cond=0
    let encoding: u32 = 0x1E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_move_fp_select_field_rm_1_poweroftwo_c00_1e210c00() {
    // Encoding: 0x1E210C00
    // Test aarch64_float_move_fp_select field Rm = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, cond=0, Rm=1, type1=0
    let encoding: u32 = 0x1E210C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s1, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_move_fp_select_field_rm_30_poweroftwominusone_c00_1e3e0c00() {
    // Encoding: 0x1E3E0C00
    // Test aarch64_float_move_fp_select field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rm=30, Rd=0, cond=0, type1=0
    let encoding: u32 = 0x1E3E0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s30, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_float_move_fp_select_field_rm_31_max_c00_1e3f0c00() {
    // Encoding: 0x1E3F0C00
    // Test aarch64_float_move_fp_select field Rm = 31 (Max)
    // Fields: type1=0, Rm=31, cond=0, Rn=0, Rd=0
    let encoding: u32 = 0x1E3F0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s31, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_0_min_c00_1e200c00() {
    // Encoding: 0x1E200C00
    // Test aarch64_float_move_fp_select field cond = 0 (Min)
    // Fields: cond=0, Rm=0, type1=0, Rd=0, Rn=0
    let encoding: u32 = 0x1E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_1_poweroftwo_c00_1e201c00() {
    // Encoding: 0x1E201C00
    // Test aarch64_float_move_fp_select field cond = 1 (PowerOfTwo)
    // Fields: Rm=0, type1=0, cond=1, Rn=0, Rd=0
    let encoding: u32 = 0x1E201C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, ne
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_2_poweroftwo_c00_1e202c00() {
    // Encoding: 0x1E202C00
    // Test aarch64_float_move_fp_select field cond = 2 (PowerOfTwo)
    // Fields: Rd=0, type1=0, Rm=0, cond=2, Rn=0
    let encoding: u32 = 0x1E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, hs
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_3_poweroftwo_c00_1e203c00() {
    // Encoding: 0x1E203C00
    // Test aarch64_float_move_fp_select field cond = 3 (PowerOfTwo)
    // Fields: Rm=0, type1=0, Rd=0, Rn=0, cond=3
    let encoding: u32 = 0x1E203C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, lo
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_4_poweroftwo_c00_1e204c00() {
    // Encoding: 0x1E204C00
    // Test aarch64_float_move_fp_select field cond = 4 (PowerOfTwo)
    // Fields: type1=0, Rm=0, Rn=0, Rd=0, cond=4
    let encoding: u32 = 0x1E204C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, mi
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_5_poweroftwo_c00_1e205c00() {
    // Encoding: 0x1E205C00
    // Test aarch64_float_move_fp_select field cond = 5 (PowerOfTwo)
    // Fields: Rd=0, Rm=0, Rn=0, type1=0, cond=5
    let encoding: u32 = 0x1E205C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, pl
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_6_poweroftwo_c00_1e206c00() {
    // Encoding: 0x1E206C00
    // Test aarch64_float_move_fp_select field cond = 6 (PowerOfTwo)
    // Fields: Rm=0, Rd=0, cond=6, type1=0, Rn=0
    let encoding: u32 = 0x1E206C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, vs
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_7_poweroftwo_c00_1e207c00() {
    // Encoding: 0x1E207C00
    // Test aarch64_float_move_fp_select field cond = 7 (PowerOfTwo)
    // Fields: type1=0, cond=7, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x1E207C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, vc
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_8_poweroftwo_c00_1e208c00() {
    // Encoding: 0x1E208C00
    // Test aarch64_float_move_fp_select field cond = 8 (PowerOfTwo)
    // Fields: cond=8, Rm=0, Rd=0, type1=0, Rn=0
    let encoding: u32 = 0x1E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, hi
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_9_poweroftwo_c00_1e209c00() {
    // Encoding: 0x1E209C00
    // Test aarch64_float_move_fp_select field cond = 9 (PowerOfTwo)
    // Fields: type1=0, Rd=0, Rm=0, Rn=0, cond=9
    let encoding: u32 = 0x1E209C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, ls
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_10_poweroftwo_c00_1e20ac00() {
    // Encoding: 0x1E20AC00
    // Test aarch64_float_move_fp_select field cond = 10 (PowerOfTwo)
    // Fields: cond=10, Rd=0, Rn=0, Rm=0, type1=0
    let encoding: u32 = 0x1E20AC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, ge
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_11_poweroftwo_c00_1e20bc00() {
    // Encoding: 0x1E20BC00
    // Test aarch64_float_move_fp_select field cond = 11 (PowerOfTwo)
    // Fields: cond=11, Rd=0, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0x1E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, lt
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_12_poweroftwo_c00_1e20cc00() {
    // Encoding: 0x1E20CC00
    // Test aarch64_float_move_fp_select field cond = 12 (PowerOfTwo)
    // Fields: Rd=0, Rm=0, type1=0, cond=12, Rn=0
    let encoding: u32 = 0x1E20CC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, gt
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_13_poweroftwo_c00_1e20dc00() {
    // Encoding: 0x1E20DC00
    // Test aarch64_float_move_fp_select field cond = 13 (PowerOfTwo)
    // Fields: Rm=0, Rd=0, type1=0, Rn=0, cond=13
    let encoding: u32 = 0x1E20DC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, le
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_14_poweroftwo_c00_1e20ec00() {
    // Encoding: 0x1E20EC00
    // Test aarch64_float_move_fp_select field cond = 14 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, type1=0, Rm=0, cond=14
    let encoding: u32 = 0x1E20EC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, al
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond 12 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch64_float_move_fp_select_field_cond_15_max_c00_1e20fc00() {
    // Encoding: 0x1E20FC00
    // Test aarch64_float_move_fp_select field cond = 15 (Max)
    // Fields: Rn=0, cond=15, Rd=0, type1=0, Rm=0
    let encoding: u32 = 0x1E20FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, nv
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_move_fp_select_field_rn_0_min_c00_1e200c00() {
    // Encoding: 0x1E200C00
    // Test aarch64_float_move_fp_select field Rn = 0 (Min)
    // Fields: type1=0, Rd=0, cond=0, Rm=0, Rn=0
    let encoding: u32 = 0x1E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_move_fp_select_field_rn_1_poweroftwo_c00_1e200c20() {
    // Encoding: 0x1E200C20
    // Test aarch64_float_move_fp_select field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Rm=0, cond=0, Rn=1, type1=0
    let encoding: u32 = 0x1E200C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s1, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_move_fp_select_field_rn_30_poweroftwominusone_c00_1e200fc0() {
    // Encoding: 0x1E200FC0
    // Test aarch64_float_move_fp_select field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, type1=0, cond=0, Rn=30, Rd=0
    let encoding: u32 = 0x1E200FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s30, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_float_move_fp_select_field_rn_31_max_c00_1e200fe0() {
    // Encoding: 0x1E200FE0
    // Test aarch64_float_move_fp_select field Rn = 31 (Max)
    // Fields: Rm=0, cond=0, Rn=31, Rd=0, type1=0
    let encoding: u32 = 0x1E200FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s31, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_move_fp_select_field_rd_0_min_c00_1e200c00() {
    // Encoding: 0x1E200C00
    // Test aarch64_float_move_fp_select field Rd = 0 (Min)
    // Fields: type1=0, cond=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x1E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_move_fp_select_field_rd_1_poweroftwo_c00_1e200c01() {
    // Encoding: 0x1E200C01
    // Test aarch64_float_move_fp_select field Rd = 1 (PowerOfTwo)
    // Fields: Rm=0, cond=0, type1=0, Rd=1, Rn=0
    let encoding: u32 = 0x1E200C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s1, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_move_fp_select_field_rd_30_poweroftwominusone_c00_1e200c1e() {
    // Encoding: 0x1E200C1E
    // Test aarch64_float_move_fp_select field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, cond=0, type1=0, Rm=0, Rd=30
    let encoding: u32 = 0x1E200C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s30, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_float_move_fp_select_field_rd_31_max_c00_1e200c1f() {
    // Encoding: 0x1E200C1F
    // Test aarch64_float_move_fp_select field Rd = 31 (Max)
    // Fields: Rd=31, type1=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x1E200C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s31, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=0 (minimum value)
#[test]
fn test_aarch64_float_move_fp_select_combo_0_c00_1e200c00() {
    // Encoding: 0x1E200C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=0, Rn=0, Rd=0
    // Fields: cond=0, Rm=0, type1=0, Rn=0, Rd=0
    let encoding: u32 = 0x1E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=1 (value 1)
#[test]
fn test_aarch64_float_move_fp_select_combo_1_c00_1e600c00() {
    // Encoding: 0x1E600C00
    // Test aarch64_float_move_fp_select field combination: type1=1, Rm=0, cond=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, type1=1, cond=0, Rn=0
    let encoding: u32 = 0x1E600C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	d0, d0, d0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=3 (maximum value (3))
#[test]
fn test_aarch64_float_move_fp_select_combo_2_c00_1ee00c00() {
    // Encoding: 0x1EE00C00
    // Test aarch64_float_move_fp_select field combination: type1=3, Rm=0, cond=0, Rn=0, Rd=0
    // Fields: Rm=0, cond=0, Rd=0, type1=3, Rn=0
    let encoding: u32 = 0x1EE00C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	h0, h0, h0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_move_fp_select_combo_3_c00_1e200c00() {
    // Encoding: 0x1E200C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=0, Rn=0, Rd=0
    // Fields: Rd=0, type1=0, Rm=0, cond=0, Rn=0
    let encoding: u32 = 0x1E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_move_fp_select_combo_4_c00_1e210c00() {
    // Encoding: 0x1E210C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=1, cond=0, Rn=0, Rd=0
    // Fields: type1=0, cond=0, Rn=0, Rm=1, Rd=0
    let encoding: u32 = 0x1E210C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s1, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_move_fp_select_combo_5_c00_1e3e0c00() {
    // Encoding: 0x1E3E0C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=30, cond=0, Rn=0, Rd=0
    // Fields: cond=0, Rm=30, Rd=0, type1=0, Rn=0
    let encoding: u32 = 0x1E3E0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s30, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_float_move_fp_select_combo_6_c00_1e3f0c00() {
    // Encoding: 0x1E3F0C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=31, cond=0, Rn=0, Rd=0
    // Fields: Rn=0, Rm=31, Rd=0, cond=0, type1=0
    let encoding: u32 = 0x1E3F0C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s31, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch64_float_move_fp_select_combo_7_c00_1e200c00() {
    // Encoding: 0x1E200C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=0, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, Rn=0, type1=0, cond=0
    let encoding: u32 = 0x1E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=1 (condition NE (not equal))
#[test]
fn test_aarch64_float_move_fp_select_combo_8_c00_1e201c00() {
    // Encoding: 0x1E201C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=1, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, cond=1, type1=0, Rn=0
    let encoding: u32 = 0x1E201C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, ne
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=2 (condition CS/HS (carry set))
#[test]
fn test_aarch64_float_move_fp_select_combo_9_c00_1e202c00() {
    // Encoding: 0x1E202C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=2, Rn=0, Rd=0
    // Fields: cond=2, type1=0, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x1E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, hs
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=3 (condition CC/LO (carry clear))
#[test]
fn test_aarch64_float_move_fp_select_combo_10_c00_1e203c00() {
    // Encoding: 0x1E203C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=3, Rn=0, Rd=0
    // Fields: type1=0, Rm=0, cond=3, Rn=0, Rd=0
    let encoding: u32 = 0x1E203C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, lo
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=4 (condition MI (minus/negative))
#[test]
fn test_aarch64_float_move_fp_select_combo_11_c00_1e204c00() {
    // Encoding: 0x1E204C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=4, Rn=0, Rd=0
    // Fields: type1=0, Rn=0, cond=4, Rd=0, Rm=0
    let encoding: u32 = 0x1E204C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, mi
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=5 (condition PL (plus/positive))
#[test]
fn test_aarch64_float_move_fp_select_combo_12_c00_1e205c00() {
    // Encoding: 0x1E205C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=5, Rn=0, Rd=0
    // Fields: type1=0, cond=5, Rm=0, Rd=0, Rn=0
    let encoding: u32 = 0x1E205C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, pl
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=6 (condition VS (overflow set))
#[test]
fn test_aarch64_float_move_fp_select_combo_13_c00_1e206c00() {
    // Encoding: 0x1E206C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=6, Rn=0, Rd=0
    // Fields: Rm=0, Rd=0, type1=0, cond=6, Rn=0
    let encoding: u32 = 0x1E206C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, vs
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=7 (condition VC (overflow clear))
#[test]
fn test_aarch64_float_move_fp_select_combo_14_c00_1e207c00() {
    // Encoding: 0x1E207C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=7, Rn=0, Rd=0
    // Fields: Rn=0, type1=0, Rm=0, cond=7, Rd=0
    let encoding: u32 = 0x1E207C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, vc
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=8 (condition HI (unsigned higher))
#[test]
fn test_aarch64_float_move_fp_select_combo_15_c00_1e208c00() {
    // Encoding: 0x1E208C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=8, Rn=0, Rd=0
    // Fields: Rd=0, Rm=0, type1=0, cond=8, Rn=0
    let encoding: u32 = 0x1E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, hi
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=9 (condition LS (unsigned lower or same))
#[test]
fn test_aarch64_float_move_fp_select_combo_16_c00_1e209c00() {
    // Encoding: 0x1E209C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=9, Rn=0, Rd=0
    // Fields: cond=9, Rd=0, Rm=0, type1=0, Rn=0
    let encoding: u32 = 0x1E209C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, ls
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=10 (condition GE (signed >=))
#[test]
fn test_aarch64_float_move_fp_select_combo_17_c00_1e20ac00() {
    // Encoding: 0x1E20AC00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=10, Rn=0, Rd=0
    // Fields: Rd=0, type1=0, cond=10, Rm=0, Rn=0
    let encoding: u32 = 0x1E20AC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, ge
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=11 (condition LT (signed <))
#[test]
fn test_aarch64_float_move_fp_select_combo_18_c00_1e20bc00() {
    // Encoding: 0x1E20BC00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=11, Rn=0, Rd=0
    // Fields: Rm=0, type1=0, cond=11, Rn=0, Rd=0
    let encoding: u32 = 0x1E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, lt
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=12 (condition GT (signed >))
#[test]
fn test_aarch64_float_move_fp_select_combo_19_c00_1e20cc00() {
    // Encoding: 0x1E20CC00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=12, Rn=0, Rd=0
    // Fields: Rd=0, type1=0, Rm=0, cond=12, Rn=0
    let encoding: u32 = 0x1E20CC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, gt
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=13 (condition LE (signed <=))
#[test]
fn test_aarch64_float_move_fp_select_combo_20_c00_1e20dc00() {
    // Encoding: 0x1E20DC00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=13, Rn=0, Rd=0
    // Fields: type1=0, cond=13, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x1E20DC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, le
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=14 (condition AL (always))
#[test]
fn test_aarch64_float_move_fp_select_combo_21_c00_1e20ec00() {
    // Encoding: 0x1E20EC00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=14, Rn=0, Rd=0
    // Fields: type1=0, cond=14, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x1E20EC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, al
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=15 (condition NV (never, reserved))
#[test]
fn test_aarch64_float_move_fp_select_combo_22_c00_1e20fc00() {
    // Encoding: 0x1E20FC00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=15, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Rm=0, cond=15, type1=0
    let encoding: u32 = 0x1E20FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, nv
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_move_fp_select_combo_23_c00_1e200c00() {
    // Encoding: 0x1E200C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=0, Rn=0, Rd=0
    // Fields: Rm=0, Rn=0, Rd=0, type1=0, cond=0
    let encoding: u32 = 0x1E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_move_fp_select_combo_24_c00_1e200c20() {
    // Encoding: 0x1E200C20
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=0, Rn=1, Rd=0
    // Fields: Rm=0, Rn=1, cond=0, type1=0, Rd=0
    let encoding: u32 = 0x1E200C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s1, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_move_fp_select_combo_25_c00_1e200fc0() {
    // Encoding: 0x1E200FC0
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=0, Rn=30, Rd=0
    // Fields: Rd=0, Rm=0, Rn=30, cond=0, type1=0
    let encoding: u32 = 0x1E200FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s30, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_float_move_fp_select_combo_26_c00_1e200fe0() {
    // Encoding: 0x1E200FE0
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=0, Rn=31, Rd=0
    // Fields: Rn=31, Rd=0, type1=0, Rm=0, cond=0
    let encoding: u32 = 0x1E200FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s31, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_move_fp_select_combo_27_c00_1e200c00() {
    // Encoding: 0x1E200C00
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=0, Rn=0, Rd=0
    // Fields: Rd=0, cond=0, Rm=0, type1=0, Rn=0
    let encoding: u32 = 0x1E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_move_fp_select_combo_28_c00_1e200c01() {
    // Encoding: 0x1E200C01
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=0, Rn=0, Rd=1
    // Fields: cond=0, Rd=1, Rm=0, Rn=0, type1=0
    let encoding: u32 = 0x1E200C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s1, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_move_fp_select_combo_29_c00_1e200c1e() {
    // Encoding: 0x1E200C1E
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=0, Rn=0, Rd=30
    // Fields: Rd=30, Rn=0, type1=0, Rm=0, cond=0
    let encoding: u32 = 0x1E200C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s30, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_float_move_fp_select_combo_30_c00_1e200c1f() {
    // Encoding: 0x1E200C1F
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=0, Rn=0, Rd=31
    // Fields: Rn=0, cond=0, Rd=31, Rm=0, type1=0
    let encoding: u32 = 0x1E200C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s31, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_float_move_fp_select_combo_31_c00_1e210c20() {
    // Encoding: 0x1E210C20
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=1, cond=0, Rn=1, Rd=0
    // Fields: type1=0, Rn=1, Rm=1, cond=0, Rd=0
    let encoding: u32 = 0x1E210C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s1, s1, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_float_move_fp_select_combo_32_c00_1e3f0fe0() {
    // Encoding: 0x1E3F0FE0
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=31, cond=0, Rn=31, Rd=0
    // Fields: Rn=31, cond=0, Rm=31, Rd=0, type1=0
    let encoding: u32 = 0x1E3F0FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s31, s31, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_float_move_fp_select_combo_33_c00_1e210c01() {
    // Encoding: 0x1E210C01
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=1, cond=0, Rn=0, Rd=1
    // Fields: type1=0, cond=0, Rm=1, Rn=0, Rd=1
    let encoding: u32 = 0x1E210C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s1, s0, s1, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_float_move_fp_select_combo_34_c00_1e3f0c1f() {
    // Encoding: 0x1E3F0C1F
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=31, cond=0, Rn=0, Rd=31
    // Fields: cond=0, Rn=0, Rm=31, type1=0, Rd=31
    let encoding: u32 = 0x1E3F0C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s31, s0, s31, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_float_move_fp_select_combo_35_c00_1e200c21() {
    // Encoding: 0x1E200C21
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=0, Rn=1, Rd=1
    // Fields: Rm=0, Rn=1, Rd=1, type1=0, cond=0
    let encoding: u32 = 0x1E200C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s1, s1, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_float_move_fp_select_combo_36_c00_1e200fff() {
    // Encoding: 0x1E200FFF
    // Test aarch64_float_move_fp_select field combination: type1=0, Rm=0, cond=0, Rn=31, Rd=31
    // Fields: type1=0, Rm=0, cond=0, Rd=31, Rn=31
    let encoding: u32 = 0x1E200FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s31, s31, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch64_float_move_fp_select_special_cond_0_condition_eq_3072_1e200c00() {
    // Encoding: 0x1E200C00
    // Test aarch64_float_move_fp_select special value cond = 0 (Condition EQ)
    // Fields: Rd=0, cond=0, Rm=0, Rn=0, type1=0
    let encoding: u32 = 0x1E200C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch64_float_move_fp_select_special_cond_1_condition_ne_3072_1e201c00() {
    // Encoding: 0x1E201C00
    // Test aarch64_float_move_fp_select special value cond = 1 (Condition NE)
    // Fields: Rn=0, Rm=0, cond=1, type1=0, Rd=0
    let encoding: u32 = 0x1E201C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, ne
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch64_float_move_fp_select_special_cond_2_condition_cs_hs_3072_1e202c00() {
    // Encoding: 0x1E202C00
    // Test aarch64_float_move_fp_select special value cond = 2 (Condition CS/HS)
    // Fields: type1=0, Rd=0, Rn=0, cond=2, Rm=0
    let encoding: u32 = 0x1E202C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, hs
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch64_float_move_fp_select_special_cond_3_condition_cc_lo_3072_1e203c00() {
    // Encoding: 0x1E203C00
    // Test aarch64_float_move_fp_select special value cond = 3 (Condition CC/LO)
    // Fields: Rm=0, cond=3, Rn=0, Rd=0, type1=0
    let encoding: u32 = 0x1E203C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, lo
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch64_float_move_fp_select_special_cond_4_condition_mi_3072_1e204c00() {
    // Encoding: 0x1E204C00
    // Test aarch64_float_move_fp_select special value cond = 4 (Condition MI)
    // Fields: type1=0, Rm=0, Rd=0, cond=4, Rn=0
    let encoding: u32 = 0x1E204C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, mi
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch64_float_move_fp_select_special_cond_5_condition_pl_3072_1e205c00() {
    // Encoding: 0x1E205C00
    // Test aarch64_float_move_fp_select special value cond = 5 (Condition PL)
    // Fields: type1=0, cond=5, Rn=0, Rm=0, Rd=0
    let encoding: u32 = 0x1E205C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, pl
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch64_float_move_fp_select_special_cond_6_condition_vs_3072_1e206c00() {
    // Encoding: 0x1E206C00
    // Test aarch64_float_move_fp_select special value cond = 6 (Condition VS)
    // Fields: Rn=0, Rm=0, cond=6, type1=0, Rd=0
    let encoding: u32 = 0x1E206C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, vs
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch64_float_move_fp_select_special_cond_7_condition_vc_3072_1e207c00() {
    // Encoding: 0x1E207C00
    // Test aarch64_float_move_fp_select special value cond = 7 (Condition VC)
    // Fields: type1=0, Rm=0, Rd=0, cond=7, Rn=0
    let encoding: u32 = 0x1E207C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, vc
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch64_float_move_fp_select_special_cond_8_condition_hi_3072_1e208c00() {
    // Encoding: 0x1E208C00
    // Test aarch64_float_move_fp_select special value cond = 8 (Condition HI)
    // Fields: type1=0, cond=8, Rn=0, Rd=0, Rm=0
    let encoding: u32 = 0x1E208C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, hi
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch64_float_move_fp_select_special_cond_9_condition_ls_3072_1e209c00() {
    // Encoding: 0x1E209C00
    // Test aarch64_float_move_fp_select special value cond = 9 (Condition LS)
    // Fields: type1=0, Rn=0, Rd=0, cond=9, Rm=0
    let encoding: u32 = 0x1E209C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, ls
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch64_float_move_fp_select_special_cond_10_condition_ge_3072_1e20ac00() {
    // Encoding: 0x1E20AC00
    // Test aarch64_float_move_fp_select special value cond = 10 (Condition GE)
    // Fields: cond=10, type1=0, Rd=0, Rm=0, Rn=0
    let encoding: u32 = 0x1E20AC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, ge
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch64_float_move_fp_select_special_cond_11_condition_lt_3072_1e20bc00() {
    // Encoding: 0x1E20BC00
    // Test aarch64_float_move_fp_select special value cond = 11 (Condition LT)
    // Fields: Rn=0, Rd=0, type1=0, cond=11, Rm=0
    let encoding: u32 = 0x1E20BC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, lt
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch64_float_move_fp_select_special_cond_12_condition_gt_3072_1e20cc00() {
    // Encoding: 0x1E20CC00
    // Test aarch64_float_move_fp_select special value cond = 12 (Condition GT)
    // Fields: Rm=0, type1=0, cond=12, Rn=0, Rd=0
    let encoding: u32 = 0x1E20CC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, gt
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch64_float_move_fp_select_special_cond_13_condition_le_3072_1e20dc00() {
    // Encoding: 0x1E20DC00
    // Test aarch64_float_move_fp_select special value cond = 13 (Condition LE)
    // Fields: cond=13, Rd=0, type1=0, Rm=0, Rn=0
    let encoding: u32 = 0x1E20DC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, le
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch64_float_move_fp_select_special_cond_14_condition_al_3072_1e20ec00() {
    // Encoding: 0x1E20EC00
    // Test aarch64_float_move_fp_select special value cond = 14 (Condition AL)
    // Fields: Rm=0, Rd=0, type1=0, Rn=0, cond=14
    let encoding: u32 = 0x1E20EC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, al
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch64_float_move_fp_select_special_cond_15_condition_nv_3072_1e20fc00() {
    // Encoding: 0x1E20FC00
    // Test aarch64_float_move_fp_select special value cond = 15 (Condition NV)
    // Fields: Rm=0, Rn=0, Rd=0, cond=15, type1=0
    let encoding: u32 = 0x1E20FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s0, s0, nv
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_float_move_fp_select_special_rn_31_stack_pointer_sp_may_require_alignment_3072_1e200fe0()
 {
    // Encoding: 0x1E200FE0
    // Test aarch64_float_move_fp_select special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rm=0, type1=0, Rn=31, cond=0, Rd=0
    let encoding: u32 = 0x1E200FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s0, s31, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_select
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_float_move_fp_select_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_3072_1e200c1f()
 {
    // Encoding: 0x1E200C1F
    // Test aarch64_float_move_fp_select special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rm=0, type1=0, Rd=31, Rn=0, cond=0
    let encoding: u32 = 0x1E200C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcsel	s31, s0, s0, eq
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// aarch64_float_move_fp_imm Tests
// ============================================================================

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_move_fp_imm_field_type1_0_min_1000_1e201000() {
    // Encoding: 0x1E201000
    // Test aarch64_float_move_fp_imm field type1 = 0 (Min)
    // Fields: type1=0, Rd=0, imm8=0
    let encoding: u32 = 0x1E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_float_move_fp_imm_field_type1_1_poweroftwo_1000_1e601000() {
    // Encoding: 0x1E601000
    // Test aarch64_float_move_fp_imm field type1 = 1 (PowerOfTwo)
    // Fields: type1=1, imm8=0, Rd=0
    let encoding: u32 = 0x1E601000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	d0, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch64_float_move_fp_imm_field_type1_3_max_1000_1ee01000() {
    // Encoding: 0x1EE01000
    // Test aarch64_float_move_fp_imm field type1 = 3 (Max)
    // Fields: Rd=0, imm8=0, type1=3
    let encoding: u32 = 0x1EE01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	h0, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_0_zero_1000_1e201000() {
    // Encoding: 0x1E201000
    // Test aarch64_float_move_fp_imm field imm8 = 0 (Zero)
    // Fields: Rd=0, imm8=0, type1=0
    let encoding: u32 = 0x1E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_1_poweroftwo_1000_1e203000() {
    // Encoding: 0x1E203000
    // Test aarch64_float_move_fp_imm field imm8 = 1 (PowerOfTwo)
    // Fields: type1=0, imm8=1, Rd=0
    let encoding: u32 = 0x1E203000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.12500000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_3_poweroftwominusone_1000_1e207000() {
    // Encoding: 0x1E207000
    // Test aarch64_float_move_fp_imm field imm8 = 3 (PowerOfTwoMinusOne)
    // Fields: Rd=0, imm8=3, type1=0
    let encoding: u32 = 0x1E207000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.37500000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_4_poweroftwo_1000_1e209000() {
    // Encoding: 0x1E209000
    // Test aarch64_float_move_fp_imm field imm8 = 4 (PowerOfTwo)
    // Fields: Rd=0, imm8=4, type1=0
    let encoding: u32 = 0x1E209000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.50000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_7_poweroftwominusone_1000_1e20f000() {
    // Encoding: 0x1E20F000
    // Test aarch64_float_move_fp_imm field imm8 = 7 (PowerOfTwoMinusOne)
    // Fields: Rd=0, imm8=7, type1=0
    let encoding: u32 = 0x1E20F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.87500000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_8_poweroftwo_1000_1e211000() {
    // Encoding: 0x1E211000
    // Test aarch64_float_move_fp_imm field imm8 = 8 (PowerOfTwo)
    // Fields: type1=0, Rd=0, imm8=8
    let encoding: u32 = 0x1E211000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #3.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_15_poweroftwominusone_1000_1e21f000() {
    // Encoding: 0x1E21F000
    // Test aarch64_float_move_fp_imm field imm8 = 15 (PowerOfTwoMinusOne)
    // Fields: type1=0, Rd=0, imm8=15
    let encoding: u32 = 0x1E21F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #3.87500000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_16_poweroftwo_1000_1e221000() {
    // Encoding: 0x1E221000
    // Test aarch64_float_move_fp_imm field imm8 = 16 (PowerOfTwo)
    // Fields: type1=0, Rd=0, imm8=16
    let encoding: u32 = 0x1E221000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #4.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_31_poweroftwominusone_1000_1e23f000() {
    // Encoding: 0x1E23F000
    // Test aarch64_float_move_fp_imm field imm8 = 31 (PowerOfTwoMinusOne)
    // Fields: type1=0, Rd=0, imm8=31
    let encoding: u32 = 0x1E23F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #7.75000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_32_poweroftwo_1000_1e241000() {
    // Encoding: 0x1E241000
    // Test aarch64_float_move_fp_imm field imm8 = 32 (PowerOfTwo)
    // Fields: imm8=32, Rd=0, type1=0
    let encoding: u32 = 0x1E241000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #8.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 63, boundary: PowerOfTwoMinusOne }
/// 2^6 - 1 = 63
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_63_poweroftwominusone_1000_1e27f000() {
    // Encoding: 0x1E27F000
    // Test aarch64_float_move_fp_imm field imm8 = 63 (PowerOfTwoMinusOne)
    // Fields: type1=0, imm8=63, Rd=0
    let encoding: u32 = 0x1E27F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #31.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_64_poweroftwo_1000_1e281000() {
    // Encoding: 0x1E281000
    // Test aarch64_float_move_fp_imm field imm8 = 64 (PowerOfTwo)
    // Fields: type1=0, imm8=64, Rd=0
    let encoding: u32 = 0x1E281000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #0.12500000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 127, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (127)
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_127_poweroftwominusone_1000_1e2ff000() {
    // Encoding: 0x1E2FF000
    // Test aarch64_float_move_fp_imm field imm8 = 127 (PowerOfTwoMinusOne)
    // Fields: type1=0, imm8=127, Rd=0
    let encoding: u32 = 0x1E2FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #1.93750000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 128, boundary: PowerOfTwo }
/// power of 2 (2^7 = 128)
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_128_poweroftwo_1000_1e301000() {
    // Encoding: 0x1E301000
    // Test aarch64_float_move_fp_imm field imm8 = 128 (PowerOfTwo)
    // Fields: type1=0, Rd=0, imm8=128
    let encoding: u32 = 0x1E301000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #-2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field imm8 13 +: 8`
/// Requirement: FieldBoundary { field: "imm8", value: 255, boundary: Max }
/// maximum immediate (255)
#[test]
fn test_aarch64_float_move_fp_imm_field_imm8_255_max_1000_1e3ff000() {
    // Encoding: 0x1E3FF000
    // Test aarch64_float_move_fp_imm field imm8 = 255 (Max)
    // Fields: type1=0, Rd=0, imm8=255
    let encoding: u32 = 0x1E3FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #-1.93750000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_move_fp_imm_field_rd_0_min_1000_1e201000() {
    // Encoding: 0x1E201000
    // Test aarch64_float_move_fp_imm field Rd = 0 (Min)
    // Fields: type1=0, imm8=0, Rd=0
    let encoding: u32 = 0x1E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_move_fp_imm_field_rd_1_poweroftwo_1000_1e201001() {
    // Encoding: 0x1E201001
    // Test aarch64_float_move_fp_imm field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, type1=0, imm8=0
    let encoding: u32 = 0x1E201001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s1, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_move_fp_imm_field_rd_30_poweroftwominusone_1000_1e20101e() {
    // Encoding: 0x1E20101E
    // Test aarch64_float_move_fp_imm field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: imm8=0, type1=0, Rd=30
    let encoding: u32 = 0x1E20101E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s30, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_float_move_fp_imm_field_rd_31_max_1000_1e20101f() {
    // Encoding: 0x1E20101F
    // Test aarch64_float_move_fp_imm field Rd = 31 (Max)
    // Fields: Rd=31, imm8=0, type1=0
    let encoding: u32 = 0x1E20101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s31, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=0 (minimum value)
#[test]
fn test_aarch64_float_move_fp_imm_combo_0_1000_1e201000() {
    // Encoding: 0x1E201000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=0, Rd=0
    // Fields: Rd=0, imm8=0, type1=0
    let encoding: u32 = 0x1E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=1 (value 1)
#[test]
fn test_aarch64_float_move_fp_imm_combo_1_1000_1e601000() {
    // Encoding: 0x1E601000
    // Test aarch64_float_move_fp_imm field combination: type1=1, imm8=0, Rd=0
    // Fields: imm8=0, type1=1, Rd=0
    let encoding: u32 = 0x1E601000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	d0, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=3 (maximum value (3))
#[test]
fn test_aarch64_float_move_fp_imm_combo_2_1000_1ee01000() {
    // Encoding: 0x1EE01000
    // Test aarch64_float_move_fp_imm field combination: type1=3, imm8=0, Rd=0
    // Fields: Rd=0, type1=3, imm8=0
    let encoding: u32 = 0x1EE01000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	h0, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=0 (immediate value 0)
#[test]
fn test_aarch64_float_move_fp_imm_combo_3_1000_1e201000() {
    // Encoding: 0x1E201000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=0, Rd=0
    // Fields: type1=0, Rd=0, imm8=0
    let encoding: u32 = 0x1E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=1 (immediate value 1)
#[test]
fn test_aarch64_float_move_fp_imm_combo_4_1000_1e203000() {
    // Encoding: 0x1E203000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=1, Rd=0
    // Fields: imm8=1, type1=0, Rd=0
    let encoding: u32 = 0x1E203000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.12500000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_float_move_fp_imm_combo_5_1000_1e207000() {
    // Encoding: 0x1E207000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=3, Rd=0
    // Fields: type1=0, imm8=3, Rd=0
    let encoding: u32 = 0x1E207000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.37500000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_float_move_fp_imm_combo_6_1000_1e209000() {
    // Encoding: 0x1E209000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=4, Rd=0
    // Fields: type1=0, imm8=4, Rd=0
    let encoding: u32 = 0x1E209000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.50000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_float_move_fp_imm_combo_7_1000_1e20f000() {
    // Encoding: 0x1E20F000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=7, Rd=0
    // Fields: imm8=7, Rd=0, type1=0
    let encoding: u32 = 0x1E20F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.87500000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_float_move_fp_imm_combo_8_1000_1e211000() {
    // Encoding: 0x1E211000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=8, Rd=0
    // Fields: type1=0, Rd=0, imm8=8
    let encoding: u32 = 0x1E211000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #3.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_float_move_fp_imm_combo_9_1000_1e21f000() {
    // Encoding: 0x1E21F000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=15, Rd=0
    // Fields: type1=0, Rd=0, imm8=15
    let encoding: u32 = 0x1E21F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #3.87500000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_float_move_fp_imm_combo_10_1000_1e221000() {
    // Encoding: 0x1E221000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=16, Rd=0
    // Fields: Rd=0, type1=0, imm8=16
    let encoding: u32 = 0x1E221000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #4.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_float_move_fp_imm_combo_11_1000_1e23f000() {
    // Encoding: 0x1E23F000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=31, Rd=0
    // Fields: type1=0, imm8=31, Rd=0
    let encoding: u32 = 0x1E23F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #7.75000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_float_move_fp_imm_combo_12_1000_1e241000() {
    // Encoding: 0x1E241000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=32, Rd=0
    // Fields: type1=0, imm8=32, Rd=0
    let encoding: u32 = 0x1E241000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #8.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=63 (2^6 - 1 = 63)
#[test]
fn test_aarch64_float_move_fp_imm_combo_13_1000_1e27f000() {
    // Encoding: 0x1E27F000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=63, Rd=0
    // Fields: type1=0, imm8=63, Rd=0
    let encoding: u32 = 0x1E27F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #31.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_float_move_fp_imm_combo_14_1000_1e281000() {
    // Encoding: 0x1E281000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=64, Rd=0
    // Fields: Rd=0, type1=0, imm8=64
    let encoding: u32 = 0x1E281000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #0.12500000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=127 (immediate midpoint (127))
#[test]
fn test_aarch64_float_move_fp_imm_combo_15_1000_1e2ff000() {
    // Encoding: 0x1E2FF000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=127, Rd=0
    // Fields: type1=0, imm8=127, Rd=0
    let encoding: u32 = 0x1E2FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #1.93750000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=128 (power of 2 (2^7 = 128))
#[test]
fn test_aarch64_float_move_fp_imm_combo_16_1000_1e301000() {
    // Encoding: 0x1E301000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=128, Rd=0
    // Fields: type1=0, Rd=0, imm8=128
    let encoding: u32 = 0x1E301000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #-2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm8=255 (maximum immediate (255))
#[test]
fn test_aarch64_float_move_fp_imm_combo_17_1000_1e3ff000() {
    // Encoding: 0x1E3FF000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=255, Rd=0
    // Fields: imm8=255, type1=0, Rd=0
    let encoding: u32 = 0x1E3FF000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #-1.93750000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_move_fp_imm_combo_18_1000_1e201000() {
    // Encoding: 0x1E201000
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=0, Rd=0
    // Fields: imm8=0, type1=0, Rd=0
    let encoding: u32 = 0x1E201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_move_fp_imm_combo_19_1000_1e201001() {
    // Encoding: 0x1E201001
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=0, Rd=1
    // Fields: imm8=0, Rd=1, type1=0
    let encoding: u32 = 0x1E201001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s1, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_move_fp_imm_combo_20_1000_1e20101e() {
    // Encoding: 0x1E20101E
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=0, Rd=30
    // Fields: type1=0, Rd=30, imm8=0
    let encoding: u32 = 0x1E20101E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s30, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_float_move_fp_imm_combo_21_1000_1e20101f() {
    // Encoding: 0x1E20101F
    // Test aarch64_float_move_fp_imm field combination: type1=0, imm8=0, Rd=31
    // Fields: type1=0, Rd=31, imm8=0
    let encoding: u32 = 0x1E20101F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s31, #2.00000000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_move_fp_imm
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_float_move_fp_imm_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_4096_1e20301f()
 {
    // Encoding: 0x1E20301F
    // Test aarch64_float_move_fp_imm special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: imm8=1, Rd=31, type1=0
    let encoding: u32 = 0x1E20301F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s31, #2.12500000
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}
