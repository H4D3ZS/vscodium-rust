//! A64 vector shift tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_vector_shift_right_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_right_sisd_field_u_0_min_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field U = 0 (Min)
    // Fields: o1=0, Rn=0, immh=0, immb=0, U=0, o0=0, Rd=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_u_1_max_400_7f000400() {
    // Encoding: 0x7F000400
    // Test aarch64_vector_shift_right_sisd field U = 1 (Max)
    // Fields: o1=0, Rd=0, U=1, o0=0, Rn=0, immh=0, immb=0
    let encoding: u32 = 0x7F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_sisd_field_immh_0_zero_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field immh = 0 (Zero)
    // Fields: o1=0, Rn=0, o0=0, U=0, Rd=0, immh=0, immb=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_sisd_field_immh_1_poweroftwo_400_5f080400() {
    // Encoding: 0x5F080400
    // Test aarch64_vector_shift_right_sisd field immh = 1 (PowerOfTwo)
    // Fields: o0=0, Rd=0, Rn=0, immb=0, U=0, o1=0, immh=1
    let encoding: u32 = 0x5F080400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_right_sisd_field_immh_3_poweroftwominusone_400_5f180400() {
    // Encoding: 0x5F180400
    // Test aarch64_vector_shift_right_sisd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: Rn=0, immh=3, o0=0, Rd=0, o1=0, U=0, immb=0
    let encoding: u32 = 0x5F180400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_immh_4_poweroftwo_400_5f200400() {
    // Encoding: 0x5F200400
    // Test aarch64_vector_shift_right_sisd field immh = 4 (PowerOfTwo)
    // Fields: Rd=0, o0=0, o1=0, immb=0, U=0, immh=4, Rn=0
    let encoding: u32 = 0x5F200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_immh_7_poweroftwominusone_400_5f380400() {
    // Encoding: 0x5F380400
    // Test aarch64_vector_shift_right_sisd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=0, o1=0, immh=7, o0=0, U=0, immb=0
    let encoding: u32 = 0x5F380400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_immh_8_poweroftwo_400_5f400400() {
    // Encoding: 0x5F400400
    // Test aarch64_vector_shift_right_sisd field immh = 8 (PowerOfTwo)
    // Fields: immh=8, o1=0, Rn=0, Rd=0, U=0, immb=0, o0=0
    let encoding: u32 = 0x5F400400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_immh_15_max_400_5f780400() {
    // Encoding: 0x5F780400
    // Test aarch64_vector_shift_right_sisd field immh = 15 (Max)
    // Fields: U=0, Rn=0, Rd=0, immh=15, o1=0, o0=0, immb=0
    let encoding: u32 = 0x5F780400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_sisd_field_immb_0_zero_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field immb = 0 (Zero)
    // Fields: Rn=0, immh=0, U=0, immb=0, o1=0, o0=0, Rd=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_sisd_field_immb_1_poweroftwo_400_5f010400() {
    // Encoding: 0x5F010400
    // Test aarch64_vector_shift_right_sisd field immb = 1 (PowerOfTwo)
    // Fields: immh=0, U=0, Rd=0, Rn=0, o0=0, immb=1, o1=0
    let encoding: u32 = 0x5F010400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_immb_3_poweroftwominusone_400_5f030400() {
    // Encoding: 0x5F030400
    // Test aarch64_vector_shift_right_sisd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: immb=3, U=0, immh=0, o0=0, o1=0, Rn=0, Rd=0
    let encoding: u32 = 0x5F030400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_immb_7_max_400_5f070400() {
    // Encoding: 0x5F070400
    // Test aarch64_vector_shift_right_sisd field immb = 7 (Max)
    // Fields: o1=0, o0=0, Rd=0, U=0, immb=7, immh=0, Rn=0
    let encoding: u32 = 0x5F070400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field o1 13 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_right_sisd_field_o1_0_min_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field o1 = 0 (Min)
    // Fields: immb=0, U=0, o0=0, Rn=0, immh=0, Rd=0, o1=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field o1 13 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_o1_1_max_400_5f002400() {
    // Encoding: 0x5F002400
    // Test aarch64_vector_shift_right_sisd field o1 = 1 (Max)
    // Fields: o1=1, Rn=0, Rd=0, o0=0, immh=0, immb=0, U=0
    let encoding: u32 = 0x5F002400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field o0 12 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_right_sisd_field_o0_0_min_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field o0 = 0 (Min)
    // Fields: Rn=0, U=0, immb=0, Rd=0, o1=0, o0=0, immh=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field o0 12 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_o0_1_max_400_5f001400() {
    // Encoding: 0x5F001400
    // Test aarch64_vector_shift_right_sisd field o0 = 1 (Max)
    // Fields: o1=0, o0=1, Rn=0, Rd=0, immh=0, U=0, immb=0
    let encoding: u32 = 0x5F001400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_rn_0_min_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field Rn = 0 (Min)
    // Fields: Rd=0, immh=0, U=0, Rn=0, immb=0, o1=0, o0=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_rn_1_poweroftwo_400_5f000420() {
    // Encoding: 0x5F000420
    // Test aarch64_vector_shift_right_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, immh=0, U=0, o1=0, immb=0, Rd=0, o0=0
    let encoding: u32 = 0x5F000420;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_rn_30_poweroftwominusone_400_5f0007c0() {
    // Encoding: 0x5F0007C0
    // Test aarch64_vector_shift_right_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, U=0, immh=0, immb=0, o0=0, Rn=30, o1=0
    let encoding: u32 = 0x5F0007C0;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_rn_31_max_400_5f0007e0() {
    // Encoding: 0x5F0007E0
    // Test aarch64_vector_shift_right_sisd field Rn = 31 (Max)
    // Fields: U=0, immh=0, Rn=31, Rd=0, immb=0, o1=0, o0=0
    let encoding: u32 = 0x5F0007E0;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_rd_0_min_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field Rd = 0 (Min)
    // Fields: Rd=0, immh=0, o1=0, o0=0, Rn=0, immb=0, U=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_rd_1_poweroftwo_400_5f000401() {
    // Encoding: 0x5F000401
    // Test aarch64_vector_shift_right_sisd field Rd = 1 (PowerOfTwo)
    // Fields: immh=0, o1=0, immb=0, o0=0, Rn=0, Rd=1, U=0
    let encoding: u32 = 0x5F000401;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_rd_30_poweroftwominusone_400_5f00041e() {
    // Encoding: 0x5F00041E
    // Test aarch64_vector_shift_right_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: immb=0, Rd=30, immh=0, o1=0, Rn=0, U=0, o0=0
    let encoding: u32 = 0x5F00041E;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_right_sisd_field_rd_31_max_400_5f00041f() {
    // Encoding: 0x5F00041F
    // Test aarch64_vector_shift_right_sisd field Rd = 31 (Max)
    // Fields: o0=0, immb=0, o1=0, Rd=31, immh=0, U=0, Rn=0
    let encoding: u32 = 0x5F00041F;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_0_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: U=0, Rn=0, Rd=0, immh=0, o1=0, immb=0, o0=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_1_400_7f000400() {
    // Encoding: 0x7F000400
    // Test aarch64_vector_shift_right_sisd field combination: U=1, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: o1=0, o0=0, Rn=0, Rd=0, U=1, immh=0, immb=0
    let encoding: u32 = 0x7F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_2_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: o0=0, Rn=0, o1=0, Rd=0, U=0, immh=0, immb=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_3_400_5f080400() {
    // Encoding: 0x5F080400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=1, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immb=0, o1=0, Rn=0, Rd=0, o0=0, immh=1, U=0
    let encoding: u32 = 0x5F080400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_4_400_5f180400() {
    // Encoding: 0x5F180400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=3, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: o1=0, Rd=0, immh=3, U=0, o0=0, immb=0, Rn=0
    let encoding: u32 = 0x5F180400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_5_400_5f200400() {
    // Encoding: 0x5F200400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=4, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immh=4, immb=0, o0=0, o1=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x5F200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_6_400_5f380400() {
    // Encoding: 0x5F380400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=7, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immb=0, U=0, o1=0, immh=7, Rn=0, o0=0, Rd=0
    let encoding: u32 = 0x5F380400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_7_400_5f400400() {
    // Encoding: 0x5F400400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=8, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: o1=0, o0=0, immh=8, Rn=0, Rd=0, U=0, immb=0
    let encoding: u32 = 0x5F400400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_8_400_5f780400() {
    // Encoding: 0x5F780400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=15, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immh=15, immb=0, U=0, o1=0, o0=0, Rn=0, Rd=0
    let encoding: u32 = 0x5F780400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_9_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: Rn=0, immh=0, U=0, o0=0, immb=0, o1=0, Rd=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_10_400_5f010400() {
    // Encoding: 0x5F010400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=1, o1=0, o0=0, Rn=0, Rd=0
    // Fields: U=0, o1=0, Rn=0, immh=0, o0=0, Rd=0, immb=1
    let encoding: u32 = 0x5F010400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_11_400_5f030400() {
    // Encoding: 0x5F030400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=3, o1=0, o0=0, Rn=0, Rd=0
    // Fields: U=0, Rn=0, immh=0, immb=3, o1=0, o0=0, Rd=0
    let encoding: u32 = 0x5F030400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_12_400_5f070400() {
    // Encoding: 0x5F070400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=7, o1=0, o0=0, Rn=0, Rd=0
    // Fields: Rn=0, o0=0, U=0, o1=0, Rd=0, immb=7, immh=0
    let encoding: u32 = 0x5F070400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_13_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immb=0, o1=0, o0=0, Rd=0, Rn=0, U=0, immh=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_14_400_5f002400() {
    // Encoding: 0x5F002400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=1, o0=0, Rn=0, Rd=0
    // Fields: o0=0, immb=0, o1=1, Rn=0, Rd=0, U=0, immh=0
    let encoding: u32 = 0x5F002400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_15_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: Rd=0, immb=0, immh=0, U=0, o1=0, Rn=0, o0=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_16_400_5f001400() {
    // Encoding: 0x5F001400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=1, Rn=0, Rd=0
    // Fields: o1=0, o0=1, Rn=0, U=0, Rd=0, immb=0, immh=0
    let encoding: u32 = 0x5F001400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_17_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immb=0, immh=0, o1=0, Rd=0, o0=0, U=0, Rn=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_18_400_5f000420() {
    // Encoding: 0x5F000420
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=1, Rd=0
    // Fields: U=0, Rn=1, o0=0, immh=0, Rd=0, o1=0, immb=0
    let encoding: u32 = 0x5F000420;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_19_400_5f0007c0() {
    // Encoding: 0x5F0007C0
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=30, Rd=0
    // Fields: Rn=30, U=0, o1=0, Rd=0, immb=0, o0=0, immh=0
    let encoding: u32 = 0x5F0007C0;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_20_400_5f0007e0() {
    // Encoding: 0x5F0007E0
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=31, Rd=0
    // Fields: U=0, immh=0, o1=0, immb=0, Rn=31, Rd=0, o0=0
    let encoding: u32 = 0x5F0007E0;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_21_400_5f000400() {
    // Encoding: 0x5F000400
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: U=0, immh=0, o1=0, immb=0, o0=0, Rn=0, Rd=0
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_22_400_5f000401() {
    // Encoding: 0x5F000401
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=1
    // Fields: immh=0, o0=0, o1=0, U=0, immb=0, Rn=0, Rd=1
    let encoding: u32 = 0x5F000401;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_23_400_5f00041e() {
    // Encoding: 0x5F00041E
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=30
    // Fields: immh=0, immb=0, U=0, o0=0, Rn=0, o1=0, Rd=30
    let encoding: u32 = 0x5F00041E;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_24_400_5f00041f() {
    // Encoding: 0x5F00041F
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=31
    // Fields: immh=0, U=0, immb=0, o1=0, o0=0, Rn=0, Rd=31
    let encoding: u32 = 0x5F00041F;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_25_400_5f000421() {
    // Encoding: 0x5F000421
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=1, Rd=1
    // Fields: Rd=1, immb=0, immh=0, o1=0, o0=0, U=0, Rn=1
    let encoding: u32 = 0x5F000421;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_right_sisd_combo_26_400_5f0007ff() {
    // Encoding: 0x5F0007FF
    // Test aarch64_vector_shift_right_sisd field combination: U=0, immh=0, immb=0, o1=0, o0=0, Rn=31, Rd=31
    // Fields: immh=0, immb=0, U=0, o1=0, o0=0, Rn=31, Rd=31
    let encoding: u32 = 0x5F0007FF;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_right_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_1024_5f0907e0()
 {
    // Encoding: 0x5F0907E0
    // Test aarch64_vector_shift_right_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: U=0, immh=1, o0=0, Rn=31, Rd=0, immb=1, o1=0
    let encoding: u32 = 0x5F0907E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_right_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_1024_5f09041f()
 {
    // Encoding: 0x5F09041F
    // Test aarch64_vector_shift_right_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: o0=0, Rn=0, o1=0, Rd=31, U=0, immh=1, immb=1
    let encoding: u32 = 0x5F09041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_shift_right_simd_field_q_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field Q = 0 (Min)
    // Fields: U=0, Rn=0, immb=0, o0=0, o1=0, Rd=0, immh=0, Q=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_shift_right_simd_field_q_1_max_400_4f000400() {
    // Encoding: 0x4F000400
    // Test aarch64_vector_shift_right_simd field Q = 1 (Max)
    // Fields: Q=1, U=0, immh=0, o0=0, immb=0, o1=0, Rd=0, Rn=0
    let encoding: u32 = 0x4F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_right_simd_field_u_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field U = 0 (Min)
    // Fields: immb=0, Rn=0, o0=0, o1=0, Rd=0, U=0, immh=0, Q=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_right_simd_field_u_1_max_400_2f000400() {
    // Encoding: 0x2F000400
    // Test aarch64_vector_shift_right_simd field U = 1 (Max)
    // Fields: Q=0, Rd=0, immb=0, o0=0, Rn=0, immh=0, U=1, o1=0
    let encoding: u32 = 0x2F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_simd_field_immh_0_zero_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field immh = 0 (Zero)
    // Fields: immb=0, o1=0, Rd=0, Rn=0, Q=0, U=0, immh=0, o0=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_simd_field_immh_1_poweroftwo_400_0f080400() {
    // Encoding: 0x0F080400
    // Test aarch64_vector_shift_right_simd field immh = 1 (PowerOfTwo)
    // Fields: U=0, Rd=0, Q=0, immb=0, Rn=0, o0=0, immh=1, o1=0
    let encoding: u32 = 0x0F080400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_right_simd_field_immh_3_poweroftwominusone_400_0f180400() {
    // Encoding: 0x0F180400
    // Test aarch64_vector_shift_right_simd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: o0=0, Q=0, o1=0, U=0, immh=3, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x0F180400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_right_simd_field_immh_4_poweroftwo_400_0f200400() {
    // Encoding: 0x0F200400
    // Test aarch64_vector_shift_right_simd field immh = 4 (PowerOfTwo)
    // Fields: U=0, immb=0, immh=4, Q=0, o0=0, Rn=0, o1=0, Rd=0
    let encoding: u32 = 0x0F200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_right_simd_field_immh_7_poweroftwominusone_400_0f380400() {
    // Encoding: 0x0F380400
    // Test aarch64_vector_shift_right_simd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rn=0, Rd=0, immb=0, o0=0, immh=7, U=0, o1=0
    let encoding: u32 = 0x0F380400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_right_simd_field_immh_8_poweroftwo_400_0f400400() {
    // Encoding: 0x0F400400
    // Test aarch64_vector_shift_right_simd field immh = 8 (PowerOfTwo)
    // Fields: Q=0, U=0, immb=0, o0=0, Rd=0, Rn=0, o1=0, immh=8
    let encoding: u32 = 0x0F400400;
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

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_right_simd_field_immh_15_max_400_0f780400() {
    // Encoding: 0x0F780400
    // Test aarch64_vector_shift_right_simd field immh = 15 (Max)
    // Fields: Q=0, immb=0, Rn=0, immh=15, o1=0, o0=0, U=0, Rd=0
    let encoding: u32 = 0x0F780400;
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

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_simd_field_immb_0_zero_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field immb = 0 (Zero)
    // Fields: U=0, immh=0, o1=0, o0=0, Q=0, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_simd_field_immb_1_poweroftwo_400_0f010400() {
    // Encoding: 0x0F010400
    // Test aarch64_vector_shift_right_simd field immb = 1 (PowerOfTwo)
    // Fields: o1=0, o0=0, Q=0, U=0, immb=1, Rn=0, Rd=0, immh=0
    let encoding: u32 = 0x0F010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_right_simd_field_immb_3_poweroftwominusone_400_0f030400() {
    // Encoding: 0x0F030400
    // Test aarch64_vector_shift_right_simd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: immb=3, Q=0, immh=0, U=0, o0=0, Rn=0, o1=0, Rd=0
    let encoding: u32 = 0x0F030400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_right_simd_field_immb_7_max_400_0f070400() {
    // Encoding: 0x0F070400
    // Test aarch64_vector_shift_right_simd field immb = 7 (Max)
    // Fields: immb=7, o0=0, Q=0, immh=0, U=0, Rn=0, Rd=0, o1=0
    let encoding: u32 = 0x0F070400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field o1 13 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_right_simd_field_o1_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field o1 = 0 (Min)
    // Fields: immh=0, o1=0, o0=0, immb=0, Q=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field o1 13 +: 1`
/// Requirement: FieldBoundary { field: "o1", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_right_simd_field_o1_1_max_400_0f002400() {
    // Encoding: 0x0F002400
    // Test aarch64_vector_shift_right_simd field o1 = 1 (Max)
    // Fields: Rn=0, o0=0, immh=0, o1=1, U=0, Rd=0, Q=0, immb=0
    let encoding: u32 = 0x0F002400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field o0 12 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_right_simd_field_o0_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field o0 = 0 (Min)
    // Fields: immh=0, immb=0, o0=0, Rn=0, Q=0, o1=0, Rd=0, U=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field o0 12 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_right_simd_field_o0_1_max_400_0f001400() {
    // Encoding: 0x0F001400
    // Test aarch64_vector_shift_right_simd field o0 = 1 (Max)
    // Fields: Rn=0, Q=0, immh=0, U=0, o0=1, o1=0, Rd=0, immb=0
    let encoding: u32 = 0x0F001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_simd_field_rn_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field Rn = 0 (Min)
    // Fields: U=0, Rn=0, o0=0, Rd=0, immh=0, o1=0, immb=0, Q=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_simd_field_rn_1_poweroftwo_400_0f000420() {
    // Encoding: 0x0F000420
    // Test aarch64_vector_shift_right_simd field Rn = 1 (PowerOfTwo)
    // Fields: Q=0, U=0, o1=0, immh=0, Rn=1, o0=0, immb=0, Rd=0
    let encoding: u32 = 0x0F000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_simd_field_rn_30_poweroftwominusone_400_0f0007c0() {
    // Encoding: 0x0F0007C0
    // Test aarch64_vector_shift_right_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Q=0, o0=0, Rn=30, immh=0, immb=0, U=0, o1=0
    let encoding: u32 = 0x0F0007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_right_simd_field_rn_31_max_400_0f0007e0() {
    // Encoding: 0x0F0007E0
    // Test aarch64_vector_shift_right_simd field Rn = 31 (Max)
    // Fields: Rd=0, immh=0, o1=0, immb=0, o0=0, Q=0, U=0, Rn=31
    let encoding: u32 = 0x0F0007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_simd_field_rd_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field Rd = 0 (Min)
    // Fields: Rn=0, o1=0, immb=0, Q=0, o0=0, U=0, immh=0, Rd=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_simd_field_rd_1_poweroftwo_400_0f000401() {
    // Encoding: 0x0F000401
    // Test aarch64_vector_shift_right_simd field Rd = 1 (PowerOfTwo)
    // Fields: o0=0, U=0, immb=0, immh=0, o1=0, Rn=0, Q=0, Rd=1
    let encoding: u32 = 0x0F000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_simd_field_rd_30_poweroftwominusone_400_0f00041e() {
    // Encoding: 0x0F00041E
    // Test aarch64_vector_shift_right_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: o1=0, immb=0, immh=0, Rd=30, Q=0, U=0, o0=0, Rn=0
    let encoding: u32 = 0x0F00041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_right_simd_field_rd_31_max_400_0f00041f() {
    // Encoding: 0x0F00041F
    // Test aarch64_vector_shift_right_simd field Rd = 31 (Max)
    // Fields: o1=0, Q=0, Rd=31, Rn=0, U=0, immb=0, immh=0, o0=0
    let encoding: u32 = 0x0F00041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_shift_right_simd_combo_0_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: Rd=0, immb=0, o1=0, U=0, immh=0, Q=0, o0=0, Rn=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_shift_right_simd_combo_1_400_4f000400() {
    // Encoding: 0x4F000400
    // Test aarch64_vector_shift_right_simd field combination: Q=1, U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immh=0, Rn=0, o1=0, o0=0, Q=1, U=0, immb=0, Rd=0
    let encoding: u32 = 0x4F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_right_simd_combo_2_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: Rd=0, immb=0, o1=0, immh=0, U=0, Rn=0, Q=0, o0=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_3_400_2f000400() {
    // Encoding: 0x2F000400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=1, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, immb=0, Rn=0, immh=0, U=1, o0=0, o1=0
    let encoding: u32 = 0x2F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_simd_combo_4_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, immb=0, immh=0, o1=0, U=0, o0=0, Rn=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_simd_combo_5_400_0f080400() {
    // Encoding: 0x0F080400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=1, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immh=1, immb=0, Rn=0, o0=0, U=0, Q=0, o1=0, Rd=0
    let encoding: u32 = 0x0F080400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_right_simd_combo_6_400_0f180400() {
    // Encoding: 0x0F180400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=3, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: U=0, o0=0, Rn=0, Q=0, o1=0, Rd=0, immh=3, immb=0
    let encoding: u32 = 0x0F180400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_7_400_0f200400() {
    // Encoding: 0x0F200400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=4, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: U=0, Q=0, Rd=0, immh=4, o1=0, o0=0, Rn=0, immb=0
    let encoding: u32 = 0x0F200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_8_400_0f380400() {
    // Encoding: 0x0F380400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=7, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: Q=0, o1=0, immh=7, Rd=0, Rn=0, immb=0, o0=0, U=0
    let encoding: u32 = 0x0F380400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_9_400_0f400400() {
    // Encoding: 0x0F400400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=8, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: o0=0, U=0, Rd=0, Q=0, Rn=0, immh=8, immb=0, o1=0
    let encoding: u32 = 0x0F400400;
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

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_10_400_0f780400() {
    // Encoding: 0x0F780400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=15, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immh=15, Rd=0, o0=0, Q=0, U=0, immb=0, Rn=0, o1=0
    let encoding: u32 = 0x0F780400;
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

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_simd_combo_11_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immh=0, Rd=0, U=0, o0=0, immb=0, Q=0, Rn=0, o1=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_simd_combo_12_400_0f010400() {
    // Encoding: 0x0F010400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=1, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immb=1, Rn=0, o0=0, Rd=0, Q=0, U=0, immh=0, o1=0
    let encoding: u32 = 0x0F010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_13_400_0f030400() {
    // Encoding: 0x0F030400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=3, o1=0, o0=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, o0=0, immh=0, U=0, immb=3, o1=0, Rd=0
    let encoding: u32 = 0x0F030400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_14_400_0f070400() {
    // Encoding: 0x0F070400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=7, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immb=7, Q=0, Rd=0, immh=0, U=0, Rn=0, o1=0, o0=0
    let encoding: u32 = 0x0F070400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_right_simd_combo_15_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immh=0, Rn=0, U=0, o0=0, Q=0, o1=0, Rd=0, immb=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o1=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_16_400_0f002400() {
    // Encoding: 0x0F002400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=1, o0=0, Rn=0, Rd=0
    // Fields: immb=0, o1=1, Q=0, Rd=0, immh=0, U=0, Rn=0, o0=0
    let encoding: u32 = 0x0F002400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_right_simd_combo_17_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: Q=0, immb=0, U=0, o1=0, immh=0, Rn=0, o0=0, Rd=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_18_400_0f001400() {
    // Encoding: 0x0F001400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=1, Rn=0, Rd=0
    // Fields: o0=1, U=0, Rd=0, o1=0, immh=0, Q=0, immb=0, Rn=0
    let encoding: u32 = 0x0F001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_19_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: immh=0, Rd=0, o1=0, U=0, immb=0, Rn=0, o0=0, Q=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_20_400_0f000420() {
    // Encoding: 0x0F000420
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=1, Rd=0
    // Fields: Rn=1, immb=0, o0=0, U=0, o1=0, immh=0, Q=0, Rd=0
    let encoding: u32 = 0x0F000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_21_400_0f0007c0() {
    // Encoding: 0x0F0007C0
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=30, Rd=0
    // Fields: o1=0, Rd=0, o0=0, immb=0, immh=0, Q=0, Rn=30, U=0
    let encoding: u32 = 0x0F0007C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_22_400_0f0007e0() {
    // Encoding: 0x0F0007E0
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=31, Rd=0
    // Fields: o1=0, U=0, Q=0, Rd=0, Rn=31, o0=0, immh=0, immb=0
    let encoding: u32 = 0x0F0007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_23_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=0
    // Fields: Rd=0, immb=0, Rn=0, o0=0, o1=0, Q=0, U=0, immh=0
    let encoding: u32 = 0x0F000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_24_400_0f000401() {
    // Encoding: 0x0F000401
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=1
    // Fields: o0=0, immh=0, U=0, o1=0, immb=0, Q=0, Rd=1, Rn=0
    let encoding: u32 = 0x0F000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_25_400_0f00041e() {
    // Encoding: 0x0F00041E
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=30
    // Fields: Rd=30, Q=0, o0=0, immb=0, immh=0, o1=0, U=0, Rn=0
    let encoding: u32 = 0x0F00041E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_26_400_0f00041f() {
    // Encoding: 0x0F00041F
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=0, Rd=31
    // Fields: o0=0, Q=0, o1=0, Rn=0, U=0, immh=0, Rd=31, immb=0
    let encoding: u32 = 0x0F00041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_27_400_0f000421() {
    // Encoding: 0x0F000421
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=1, Rd=1
    // Fields: o0=0, Rd=1, Q=0, immb=0, U=0, o1=0, Rn=1, immh=0
    let encoding: u32 = 0x0F000421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_right_simd_combo_28_400_0f0007ff() {
    // Encoding: 0x0F0007FF
    // Test aarch64_vector_shift_right_simd field combination: Q=0, U=0, immh=0, immb=0, o1=0, o0=0, Rn=31, Rd=31
    // Fields: Rn=31, o1=0, immh=0, U=0, immb=0, Rd=31, Q=0, o0=0
    let encoding: u32 = 0x0F0007FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_shift_right_simd_special_q_0_size_variant_0_1024_0f090400() {
    // Encoding: 0x0F090400
    // Test aarch64_vector_shift_right_simd special value Q = 0 (Size variant 0)
    // Fields: immh=1, Q=0, Rd=0, o0=0, Rn=0, o1=0, U=0, immb=1
    let encoding: u32 = 0x0F090400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_shift_right_simd_special_q_1_size_variant_1_1024_4f090400() {
    // Encoding: 0x4F090400
    // Test aarch64_vector_shift_right_simd special value Q = 1 (Size variant 1)
    // Fields: Rn=0, immh=1, o1=0, Rd=0, Q=1, immb=1, U=0, o0=0
    let encoding: u32 = 0x4F090400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_right_simd_special_rn_31_stack_pointer_sp_may_require_alignment_1024_0f0907e0()
 {
    // Encoding: 0x0F0907E0
    // Test aarch64_vector_shift_right_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: immh=1, U=0, immb=1, Rd=0, Q=0, o0=0, Rn=31, o1=0
    let encoding: u32 = 0x0F0907E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_right_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_1024_0f09041f()
 {
    // Encoding: 0x0F09041F
    // Test aarch64_vector_shift_right_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: immh=1, Rn=0, Q=0, immb=1, o1=0, Rd=31, U=0, o0=0
    let encoding: u32 = 0x0F09041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_right_sisd_reg_write_0_5f000400() {
    // Test aarch64_vector_shift_right_sisd register write: SimdFromField("d")
    // Encoding: 0x5F000400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F000400;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_right_sisd_sp_rn_5f0007e0() {
    // Test aarch64_vector_shift_right_sisd with Rn = SP (31)
    // Encoding: 0x5F0007E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F0007E0;
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

/// Provenance: aarch64_vector_shift_right_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_right_sisd_zr_rd_5f00041f() {
    // Test aarch64_vector_shift_right_sisd with Rd = ZR (31)
    // Encoding: 0x5F00041F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F00041F;
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

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_right_simd_reg_write_0_0f000400() {
    // Test aarch64_vector_shift_right_simd register write: SimdFromField("d")
    // Encoding: 0x0F000400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F000400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_right_simd_sp_rn_0f0007e0() {
    // Test aarch64_vector_shift_right_simd with Rn = SP (31)
    // Encoding: 0x0F0007E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F0007E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_right_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_right_simd_zr_rd_0f00041f() {
    // Test aarch64_vector_shift_right_simd with Rd = ZR (31)
    // Encoding: 0x0F00041F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00041F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_shift_right_narrow_nonuniform_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_immh_0_zero_8400_7f008400() {
    // Encoding: 0x7F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field immh = 0 (Zero)
    // Fields: op=0, immh=0, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F008400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_immh_1_poweroftwo_8400_7f088400() {
    // Encoding: 0x7F088400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field immh = 1 (PowerOfTwo)
    // Fields: op=0, Rn=0, Rd=0, immh=1, immb=0
    let encoding: u32 = 0x7F088400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_immh_3_poweroftwominusone_8400_7f188400()
 {
    // Encoding: 0x7F188400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: immb=0, immh=3, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F188400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_immh_4_poweroftwo_8400_7f208400() {
    // Encoding: 0x7F208400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field immh = 4 (PowerOfTwo)
    // Fields: immh=4, Rn=0, Rd=0, immb=0, op=0
    let encoding: u32 = 0x7F208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_immh_7_poweroftwominusone_8400_7f388400()
 {
    // Encoding: 0x7F388400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: op=0, immb=0, Rn=0, immh=7, Rd=0
    let encoding: u32 = 0x7F388400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_immh_8_poweroftwo_8400_7f408400() {
    // Encoding: 0x7F408400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field immh = 8 (PowerOfTwo)
    // Fields: immh=8, immb=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F408400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_immh_15_max_8400_7f788400() {
    // Encoding: 0x7F788400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field immh = 15 (Max)
    // Fields: Rd=0, op=0, immh=15, Rn=0, immb=0
    let encoding: u32 = 0x7F788400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_immb_0_zero_8400_7f008400() {
    // Encoding: 0x7F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field immb = 0 (Zero)
    // Fields: immh=0, immb=0, Rn=0, Rd=0, op=0
    let encoding: u32 = 0x7F008400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_immb_1_poweroftwo_8400_7f018400() {
    // Encoding: 0x7F018400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field immb = 1 (PowerOfTwo)
    // Fields: op=0, immh=0, Rn=0, Rd=0, immb=1
    let encoding: u32 = 0x7F018400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_immb_3_poweroftwominusone_8400_7f038400()
 {
    // Encoding: 0x7F038400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: op=0, Rn=0, immb=3, Rd=0, immh=0
    let encoding: u32 = 0x7F038400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_immb_7_max_8400_7f078400() {
    // Encoding: 0x7F078400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field immb = 7 (Max)
    // Fields: immb=7, op=0, immh=0, Rd=0, Rn=0
    let encoding: u32 = 0x7F078400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field op 11 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_op_0_min_8400_7f008400() {
    // Encoding: 0x7F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field op = 0 (Min)
    // Fields: immh=0, immb=0, Rn=0, op=0, Rd=0
    let encoding: u32 = 0x7F008400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field op 11 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_op_1_max_8400_7f008c00() {
    // Encoding: 0x7F008C00
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field op = 1 (Max)
    // Fields: immb=0, Rn=0, op=1, Rd=0, immh=0
    let encoding: u32 = 0x7F008C00;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_rn_0_min_8400_7f008400() {
    // Encoding: 0x7F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field Rn = 0 (Min)
    // Fields: immh=0, Rn=0, immb=0, Rd=0, op=0
    let encoding: u32 = 0x7F008400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_rn_1_poweroftwo_8400_7f008420() {
    // Encoding: 0x7F008420
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field Rn = 1 (PowerOfTwo)
    // Fields: immb=0, op=0, Rn=1, Rd=0, immh=0
    let encoding: u32 = 0x7F008420;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_rn_30_poweroftwominusone_8400_7f0087c0()
 {
    // Encoding: 0x7F0087C0
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, op=0, immb=0, immh=0, Rd=0
    let encoding: u32 = 0x7F0087C0;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_rn_31_max_8400_7f0087e0() {
    // Encoding: 0x7F0087E0
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field Rn = 31 (Max)
    // Fields: op=0, immb=0, immh=0, Rn=31, Rd=0
    let encoding: u32 = 0x7F0087E0;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_rd_0_min_8400_7f008400() {
    // Encoding: 0x7F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field Rd = 0 (Min)
    // Fields: immh=0, Rn=0, immb=0, Rd=0, op=0
    let encoding: u32 = 0x7F008400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_rd_1_poweroftwo_8400_7f008401() {
    // Encoding: 0x7F008401
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field Rd = 1 (PowerOfTwo)
    // Fields: op=0, Rn=0, immh=0, immb=0, Rd=1
    let encoding: u32 = 0x7F008401;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_rd_30_poweroftwominusone_8400_7f00841e()
 {
    // Encoding: 0x7F00841E
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, Rn=0, immh=0, immb=0, op=0
    let encoding: u32 = 0x7F00841E;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_field_rd_31_max_8400_7f00841f() {
    // Encoding: 0x7F00841F
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field Rd = 31 (Max)
    // Fields: op=0, Rd=31, Rn=0, immh=0, immb=0
    let encoding: u32 = 0x7F00841F;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_0_8400_7f008400() {
    // Encoding: 0x7F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=0, immb=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F008400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_1_8400_7f088400() {
    // Encoding: 0x7F088400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=1, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, immh=1, op=0, Rd=0, Rn=0
    let encoding: u32 = 0x7F088400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_2_8400_7f188400() {
    // Encoding: 0x7F188400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=3, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, op=0, Rd=0, immh=3, immb=0
    let encoding: u32 = 0x7F188400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_3_8400_7f208400() {
    // Encoding: 0x7F208400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=4, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=4, immb=0, op=0, Rd=0, Rn=0
    let encoding: u32 = 0x7F208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_4_8400_7f388400() {
    // Encoding: 0x7F388400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=7, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, immh=7, op=0, Rn=0, immb=0
    let encoding: u32 = 0x7F388400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_5_8400_7f408400() {
    // Encoding: 0x7F408400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=8, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=8, Rn=0, immb=0, op=0, Rd=0
    let encoding: u32 = 0x7F408400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_6_8400_7f788400() {
    // Encoding: 0x7F788400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=15, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, immh=15, Rn=0, immb=0, op=0
    let encoding: u32 = 0x7F788400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_7_8400_7f008400() {
    // Encoding: 0x7F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, immh=0, immb=0, op=0, Rd=0
    let encoding: u32 = 0x7F008400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_8_8400_7f018400() {
    // Encoding: 0x7F018400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=1, op=0, Rn=0, Rd=0
    // Fields: immh=0, op=0, Rn=0, Rd=0, immb=1
    let encoding: u32 = 0x7F018400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_9_8400_7f038400() {
    // Encoding: 0x7F038400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=3, op=0, Rn=0, Rd=0
    // Fields: op=0, immb=3, immh=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F038400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_10_8400_7f078400() {
    // Encoding: 0x7F078400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=7, op=0, Rn=0, Rd=0
    // Fields: immb=7, immh=0, Rd=0, op=0, Rn=0
    let encoding: u32 = 0x7F078400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_11_8400_7f008400() {
    // Encoding: 0x7F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, op=0, Rd=0, immb=0, immh=0
    let encoding: u32 = 0x7F008400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_12_8400_7f008c00() {
    // Encoding: 0x7F008C00
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=1, Rn=0, Rd=0
    // Fields: Rn=0, immh=0, op=1, Rd=0, immb=0
    let encoding: u32 = 0x7F008C00;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_13_8400_7f008400() {
    // Encoding: 0x7F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, immh=0, Rn=0, Rd=0, op=0
    let encoding: u32 = 0x7F008400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_14_8400_7f008420() {
    // Encoding: 0x7F008420
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=0, Rn=1, Rd=0
    // Fields: Rn=1, Rd=0, op=0, immh=0, immb=0
    let encoding: u32 = 0x7F008420;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_15_8400_7f0087c0() {
    // Encoding: 0x7F0087C0
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=0, Rn=30, Rd=0
    // Fields: immb=0, Rd=0, op=0, Rn=30, immh=0
    let encoding: u32 = 0x7F0087C0;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_16_8400_7f0087e0() {
    // Encoding: 0x7F0087E0
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=0, Rn=31, Rd=0
    // Fields: op=0, Rn=31, Rd=0, immh=0, immb=0
    let encoding: u32 = 0x7F0087E0;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_17_8400_7f008400() {
    // Encoding: 0x7F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=0, immb=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F008400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_18_8400_7f008401() {
    // Encoding: 0x7F008401
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=0, Rn=0, Rd=1
    // Fields: Rn=0, immb=0, Rd=1, op=0, immh=0
    let encoding: u32 = 0x7F008401;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_19_8400_7f00841e() {
    // Encoding: 0x7F00841E
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=0, Rn=0, Rd=30
    // Fields: immh=0, Rn=0, Rd=30, op=0, immb=0
    let encoding: u32 = 0x7F00841E;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_20_8400_7f00841f() {
    // Encoding: 0x7F00841F
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=0, Rn=0, Rd=31
    // Fields: op=0, Rn=0, immb=0, immh=0, Rd=31
    let encoding: u32 = 0x7F00841F;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_21_8400_7f008421() {
    // Encoding: 0x7F008421
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=0, Rn=1, Rd=1
    // Fields: immb=0, Rd=1, immh=0, Rn=1, op=0
    let encoding: u32 = 0x7F008421;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_combo_22_8400_7f0087ff() {
    // Encoding: 0x7F0087FF
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd field combination: immh=0, immb=0, op=0, Rn=31, Rd=31
    // Fields: immh=0, op=0, immb=0, Rd=31, Rn=31
    let encoding: u32 = 0x7F0087FF;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_33792_7f0987e0()
 {
    // Encoding: 0x7F0987E0
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: op=0, Rd=0, immb=1, immh=1, Rn=31
    let encoding: u32 = 0x7F0987E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_33792_7f09841f()
 {
    // Encoding: 0x7F09841F
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: immb=1, Rd=31, immh=1, op=0, Rn=0
    let encoding: u32 = 0x7F09841F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_q_0_min_8400_2f008400() {
    // Encoding: 0x2F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field Q = 0 (Min)
    // Fields: immb=0, op=0, Rn=0, immh=0, Q=0, Rd=0
    let encoding: u32 = 0x2F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_q_1_max_8400_6f008400() {
    // Encoding: 0x6F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field Q = 1 (Max)
    // Fields: Q=1, immh=0, op=0, Rd=0, immb=0, Rn=0
    let encoding: u32 = 0x6F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_immh_0_zero_8400_2f008400() {
    // Encoding: 0x2F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field immh = 0 (Zero)
    // Fields: Q=0, immh=0, immb=0, op=0, Rd=0, Rn=0
    let encoding: u32 = 0x2F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_immh_1_poweroftwo_8400_2f088400() {
    // Encoding: 0x2F088400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field immh = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, immb=0, Q=0, immh=1, op=0
    let encoding: u32 = 0x2F088400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_immh_3_poweroftwominusone_8400_2f188400()
 {
    // Encoding: 0x2F188400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: Rn=0, immb=0, Rd=0, Q=0, op=0, immh=3
    let encoding: u32 = 0x2F188400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_immh_4_poweroftwo_8400_2f208400() {
    // Encoding: 0x2F208400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field immh = 4 (PowerOfTwo)
    // Fields: op=0, Q=0, Rd=0, Rn=0, immh=4, immb=0
    let encoding: u32 = 0x2F208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_immh_7_poweroftwominusone_8400_2f388400()
 {
    // Encoding: 0x2F388400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rn=0, Rd=0, immh=7, immb=0, op=0
    let encoding: u32 = 0x2F388400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_immh_8_poweroftwo_8400_2f408400() {
    // Encoding: 0x2F408400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field immh = 8 (PowerOfTwo)
    // Fields: immb=0, immh=8, Rn=0, Q=0, op=0, Rd=0
    let encoding: u32 = 0x2F408400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_immh_15_max_8400_2f788400() {
    // Encoding: 0x2F788400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field immh = 15 (Max)
    // Fields: immb=0, immh=15, op=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x2F788400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_immb_0_zero_8400_2f008400() {
    // Encoding: 0x2F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field immb = 0 (Zero)
    // Fields: immh=0, Q=0, immb=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x2F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_immb_1_poweroftwo_8400_2f018400() {
    // Encoding: 0x2F018400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field immb = 1 (PowerOfTwo)
    // Fields: Q=0, op=0, Rn=0, immb=1, immh=0, Rd=0
    let encoding: u32 = 0x2F018400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_immb_3_poweroftwominusone_8400_2f038400()
 {
    // Encoding: 0x2F038400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: Q=0, immh=0, Rn=0, Rd=0, op=0, immb=3
    let encoding: u32 = 0x2F038400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_immb_7_max_8400_2f078400() {
    // Encoding: 0x2F078400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field immb = 7 (Max)
    // Fields: immh=0, immb=7, Q=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x2F078400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field op 11 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_op_0_min_8400_2f008400() {
    // Encoding: 0x2F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field op = 0 (Min)
    // Fields: op=0, Rn=0, immb=0, Rd=0, Q=0, immh=0
    let encoding: u32 = 0x2F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field op 11 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_op_1_max_8400_2f008c00() {
    // Encoding: 0x2F008C00
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field op = 1 (Max)
    // Fields: op=1, Rn=0, Rd=0, Q=0, immh=0, immb=0
    let encoding: u32 = 0x2F008C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_rn_0_min_8400_2f008400() {
    // Encoding: 0x2F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field Rn = 0 (Min)
    // Fields: Q=0, immb=0, Rd=0, immh=0, op=0, Rn=0
    let encoding: u32 = 0x2F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_rn_1_poweroftwo_8400_2f008420() {
    // Encoding: 0x2F008420
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, immh=0, immb=0, op=0, Rd=0, Q=0
    let encoding: u32 = 0x2F008420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_rn_30_poweroftwominusone_8400_2f0087c0()
 {
    // Encoding: 0x2F0087C0
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: op=0, Q=0, immh=0, Rd=0, Rn=30, immb=0
    let encoding: u32 = 0x2F0087C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_rn_31_max_8400_2f0087e0() {
    // Encoding: 0x2F0087E0
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field Rn = 31 (Max)
    // Fields: Rn=31, Rd=0, op=0, immh=0, immb=0, Q=0
    let encoding: u32 = 0x2F0087E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_rd_0_min_8400_2f008400() {
    // Encoding: 0x2F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field Rd = 0 (Min)
    // Fields: Q=0, immb=0, op=0, Rn=0, immh=0, Rd=0
    let encoding: u32 = 0x2F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_rd_1_poweroftwo_8400_2f008401() {
    // Encoding: 0x2F008401
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field Rd = 1 (PowerOfTwo)
    // Fields: Q=0, immb=0, op=0, Rn=0, Rd=1, immh=0
    let encoding: u32 = 0x2F008401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_rd_30_poweroftwominusone_8400_2f00841e()
 {
    // Encoding: 0x2F00841E
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: op=0, immb=0, Q=0, immh=0, Rn=0, Rd=30
    let encoding: u32 = 0x2F00841E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_field_rd_31_max_8400_2f00841f() {
    // Encoding: 0x2F00841F
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field Rd = 31 (Max)
    // Fields: immb=0, op=0, Rn=0, Rd=31, immh=0, Q=0
    let encoding: u32 = 0x2F00841F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_0_8400_2f008400() {
    // Encoding: 0x2F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, immb=0, op=0, Rn=0, Rd=0, immh=0
    let encoding: u32 = 0x2F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_1_8400_6f008400() {
    // Encoding: 0x6F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=1, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: op=0, Q=1, immh=0, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x6F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_2_8400_2f008400() {
    // Encoding: 0x2F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, op=0, immb=0, immh=0
    let encoding: u32 = 0x2F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_3_8400_2f088400() {
    // Encoding: 0x2F088400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=1, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, immh=1, immb=0, op=0, Rn=0, Q=0
    let encoding: u32 = 0x2F088400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_4_8400_2f188400() {
    // Encoding: 0x2F188400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=3, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, immb=0, Rn=0, Rd=0, immh=3, op=0
    let encoding: u32 = 0x2F188400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_5_8400_2f208400() {
    // Encoding: 0x2F208400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=4, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=4, immb=0, Q=0, Rn=0, Rd=0, op=0
    let encoding: u32 = 0x2F208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_6_8400_2f388400() {
    // Encoding: 0x2F388400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=7, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, immh=7, op=0, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x2F388400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_7_8400_2f408400() {
    // Encoding: 0x2F408400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=8, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=8, Rn=0, Rd=0, op=0, Q=0, immb=0
    let encoding: u32 = 0x2F408400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_8_8400_2f788400() {
    // Encoding: 0x2F788400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=15, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, immb=0, immh=15, Rd=0, op=0
    let encoding: u32 = 0x2F788400;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_9_8400_2f008400() {
    // Encoding: 0x2F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, op=0, immh=0, immb=0, Q=0
    let encoding: u32 = 0x2F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_10_8400_2f018400() {
    // Encoding: 0x2F018400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=1, op=0, Rn=0, Rd=0
    // Fields: Rd=0, op=0, Rn=0, Q=0, immh=0, immb=1
    let encoding: u32 = 0x2F018400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_11_8400_2f038400() {
    // Encoding: 0x2F038400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=3, op=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Q=0, immb=3, immh=0, op=0
    let encoding: u32 = 0x2F038400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_12_8400_2f078400() {
    // Encoding: 0x2F078400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=7, op=0, Rn=0, Rd=0
    // Fields: immb=7, Rd=0, immh=0, op=0, Rn=0, Q=0
    let encoding: u32 = 0x2F078400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_13_8400_2f008400() {
    // Encoding: 0x2F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, op=0, Rd=0, Q=0, immh=0, Rn=0
    let encoding: u32 = 0x2F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_14_8400_2f008c00() {
    // Encoding: 0x2F008C00
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=1, Rn=0, Rd=0
    // Fields: immb=0, op=1, Rn=0, immh=0, Q=0, Rd=0
    let encoding: u32 = 0x2F008C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_15_8400_2f008400() {
    // Encoding: 0x2F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, op=0, Rn=0, immh=0, immb=0, Rd=0
    let encoding: u32 = 0x2F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_16_8400_2f008420() {
    // Encoding: 0x2F008420
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=1, Rd=0
    // Fields: Rn=1, Rd=0, op=0, immh=0, Q=0, immb=0
    let encoding: u32 = 0x2F008420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_17_8400_2f0087c0() {
    // Encoding: 0x2F0087C0
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=30, Rd=0
    // Fields: Q=0, immh=0, immb=0, op=0, Rn=30, Rd=0
    let encoding: u32 = 0x2F0087C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_18_8400_2f0087e0() {
    // Encoding: 0x2F0087E0
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=31, Rd=0
    // Fields: Rn=31, immh=0, immb=0, Q=0, Rd=0, op=0
    let encoding: u32 = 0x2F0087E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_19_8400_2f008400() {
    // Encoding: 0x2F008400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=0, Rd=0, immb=0, Q=0, op=0, Rn=0
    let encoding: u32 = 0x2F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_20_8400_2f008401() {
    // Encoding: 0x2F008401
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=1
    // Fields: immb=0, immh=0, Q=0, op=0, Rn=0, Rd=1
    let encoding: u32 = 0x2F008401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_21_8400_2f00841e() {
    // Encoding: 0x2F00841E
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=30
    // Fields: immh=0, op=0, Rd=30, immb=0, Rn=0, Q=0
    let encoding: u32 = 0x2F00841E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_22_8400_2f00841f() {
    // Encoding: 0x2F00841F
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=31
    // Fields: op=0, Q=0, Rn=0, Rd=31, immb=0, immh=0
    let encoding: u32 = 0x2F00841F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_23_8400_2f008421() {
    // Encoding: 0x2F008421
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=1, Rd=1
    // Fields: Q=0, Rd=1, immb=0, Rn=1, op=0, immh=0
    let encoding: u32 = 0x2F008421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_combo_24_8400_2f0087ff() {
    // Encoding: 0x2F0087FF
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd field combination: Q=0, immh=0, immb=0, op=0, Rn=31, Rd=31
    // Fields: op=0, Rn=31, Rd=31, immh=0, immb=0, Q=0
    let encoding: u32 = 0x2F0087FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_special_q_0_size_variant_0_33792_2f098400()
 {
    // Encoding: 0x2F098400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd special value Q = 0 (Size variant 0)
    // Fields: Rn=0, immb=1, Q=0, Rd=0, op=0, immh=1
    let encoding: u32 = 0x2F098400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_special_q_1_size_variant_1_33792_6f098400()
 {
    // Encoding: 0x6F098400
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd special value Q = 1 (Size variant 1)
    // Fields: Rd=0, op=0, Rn=0, immb=1, immh=1, Q=1
    let encoding: u32 = 0x6F098400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_special_rn_31_stack_pointer_sp_may_require_alignment_33792_2f0987e0()
 {
    // Encoding: 0x2F0987E0
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, immh=1, Q=0, op=0, Rn=31, immb=1
    let encoding: u32 = 0x2F0987E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_33792_2f09841f()
 {
    // Encoding: 0x2F09841F
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, Rn=0, Q=0, immb=1, op=0, immh=1
    let encoding: u32 = 0x2F09841F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_sp_rn_7f0087e0() {
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd with Rn = SP (31)
    // Encoding: 0x7F0087E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7F0087E0;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_sisd_zr_rd_7f00841f() {
    // Test aarch64_vector_shift_right_narrow_nonuniform_sisd with Rd = ZR (31)
    // Encoding: 0x7F00841F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7F00841F;
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

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_sp_rn_2f0087e0() {
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd with Rn = SP (31)
    // Encoding: 0x2F0087E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2F0087E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_right_narrow_nonuniform_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_right_narrow_nonuniform_simd_zr_rd_2f00841f() {
    // Test aarch64_vector_shift_right_narrow_nonuniform_simd with Rd = ZR (31)
    // Encoding: 0x2F00841F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2F00841F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_shift_right_narrow_uniform_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_u_0_min_9400_5f009400() {
    // Encoding: 0x5F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field U = 0 (Min)
    // Fields: immh=0, immb=0, U=0, op=0, Rd=0, Rn=0
    let encoding: u32 = 0x5F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_u_1_max_9400_7f009400() {
    // Encoding: 0x7F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field U = 1 (Max)
    // Fields: U=1, immh=0, immb=0, Rn=0, Rd=0, op=0
    let encoding: u32 = 0x7F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_immh_0_zero_9400_5f009400() {
    // Encoding: 0x5F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field immh = 0 (Zero)
    // Fields: op=0, U=0, immh=0, Rd=0, immb=0, Rn=0
    let encoding: u32 = 0x5F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_immh_1_poweroftwo_9400_5f089400() {
    // Encoding: 0x5F089400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field immh = 1 (PowerOfTwo)
    // Fields: U=0, op=0, immb=0, immh=1, Rd=0, Rn=0
    let encoding: u32 = 0x5F089400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_immh_3_poweroftwominusone_9400_5f189400()
 {
    // Encoding: 0x5F189400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: U=0, Rd=0, op=0, immb=0, immh=3, Rn=0
    let encoding: u32 = 0x5F189400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_immh_4_poweroftwo_9400_5f209400() {
    // Encoding: 0x5F209400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field immh = 4 (PowerOfTwo)
    // Fields: Rd=0, Rn=0, immh=4, op=0, immb=0, U=0
    let encoding: u32 = 0x5F209400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_immh_7_poweroftwominusone_9400_5f389400()
 {
    // Encoding: 0x5F389400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: immh=7, U=0, op=0, Rn=0, Rd=0, immb=0
    let encoding: u32 = 0x5F389400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_immh_8_poweroftwo_9400_5f409400() {
    // Encoding: 0x5F409400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field immh = 8 (PowerOfTwo)
    // Fields: Rn=0, immb=0, U=0, immh=8, op=0, Rd=0
    let encoding: u32 = 0x5F409400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_immh_15_max_9400_5f789400() {
    // Encoding: 0x5F789400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field immh = 15 (Max)
    // Fields: op=0, U=0, immh=15, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x5F789400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_immb_0_zero_9400_5f009400() {
    // Encoding: 0x5F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field immb = 0 (Zero)
    // Fields: immb=0, Rd=0, immh=0, op=0, U=0, Rn=0
    let encoding: u32 = 0x5F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_immb_1_poweroftwo_9400_5f019400() {
    // Encoding: 0x5F019400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field immb = 1 (PowerOfTwo)
    // Fields: op=0, Rd=0, Rn=0, immb=1, U=0, immh=0
    let encoding: u32 = 0x5F019400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_immb_3_poweroftwominusone_9400_5f039400()
 {
    // Encoding: 0x5F039400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: immb=3, Rd=0, op=0, U=0, immh=0, Rn=0
    let encoding: u32 = 0x5F039400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_immb_7_max_9400_5f079400() {
    // Encoding: 0x5F079400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field immb = 7 (Max)
    // Fields: immh=0, Rd=0, Rn=0, op=0, U=0, immb=7
    let encoding: u32 = 0x5F079400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field op 11 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_op_0_min_9400_5f009400() {
    // Encoding: 0x5F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field op = 0 (Min)
    // Fields: Rn=0, Rd=0, immh=0, op=0, U=0, immb=0
    let encoding: u32 = 0x5F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field op 11 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_op_1_max_9400_5f009c00() {
    // Encoding: 0x5F009C00
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field op = 1 (Max)
    // Fields: op=1, Rn=0, Rd=0, immh=0, U=0, immb=0
    let encoding: u32 = 0x5F009C00;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_rn_0_min_9400_5f009400() {
    // Encoding: 0x5F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field Rn = 0 (Min)
    // Fields: immh=0, immb=0, op=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x5F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_rn_1_poweroftwo_9400_5f009420() {
    // Encoding: 0x5F009420
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, immh=0, U=0, immb=0, op=0, Rn=1
    let encoding: u32 = 0x5F009420;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_rn_30_poweroftwominusone_9400_5f0097c0()
 {
    // Encoding: 0x5F0097C0
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: op=0, immh=0, Rd=0, immb=0, Rn=30, U=0
    let encoding: u32 = 0x5F0097C0;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_rn_31_max_9400_5f0097e0() {
    // Encoding: 0x5F0097E0
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field Rn = 31 (Max)
    // Fields: op=0, Rn=31, Rd=0, immh=0, U=0, immb=0
    let encoding: u32 = 0x5F0097E0;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_rd_0_min_9400_5f009400() {
    // Encoding: 0x5F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field Rd = 0 (Min)
    // Fields: Rd=0, immh=0, op=0, immb=0, Rn=0, U=0
    let encoding: u32 = 0x5F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_rd_1_poweroftwo_9400_5f009401() {
    // Encoding: 0x5F009401
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, U=0, immh=0, immb=0, Rd=1, op=0
    let encoding: u32 = 0x5F009401;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_rd_30_poweroftwominusone_9400_5f00941e()
 {
    // Encoding: 0x5F00941E
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, Rn=0, op=0, immh=0, U=0, immb=0
    let encoding: u32 = 0x5F00941E;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_field_rd_31_max_9400_5f00941f() {
    // Encoding: 0x5F00941F
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field Rd = 31 (Max)
    // Fields: immh=0, U=0, Rn=0, op=0, Rd=31, immb=0
    let encoding: u32 = 0x5F00941F;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_0_9400_5f009400() {
    // Encoding: 0x5F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=0, immb=0, Rd=0, Rn=0, U=0, op=0
    let encoding: u32 = 0x5F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_1_9400_7f009400() {
    // Encoding: 0x7F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=1, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=0, Rd=0, op=0, Rn=0, immb=0, U=1
    let encoding: u32 = 0x7F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_2_9400_5f009400() {
    // Encoding: 0x5F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: op=0, Rn=0, immh=0, Rd=0, U=0, immb=0
    let encoding: u32 = 0x5F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_3_9400_5f089400() {
    // Encoding: 0x5F089400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=1, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=1, immb=0, Rn=0, Rd=0, op=0, U=0
    let encoding: u32 = 0x5F089400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_4_9400_5f189400() {
    // Encoding: 0x5F189400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=3, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, immh=3, U=0, Rd=0, op=0
    let encoding: u32 = 0x5F189400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_5_9400_5f209400() {
    // Encoding: 0x5F209400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=4, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=4, op=0, Rd=0, Rn=0, U=0, immb=0
    let encoding: u32 = 0x5F209400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_6_9400_5f389400() {
    // Encoding: 0x5F389400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=7, immb=0, op=0, Rn=0, Rd=0
    // Fields: U=0, immb=0, Rd=0, immh=7, op=0, Rn=0
    let encoding: u32 = 0x5F389400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_7_9400_5f409400() {
    // Encoding: 0x5F409400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=8, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, op=0, Rn=0, U=0, immh=8, immb=0
    let encoding: u32 = 0x5F409400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_8_9400_5f789400() {
    // Encoding: 0x5F789400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=15, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, immb=0, op=0, U=0, Rd=0, immh=15
    let encoding: u32 = 0x5F789400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_9_9400_5f009400() {
    // Encoding: 0x5F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=0, op=0, Rd=0, Rn=0, immb=0, U=0
    let encoding: u32 = 0x5F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_10_9400_5f019400() {
    // Encoding: 0x5F019400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=1, op=0, Rn=0, Rd=0
    // Fields: immh=0, immb=1, op=0, U=0, Rd=0, Rn=0
    let encoding: u32 = 0x5F019400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_11_9400_5f039400() {
    // Encoding: 0x5F039400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=3, op=0, Rn=0, Rd=0
    // Fields: immb=3, immh=0, op=0, U=0, Rd=0, Rn=0
    let encoding: u32 = 0x5F039400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_12_9400_5f079400() {
    // Encoding: 0x5F079400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=7, op=0, Rn=0, Rd=0
    // Fields: immh=0, Rn=0, U=0, immb=7, Rd=0, op=0
    let encoding: u32 = 0x5F079400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_13_9400_5f009400() {
    // Encoding: 0x5F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=0, immb=0, op=0, Rn=0, U=0, Rd=0
    let encoding: u32 = 0x5F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_14_9400_5f009c00() {
    // Encoding: 0x5F009C00
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=1, Rn=0, Rd=0
    // Fields: Rd=0, immb=0, U=0, Rn=0, immh=0, op=1
    let encoding: u32 = 0x5F009C00;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_15_9400_5f009400() {
    // Encoding: 0x5F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, op=0, immb=0, U=0, Rn=0, immh=0
    let encoding: u32 = 0x5F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_16_9400_5f009420() {
    // Encoding: 0x5F009420
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=1, Rd=0
    // Fields: immh=0, op=0, immb=0, U=0, Rn=1, Rd=0
    let encoding: u32 = 0x5F009420;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_17_9400_5f0097c0() {
    // Encoding: 0x5F0097C0
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=30, Rd=0
    // Fields: Rd=0, immh=0, immb=0, op=0, Rn=30, U=0
    let encoding: u32 = 0x5F0097C0;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_18_9400_5f0097e0() {
    // Encoding: 0x5F0097E0
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=31, Rd=0
    // Fields: U=0, op=0, immh=0, Rd=0, Rn=31, immb=0
    let encoding: u32 = 0x5F0097E0;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_19_9400_5f009400() {
    // Encoding: 0x5F009400
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, U=0, op=0, immh=0, Rd=0
    let encoding: u32 = 0x5F009400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_20_9400_5f009401() {
    // Encoding: 0x5F009401
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=1
    // Fields: immh=0, U=0, Rd=1, op=0, immb=0, Rn=0
    let encoding: u32 = 0x5F009401;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_21_9400_5f00941e() {
    // Encoding: 0x5F00941E
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=30
    // Fields: op=0, immh=0, immb=0, Rn=0, U=0, Rd=30
    let encoding: u32 = 0x5F00941E;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_22_9400_5f00941f() {
    // Encoding: 0x5F00941F
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=31
    // Fields: op=0, Rd=31, Rn=0, U=0, immb=0, immh=0
    let encoding: u32 = 0x5F00941F;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_23_9400_5f009421() {
    // Encoding: 0x5F009421
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=1, Rd=1
    // Fields: op=0, immh=0, Rd=1, immb=0, Rn=1, U=0
    let encoding: u32 = 0x5F009421;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_combo_24_9400_5f0097ff() {
    // Encoding: 0x5F0097FF
    // Test aarch64_vector_shift_right_narrow_uniform_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=31, Rd=31
    // Fields: immh=0, immb=0, Rn=31, U=0, Rd=31, op=0
    let encoding: u32 = 0x5F0097FF;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_37888_5f0997e0()
 {
    // Encoding: 0x5F0997E0
    // Test aarch64_vector_shift_right_narrow_uniform_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: immh=1, Rd=0, immb=1, Rn=31, U=0, op=0
    let encoding: u32 = 0x5F0997E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_37888_5f09941f()
 {
    // Encoding: 0x5F09941F
    // Test aarch64_vector_shift_right_narrow_uniform_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: op=0, immh=1, U=0, Rn=0, Rd=31, immb=1
    let encoding: u32 = 0x5F09941F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_q_0_min_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field Q = 0 (Min)
    // Fields: U=0, Rn=0, immh=0, Rd=0, Q=0, immb=0, op=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_q_1_max_9400_4f009400() {
    // Encoding: 0x4F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field Q = 1 (Max)
    // Fields: U=0, Q=1, immb=0, op=0, immh=0, Rn=0, Rd=0
    let encoding: u32 = 0x4F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_u_0_min_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field U = 0 (Min)
    // Fields: op=0, U=0, Rd=0, Q=0, immh=0, Rn=0, immb=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_u_1_max_9400_2f009400() {
    // Encoding: 0x2F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field U = 1 (Max)
    // Fields: Rd=0, Q=0, immb=0, immh=0, op=0, U=1, Rn=0
    let encoding: u32 = 0x2F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_immh_0_zero_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field immh = 0 (Zero)
    // Fields: immh=0, U=0, op=0, Rn=0, immb=0, Rd=0, Q=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_immh_1_poweroftwo_9400_0f089400() {
    // Encoding: 0x0F089400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field immh = 1 (PowerOfTwo)
    // Fields: U=0, op=0, immh=1, immb=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0F089400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_immh_3_poweroftwominusone_9400_0f189400()
 {
    // Encoding: 0x0F189400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: op=0, Q=0, U=0, immh=3, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x0F189400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_immh_4_poweroftwo_9400_0f209400() {
    // Encoding: 0x0F209400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field immh = 4 (PowerOfTwo)
    // Fields: Q=0, U=0, immh=4, immb=0, Rn=0, op=0, Rd=0
    let encoding: u32 = 0x0F209400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_immh_7_poweroftwominusone_9400_0f389400()
 {
    // Encoding: 0x0F389400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: immh=7, op=0, immb=0, Rn=0, Rd=0, Q=0, U=0
    let encoding: u32 = 0x0F389400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_immh_8_poweroftwo_9400_0f409400() {
    // Encoding: 0x0F409400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field immh = 8 (PowerOfTwo)
    // Fields: Q=0, Rn=0, immh=8, Rd=0, U=0, immb=0, op=0
    let encoding: u32 = 0x0F409400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_immh_15_max_9400_0f789400() {
    // Encoding: 0x0F789400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field immh = 15 (Max)
    // Fields: Rd=0, Q=0, op=0, Rn=0, immh=15, U=0, immb=0
    let encoding: u32 = 0x0F789400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_immb_0_zero_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field immb = 0 (Zero)
    // Fields: U=0, Rn=0, immb=0, op=0, immh=0, Rd=0, Q=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_immb_1_poweroftwo_9400_0f019400() {
    // Encoding: 0x0F019400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field immb = 1 (PowerOfTwo)
    // Fields: Rd=0, U=0, Rn=0, op=0, immh=0, Q=0, immb=1
    let encoding: u32 = 0x0F019400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_immb_3_poweroftwominusone_9400_0f039400()
 {
    // Encoding: 0x0F039400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: immh=0, Rd=0, Q=0, immb=3, op=0, U=0, Rn=0
    let encoding: u32 = 0x0F039400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_immb_7_max_9400_0f079400() {
    // Encoding: 0x0F079400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field immb = 7 (Max)
    // Fields: immh=0, Rn=0, U=0, Q=0, immb=7, op=0, Rd=0
    let encoding: u32 = 0x0F079400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field op 11 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_op_0_min_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field op = 0 (Min)
    // Fields: U=0, op=0, immh=0, immb=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field op 11 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_op_1_max_9400_0f009c00() {
    // Encoding: 0x0F009C00
    // Test aarch64_vector_shift_right_narrow_uniform_simd field op = 1 (Max)
    // Fields: Q=0, Rd=0, immh=0, immb=0, U=0, op=1, Rn=0
    let encoding: u32 = 0x0F009C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_rn_0_min_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field Rn = 0 (Min)
    // Fields: Q=0, U=0, immh=0, Rn=0, immb=0, op=0, Rd=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_rn_1_poweroftwo_9400_0f009420() {
    // Encoding: 0x0F009420
    // Test aarch64_vector_shift_right_narrow_uniform_simd field Rn = 1 (PowerOfTwo)
    // Fields: Q=0, immh=0, immb=0, U=0, Rd=0, Rn=1, op=0
    let encoding: u32 = 0x0F009420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_rn_30_poweroftwominusone_9400_0f0097c0()
 {
    // Encoding: 0x0F0097C0
    // Test aarch64_vector_shift_right_narrow_uniform_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, U=0, immb=0, op=0, immh=0, Rn=30, Rd=0
    let encoding: u32 = 0x0F0097C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_rn_31_max_9400_0f0097e0() {
    // Encoding: 0x0F0097E0
    // Test aarch64_vector_shift_right_narrow_uniform_simd field Rn = 31 (Max)
    // Fields: immh=0, Rn=31, immb=0, Rd=0, op=0, Q=0, U=0
    let encoding: u32 = 0x0F0097E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_rd_0_min_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field Rd = 0 (Min)
    // Fields: immh=0, immb=0, Rd=0, U=0, Rn=0, op=0, Q=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_rd_1_poweroftwo_9400_0f009401() {
    // Encoding: 0x0F009401
    // Test aarch64_vector_shift_right_narrow_uniform_simd field Rd = 1 (PowerOfTwo)
    // Fields: U=0, Rn=0, immb=0, op=0, Rd=1, immh=0, Q=0
    let encoding: u32 = 0x0F009401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_rd_30_poweroftwominusone_9400_0f00941e()
 {
    // Encoding: 0x0F00941E
    // Test aarch64_vector_shift_right_narrow_uniform_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, Q=0, U=0, immb=0, op=0, Rn=0, immh=0
    let encoding: u32 = 0x0F00941E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_field_rd_31_max_9400_0f00941f() {
    // Encoding: 0x0F00941F
    // Test aarch64_vector_shift_right_narrow_uniform_simd field Rd = 31 (Max)
    // Fields: Rn=0, Rd=31, op=0, U=0, immh=0, Q=0, immb=0
    let encoding: u32 = 0x0F00941F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_0_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, U=0, Rn=0, Rd=0, op=0, immh=0, Q=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_1_9400_4f009400() {
    // Encoding: 0x4F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=1, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=1, immh=0, op=0, immb=0, U=0
    let encoding: u32 = 0x4F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_2_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: op=0, Rd=0, Rn=0, U=0, immh=0, immb=0, Q=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_3_9400_2f009400() {
    // Encoding: 0x2F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=1, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, immb=0, Rn=0, Rd=0, op=0, immh=0, U=1
    let encoding: u32 = 0x2F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_4_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, op=0, Rd=0, immh=0, immb=0, U=0, Rn=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_5_9400_0f089400() {
    // Encoding: 0x0F089400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=1, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=1, immb=0, op=0, Rn=0, Rd=0, Q=0, U=0
    let encoding: u32 = 0x0F089400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_6_9400_0f189400() {
    // Encoding: 0x0F189400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=3, immb=0, op=0, Rn=0, Rd=0
    // Fields: U=0, immh=3, Rd=0, immb=0, op=0, Rn=0, Q=0
    let encoding: u32 = 0x0F189400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_7_9400_0f209400() {
    // Encoding: 0x0F209400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=4, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, U=0, Q=0, op=0, Rn=0, immh=4, Rd=0
    let encoding: u32 = 0x0F209400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_8_9400_0f389400() {
    // Encoding: 0x0F389400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=7, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, immb=0, op=0, Rn=0, Rd=0, immh=7
    let encoding: u32 = 0x0F389400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_9_9400_0f409400() {
    // Encoding: 0x0F409400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=8, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, op=0, Rd=0, Rn=0, U=0, immh=8, immb=0
    let encoding: u32 = 0x0F409400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_10_9400_0f789400() {
    // Encoding: 0x0F789400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=15, immb=0, op=0, Rn=0, Rd=0
    // Fields: U=0, Q=0, immh=15, immb=0, op=0, Rd=0, Rn=0
    let encoding: u32 = 0x0F789400;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_11_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: U=0, Rn=0, Rd=0, immh=0, Q=0, immb=0, op=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_12_9400_0f019400() {
    // Encoding: 0x0F019400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=1, op=0, Rn=0, Rd=0
    // Fields: op=0, immh=0, Rn=0, immb=1, U=0, Rd=0, Q=0
    let encoding: u32 = 0x0F019400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_13_9400_0f039400() {
    // Encoding: 0x0F039400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=3, op=0, Rn=0, Rd=0
    // Fields: op=0, Rd=0, U=0, Q=0, immh=0, immb=3, Rn=0
    let encoding: u32 = 0x0F039400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_14_9400_0f079400() {
    // Encoding: 0x0F079400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=7, op=0, Rn=0, Rd=0
    // Fields: op=0, U=0, immh=0, Q=0, Rn=0, Rd=0, immb=7
    let encoding: u32 = 0x0F079400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_15_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=0, Q=0, immb=0, op=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_16_9400_0f009c00() {
    // Encoding: 0x0F009C00
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=1, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, op=1, Q=0, U=0, immh=0, Rd=0
    let encoding: u32 = 0x0F009C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_17_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, immb=0, Rd=0, Q=0, U=0, immh=0, op=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_18_9400_0f009420() {
    // Encoding: 0x0F009420
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=1, Rd=0
    // Fields: immh=0, immb=0, Q=0, op=0, Rd=0, U=0, Rn=1
    let encoding: u32 = 0x0F009420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_19_9400_0f0097c0() {
    // Encoding: 0x0F0097C0
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=30, Rd=0
    // Fields: immb=0, op=0, Q=0, Rd=0, Rn=30, immh=0, U=0
    let encoding: u32 = 0x0F0097C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_20_9400_0f0097e0() {
    // Encoding: 0x0F0097E0
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=31, Rd=0
    // Fields: immb=0, op=0, Rd=0, Rn=31, Q=0, immh=0, U=0
    let encoding: u32 = 0x0F0097E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_21_9400_0f009400() {
    // Encoding: 0x0F009400
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, U=0, immh=0, Rd=0, immb=0, op=0, Q=0
    let encoding: u32 = 0x0F009400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_22_9400_0f009401() {
    // Encoding: 0x0F009401
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=1
    // Fields: U=0, Q=0, Rn=0, immh=0, immb=0, Rd=1, op=0
    let encoding: u32 = 0x0F009401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_23_9400_0f00941e() {
    // Encoding: 0x0F00941E
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=30
    // Fields: Q=0, immh=0, Rn=0, Rd=30, U=0, immb=0, op=0
    let encoding: u32 = 0x0F00941E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_24_9400_0f00941f() {
    // Encoding: 0x0F00941F
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=31
    // Fields: immb=0, op=0, Rn=0, U=0, Rd=31, immh=0, Q=0
    let encoding: u32 = 0x0F00941F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_25_9400_0f009421() {
    // Encoding: 0x0F009421
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=1, Rd=1
    // Fields: immh=0, op=0, U=0, Q=0, immb=0, Rn=1, Rd=1
    let encoding: u32 = 0x0F009421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_combo_26_9400_0f0097ff() {
    // Encoding: 0x0F0097FF
    // Test aarch64_vector_shift_right_narrow_uniform_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=31, Rd=31
    // Fields: Rd=31, immh=0, Q=0, immb=0, op=0, U=0, Rn=31
    let encoding: u32 = 0x0F0097FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_special_q_0_size_variant_0_37888_0f099400() {
    // Encoding: 0x0F099400
    // Test aarch64_vector_shift_right_narrow_uniform_simd special value Q = 0 (Size variant 0)
    // Fields: immh=1, Q=0, immb=1, op=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0F099400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_special_q_1_size_variant_1_37888_4f099400() {
    // Encoding: 0x4F099400
    // Test aarch64_vector_shift_right_narrow_uniform_simd special value Q = 1 (Size variant 1)
    // Fields: U=0, Q=1, Rn=0, immb=1, Rd=0, op=0, immh=1
    let encoding: u32 = 0x4F099400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_special_rn_31_stack_pointer_sp_may_require_alignment_37888_0f0997e0()
 {
    // Encoding: 0x0F0997E0
    // Test aarch64_vector_shift_right_narrow_uniform_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: immh=1, op=0, immb=1, Q=0, Rd=0, U=0, Rn=31
    let encoding: u32 = 0x0F0997E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_37888_0f09941f()
 {
    // Encoding: 0x0F09941F
    // Test aarch64_vector_shift_right_narrow_uniform_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, op=0, Q=0, immh=1, immb=1, Rd=31, U=0
    let encoding: u32 = 0x0F09941F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_sp_rn_5f0097e0() {
    // Test aarch64_vector_shift_right_narrow_uniform_sisd with Rn = SP (31)
    // Encoding: 0x5F0097E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F0097E0;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_sisd_zr_rd_5f00941f() {
    // Test aarch64_vector_shift_right_narrow_uniform_sisd with Rd = ZR (31)
    // Encoding: 0x5F00941F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F00941F;
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

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_sp_rn_0f0097e0() {
    // Test aarch64_vector_shift_right_narrow_uniform_simd with Rn = SP (31)
    // Encoding: 0x0F0097E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F0097E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_right_narrow_uniform_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_right_narrow_uniform_simd_zr_rd_0f00941f() {
    // Test aarch64_vector_shift_right_narrow_uniform_simd with Rd = ZR (31)
    // Encoding: 0x0F00941F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00941F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_shift_left_long Tests
// ============================================================================

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_shift_left_long_field_q_0_min_a400_0f00a400() {
    // Encoding: 0x0F00A400
    // Test aarch64_vector_shift_left_long field Q = 0 (Min)
    // Fields: immb=0, Q=0, immh=0, U=0, Rd=0, Rn=0
    let encoding: u32 = 0x0F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_shift_left_long_field_q_1_max_a400_4f00a400() {
    // Encoding: 0x4F00A400
    // Test aarch64_vector_shift_left_long field Q = 1 (Max)
    // Fields: immb=0, Q=1, Rn=0, immh=0, Rd=0, U=0
    let encoding: u32 = 0x4F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_left_long_field_u_0_min_a400_0f00a400() {
    // Encoding: 0x0F00A400
    // Test aarch64_vector_shift_left_long field U = 0 (Min)
    // Fields: Q=0, Rd=0, U=0, immh=0, immb=0, Rn=0
    let encoding: u32 = 0x0F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_left_long_field_u_1_max_a400_2f00a400() {
    // Encoding: 0x2F00A400
    // Test aarch64_vector_shift_left_long field U = 1 (Max)
    // Fields: Q=0, U=1, immb=0, immh=0, Rn=0, Rd=0
    let encoding: u32 = 0x2F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_long_field_immh_0_zero_a400_0f00a400() {
    // Encoding: 0x0F00A400
    // Test aarch64_vector_shift_left_long field immh = 0 (Zero)
    // Fields: immh=0, U=0, Rd=0, immb=0, Rn=0, Q=0
    let encoding: u32 = 0x0F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_long_field_immh_1_poweroftwo_a400_0f08a400() {
    // Encoding: 0x0F08A400
    // Test aarch64_vector_shift_left_long field immh = 1 (PowerOfTwo)
    // Fields: immh=1, Rn=0, Q=0, immb=0, Rd=0, U=0
    let encoding: u32 = 0x0F08A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_left_long_field_immh_3_poweroftwominusone_a400_0f18a400() {
    // Encoding: 0x0F18A400
    // Test aarch64_vector_shift_left_long field immh = 3 (PowerOfTwoMinusOne)
    // Fields: Rn=0, immh=3, Rd=0, U=0, immb=0, Q=0
    let encoding: u32 = 0x0F18A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_left_long_field_immh_4_poweroftwo_a400_0f20a400() {
    // Encoding: 0x0F20A400
    // Test aarch64_vector_shift_left_long field immh = 4 (PowerOfTwo)
    // Fields: immb=0, Rn=0, Q=0, U=0, immh=4, Rd=0
    let encoding: u32 = 0x0F20A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_left_long_field_immh_7_poweroftwominusone_a400_0f38a400() {
    // Encoding: 0x0F38A400
    // Test aarch64_vector_shift_left_long field immh = 7 (PowerOfTwoMinusOne)
    // Fields: U=0, immh=7, immb=0, Rd=0, Rn=0, Q=0
    let encoding: u32 = 0x0F38A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_left_long_field_immh_8_poweroftwo_a400_0f40a400() {
    // Encoding: 0x0F40A400
    // Test aarch64_vector_shift_left_long field immh = 8 (PowerOfTwo)
    // Fields: Q=0, immh=8, U=0, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x0F40A400;
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

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_left_long_field_immh_15_max_a400_0f78a400() {
    // Encoding: 0x0F78A400
    // Test aarch64_vector_shift_left_long field immh = 15 (Max)
    // Fields: Rd=0, immb=0, immh=15, Q=0, U=0, Rn=0
    let encoding: u32 = 0x0F78A400;
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

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_long_field_immb_0_zero_a400_0f00a400() {
    // Encoding: 0x0F00A400
    // Test aarch64_vector_shift_left_long field immb = 0 (Zero)
    // Fields: Q=0, Rn=0, immh=0, U=0, immb=0, Rd=0
    let encoding: u32 = 0x0F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_long_field_immb_1_poweroftwo_a400_0f01a400() {
    // Encoding: 0x0F01A400
    // Test aarch64_vector_shift_left_long field immb = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=0, U=0, immh=0, immb=1, Q=0
    let encoding: u32 = 0x0F01A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_left_long_field_immb_3_poweroftwominusone_a400_0f03a400() {
    // Encoding: 0x0F03A400
    // Test aarch64_vector_shift_left_long field immb = 3 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=0, immb=3, U=0, Q=0, immh=0
    let encoding: u32 = 0x0F03A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_left_long_field_immb_7_max_a400_0f07a400() {
    // Encoding: 0x0F07A400
    // Test aarch64_vector_shift_left_long field immb = 7 (Max)
    // Fields: Rd=0, immh=0, Rn=0, Q=0, U=0, immb=7
    let encoding: u32 = 0x0F07A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_long_field_rn_0_min_a400_0f00a400() {
    // Encoding: 0x0F00A400
    // Test aarch64_vector_shift_left_long field Rn = 0 (Min)
    // Fields: U=0, immh=0, Rn=0, Rd=0, immb=0, Q=0
    let encoding: u32 = 0x0F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_long_field_rn_1_poweroftwo_a400_0f00a420() {
    // Encoding: 0x0F00A420
    // Test aarch64_vector_shift_left_long field Rn = 1 (PowerOfTwo)
    // Fields: Q=0, immh=0, immb=0, U=0, Rn=1, Rd=0
    let encoding: u32 = 0x0F00A420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_long_field_rn_30_poweroftwominusone_a400_0f00a7c0() {
    // Encoding: 0x0F00A7C0
    // Test aarch64_vector_shift_left_long field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rd=0, Q=0, U=0, immh=0, immb=0
    let encoding: u32 = 0x0F00A7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_left_long_field_rn_31_max_a400_0f00a7e0() {
    // Encoding: 0x0F00A7E0
    // Test aarch64_vector_shift_left_long field Rn = 31 (Max)
    // Fields: Q=0, immb=0, immh=0, Rd=0, Rn=31, U=0
    let encoding: u32 = 0x0F00A7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_long_field_rd_0_min_a400_0f00a400() {
    // Encoding: 0x0F00A400
    // Test aarch64_vector_shift_left_long field Rd = 0 (Min)
    // Fields: Rd=0, Rn=0, immh=0, U=0, Q=0, immb=0
    let encoding: u32 = 0x0F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_long_field_rd_1_poweroftwo_a400_0f00a401() {
    // Encoding: 0x0F00A401
    // Test aarch64_vector_shift_left_long field Rd = 1 (PowerOfTwo)
    // Fields: immh=0, Q=0, Rn=0, U=0, Rd=1, immb=0
    let encoding: u32 = 0x0F00A401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_long_field_rd_30_poweroftwominusone_a400_0f00a41e() {
    // Encoding: 0x0F00A41E
    // Test aarch64_vector_shift_left_long field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, immb=0, Q=0, Rn=0, Rd=30, immh=0
    let encoding: u32 = 0x0F00A41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_left_long_field_rd_31_max_a400_0f00a41f() {
    // Encoding: 0x0F00A41F
    // Test aarch64_vector_shift_left_long field Rd = 31 (Max)
    // Fields: U=0, immh=0, Rn=0, Rd=31, immb=0, Q=0
    let encoding: u32 = 0x0F00A41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_shift_left_long_combo_0_a400_0f00a400() {
    // Encoding: 0x0F00A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immh=0, U=0, immb=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_shift_left_long_combo_1_a400_4f00a400() {
    // Encoding: 0x4F00A400
    // Test aarch64_vector_shift_left_long field combination: Q=1, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immb=0, U=0, Q=1, Rn=0, Rd=0, immh=0
    let encoding: u32 = 0x4F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_left_long_combo_2_a400_0f00a400() {
    // Encoding: 0x0F00A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immh=0, immb=0, Rn=0, Rd=0, U=0, Q=0
    let encoding: u32 = 0x0F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_left_long_combo_3_a400_2f00a400() {
    // Encoding: 0x2F00A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=1, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immh=0, Q=0, immb=0, Rn=0, Rd=0, U=1
    let encoding: u32 = 0x2F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_long_combo_4_a400_0f00a400() {
    // Encoding: 0x0F00A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, U=0, immh=0, immb=0
    let encoding: u32 = 0x0F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_long_combo_5_a400_0f08a400() {
    // Encoding: 0x0F08A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=1, immb=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rn=0, immb=0, Rd=0, immh=1
    let encoding: u32 = 0x0F08A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_left_long_combo_6_a400_0f18a400() {
    // Encoding: 0x0F18A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=3, immb=0, Rn=0, Rd=0
    // Fields: U=0, Q=0, immb=0, Rd=0, Rn=0, immh=3
    let encoding: u32 = 0x0F18A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_left_long_combo_7_a400_0f20a400() {
    // Encoding: 0x0F20A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=4, immb=0, Rn=0, Rd=0
    // Fields: immh=4, U=0, Q=0, Rn=0, Rd=0, immb=0
    let encoding: u32 = 0x0F20A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_left_long_combo_8_a400_0f38a400() {
    // Encoding: 0x0F38A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=7, immb=0, Rn=0, Rd=0
    // Fields: immh=7, U=0, Rd=0, Rn=0, Q=0, immb=0
    let encoding: u32 = 0x0F38A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_left_long_combo_9_a400_0f40a400() {
    // Encoding: 0x0F40A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=8, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, immh=8, U=0, immb=0
    let encoding: u32 = 0x0F40A400;
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

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_left_long_combo_10_a400_0f78a400() {
    // Encoding: 0x0F78A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=15, immb=0, Rn=0, Rd=0
    // Fields: immb=0, immh=15, U=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0F78A400;
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

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_long_combo_11_a400_0f00a400() {
    // Encoding: 0x0F00A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Q=0, immh=0, U=0, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x0F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_long_combo_12_a400_0f01a400() {
    // Encoding: 0x0F01A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=1, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, immh=0, Q=0, U=0, immb=1
    let encoding: u32 = 0x0F01A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_left_long_combo_13_a400_0f03a400() {
    // Encoding: 0x0F03A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=3, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, immh=0, immb=3, U=0, Rd=0
    let encoding: u32 = 0x0F03A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_left_long_combo_14_a400_0f07a400() {
    // Encoding: 0x0F07A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=7, Rn=0, Rd=0
    // Fields: immb=7, Rd=0, immh=0, Rn=0, U=0, Q=0
    let encoding: u32 = 0x0F07A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_long_combo_15_a400_0f00a400() {
    // Encoding: 0x0F00A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, immb=0, Rd=0, Rn=0, immh=0
    let encoding: u32 = 0x0F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_long_combo_16_a400_0f00a420() {
    // Encoding: 0x0F00A420
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=1, Rd=0
    // Fields: immh=0, Rd=0, Q=0, immb=0, U=0, Rn=1
    let encoding: u32 = 0x0F00A420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_long_combo_17_a400_0f00a7c0() {
    // Encoding: 0x0F00A7C0
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=30, Rd=0
    // Fields: Rn=30, Q=0, immh=0, immb=0, Rd=0, U=0
    let encoding: u32 = 0x0F00A7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_left_long_combo_18_a400_0f00a7e0() {
    // Encoding: 0x0F00A7E0
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=31, Rd=0
    // Fields: immh=0, Q=0, U=0, Rn=31, Rd=0, immb=0
    let encoding: u32 = 0x0F00A7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_long_combo_19_a400_0f00a400() {
    // Encoding: 0x0F00A400
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: U=0, Q=0, immb=0, Rn=0, immh=0, Rd=0
    let encoding: u32 = 0x0F00A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_long_combo_20_a400_0f00a401() {
    // Encoding: 0x0F00A401
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=1
    // Fields: immh=0, U=0, Rn=0, immb=0, Q=0, Rd=1
    let encoding: u32 = 0x0F00A401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_long_combo_21_a400_0f00a41e() {
    // Encoding: 0x0F00A41E
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=30
    // Fields: U=0, immb=0, immh=0, Rn=0, Rd=30, Q=0
    let encoding: u32 = 0x0F00A41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_left_long_combo_22_a400_0f00a41f() {
    // Encoding: 0x0F00A41F
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=31
    // Fields: immh=0, Rn=0, immb=0, U=0, Rd=31, Q=0
    let encoding: u32 = 0x0F00A41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_left_long_combo_23_a400_0f00a421() {
    // Encoding: 0x0F00A421
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=1, Rd=1
    // Fields: U=0, Rd=1, Q=0, immh=0, Rn=1, immb=0
    let encoding: u32 = 0x0F00A421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_left_long_combo_24_a400_0f00a7ff() {
    // Encoding: 0x0F00A7FF
    // Test aarch64_vector_shift_left_long field combination: Q=0, U=0, immh=0, immb=0, Rn=31, Rd=31
    // Fields: immb=0, U=0, Rn=31, Q=0, Rd=31, immh=0
    let encoding: u32 = 0x0F00A7FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_shift_left_long_special_q_0_size_variant_0_41984_0f09a400() {
    // Encoding: 0x0F09A400
    // Test aarch64_vector_shift_left_long special value Q = 0 (Size variant 0)
    // Fields: Rn=0, Rd=0, Q=0, U=0, immh=1, immb=1
    let encoding: u32 = 0x0F09A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_shift_left_long_special_q_1_size_variant_1_41984_4f09a400() {
    // Encoding: 0x4F09A400
    // Test aarch64_vector_shift_left_long special value Q = 1 (Size variant 1)
    // Fields: immb=1, Rn=0, Rd=0, Q=1, U=0, immh=1
    let encoding: u32 = 0x4F09A400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_left_long_special_rn_31_stack_pointer_sp_may_require_alignment_41984_0f09a7e0()
 {
    // Encoding: 0x0F09A7E0
    // Test aarch64_vector_shift_left_long special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: U=0, immb=1, immh=1, Rn=31, Rd=0, Q=0
    let encoding: u32 = 0x0F09A7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_left_long_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_41984_0f09a41f()
 {
    // Encoding: 0x0F09A41F
    // Test aarch64_vector_shift_left_long special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: immb=1, Rn=0, Rd=31, Q=0, U=0, immh=1
    let encoding: u32 = 0x0F09A41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_left_long_reg_write_0_0f00a400() {
    // Test aarch64_vector_shift_left_long register write: SimdFromField("d")
    // Encoding: 0x0F00A400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00A400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_left_long_sp_rn_0f00a7e0() {
    // Test aarch64_vector_shift_left_long with Rn = SP (31)
    // Encoding: 0x0F00A7E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00A7E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_left_long
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_left_long_zr_rd_0f00a41f() {
    // Test aarch64_vector_shift_left_long with Rd = ZR (31)
    // Encoding: 0x0F00A41F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00A41F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_shift_left_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_sisd_field_immh_0_zero_5400_5f005400() {
    // Encoding: 0x5F005400
    // Test aarch64_vector_shift_left_sisd field immh = 0 (Zero)
    // Fields: immh=0, Rn=0, immb=0, Rd=0
    let encoding: u32 = 0x5F005400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_sisd_field_immh_1_poweroftwo_5400_5f085400() {
    // Encoding: 0x5F085400
    // Test aarch64_vector_shift_left_sisd field immh = 1 (PowerOfTwo)
    // Fields: immh=1, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x5F085400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_left_sisd_field_immh_3_poweroftwominusone_5400_5f185400() {
    // Encoding: 0x5F185400
    // Test aarch64_vector_shift_left_sisd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: immh=3, Rn=0, immb=0, Rd=0
    let encoding: u32 = 0x5F185400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_immh_4_poweroftwo_5400_5f205400() {
    // Encoding: 0x5F205400
    // Test aarch64_vector_shift_left_sisd field immh = 4 (PowerOfTwo)
    // Fields: immh=4, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x5F205400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_immh_7_poweroftwominusone_5400_5f385400() {
    // Encoding: 0x5F385400
    // Test aarch64_vector_shift_left_sisd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: immh=7, Rd=0, immb=0, Rn=0
    let encoding: u32 = 0x5F385400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_immh_8_poweroftwo_5400_5f405400() {
    // Encoding: 0x5F405400
    // Test aarch64_vector_shift_left_sisd field immh = 8 (PowerOfTwo)
    // Fields: Rn=0, immb=0, immh=8, Rd=0
    let encoding: u32 = 0x5F405400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_immh_15_max_5400_5f785400() {
    // Encoding: 0x5F785400
    // Test aarch64_vector_shift_left_sisd field immh = 15 (Max)
    // Fields: Rn=0, immh=15, Rd=0, immb=0
    let encoding: u32 = 0x5F785400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_sisd_field_immb_0_zero_5400_5f005400() {
    // Encoding: 0x5F005400
    // Test aarch64_vector_shift_left_sisd field immb = 0 (Zero)
    // Fields: Rd=0, immb=0, Rn=0, immh=0
    let encoding: u32 = 0x5F005400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_sisd_field_immb_1_poweroftwo_5400_5f015400() {
    // Encoding: 0x5F015400
    // Test aarch64_vector_shift_left_sisd field immb = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, immh=0, immb=1
    let encoding: u32 = 0x5F015400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_immb_3_poweroftwominusone_5400_5f035400() {
    // Encoding: 0x5F035400
    // Test aarch64_vector_shift_left_sisd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: immh=0, Rd=0, immb=3, Rn=0
    let encoding: u32 = 0x5F035400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_immb_7_max_5400_5f075400() {
    // Encoding: 0x5F075400
    // Test aarch64_vector_shift_left_sisd field immb = 7 (Max)
    // Fields: Rn=0, Rd=0, immh=0, immb=7
    let encoding: u32 = 0x5F075400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_rn_0_min_5400_5f005400() {
    // Encoding: 0x5F005400
    // Test aarch64_vector_shift_left_sisd field Rn = 0 (Min)
    // Fields: immb=0, Rn=0, immh=0, Rd=0
    let encoding: u32 = 0x5F005400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_rn_1_poweroftwo_5400_5f005420() {
    // Encoding: 0x5F005420
    // Test aarch64_vector_shift_left_sisd field Rn = 1 (PowerOfTwo)
    // Fields: immb=0, Rd=0, Rn=1, immh=0
    let encoding: u32 = 0x5F005420;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_rn_30_poweroftwominusone_5400_5f0057c0() {
    // Encoding: 0x5F0057C0
    // Test aarch64_vector_shift_left_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, immh=0, immb=0, Rn=30
    let encoding: u32 = 0x5F0057C0;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_rn_31_max_5400_5f0057e0() {
    // Encoding: 0x5F0057E0
    // Test aarch64_vector_shift_left_sisd field Rn = 31 (Max)
    // Fields: immh=0, Rd=0, Rn=31, immb=0
    let encoding: u32 = 0x5F0057E0;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_rd_0_min_5400_5f005400() {
    // Encoding: 0x5F005400
    // Test aarch64_vector_shift_left_sisd field Rd = 0 (Min)
    // Fields: Rd=0, immh=0, immb=0, Rn=0
    let encoding: u32 = 0x5F005400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_rd_1_poweroftwo_5400_5f005401() {
    // Encoding: 0x5F005401
    // Test aarch64_vector_shift_left_sisd field Rd = 1 (PowerOfTwo)
    // Fields: immh=0, Rd=1, Rn=0, immb=0
    let encoding: u32 = 0x5F005401;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_rd_30_poweroftwominusone_5400_5f00541e() {
    // Encoding: 0x5F00541E
    // Test aarch64_vector_shift_left_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, immb=0, immh=0, Rn=0
    let encoding: u32 = 0x5F00541E;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_left_sisd_field_rd_31_max_5400_5f00541f() {
    // Encoding: 0x5F00541F
    // Test aarch64_vector_shift_left_sisd field Rd = 31 (Max)
    // Fields: Rn=0, immh=0, immb=0, Rd=31
    let encoding: u32 = 0x5F00541F;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_0_5400_5f005400() {
    // Encoding: 0x5F005400
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=0, Rn=0, Rd=0
    // Fields: immh=0, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x5F005400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_1_5400_5f085400() {
    // Encoding: 0x5F085400
    // Test aarch64_vector_shift_left_sisd field combination: immh=1, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immh=1, immb=0, Rd=0
    let encoding: u32 = 0x5F085400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_2_5400_5f185400() {
    // Encoding: 0x5F185400
    // Test aarch64_vector_shift_left_sisd field combination: immh=3, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immb=0, immh=3, Rd=0
    let encoding: u32 = 0x5F185400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_3_5400_5f205400() {
    // Encoding: 0x5F205400
    // Test aarch64_vector_shift_left_sisd field combination: immh=4, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, immh=4, immb=0
    let encoding: u32 = 0x5F205400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_4_5400_5f385400() {
    // Encoding: 0x5F385400
    // Test aarch64_vector_shift_left_sisd field combination: immh=7, immb=0, Rn=0, Rd=0
    // Fields: immh=7, Rn=0, Rd=0, immb=0
    let encoding: u32 = 0x5F385400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_5_5400_5f405400() {
    // Encoding: 0x5F405400
    // Test aarch64_vector_shift_left_sisd field combination: immh=8, immb=0, Rn=0, Rd=0
    // Fields: immh=8, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x5F405400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_6_5400_5f785400() {
    // Encoding: 0x5F785400
    // Test aarch64_vector_shift_left_sisd field combination: immh=15, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, immh=15, immb=0
    let encoding: u32 = 0x5F785400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_7_5400_5f005400() {
    // Encoding: 0x5F005400
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, immh=0, Rd=0
    let encoding: u32 = 0x5F005400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_8_5400_5f015400() {
    // Encoding: 0x5F015400
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=1, Rn=0, Rd=0
    // Fields: immh=0, immb=1, Rn=0, Rd=0
    let encoding: u32 = 0x5F015400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_9_5400_5f035400() {
    // Encoding: 0x5F035400
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=3, Rn=0, Rd=0
    // Fields: immb=3, Rn=0, Rd=0, immh=0
    let encoding: u32 = 0x5F035400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_10_5400_5f075400() {
    // Encoding: 0x5F075400
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=7, Rn=0, Rd=0
    // Fields: Rd=0, immh=0, Rn=0, immb=7
    let encoding: u32 = 0x5F075400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_11_5400_5f005400() {
    // Encoding: 0x5F005400
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immh=0, immb=0, Rd=0
    let encoding: u32 = 0x5F005400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_12_5400_5f005420() {
    // Encoding: 0x5F005420
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=0, Rn=1, Rd=0
    // Fields: Rd=0, immb=0, Rn=1, immh=0
    let encoding: u32 = 0x5F005420;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_13_5400_5f0057c0() {
    // Encoding: 0x5F0057C0
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=0, Rn=30, Rd=0
    // Fields: immh=0, Rn=30, Rd=0, immb=0
    let encoding: u32 = 0x5F0057C0;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_14_5400_5f0057e0() {
    // Encoding: 0x5F0057E0
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=0, Rn=31, Rd=0
    // Fields: immh=0, immb=0, Rd=0, Rn=31
    let encoding: u32 = 0x5F0057E0;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_15_5400_5f005400() {
    // Encoding: 0x5F005400
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, Rd=0, immh=0
    let encoding: u32 = 0x5F005400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_16_5400_5f005401() {
    // Encoding: 0x5F005401
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=0, Rn=0, Rd=1
    // Fields: Rn=0, immh=0, immb=0, Rd=1
    let encoding: u32 = 0x5F005401;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_17_5400_5f00541e() {
    // Encoding: 0x5F00541E
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=0, Rn=0, Rd=30
    // Fields: immb=0, immh=0, Rn=0, Rd=30
    let encoding: u32 = 0x5F00541E;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_18_5400_5f00541f() {
    // Encoding: 0x5F00541F
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=0, Rn=0, Rd=31
    // Fields: immb=0, Rn=0, immh=0, Rd=31
    let encoding: u32 = 0x5F00541F;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_19_5400_5f005421() {
    // Encoding: 0x5F005421
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=0, Rn=1, Rd=1
    // Fields: Rn=1, immh=0, Rd=1, immb=0
    let encoding: u32 = 0x5F005421;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_left_sisd_combo_20_5400_5f0057ff() {
    // Encoding: 0x5F0057FF
    // Test aarch64_vector_shift_left_sisd field combination: immh=0, immb=0, Rn=31, Rd=31
    // Fields: immh=0, Rn=31, immb=0, Rd=31
    let encoding: u32 = 0x5F0057FF;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_left_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_21504_5f0957e0()
 {
    // Encoding: 0x5F0957E0
    // Test aarch64_vector_shift_left_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: immh=1, Rn=31, immb=1, Rd=0
    let encoding: u32 = 0x5F0957E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_left_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_21504_5f09541f()
 {
    // Encoding: 0x5F09541F
    // Test aarch64_vector_shift_left_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: immh=1, Rd=31, Rn=0, immb=1
    let encoding: u32 = 0x5F09541F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_shift_left_simd_field_q_0_min_5400_0f005400() {
    // Encoding: 0x0F005400
    // Test aarch64_vector_shift_left_simd field Q = 0 (Min)
    // Fields: Q=0, Rd=0, immb=0, immh=0, Rn=0
    let encoding: u32 = 0x0F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_shift_left_simd_field_q_1_max_5400_4f005400() {
    // Encoding: 0x4F005400
    // Test aarch64_vector_shift_left_simd field Q = 1 (Max)
    // Fields: Rn=0, Rd=0, immb=0, immh=0, Q=1
    let encoding: u32 = 0x4F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_simd_field_immh_0_zero_5400_0f005400() {
    // Encoding: 0x0F005400
    // Test aarch64_vector_shift_left_simd field immh = 0 (Zero)
    // Fields: immb=0, Rn=0, immh=0, Rd=0, Q=0
    let encoding: u32 = 0x0F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_simd_field_immh_1_poweroftwo_5400_0f085400() {
    // Encoding: 0x0F085400
    // Test aarch64_vector_shift_left_simd field immh = 1 (PowerOfTwo)
    // Fields: Rd=0, immh=1, Q=0, immb=0, Rn=0
    let encoding: u32 = 0x0F085400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_left_simd_field_immh_3_poweroftwominusone_5400_0f185400() {
    // Encoding: 0x0F185400
    // Test aarch64_vector_shift_left_simd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: Q=0, immb=0, Rd=0, immh=3, Rn=0
    let encoding: u32 = 0x0F185400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_left_simd_field_immh_4_poweroftwo_5400_0f205400() {
    // Encoding: 0x0F205400
    // Test aarch64_vector_shift_left_simd field immh = 4 (PowerOfTwo)
    // Fields: Q=0, Rd=0, Rn=0, immh=4, immb=0
    let encoding: u32 = 0x0F205400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_left_simd_field_immh_7_poweroftwominusone_5400_0f385400() {
    // Encoding: 0x0F385400
    // Test aarch64_vector_shift_left_simd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: immb=0, Q=0, Rn=0, immh=7, Rd=0
    let encoding: u32 = 0x0F385400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_left_simd_field_immh_8_poweroftwo_5400_0f405400() {
    // Encoding: 0x0F405400
    // Test aarch64_vector_shift_left_simd field immh = 8 (PowerOfTwo)
    // Fields: Rd=0, Rn=0, Q=0, immh=8, immb=0
    let encoding: u32 = 0x0F405400;
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

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_left_simd_field_immh_15_max_5400_0f785400() {
    // Encoding: 0x0F785400
    // Test aarch64_vector_shift_left_simd field immh = 15 (Max)
    // Fields: Rn=0, immb=0, Rd=0, immh=15, Q=0
    let encoding: u32 = 0x0F785400;
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

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_simd_field_immb_0_zero_5400_0f005400() {
    // Encoding: 0x0F005400
    // Test aarch64_vector_shift_left_simd field immb = 0 (Zero)
    // Fields: Rn=0, immh=0, Rd=0, immb=0, Q=0
    let encoding: u32 = 0x0F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_simd_field_immb_1_poweroftwo_5400_0f015400() {
    // Encoding: 0x0F015400
    // Test aarch64_vector_shift_left_simd field immb = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, immb=1, Q=0, immh=0
    let encoding: u32 = 0x0F015400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_left_simd_field_immb_3_poweroftwominusone_5400_0f035400() {
    // Encoding: 0x0F035400
    // Test aarch64_vector_shift_left_simd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: immb=3, Rd=0, Rn=0, Q=0, immh=0
    let encoding: u32 = 0x0F035400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_left_simd_field_immb_7_max_5400_0f075400() {
    // Encoding: 0x0F075400
    // Test aarch64_vector_shift_left_simd field immb = 7 (Max)
    // Fields: Rd=0, Q=0, immb=7, Rn=0, immh=0
    let encoding: u32 = 0x0F075400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_simd_field_rn_0_min_5400_0f005400() {
    // Encoding: 0x0F005400
    // Test aarch64_vector_shift_left_simd field Rn = 0 (Min)
    // Fields: Rn=0, immh=0, Q=0, immb=0, Rd=0
    let encoding: u32 = 0x0F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_simd_field_rn_1_poweroftwo_5400_0f005420() {
    // Encoding: 0x0F005420
    // Test aarch64_vector_shift_left_simd field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Q=0, Rn=1, immh=0, immb=0
    let encoding: u32 = 0x0F005420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_simd_field_rn_30_poweroftwominusone_5400_0f0057c0() {
    // Encoding: 0x0F0057C0
    // Test aarch64_vector_shift_left_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rd=0, immb=0, immh=0, Q=0
    let encoding: u32 = 0x0F0057C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_left_simd_field_rn_31_max_5400_0f0057e0() {
    // Encoding: 0x0F0057E0
    // Test aarch64_vector_shift_left_simd field Rn = 31 (Max)
    // Fields: immb=0, Rn=31, Rd=0, immh=0, Q=0
    let encoding: u32 = 0x0F0057E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_simd_field_rd_0_min_5400_0f005400() {
    // Encoding: 0x0F005400
    // Test aarch64_vector_shift_left_simd field Rd = 0 (Min)
    // Fields: immb=0, Q=0, Rn=0, Rd=0, immh=0
    let encoding: u32 = 0x0F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_simd_field_rd_1_poweroftwo_5400_0f005401() {
    // Encoding: 0x0F005401
    // Test aarch64_vector_shift_left_simd field Rd = 1 (PowerOfTwo)
    // Fields: immh=0, immb=0, Q=0, Rn=0, Rd=1
    let encoding: u32 = 0x0F005401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_simd_field_rd_30_poweroftwominusone_5400_0f00541e() {
    // Encoding: 0x0F00541E
    // Test aarch64_vector_shift_left_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: immb=0, Rn=0, Q=0, Rd=30, immh=0
    let encoding: u32 = 0x0F00541E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_left_simd_field_rd_31_max_5400_0f00541f() {
    // Encoding: 0x0F00541F
    // Test aarch64_vector_shift_left_simd field Rd = 31 (Max)
    // Fields: immb=0, Q=0, Rn=0, Rd=31, immh=0
    let encoding: u32 = 0x0F00541F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_shift_left_simd_combo_0_5400_0f005400() {
    // Encoding: 0x0F005400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immh=0, Q=0, Rn=0, Rd=0, immb=0
    let encoding: u32 = 0x0F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_shift_left_simd_combo_1_5400_4f005400() {
    // Encoding: 0x4F005400
    // Test aarch64_vector_shift_left_simd field combination: Q=1, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Q=1, immh=0, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x4F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_simd_combo_2_5400_0f005400() {
    // Encoding: 0x0F005400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Q=0, immh=0, Rn=0, Rd=0, immb=0
    let encoding: u32 = 0x0F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_simd_combo_3_5400_0f085400() {
    // Encoding: 0x0F085400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=1, immb=0, Rn=0, Rd=0
    // Fields: immh=1, Rd=0, immb=0, Q=0, Rn=0
    let encoding: u32 = 0x0F085400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_left_simd_combo_4_5400_0f185400() {
    // Encoding: 0x0F185400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=3, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immh=3, Rd=0, immb=0, Q=0
    let encoding: u32 = 0x0F185400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_5_5400_0f205400() {
    // Encoding: 0x0F205400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=4, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, immh=4, immb=0
    let encoding: u32 = 0x0F205400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_6_5400_0f385400() {
    // Encoding: 0x0F385400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=7, immb=0, Rn=0, Rd=0
    // Fields: immh=7, Rd=0, Q=0, Rn=0, immb=0
    let encoding: u32 = 0x0F385400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_7_5400_0f405400() {
    // Encoding: 0x0F405400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=8, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, Rd=0, immh=8, immb=0
    let encoding: u32 = 0x0F405400;
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

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_8_5400_0f785400() {
    // Encoding: 0x0F785400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=15, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, immh=15, immb=0
    let encoding: u32 = 0x0F785400;
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

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_simd_combo_9_5400_0f005400() {
    // Encoding: 0x0F005400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, immh=0, Q=0, Rn=0, immb=0
    let encoding: u32 = 0x0F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_simd_combo_10_5400_0f015400() {
    // Encoding: 0x0F015400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=1, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, Q=0, immh=0, immb=1
    let encoding: u32 = 0x0F015400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_11_5400_0f035400() {
    // Encoding: 0x0F035400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=3, Rn=0, Rd=0
    // Fields: Q=0, immh=0, Rn=0, Rd=0, immb=3
    let encoding: u32 = 0x0F035400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_12_5400_0f075400() {
    // Encoding: 0x0F075400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=7, Rn=0, Rd=0
    // Fields: immh=0, immb=7, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x0F075400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_13_5400_0f005400() {
    // Encoding: 0x0F005400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, Q=0, immh=0, immb=0
    let encoding: u32 = 0x0F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_14_5400_0f005420() {
    // Encoding: 0x0F005420
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=0, Rn=1, Rd=0
    // Fields: immb=0, Q=0, Rn=1, Rd=0, immh=0
    let encoding: u32 = 0x0F005420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_15_5400_0f0057c0() {
    // Encoding: 0x0F0057C0
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=0, Rn=30, Rd=0
    // Fields: Q=0, Rn=30, immb=0, immh=0, Rd=0
    let encoding: u32 = 0x0F0057C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_16_5400_0f0057e0() {
    // Encoding: 0x0F0057E0
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=0, Rn=31, Rd=0
    // Fields: immh=0, Q=0, Rn=31, Rd=0, immb=0
    let encoding: u32 = 0x0F0057E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_17_5400_0f005400() {
    // Encoding: 0x0F005400
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, immb=0, Rd=0, immh=0
    let encoding: u32 = 0x0F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_18_5400_0f005401() {
    // Encoding: 0x0F005401
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=1
    // Fields: Q=0, immh=0, Rn=0, immb=0, Rd=1
    let encoding: u32 = 0x0F005401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_19_5400_0f00541e() {
    // Encoding: 0x0F00541E
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=30
    // Fields: immb=0, Rd=30, Rn=0, immh=0, Q=0
    let encoding: u32 = 0x0F00541E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_20_5400_0f00541f() {
    // Encoding: 0x0F00541F
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=31
    // Fields: immh=0, Rd=31, Q=0, immb=0, Rn=0
    let encoding: u32 = 0x0F00541F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_21_5400_0f005421() {
    // Encoding: 0x0F005421
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=0, Rn=1, Rd=1
    // Fields: immh=0, Q=0, Rd=1, Rn=1, immb=0
    let encoding: u32 = 0x0F005421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_left_simd_combo_22_5400_0f0057ff() {
    // Encoding: 0x0F0057FF
    // Test aarch64_vector_shift_left_simd field combination: Q=0, immh=0, immb=0, Rn=31, Rd=31
    // Fields: Q=0, Rd=31, Rn=31, immb=0, immh=0
    let encoding: u32 = 0x0F0057FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_shift_left_simd_special_q_0_size_variant_0_21504_0f095400() {
    // Encoding: 0x0F095400
    // Test aarch64_vector_shift_left_simd special value Q = 0 (Size variant 0)
    // Fields: Rn=0, Q=0, Rd=0, immh=1, immb=1
    let encoding: u32 = 0x0F095400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_shift_left_simd_special_q_1_size_variant_1_21504_4f095400() {
    // Encoding: 0x4F095400
    // Test aarch64_vector_shift_left_simd special value Q = 1 (Size variant 1)
    // Fields: Q=1, immb=1, Rn=0, immh=1, Rd=0
    let encoding: u32 = 0x4F095400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_left_simd_special_rn_31_stack_pointer_sp_may_require_alignment_21504_0f0957e0()
 {
    // Encoding: 0x0F0957E0
    // Test aarch64_vector_shift_left_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: immh=1, Rn=31, immb=1, Q=0, Rd=0
    let encoding: u32 = 0x0F0957E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_left_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_21504_0f09541f()
 {
    // Encoding: 0x0F09541F
    // Test aarch64_vector_shift_left_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: immb=1, Rd=31, immh=1, Q=0, Rn=0
    let encoding: u32 = 0x0F09541F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_left_sisd_reg_write_0_5f005400() {
    // Test aarch64_vector_shift_left_sisd register write: SimdFromField("d")
    // Encoding: 0x5F005400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F005400;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_left_sisd_sp_rn_5f0057e0() {
    // Test aarch64_vector_shift_left_sisd with Rn = SP (31)
    // Encoding: 0x5F0057E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F0057E0;
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

/// Provenance: aarch64_vector_shift_left_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_left_sisd_zr_rd_5f00541f() {
    // Test aarch64_vector_shift_left_sisd with Rd = ZR (31)
    // Encoding: 0x5F00541F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F00541F;
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

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_left_simd_reg_write_0_0f005400() {
    // Test aarch64_vector_shift_left_simd register write: SimdFromField("d")
    // Encoding: 0x0F005400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F005400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_left_simd_sp_rn_0f0057e0() {
    // Test aarch64_vector_shift_left_simd with Rn = SP (31)
    // Encoding: 0x0F0057E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F0057E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_left_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_left_simd_zr_rd_0f00541f() {
    // Test aarch64_vector_shift_left_simd with Rd = ZR (31)
    // Encoding: 0x0F00541F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00541F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_shift_left_sat_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_u_0_min_6400_5f006400() {
    // Encoding: 0x5F006400
    // Test aarch64_vector_shift_left_sat_sisd field U = 0 (Min)
    // Fields: op=0, immh=0, immb=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x5F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_u_1_max_6400_7f006400() {
    // Encoding: 0x7F006400
    // Test aarch64_vector_shift_left_sat_sisd field U = 1 (Max)
    // Fields: immh=0, Rn=0, immb=0, Rd=0, U=1, op=0
    let encoding: u32 = 0x7F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_immh_0_zero_6400_5f006400() {
    // Encoding: 0x5F006400
    // Test aarch64_vector_shift_left_sat_sisd field immh = 0 (Zero)
    // Fields: U=0, Rd=0, Rn=0, op=0, immh=0, immb=0
    let encoding: u32 = 0x5F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_immh_1_poweroftwo_6400_5f086400() {
    // Encoding: 0x5F086400
    // Test aarch64_vector_shift_left_sat_sisd field immh = 1 (PowerOfTwo)
    // Fields: U=0, immb=0, Rn=0, immh=1, op=0, Rd=0
    let encoding: u32 = 0x5F086400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_immh_3_poweroftwominusone_6400_5f186400() {
    // Encoding: 0x5F186400
    // Test aarch64_vector_shift_left_sat_sisd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: immb=0, op=0, Rn=0, immh=3, U=0, Rd=0
    let encoding: u32 = 0x5F186400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_immh_4_poweroftwo_6400_5f206400() {
    // Encoding: 0x5F206400
    // Test aarch64_vector_shift_left_sat_sisd field immh = 4 (PowerOfTwo)
    // Fields: Rn=0, U=0, Rd=0, immh=4, immb=0, op=0
    let encoding: u32 = 0x5F206400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_immh_7_poweroftwominusone_6400_5f386400() {
    // Encoding: 0x5F386400
    // Test aarch64_vector_shift_left_sat_sisd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: op=0, Rd=0, Rn=0, immb=0, U=0, immh=7
    let encoding: u32 = 0x5F386400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_immh_8_poweroftwo_6400_5f406400() {
    // Encoding: 0x5F406400
    // Test aarch64_vector_shift_left_sat_sisd field immh = 8 (PowerOfTwo)
    // Fields: U=0, immh=8, op=0, Rn=0, Rd=0, immb=0
    let encoding: u32 = 0x5F406400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_immh_15_max_6400_5f786400() {
    // Encoding: 0x5F786400
    // Test aarch64_vector_shift_left_sat_sisd field immh = 15 (Max)
    // Fields: Rn=0, immb=0, op=0, U=0, immh=15, Rd=0
    let encoding: u32 = 0x5F786400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_immb_0_zero_6400_5f006400() {
    // Encoding: 0x5F006400
    // Test aarch64_vector_shift_left_sat_sisd field immb = 0 (Zero)
    // Fields: U=0, Rn=0, Rd=0, immb=0, op=0, immh=0
    let encoding: u32 = 0x5F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_immb_1_poweroftwo_6400_5f016400() {
    // Encoding: 0x5F016400
    // Test aarch64_vector_shift_left_sat_sisd field immb = 1 (PowerOfTwo)
    // Fields: Rn=0, U=0, op=0, Rd=0, immb=1, immh=0
    let encoding: u32 = 0x5F016400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_immb_3_poweroftwominusone_6400_5f036400() {
    // Encoding: 0x5F036400
    // Test aarch64_vector_shift_left_sat_sisd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: immh=0, U=0, op=0, immb=3, Rd=0, Rn=0
    let encoding: u32 = 0x5F036400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_immb_7_max_6400_5f076400() {
    // Encoding: 0x5F076400
    // Test aarch64_vector_shift_left_sat_sisd field immb = 7 (Max)
    // Fields: Rd=0, op=0, immh=0, U=0, Rn=0, immb=7
    let encoding: u32 = 0x5F076400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_op_0_min_6400_5f006400() {
    // Encoding: 0x5F006400
    // Test aarch64_vector_shift_left_sat_sisd field op = 0 (Min)
    // Fields: immb=0, op=0, Rd=0, U=0, Rn=0, immh=0
    let encoding: u32 = 0x5F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_op_1_max_6400_5f007400() {
    // Encoding: 0x5F007400
    // Test aarch64_vector_shift_left_sat_sisd field op = 1 (Max)
    // Fields: op=1, immh=0, Rn=0, U=0, immb=0, Rd=0
    let encoding: u32 = 0x5F007400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_rn_0_min_6400_5f006400() {
    // Encoding: 0x5F006400
    // Test aarch64_vector_shift_left_sat_sisd field Rn = 0 (Min)
    // Fields: immh=0, U=0, immb=0, Rn=0, Rd=0, op=0
    let encoding: u32 = 0x5F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_rn_1_poweroftwo_6400_5f006420() {
    // Encoding: 0x5F006420
    // Test aarch64_vector_shift_left_sat_sisd field Rn = 1 (PowerOfTwo)
    // Fields: U=0, immb=0, Rn=1, op=0, Rd=0, immh=0
    let encoding: u32 = 0x5F006420;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_rn_30_poweroftwominusone_6400_5f0067c0() {
    // Encoding: 0x5F0067C0
    // Test aarch64_vector_shift_left_sat_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=30, op=0, immb=0, U=0, immh=0
    let encoding: u32 = 0x5F0067C0;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_rn_31_max_6400_5f0067e0() {
    // Encoding: 0x5F0067E0
    // Test aarch64_vector_shift_left_sat_sisd field Rn = 31 (Max)
    // Fields: immb=0, U=0, immh=0, Rd=0, Rn=31, op=0
    let encoding: u32 = 0x5F0067E0;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_rd_0_min_6400_5f006400() {
    // Encoding: 0x5F006400
    // Test aarch64_vector_shift_left_sat_sisd field Rd = 0 (Min)
    // Fields: Rn=0, Rd=0, immb=0, U=0, op=0, immh=0
    let encoding: u32 = 0x5F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_rd_1_poweroftwo_6400_5f006401() {
    // Encoding: 0x5F006401
    // Test aarch64_vector_shift_left_sat_sisd field Rd = 1 (PowerOfTwo)
    // Fields: immh=0, U=0, immb=0, Rd=1, Rn=0, op=0
    let encoding: u32 = 0x5F006401;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_rd_30_poweroftwominusone_6400_5f00641e() {
    // Encoding: 0x5F00641E
    // Test aarch64_vector_shift_left_sat_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, op=0, immb=0, Rn=0, Rd=30, immh=0
    let encoding: u32 = 0x5F00641E;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_field_rd_31_max_6400_5f00641f() {
    // Encoding: 0x5F00641F
    // Test aarch64_vector_shift_left_sat_sisd field Rd = 31 (Max)
    // Fields: immh=0, Rd=31, immb=0, Rn=0, op=0, U=0
    let encoding: u32 = 0x5F00641F;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_0_6400_5f006400() {
    // Encoding: 0x5F006400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, U=0, immb=0, immh=0, op=0, Rn=0
    let encoding: u32 = 0x5F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_1_6400_7f006400() {
    // Encoding: 0x7F006400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=1, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=0, immb=0, U=1, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_2_6400_5f006400() {
    // Encoding: 0x5F006400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=0, Rn=0, immb=0, U=0, op=0, Rd=0
    let encoding: u32 = 0x5F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_3_6400_5f086400() {
    // Encoding: 0x5F086400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=1, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, op=0, immb=0, U=0, immh=1, Rn=0
    let encoding: u32 = 0x5F086400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_4_6400_5f186400() {
    // Encoding: 0x5F186400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=3, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, U=0, Rd=0, immh=3, immb=0, op=0
    let encoding: u32 = 0x5F186400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_5_6400_5f206400() {
    // Encoding: 0x5F206400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=4, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=4, immb=0, U=0, Rn=0, Rd=0, op=0
    let encoding: u32 = 0x5F206400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_6_6400_5f386400() {
    // Encoding: 0x5F386400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=7, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, immh=7, immb=0, op=0, Rn=0, U=0
    let encoding: u32 = 0x5F386400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_7_6400_5f406400() {
    // Encoding: 0x5F406400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=8, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=8, immb=0, op=0, U=0, Rd=0, Rn=0
    let encoding: u32 = 0x5F406400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_8_6400_5f786400() {
    // Encoding: 0x5F786400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=15, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=15, immb=0, op=0, U=0, Rd=0, Rn=0
    let encoding: u32 = 0x5F786400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_9_6400_5f006400() {
    // Encoding: 0x5F006400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=0, U=0, Rn=0, Rd=0, op=0, immb=0
    let encoding: u32 = 0x5F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_10_6400_5f016400() {
    // Encoding: 0x5F016400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=1, op=0, Rn=0, Rd=0
    // Fields: op=0, immh=0, immb=1, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x5F016400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_11_6400_5f036400() {
    // Encoding: 0x5F036400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=3, op=0, Rn=0, Rd=0
    // Fields: immb=3, Rn=0, immh=0, U=0, Rd=0, op=0
    let encoding: u32 = 0x5F036400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_12_6400_5f076400() {
    // Encoding: 0x5F076400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=7, op=0, Rn=0, Rd=0
    // Fields: immh=0, op=0, Rd=0, U=0, immb=7, Rn=0
    let encoding: u32 = 0x5F076400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_13_6400_5f006400() {
    // Encoding: 0x5F006400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, Rd=0, U=0, immh=0, op=0
    let encoding: u32 = 0x5F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_14_6400_5f007400() {
    // Encoding: 0x5F007400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=1, Rn=0, Rd=0
    // Fields: immh=0, Rn=0, U=0, op=1, immb=0, Rd=0
    let encoding: u32 = 0x5F007400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_15_6400_5f006400() {
    // Encoding: 0x5F006400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, immh=0, op=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x5F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_16_6400_5f006420() {
    // Encoding: 0x5F006420
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=1, Rd=0
    // Fields: immh=0, Rn=1, immb=0, U=0, op=0, Rd=0
    let encoding: u32 = 0x5F006420;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_17_6400_5f0067c0() {
    // Encoding: 0x5F0067C0
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=30, Rd=0
    // Fields: immh=0, op=0, U=0, Rn=30, Rd=0, immb=0
    let encoding: u32 = 0x5F0067C0;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_18_6400_5f0067e0() {
    // Encoding: 0x5F0067E0
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=31, Rd=0
    // Fields: Rd=0, immb=0, Rn=31, U=0, op=0, immh=0
    let encoding: u32 = 0x5F0067E0;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_19_6400_5f006400() {
    // Encoding: 0x5F006400
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: op=0, immb=0, immh=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x5F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_20_6400_5f006401() {
    // Encoding: 0x5F006401
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=1
    // Fields: U=0, immh=0, immb=0, Rn=0, Rd=1, op=0
    let encoding: u32 = 0x5F006401;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_21_6400_5f00641e() {
    // Encoding: 0x5F00641E
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=30
    // Fields: U=0, op=0, Rn=0, Rd=30, immh=0, immb=0
    let encoding: u32 = 0x5F00641E;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_22_6400_5f00641f() {
    // Encoding: 0x5F00641F
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=0, Rd=31
    // Fields: immh=0, Rn=0, op=0, Rd=31, U=0, immb=0
    let encoding: u32 = 0x5F00641F;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_23_6400_5f006421() {
    // Encoding: 0x5F006421
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=1, Rd=1
    // Fields: U=0, immh=0, Rn=1, Rd=1, op=0, immb=0
    let encoding: u32 = 0x5F006421;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_combo_24_6400_5f0067ff() {
    // Encoding: 0x5F0067FF
    // Test aarch64_vector_shift_left_sat_sisd field combination: U=0, immh=0, immb=0, op=0, Rn=31, Rd=31
    // Fields: immh=0, U=0, Rd=31, op=0, Rn=31, immb=0
    let encoding: u32 = 0x5F0067FF;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_25600_5f0967e0()
 {
    // Encoding: 0x5F0967E0
    // Test aarch64_vector_shift_left_sat_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: U=0, immb=1, op=0, immh=1, Rn=31, Rd=0
    let encoding: u32 = 0x5F0967E0;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_25600_5f09641f()
 {
    // Encoding: 0x5F09641F
    // Test aarch64_vector_shift_left_sat_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, immb=1, op=0, U=0, immh=1, Rn=0
    let encoding: u32 = 0x5F09641F;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_q_0_min_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field Q = 0 (Min)
    // Fields: immh=0, Q=0, Rn=0, Rd=0, U=0, immb=0, op=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_q_1_max_6400_4f006400() {
    // Encoding: 0x4F006400
    // Test aarch64_vector_shift_left_sat_simd field Q = 1 (Max)
    // Fields: U=0, immb=0, op=0, Q=1, immh=0, Rn=0, Rd=0
    let encoding: u32 = 0x4F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_u_0_min_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field U = 0 (Min)
    // Fields: Rd=0, op=0, Q=0, U=0, immh=0, immb=0, Rn=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_u_1_max_6400_2f006400() {
    // Encoding: 0x2F006400
    // Test aarch64_vector_shift_left_sat_simd field U = 1 (Max)
    // Fields: Q=0, op=0, Rn=0, U=1, Rd=0, immh=0, immb=0
    let encoding: u32 = 0x2F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_immh_0_zero_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field immh = 0 (Zero)
    // Fields: Q=0, U=0, op=0, immh=0, Rn=0, immb=0, Rd=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_immh_1_poweroftwo_6400_0f086400() {
    // Encoding: 0x0F086400
    // Test aarch64_vector_shift_left_sat_simd field immh = 1 (PowerOfTwo)
    // Fields: immb=0, Rd=0, Rn=0, Q=0, U=0, immh=1, op=0
    let encoding: u32 = 0x0F086400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_immh_3_poweroftwominusone_6400_0f186400() {
    // Encoding: 0x0F186400
    // Test aarch64_vector_shift_left_sat_simd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: op=0, immb=0, Rd=0, immh=3, Rn=0, U=0, Q=0
    let encoding: u32 = 0x0F186400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_immh_4_poweroftwo_6400_0f206400() {
    // Encoding: 0x0F206400
    // Test aarch64_vector_shift_left_sat_simd field immh = 4 (PowerOfTwo)
    // Fields: U=0, Q=0, op=0, immb=0, Rn=0, immh=4, Rd=0
    let encoding: u32 = 0x0F206400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_immh_7_poweroftwominusone_6400_0f386400() {
    // Encoding: 0x0F386400
    // Test aarch64_vector_shift_left_sat_simd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: Rd=0, U=0, op=0, immh=7, Q=0, immb=0, Rn=0
    let encoding: u32 = 0x0F386400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_immh_8_poweroftwo_6400_0f406400() {
    // Encoding: 0x0F406400
    // Test aarch64_vector_shift_left_sat_simd field immh = 8 (PowerOfTwo)
    // Fields: Rd=0, immb=0, Rn=0, U=0, op=0, Q=0, immh=8
    let encoding: u32 = 0x0F406400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_immh_15_max_6400_0f786400() {
    // Encoding: 0x0F786400
    // Test aarch64_vector_shift_left_sat_simd field immh = 15 (Max)
    // Fields: op=0, Rd=0, immb=0, U=0, Rn=0, immh=15, Q=0
    let encoding: u32 = 0x0F786400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_immb_0_zero_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field immb = 0 (Zero)
    // Fields: Rn=0, Q=0, immb=0, op=0, Rd=0, U=0, immh=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_immb_1_poweroftwo_6400_0f016400() {
    // Encoding: 0x0F016400
    // Test aarch64_vector_shift_left_sat_simd field immb = 1 (PowerOfTwo)
    // Fields: Rn=0, immh=0, Rd=0, immb=1, Q=0, op=0, U=0
    let encoding: u32 = 0x0F016400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_immb_3_poweroftwominusone_6400_0f036400() {
    // Encoding: 0x0F036400
    // Test aarch64_vector_shift_left_sat_simd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: immh=0, U=0, immb=3, op=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0F036400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_immb_7_max_6400_0f076400() {
    // Encoding: 0x0F076400
    // Test aarch64_vector_shift_left_sat_simd field immb = 7 (Max)
    // Fields: U=0, Q=0, immb=7, Rd=0, immh=0, Rn=0, op=0
    let encoding: u32 = 0x0F076400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_op_0_min_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field op = 0 (Min)
    // Fields: Q=0, Rn=0, U=0, immb=0, immh=0, op=0, Rd=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field op 12 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_op_1_max_6400_0f007400() {
    // Encoding: 0x0F007400
    // Test aarch64_vector_shift_left_sat_simd field op = 1 (Max)
    // Fields: Q=0, Rd=0, immh=0, immb=0, U=0, op=1, Rn=0
    let encoding: u32 = 0x0F007400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_rn_0_min_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field Rn = 0 (Min)
    // Fields: immb=0, op=0, Rd=0, immh=0, Q=0, U=0, Rn=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_rn_1_poweroftwo_6400_0f006420() {
    // Encoding: 0x0F006420
    // Test aarch64_vector_shift_left_sat_simd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, immh=0, immb=0, Q=0, op=0, Rd=0, U=0
    let encoding: u32 = 0x0F006420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_rn_30_poweroftwominusone_6400_0f0067c0() {
    // Encoding: 0x0F0067C0
    // Test aarch64_vector_shift_left_sat_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: immh=0, Rn=30, Rd=0, immb=0, U=0, Q=0, op=0
    let encoding: u32 = 0x0F0067C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_rn_31_max_6400_0f0067e0() {
    // Encoding: 0x0F0067E0
    // Test aarch64_vector_shift_left_sat_simd field Rn = 31 (Max)
    // Fields: immb=0, Q=0, Rd=0, immh=0, op=0, U=0, Rn=31
    let encoding: u32 = 0x0F0067E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_rd_0_min_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field Rd = 0 (Min)
    // Fields: U=0, immb=0, Rn=0, Rd=0, immh=0, op=0, Q=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_rd_1_poweroftwo_6400_0f006401() {
    // Encoding: 0x0F006401
    // Test aarch64_vector_shift_left_sat_simd field Rd = 1 (PowerOfTwo)
    // Fields: U=0, immh=0, Rn=0, Rd=1, Q=0, immb=0, op=0
    let encoding: u32 = 0x0F006401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_rd_30_poweroftwominusone_6400_0f00641e() {
    // Encoding: 0x0F00641E
    // Test aarch64_vector_shift_left_sat_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: immb=0, immh=0, op=0, Q=0, U=0, Rn=0, Rd=30
    let encoding: u32 = 0x0F00641E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_field_rd_31_max_6400_0f00641f() {
    // Encoding: 0x0F00641F
    // Test aarch64_vector_shift_left_sat_simd field Rd = 31 (Max)
    // Fields: Q=0, op=0, Rn=0, Rd=31, immh=0, U=0, immb=0
    let encoding: u32 = 0x0F00641F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_0_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, op=0, Q=0, Rd=0, U=0, immb=0, immh=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_1_6400_4f006400() {
    // Encoding: 0x4F006400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=1, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=1, Rd=0, immh=0, U=0, op=0, immb=0, Rn=0
    let encoding: u32 = 0x4F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_2_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, op=0, U=0, immb=0, Rn=0, immh=0, Q=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_3_6400_2f006400() {
    // Encoding: 0x2F006400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=1, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, Rd=0, Q=0, U=1, immh=0, op=0, Rn=0
    let encoding: u32 = 0x2F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_4_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, op=0, Rn=0, Rd=0, immh=0, immb=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_5_6400_0f086400() {
    // Encoding: 0x0F086400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=1, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, immh=1, Rn=0, U=0, immb=0, op=0
    let encoding: u32 = 0x0F086400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_6_6400_0f186400() {
    // Encoding: 0x0F186400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=3, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, immh=3, op=0, Q=0, U=0, immb=0, Rn=0
    let encoding: u32 = 0x0F186400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_7_6400_0f206400() {
    // Encoding: 0x0F206400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=4, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, Rd=0, U=0, op=0, immh=4, Q=0
    let encoding: u32 = 0x0F206400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_8_6400_0f386400() {
    // Encoding: 0x0F386400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=7, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, Rd=0, immh=7, immb=0, op=0, U=0
    let encoding: u32 = 0x0F386400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_9_6400_0f406400() {
    // Encoding: 0x0F406400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=8, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, immh=8, Rd=0, Q=0, U=0, immb=0, op=0
    let encoding: u32 = 0x0F406400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_10_6400_0f786400() {
    // Encoding: 0x0F786400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=15, immb=0, op=0, Rn=0, Rd=0
    // Fields: op=0, U=0, immh=15, immb=0, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x0F786400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_11_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, immh=0, U=0, op=0, immb=0, Q=0, Rn=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_12_6400_0f016400() {
    // Encoding: 0x0F016400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=1, op=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, immh=0, Q=0, immb=1, op=0, U=0
    let encoding: u32 = 0x0F016400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_13_6400_0f036400() {
    // Encoding: 0x0F036400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=3, op=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, immh=0, immb=3, U=0, op=0, Rd=0
    let encoding: u32 = 0x0F036400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_14_6400_0f076400() {
    // Encoding: 0x0F076400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=7, op=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, op=0, immh=0, Q=0, U=0, immb=7
    let encoding: u32 = 0x0F076400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_15_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: U=0, Q=0, immh=0, Rd=0, op=0, immb=0, Rn=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_16_6400_0f007400() {
    // Encoding: 0x0F007400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=1, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, U=0, immb=0, Q=0, immh=0, op=1
    let encoding: u32 = 0x0F007400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_17_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, immh=0, immb=0, op=0, Rn=0, Q=0, U=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_18_6400_0f006420() {
    // Encoding: 0x0F006420
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=1, Rd=0
    // Fields: immb=0, Rn=1, immh=0, U=0, Rd=0, Q=0, op=0
    let encoding: u32 = 0x0F006420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_19_6400_0f0067c0() {
    // Encoding: 0x0F0067C0
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=30, Rd=0
    // Fields: Q=0, Rn=30, immb=0, op=0, U=0, immh=0, Rd=0
    let encoding: u32 = 0x0F0067C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_20_6400_0f0067e0() {
    // Encoding: 0x0F0067E0
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=31, Rd=0
    // Fields: Q=0, U=0, Rn=31, immb=0, immh=0, op=0, Rd=0
    let encoding: u32 = 0x0F0067E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_21_6400_0f006400() {
    // Encoding: 0x0F006400
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, op=0, Rd=0, immh=0, Q=0, U=0, immb=0
    let encoding: u32 = 0x0F006400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_22_6400_0f006401() {
    // Encoding: 0x0F006401
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=1
    // Fields: op=0, Rn=0, Rd=1, immh=0, U=0, Q=0, immb=0
    let encoding: u32 = 0x0F006401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_23_6400_0f00641e() {
    // Encoding: 0x0F00641E
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=30
    // Fields: Q=0, U=0, Rn=0, Rd=30, immb=0, op=0, immh=0
    let encoding: u32 = 0x0F00641E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_24_6400_0f00641f() {
    // Encoding: 0x0F00641F
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=0, Rd=31
    // Fields: Rn=0, immh=0, op=0, Rd=31, Q=0, U=0, immb=0
    let encoding: u32 = 0x0F00641F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_25_6400_0f006421() {
    // Encoding: 0x0F006421
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=1, Rd=1
    // Fields: Q=0, immh=0, immb=0, op=0, Rd=1, U=0, Rn=1
    let encoding: u32 = 0x0F006421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_left_sat_simd_combo_26_6400_0f0067ff() {
    // Encoding: 0x0F0067FF
    // Test aarch64_vector_shift_left_sat_simd field combination: Q=0, U=0, immh=0, immb=0, op=0, Rn=31, Rd=31
    // Fields: immb=0, Rn=31, immh=0, op=0, Rd=31, Q=0, U=0
    let encoding: u32 = 0x0F0067FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_shift_left_sat_simd_special_q_0_size_variant_0_25600_0f096400() {
    // Encoding: 0x0F096400
    // Test aarch64_vector_shift_left_sat_simd special value Q = 0 (Size variant 0)
    // Fields: Q=0, immh=1, Rd=0, Rn=0, immb=1, U=0, op=0
    let encoding: u32 = 0x0F096400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_shift_left_sat_simd_special_q_1_size_variant_1_25600_4f096400() {
    // Encoding: 0x4F096400
    // Test aarch64_vector_shift_left_sat_simd special value Q = 1 (Size variant 1)
    // Fields: immh=1, op=0, Q=1, Rn=0, Rd=0, U=0, immb=1
    let encoding: u32 = 0x4F096400;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_left_sat_simd_special_rn_31_stack_pointer_sp_may_require_alignment_25600_0f0967e0()
 {
    // Encoding: 0x0F0967E0
    // Test aarch64_vector_shift_left_sat_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, op=0, immb=1, Rd=0, Rn=31, U=0, immh=1
    let encoding: u32 = 0x0F0967E0;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_left_sat_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_25600_0f09641f()
 {
    // Encoding: 0x0F09641F
    // Test aarch64_vector_shift_left_sat_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Rd=31, U=0, Q=0, immh=1, immb=1, op=0
    let encoding: u32 = 0x0F09641F;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_reg_write_0_5f006400() {
    // Test aarch64_vector_shift_left_sat_sisd register write: SimdFromField("d")
    // Encoding: 0x5F006400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F006400;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_sp_rn_5f0067e0() {
    // Test aarch64_vector_shift_left_sat_sisd with Rn = SP (31)
    // Encoding: 0x5F0067E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F0067E0;
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

/// Provenance: aarch64_vector_shift_left_sat_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_left_sat_sisd_zr_rd_5f00641f() {
    // Test aarch64_vector_shift_left_sat_sisd with Rd = ZR (31)
    // Encoding: 0x5F00641F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F00641F;
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

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_left_sat_simd_reg_write_0_0f006400() {
    // Test aarch64_vector_shift_left_sat_simd register write: SimdFromField("d")
    // Encoding: 0x0F006400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F006400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_sp_rn_0f0067e0() {
    // Test aarch64_vector_shift_left_sat_simd with Rn = SP (31)
    // Encoding: 0x0F0067E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F0067E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_left_sat_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_left_sat_simd_zr_rd_0f00641f() {
    // Test aarch64_vector_shift_left_sat_simd with Rd = ZR (31)
    // Encoding: 0x0F00641F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00641F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_shift_conv_float_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_u_0_min_fc00_5f00fc00() {
    // Encoding: 0x5F00FC00
    // Test aarch64_vector_shift_conv_float_sisd field U = 0 (Min)
    // Fields: U=0, immb=0, Rn=0, immh=0, Rd=0
    let encoding: u32 = 0x5F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_u_1_max_fc00_7f00fc00() {
    // Encoding: 0x7F00FC00
    // Test aarch64_vector_shift_conv_float_sisd field U = 1 (Max)
    // Fields: immb=0, Rn=0, Rd=0, U=1, immh=0
    let encoding: u32 = 0x7F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_immh_0_zero_fc00_5f00fc00() {
    // Encoding: 0x5F00FC00
    // Test aarch64_vector_shift_conv_float_sisd field immh = 0 (Zero)
    // Fields: immh=0, U=0, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x5F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_immh_1_poweroftwo_fc00_5f08fc00() {
    // Encoding: 0x5F08FC00
    // Test aarch64_vector_shift_conv_float_sisd field immh = 1 (PowerOfTwo)
    // Fields: Rn=0, U=0, Rd=0, immh=1, immb=0
    let encoding: u32 = 0x5F08FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_immh_3_poweroftwominusone_fc00_5f18fc00() {
    // Encoding: 0x5F18FC00
    // Test aarch64_vector_shift_conv_float_sisd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: Rd=0, immh=3, Rn=0, immb=0, U=0
    let encoding: u32 = 0x5F18FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_immh_4_poweroftwo_fc00_5f20fc00() {
    // Encoding: 0x5F20FC00
    // Test aarch64_vector_shift_conv_float_sisd field immh = 4 (PowerOfTwo)
    // Fields: immh=4, Rn=0, Rd=0, immb=0, U=0
    let encoding: u32 = 0x5F20FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_immh_7_poweroftwominusone_fc00_5f38fc00() {
    // Encoding: 0x5F38FC00
    // Test aarch64_vector_shift_conv_float_sisd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: Rd=0, U=0, Rn=0, immb=0, immh=7
    let encoding: u32 = 0x5F38FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_immh_8_poweroftwo_fc00_5f40fc00() {
    // Encoding: 0x5F40FC00
    // Test aarch64_vector_shift_conv_float_sisd field immh = 8 (PowerOfTwo)
    // Fields: Rn=0, immh=8, Rd=0, U=0, immb=0
    let encoding: u32 = 0x5F40FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_immh_15_max_fc00_5f78fc00() {
    // Encoding: 0x5F78FC00
    // Test aarch64_vector_shift_conv_float_sisd field immh = 15 (Max)
    // Fields: U=0, immh=15, Rn=0, immb=0, Rd=0
    let encoding: u32 = 0x5F78FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_immb_0_zero_fc00_5f00fc00() {
    // Encoding: 0x5F00FC00
    // Test aarch64_vector_shift_conv_float_sisd field immb = 0 (Zero)
    // Fields: Rd=0, immh=0, U=0, immb=0, Rn=0
    let encoding: u32 = 0x5F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_immb_1_poweroftwo_fc00_5f01fc00() {
    // Encoding: 0x5F01FC00
    // Test aarch64_vector_shift_conv_float_sisd field immb = 1 (PowerOfTwo)
    // Fields: U=0, Rn=0, immb=1, immh=0, Rd=0
    let encoding: u32 = 0x5F01FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_immb_3_poweroftwominusone_fc00_5f03fc00() {
    // Encoding: 0x5F03FC00
    // Test aarch64_vector_shift_conv_float_sisd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=0, immb=3, U=0, immh=0
    let encoding: u32 = 0x5F03FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_immb_7_max_fc00_5f07fc00() {
    // Encoding: 0x5F07FC00
    // Test aarch64_vector_shift_conv_float_sisd field immb = 7 (Max)
    // Fields: U=0, Rd=0, immh=0, immb=7, Rn=0
    let encoding: u32 = 0x5F07FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_rn_0_min_fc00_5f00fc00() {
    // Encoding: 0x5F00FC00
    // Test aarch64_vector_shift_conv_float_sisd field Rn = 0 (Min)
    // Fields: U=0, Rn=0, Rd=0, immh=0, immb=0
    let encoding: u32 = 0x5F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_rn_1_poweroftwo_fc00_5f00fc20() {
    // Encoding: 0x5F00FC20
    // Test aarch64_vector_shift_conv_float_sisd field Rn = 1 (PowerOfTwo)
    // Fields: U=0, immh=0, immb=0, Rn=1, Rd=0
    let encoding: u32 = 0x5F00FC20;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_rn_30_poweroftwominusone_fc00_5f00ffc0() {
    // Encoding: 0x5F00FFC0
    // Test aarch64_vector_shift_conv_float_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rd=0, U=0, immh=0, immb=0
    let encoding: u32 = 0x5F00FFC0;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_rn_31_max_fc00_5f00ffe0() {
    // Encoding: 0x5F00FFE0
    // Test aarch64_vector_shift_conv_float_sisd field Rn = 31 (Max)
    // Fields: Rn=31, immh=0, Rd=0, U=0, immb=0
    let encoding: u32 = 0x5F00FFE0;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_rd_0_min_fc00_5f00fc00() {
    // Encoding: 0x5F00FC00
    // Test aarch64_vector_shift_conv_float_sisd field Rd = 0 (Min)
    // Fields: immb=0, immh=0, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x5F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_rd_1_poweroftwo_fc00_5f00fc01() {
    // Encoding: 0x5F00FC01
    // Test aarch64_vector_shift_conv_float_sisd field Rd = 1 (PowerOfTwo)
    // Fields: immh=0, Rd=1, immb=0, Rn=0, U=0
    let encoding: u32 = 0x5F00FC01;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_rd_30_poweroftwominusone_fc00_5f00fc1e() {
    // Encoding: 0x5F00FC1E
    // Test aarch64_vector_shift_conv_float_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: immh=0, U=0, Rd=30, Rn=0, immb=0
    let encoding: u32 = 0x5F00FC1E;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_field_rd_31_max_fc00_5f00fc1f() {
    // Encoding: 0x5F00FC1F
    // Test aarch64_vector_shift_conv_float_sisd field Rd = 31 (Max)
    // Fields: U=0, immb=0, immh=0, Rn=0, Rd=31
    let encoding: u32 = 0x5F00FC1F;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_0_fc00_5f00fc00() {
    // Encoding: 0x5F00FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immh=0, immb=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x5F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_1_fc00_7f00fc00() {
    // Encoding: 0x7F00FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=1, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immb=0, U=1, immh=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_2_fc00_5f00fc00() {
    // Encoding: 0x5F00FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immh=0, Rd=0, immb=0, U=0
    let encoding: u32 = 0x5F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_3_fc00_5f08fc00() {
    // Encoding: 0x5F08FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=1, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, immb=0, Rn=0, U=0, immh=1
    let encoding: u32 = 0x5F08FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_4_fc00_5f18fc00() {
    // Encoding: 0x5F18FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=3, immb=0, Rn=0, Rd=0
    // Fields: U=0, Rn=0, Rd=0, immh=3, immb=0
    let encoding: u32 = 0x5F18FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_5_fc00_5f20fc00() {
    // Encoding: 0x5F20FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=4, immb=0, Rn=0, Rd=0
    // Fields: U=0, immb=0, immh=4, Rn=0, Rd=0
    let encoding: u32 = 0x5F20FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_6_fc00_5f38fc00() {
    // Encoding: 0x5F38FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=7, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, U=0, immb=0, Rd=0, immh=7
    let encoding: u32 = 0x5F38FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_7_fc00_5f40fc00() {
    // Encoding: 0x5F40FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=8, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, immh=8, immb=0, U=0
    let encoding: u32 = 0x5F40FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_8_fc00_5f78fc00() {
    // Encoding: 0x5F78FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=15, immb=0, Rn=0, Rd=0
    // Fields: U=0, Rd=0, immh=15, immb=0, Rn=0
    let encoding: u32 = 0x5F78FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_9_fc00_5f00fc00() {
    // Encoding: 0x5F00FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immh=0, U=0, Rn=0, Rd=0, immb=0
    let encoding: u32 = 0x5F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_10_fc00_5f01fc00() {
    // Encoding: 0x5F01FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=1, Rn=0, Rd=0
    // Fields: U=0, immb=1, immh=0, Rn=0, Rd=0
    let encoding: u32 = 0x5F01FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_11_fc00_5f03fc00() {
    // Encoding: 0x5F03FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=3, Rn=0, Rd=0
    // Fields: Rd=0, immh=0, immb=3, Rn=0, U=0
    let encoding: u32 = 0x5F03FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_12_fc00_5f07fc00() {
    // Encoding: 0x5F07FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=7, Rn=0, Rd=0
    // Fields: immh=0, U=0, Rn=0, Rd=0, immb=7
    let encoding: u32 = 0x5F07FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_13_fc00_5f00fc00() {
    // Encoding: 0x5F00FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immh=0, Rn=0, U=0, immb=0, Rd=0
    let encoding: u32 = 0x5F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_14_fc00_5f00fc20() {
    // Encoding: 0x5F00FC20
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=0, Rn=1, Rd=0
    // Fields: Rd=0, Rn=1, immh=0, U=0, immb=0
    let encoding: u32 = 0x5F00FC20;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_15_fc00_5f00ffc0() {
    // Encoding: 0x5F00FFC0
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=0, Rn=30, Rd=0
    // Fields: immh=0, U=0, Rn=30, immb=0, Rd=0
    let encoding: u32 = 0x5F00FFC0;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_16_fc00_5f00ffe0() {
    // Encoding: 0x5F00FFE0
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=0, Rn=31, Rd=0
    // Fields: U=0, immb=0, Rn=31, immh=0, Rd=0
    let encoding: u32 = 0x5F00FFE0;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_17_fc00_5f00fc00() {
    // Encoding: 0x5F00FC00
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, immb=0, Rn=0, immh=0, U=0
    let encoding: u32 = 0x5F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_18_fc00_5f00fc01() {
    // Encoding: 0x5F00FC01
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=1
    // Fields: immb=0, immh=0, Rn=0, U=0, Rd=1
    let encoding: u32 = 0x5F00FC01;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_19_fc00_5f00fc1e() {
    // Encoding: 0x5F00FC1E
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=30
    // Fields: immb=0, immh=0, Rn=0, U=0, Rd=30
    let encoding: u32 = 0x5F00FC1E;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_20_fc00_5f00fc1f() {
    // Encoding: 0x5F00FC1F
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=31
    // Fields: U=0, immb=0, Rn=0, Rd=31, immh=0
    let encoding: u32 = 0x5F00FC1F;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_21_fc00_5f00fc21() {
    // Encoding: 0x5F00FC21
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=0, Rn=1, Rd=1
    // Fields: U=0, immb=0, immh=0, Rd=1, Rn=1
    let encoding: u32 = 0x5F00FC21;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_combo_22_fc00_5f00ffff() {
    // Encoding: 0x5F00FFFF
    // Test aarch64_vector_shift_conv_float_sisd field combination: U=0, immh=0, immb=0, Rn=31, Rd=31
    // Fields: Rd=31, U=0, immh=0, immb=0, Rn=31
    let encoding: u32 = 0x5F00FFFF;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_64512_5f09ffe0()
 {
    // Encoding: 0x5F09FFE0
    // Test aarch64_vector_shift_conv_float_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, immh=1, immb=1, Rn=31, U=0
    let encoding: u32 = 0x5F09FFE0;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_64512_5f09fc1f()
 {
    // Encoding: 0x5F09FC1F
    // Test aarch64_vector_shift_conv_float_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, U=0, Rn=0, immh=1, immb=1
    let encoding: u32 = 0x5F09FC1F;
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

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_q_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_shift_conv_float_simd field Q = 0 (Min)
    // Fields: Rn=0, Q=0, immb=0, Rd=0, U=0, immh=0
    let encoding: u32 = 0x0F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_q_1_max_fc00_4f00fc00() {
    // Encoding: 0x4F00FC00
    // Test aarch64_vector_shift_conv_float_simd field Q = 1 (Max)
    // Fields: immb=0, Rn=0, Rd=0, Q=1, U=0, immh=0
    let encoding: u32 = 0x4F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_u_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_shift_conv_float_simd field U = 0 (Min)
    // Fields: immh=0, immb=0, Q=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_u_1_max_fc00_2f00fc00() {
    // Encoding: 0x2F00FC00
    // Test aarch64_vector_shift_conv_float_simd field U = 1 (Max)
    // Fields: immb=0, Rd=0, Q=0, U=1, immh=0, Rn=0
    let encoding: u32 = 0x2F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_immh_0_zero_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_shift_conv_float_simd field immh = 0 (Zero)
    // Fields: Q=0, immh=0, immb=0, U=0, Rd=0, Rn=0
    let encoding: u32 = 0x0F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_immh_1_poweroftwo_fc00_0f08fc00() {
    // Encoding: 0x0F08FC00
    // Test aarch64_vector_shift_conv_float_simd field immh = 1 (PowerOfTwo)
    // Fields: immb=0, Rn=0, Q=0, Rd=0, U=0, immh=1
    let encoding: u32 = 0x0F08FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_immh_3_poweroftwominusone_fc00_0f18fc00() {
    // Encoding: 0x0F18FC00
    // Test aarch64_vector_shift_conv_float_simd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: Q=0, immh=3, Rd=0, Rn=0, U=0, immb=0
    let encoding: u32 = 0x0F18FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_immh_4_poweroftwo_fc00_0f20fc00() {
    // Encoding: 0x0F20FC00
    // Test aarch64_vector_shift_conv_float_simd field immh = 4 (PowerOfTwo)
    // Fields: Q=0, immh=4, U=0, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x0F20FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_immh_7_poweroftwominusone_fc00_0f38fc00() {
    // Encoding: 0x0F38FC00
    // Test aarch64_vector_shift_conv_float_simd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: Q=0, U=0, immb=0, immh=7, Rd=0, Rn=0
    let encoding: u32 = 0x0F38FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_immh_8_poweroftwo_fc00_0f40fc00() {
    // Encoding: 0x0F40FC00
    // Test aarch64_vector_shift_conv_float_simd field immh = 8 (PowerOfTwo)
    // Fields: U=0, immh=8, immb=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0F40FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_immh_15_max_fc00_0f78fc00() {
    // Encoding: 0x0F78FC00
    // Test aarch64_vector_shift_conv_float_simd field immh = 15 (Max)
    // Fields: Q=0, immb=0, Rd=0, Rn=0, U=0, immh=15
    let encoding: u32 = 0x0F78FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_immb_0_zero_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_shift_conv_float_simd field immb = 0 (Zero)
    // Fields: Q=0, immb=0, immh=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_immb_1_poweroftwo_fc00_0f01fc00() {
    // Encoding: 0x0F01FC00
    // Test aarch64_vector_shift_conv_float_simd field immb = 1 (PowerOfTwo)
    // Fields: U=0, Rd=0, Rn=0, immh=0, Q=0, immb=1
    let encoding: u32 = 0x0F01FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_immb_3_poweroftwominusone_fc00_0f03fc00() {
    // Encoding: 0x0F03FC00
    // Test aarch64_vector_shift_conv_float_simd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=0, U=0, Q=0, immh=0, immb=3
    let encoding: u32 = 0x0F03FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_immb_7_max_fc00_0f07fc00() {
    // Encoding: 0x0F07FC00
    // Test aarch64_vector_shift_conv_float_simd field immb = 7 (Max)
    // Fields: U=0, immb=7, Q=0, Rn=0, immh=0, Rd=0
    let encoding: u32 = 0x0F07FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_rn_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_shift_conv_float_simd field Rn = 0 (Min)
    // Fields: immb=0, Rn=0, U=0, Rd=0, Q=0, immh=0
    let encoding: u32 = 0x0F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_rn_1_poweroftwo_fc00_0f00fc20() {
    // Encoding: 0x0F00FC20
    // Test aarch64_vector_shift_conv_float_simd field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, U=0, immh=0, immb=0, Rd=0, Q=0
    let encoding: u32 = 0x0F00FC20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_rn_30_poweroftwominusone_fc00_0f00ffc0() {
    // Encoding: 0x0F00FFC0
    // Test aarch64_vector_shift_conv_float_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, immh=0, Rd=0, immb=0, Q=0, Rn=30
    let encoding: u32 = 0x0F00FFC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_rn_31_max_fc00_0f00ffe0() {
    // Encoding: 0x0F00FFE0
    // Test aarch64_vector_shift_conv_float_simd field Rn = 31 (Max)
    // Fields: Rn=31, Rd=0, U=0, Q=0, immh=0, immb=0
    let encoding: u32 = 0x0F00FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_rd_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_shift_conv_float_simd field Rd = 0 (Min)
    // Fields: U=0, Rd=0, immh=0, Q=0, immb=0, Rn=0
    let encoding: u32 = 0x0F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_rd_1_poweroftwo_fc00_0f00fc01() {
    // Encoding: 0x0F00FC01
    // Test aarch64_vector_shift_conv_float_simd field Rd = 1 (PowerOfTwo)
    // Fields: Q=0, immh=0, U=0, Rn=0, Rd=1, immb=0
    let encoding: u32 = 0x0F00FC01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_rd_30_poweroftwominusone_fc00_0f00fc1e() {
    // Encoding: 0x0F00FC1E
    // Test aarch64_vector_shift_conv_float_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, immh=0, immb=0, Rd=30, Q=0, Rn=0
    let encoding: u32 = 0x0F00FC1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_field_rd_31_max_fc00_0f00fc1f() {
    // Encoding: 0x0F00FC1F
    // Test aarch64_vector_shift_conv_float_simd field Rd = 31 (Max)
    // Fields: U=0, immh=0, Rn=0, Q=0, immb=0, Rd=31
    let encoding: u32 = 0x0F00FC1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_0_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immb=0, Rd=0, Q=0, U=0, immh=0
    let encoding: u32 = 0x0F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_1_fc00_4f00fc00() {
    // Encoding: 0x4F00FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=1, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: U=0, Q=1, immb=0, Rn=0, immh=0, Rd=0
    let encoding: u32 = 0x4F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_2_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immh=0, Q=0, U=0, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x0F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_3_fc00_2f00fc00() {
    // Encoding: 0x2F00FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=1, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immh=0, immb=0, Rn=0, Rd=0, Q=0, U=1
    let encoding: u32 = 0x2F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_4_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Q=0, immb=0, immh=0, U=0, Rn=0, Rd=0
    let encoding: u32 = 0x0F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_5_fc00_0f08fc00() {
    // Encoding: 0x0F08FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=1, immb=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, Rn=0, Rd=0, immh=1, immb=0
    let encoding: u32 = 0x0F08FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_6_fc00_0f18fc00() {
    // Encoding: 0x0F18FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=3, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, Rd=0, U=0, immb=0, immh=3
    let encoding: u32 = 0x0F18FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_7_fc00_0f20fc00() {
    // Encoding: 0x0F20FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=4, immb=0, Rn=0, Rd=0
    // Fields: U=0, immh=4, Rn=0, Rd=0, Q=0, immb=0
    let encoding: u32 = 0x0F20FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_8_fc00_0f38fc00() {
    // Encoding: 0x0F38FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=7, immb=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, immh=7, immb=0, Rd=0, U=0
    let encoding: u32 = 0x0F38FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_9_fc00_0f40fc00() {
    // Encoding: 0x0F40FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=8, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rd=0, Q=0, immh=8, Rn=0, U=0
    let encoding: u32 = 0x0F40FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_10_fc00_0f78fc00() {
    // Encoding: 0x0F78FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=15, immb=0, Rn=0, Rd=0
    // Fields: Q=0, immh=15, U=0, Rn=0, immb=0, Rd=0
    let encoding: u32 = 0x0F78FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_11_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: U=0, immh=0, Q=0, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x0F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_12_fc00_0f01fc00() {
    // Encoding: 0x0F01FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=1, Rn=0, Rd=0
    // Fields: Q=0, immh=0, Rn=0, U=0, Rd=0, immb=1
    let encoding: u32 = 0x0F01FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_13_fc00_0f03fc00() {
    // Encoding: 0x0F03FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=3, Rn=0, Rd=0
    // Fields: immh=0, Q=0, Rn=0, Rd=0, immb=3, U=0
    let encoding: u32 = 0x0F03FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_14_fc00_0f07fc00() {
    // Encoding: 0x0F07FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=7, Rn=0, Rd=0
    // Fields: immh=0, U=0, immb=7, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0F07FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_15_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, Rd=0, immh=0, Q=0, U=0
    let encoding: u32 = 0x0F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_16_fc00_0f00fc20() {
    // Encoding: 0x0F00FC20
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=1, Rd=0
    // Fields: U=0, Rd=0, immh=0, Q=0, Rn=1, immb=0
    let encoding: u32 = 0x0F00FC20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_17_fc00_0f00ffc0() {
    // Encoding: 0x0F00FFC0
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=30, Rd=0
    // Fields: Rn=30, Rd=0, U=0, Q=0, immb=0, immh=0
    let encoding: u32 = 0x0F00FFC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_18_fc00_0f00ffe0() {
    // Encoding: 0x0F00FFE0
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=31, Rd=0
    // Fields: Q=0, Rd=0, immh=0, immb=0, Rn=31, U=0
    let encoding: u32 = 0x0F00FFE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_19_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, immh=0, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x0F00FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_20_fc00_0f00fc01() {
    // Encoding: 0x0F00FC01
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=1
    // Fields: U=0, immb=0, Q=0, Rn=0, Rd=1, immh=0
    let encoding: u32 = 0x0F00FC01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_21_fc00_0f00fc1e() {
    // Encoding: 0x0F00FC1E
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=30
    // Fields: Q=0, immb=0, U=0, Rn=0, immh=0, Rd=30
    let encoding: u32 = 0x0F00FC1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_22_fc00_0f00fc1f() {
    // Encoding: 0x0F00FC1F
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=31
    // Fields: Q=0, Rd=31, Rn=0, immh=0, immb=0, U=0
    let encoding: u32 = 0x0F00FC1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_23_fc00_0f00fc21() {
    // Encoding: 0x0F00FC21
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=1, Rd=1
    // Fields: U=0, Rd=1, immb=0, immh=0, Rn=1, Q=0
    let encoding: u32 = 0x0F00FC21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_conv_float_simd_combo_24_fc00_0f00ffff() {
    // Encoding: 0x0F00FFFF
    // Test aarch64_vector_shift_conv_float_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=31, Rd=31
    // Fields: immh=0, immb=0, Rd=31, Rn=31, Q=0, U=0
    let encoding: u32 = 0x0F00FFFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_shift_conv_float_simd_special_q_0_size_variant_0_64512_0f09fc00() {
    // Encoding: 0x0F09FC00
    // Test aarch64_vector_shift_conv_float_simd special value Q = 0 (Size variant 0)
    // Fields: Rd=0, immb=1, Q=0, immh=1, U=0, Rn=0
    let encoding: u32 = 0x0F09FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_shift_conv_float_simd_special_q_1_size_variant_1_64512_4f09fc00() {
    // Encoding: 0x4F09FC00
    // Test aarch64_vector_shift_conv_float_simd special value Q = 1 (Size variant 1)
    // Fields: U=0, Rd=0, immh=1, Rn=0, Q=1, immb=1
    let encoding: u32 = 0x4F09FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_conv_float_simd_special_rn_31_stack_pointer_sp_may_require_alignment_64512_0f09ffe0()
 {
    // Encoding: 0x0F09FFE0
    // Test aarch64_vector_shift_conv_float_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, U=0, Rd=0, immh=1, Q=0, immb=1
    let encoding: u32 = 0x0F09FFE0;
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

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_conv_float_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_64512_0f09fc1f()
 {
    // Encoding: 0x0F09FC1F
    // Test aarch64_vector_shift_conv_float_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Q=0, immb=1, U=0, Rn=0, immh=1, Rd=31
    let encoding: u32 = 0x0F09FC1F;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_reg_write_0_5f00fc00() {
    // Test aarch64_vector_shift_conv_float_sisd register write: SimdFromField("d")
    // Encoding: 0x5F00FC00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F00FC00;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_sp_rn_5f00ffe0() {
    // Test aarch64_vector_shift_conv_float_sisd with Rn = SP (31)
    // Encoding: 0x5F00FFE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F00FFE0;
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

/// Provenance: aarch64_vector_shift_conv_float_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_conv_float_sisd_zr_rd_5f00fc1f() {
    // Test aarch64_vector_shift_conv_float_sisd with Rd = ZR (31)
    // Encoding: 0x5F00FC1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F00FC1F;
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

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_conv_float_simd_reg_write_0_0f00fc00() {
    // Test aarch64_vector_shift_conv_float_simd register write: SimdFromField("d")
    // Encoding: 0x0F00FC00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00FC00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_sp_rn_0f00ffe0() {
    // Test aarch64_vector_shift_conv_float_simd with Rn = SP (31)
    // Encoding: 0x0F00FFE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00FFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_conv_float_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_conv_float_simd_zr_rd_0f00fc1f() {
    // Test aarch64_vector_shift_conv_float_simd with Rd = ZR (31)
    // Encoding: 0x0F00FC1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00FC1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_shift_right_narrow_logical Tests
// ============================================================================

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_q_0_min_8400_0f008400() {
    // Encoding: 0x0F008400
    // Test aarch64_vector_shift_right_narrow_logical field Q = 0 (Min)
    // Fields: immb=0, Rd=0, immh=0, op=0, Q=0, Rn=0
    let encoding: u32 = 0x0F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_q_1_max_8400_4f008400() {
    // Encoding: 0x4F008400
    // Test aarch64_vector_shift_right_narrow_logical field Q = 1 (Max)
    // Fields: Q=1, immb=0, immh=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x4F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_immh_0_zero_8400_0f008400() {
    // Encoding: 0x0F008400
    // Test aarch64_vector_shift_right_narrow_logical field immh = 0 (Zero)
    // Fields: Rn=0, Rd=0, Q=0, immh=0, immb=0, op=0
    let encoding: u32 = 0x0F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_immh_1_poweroftwo_8400_0f088400() {
    // Encoding: 0x0F088400
    // Test aarch64_vector_shift_right_narrow_logical field immh = 1 (PowerOfTwo)
    // Fields: immh=1, Rn=0, op=0, Rd=0, Q=0, immb=0
    let encoding: u32 = 0x0F088400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_immh_3_poweroftwominusone_8400_0f188400() {
    // Encoding: 0x0F188400
    // Test aarch64_vector_shift_right_narrow_logical field immh = 3 (PowerOfTwoMinusOne)
    // Fields: immb=0, Q=0, immh=3, op=0, Rd=0, Rn=0
    let encoding: u32 = 0x0F188400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_immh_4_poweroftwo_8400_0f208400() {
    // Encoding: 0x0F208400
    // Test aarch64_vector_shift_right_narrow_logical field immh = 4 (PowerOfTwo)
    // Fields: immb=0, Rd=0, Rn=0, immh=4, Q=0, op=0
    let encoding: u32 = 0x0F208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_immh_7_poweroftwominusone_8400_0f388400() {
    // Encoding: 0x0F388400
    // Test aarch64_vector_shift_right_narrow_logical field immh = 7 (PowerOfTwoMinusOne)
    // Fields: Q=0, op=0, Rd=0, immb=0, immh=7, Rn=0
    let encoding: u32 = 0x0F388400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_immh_8_poweroftwo_8400_0f408400() {
    // Encoding: 0x0F408400
    // Test aarch64_vector_shift_right_narrow_logical field immh = 8 (PowerOfTwo)
    // Fields: immb=0, op=0, Rn=0, Rd=0, immh=8, Q=0
    let encoding: u32 = 0x0F408400;
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

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_immh_15_max_8400_0f788400() {
    // Encoding: 0x0F788400
    // Test aarch64_vector_shift_right_narrow_logical field immh = 15 (Max)
    // Fields: Rd=0, immh=15, Rn=0, op=0, immb=0, Q=0
    let encoding: u32 = 0x0F788400;
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

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_immb_0_zero_8400_0f008400() {
    // Encoding: 0x0F008400
    // Test aarch64_vector_shift_right_narrow_logical field immb = 0 (Zero)
    // Fields: immb=0, Rd=0, Rn=0, op=0, Q=0, immh=0
    let encoding: u32 = 0x0F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_immb_1_poweroftwo_8400_0f018400() {
    // Encoding: 0x0F018400
    // Test aarch64_vector_shift_right_narrow_logical field immb = 1 (PowerOfTwo)
    // Fields: immb=1, Rn=0, immh=0, op=0, Q=0, Rd=0
    let encoding: u32 = 0x0F018400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_immb_3_poweroftwominusone_8400_0f038400() {
    // Encoding: 0x0F038400
    // Test aarch64_vector_shift_right_narrow_logical field immb = 3 (PowerOfTwoMinusOne)
    // Fields: Q=0, immb=3, immh=0, op=0, Rd=0, Rn=0
    let encoding: u32 = 0x0F038400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_immb_7_max_8400_0f078400() {
    // Encoding: 0x0F078400
    // Test aarch64_vector_shift_right_narrow_logical field immb = 7 (Max)
    // Fields: Rd=0, op=0, Q=0, Rn=0, immh=0, immb=7
    let encoding: u32 = 0x0F078400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field op 11 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_op_0_min_8400_0f008400() {
    // Encoding: 0x0F008400
    // Test aarch64_vector_shift_right_narrow_logical field op = 0 (Min)
    // Fields: Q=0, immh=0, op=0, Rn=0, Rd=0, immb=0
    let encoding: u32 = 0x0F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field op 11 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_op_1_max_8400_0f008c00() {
    // Encoding: 0x0F008C00
    // Test aarch64_vector_shift_right_narrow_logical field op = 1 (Max)
    // Fields: Q=0, Rn=0, Rd=0, op=1, immb=0, immh=0
    let encoding: u32 = 0x0F008C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_rn_0_min_8400_0f008400() {
    // Encoding: 0x0F008400
    // Test aarch64_vector_shift_right_narrow_logical field Rn = 0 (Min)
    // Fields: immb=0, immh=0, op=0, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x0F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_rn_1_poweroftwo_8400_0f008420() {
    // Encoding: 0x0F008420
    // Test aarch64_vector_shift_right_narrow_logical field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, op=0, Rd=0, Q=0, immb=0, immh=0
    let encoding: u32 = 0x0F008420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_rn_30_poweroftwominusone_8400_0f0087c0() {
    // Encoding: 0x0F0087C0
    // Test aarch64_vector_shift_right_narrow_logical field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: op=0, Q=0, immb=0, Rn=30, Rd=0, immh=0
    let encoding: u32 = 0x0F0087C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_rn_31_max_8400_0f0087e0() {
    // Encoding: 0x0F0087E0
    // Test aarch64_vector_shift_right_narrow_logical field Rn = 31 (Max)
    // Fields: Q=0, immh=0, op=0, Rd=0, Rn=31, immb=0
    let encoding: u32 = 0x0F0087E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_rd_0_min_8400_0f008400() {
    // Encoding: 0x0F008400
    // Test aarch64_vector_shift_right_narrow_logical field Rd = 0 (Min)
    // Fields: Q=0, op=0, immh=0, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x0F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_rd_1_poweroftwo_8400_0f008401() {
    // Encoding: 0x0F008401
    // Test aarch64_vector_shift_right_narrow_logical field Rd = 1 (PowerOfTwo)
    // Fields: immb=0, op=0, Q=0, Rn=0, Rd=1, immh=0
    let encoding: u32 = 0x0F008401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_rd_30_poweroftwominusone_8400_0f00841e() {
    // Encoding: 0x0F00841E
    // Test aarch64_vector_shift_right_narrow_logical field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=30, op=0, Q=0, immh=0, immb=0
    let encoding: u32 = 0x0F00841E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_field_rd_31_max_8400_0f00841f() {
    // Encoding: 0x0F00841F
    // Test aarch64_vector_shift_right_narrow_logical field Rd = 31 (Max)
    // Fields: Q=0, immb=0, Rn=0, op=0, immh=0, Rd=31
    let encoding: u32 = 0x0F00841F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_0_8400_0f008400() {
    // Encoding: 0x0F008400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, op=0, Rn=0, Q=0, immh=0, Rd=0
    let encoding: u32 = 0x0F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_1_8400_4f008400() {
    // Encoding: 0x4F008400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=1, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, immh=0, Q=1, immb=0, op=0, Rd=0
    let encoding: u32 = 0x4F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_2_8400_0f008400() {
    // Encoding: 0x0F008400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, immb=0, immh=0, op=0, Rn=0, Rd=0
    let encoding: u32 = 0x0F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_3_8400_0f088400() {
    // Encoding: 0x0F088400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=1, immb=0, op=0, Rn=0, Rd=0
    // Fields: op=0, Rd=0, Rn=0, Q=0, immh=1, immb=0
    let encoding: u32 = 0x0F088400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_4_8400_0f188400() {
    // Encoding: 0x0F188400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=3, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, Rn=0, immb=0, op=0, immh=3
    let encoding: u32 = 0x0F188400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_5_8400_0f208400() {
    // Encoding: 0x0F208400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=4, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, immh=4, Rd=0, op=0, immb=0
    let encoding: u32 = 0x0F208400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_6_8400_0f388400() {
    // Encoding: 0x0F388400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=7, immb=0, op=0, Rn=0, Rd=0
    // Fields: op=0, Rn=0, Rd=0, Q=0, immh=7, immb=0
    let encoding: u32 = 0x0F388400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_7_8400_0f408400() {
    // Encoding: 0x0F408400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=8, immb=0, op=0, Rn=0, Rd=0
    // Fields: immh=8, Rn=0, Q=0, immb=0, Rd=0, op=0
    let encoding: u32 = 0x0F408400;
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

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_8_8400_0f788400() {
    // Encoding: 0x0F788400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=15, immb=0, op=0, Rn=0, Rd=0
    // Fields: Q=0, immh=15, immb=0, Rd=0, Rn=0, op=0
    let encoding: u32 = 0x0F788400;
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

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_9_8400_0f008400() {
    // Encoding: 0x0F008400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rn=0, immb=0, Rd=0, op=0, Q=0, immh=0
    let encoding: u32 = 0x0F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_10_8400_0f018400() {
    // Encoding: 0x0F018400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=1, op=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, op=0, immb=1, immh=0, Rn=0
    let encoding: u32 = 0x0F018400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_11_8400_0f038400() {
    // Encoding: 0x0F038400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=3, op=0, Rn=0, Rd=0
    // Fields: immh=0, immb=3, op=0, Rd=0, Rn=0, Q=0
    let encoding: u32 = 0x0F038400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_12_8400_0f078400() {
    // Encoding: 0x0F078400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=7, op=0, Rn=0, Rd=0
    // Fields: immb=7, Q=0, immh=0, Rn=0, op=0, Rd=0
    let encoding: u32 = 0x0F078400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_13_8400_0f008400() {
    // Encoding: 0x0F008400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, immh=0, immb=0, op=0, Rn=0
    let encoding: u32 = 0x0F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_14_8400_0f008c00() {
    // Encoding: 0x0F008C00
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=1, Rn=0, Rd=0
    // Fields: immh=0, Rn=0, Rd=0, Q=0, immb=0, op=1
    let encoding: u32 = 0x0F008C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_15_8400_0f008400() {
    // Encoding: 0x0F008400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, Q=0, Rn=0, immh=0, Rd=0, op=0
    let encoding: u32 = 0x0F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_16_8400_0f008420() {
    // Encoding: 0x0F008420
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=1, Rd=0
    // Fields: Rn=1, immh=0, Rd=0, immb=0, op=0, Q=0
    let encoding: u32 = 0x0F008420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_17_8400_0f0087c0() {
    // Encoding: 0x0F0087C0
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=30, Rd=0
    // Fields: immh=0, Q=0, op=0, immb=0, Rd=0, Rn=30
    let encoding: u32 = 0x0F0087C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_18_8400_0f0087e0() {
    // Encoding: 0x0F0087E0
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=31, Rd=0
    // Fields: op=0, Rn=31, Rd=0, immh=0, immb=0, Q=0
    let encoding: u32 = 0x0F0087E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_19_8400_0f008400() {
    // Encoding: 0x0F008400
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=0
    // Fields: immb=0, immh=0, op=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x0F008400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_20_8400_0f008401() {
    // Encoding: 0x0F008401
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=1
    // Fields: Rd=1, immh=0, Q=0, immb=0, op=0, Rn=0
    let encoding: u32 = 0x0F008401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_21_8400_0f00841e() {
    // Encoding: 0x0F00841E
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=30
    // Fields: Rn=0, immh=0, Rd=30, op=0, Q=0, immb=0
    let encoding: u32 = 0x0F00841E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_22_8400_0f00841f() {
    // Encoding: 0x0F00841F
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=0, Rd=31
    // Fields: Rd=31, op=0, Rn=0, Q=0, immb=0, immh=0
    let encoding: u32 = 0x0F00841F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_23_8400_0f008421() {
    // Encoding: 0x0F008421
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=1, Rd=1
    // Fields: Q=0, Rn=1, Rd=1, immh=0, op=0, immb=0
    let encoding: u32 = 0x0F008421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_combo_24_8400_0f0087ff() {
    // Encoding: 0x0F0087FF
    // Test aarch64_vector_shift_right_narrow_logical field combination: Q=0, immh=0, immb=0, op=0, Rn=31, Rd=31
    // Fields: op=0, Rn=31, immb=0, Rd=31, immh=0, Q=0
    let encoding: u32 = 0x0F0087FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_special_q_0_size_variant_0_33792_0f098400() {
    // Encoding: 0x0F098400
    // Test aarch64_vector_shift_right_narrow_logical special value Q = 0 (Size variant 0)
    // Fields: op=0, Rn=0, Rd=0, immb=1, Q=0, immh=1
    let encoding: u32 = 0x0F098400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_special_q_1_size_variant_1_33792_4f098400() {
    // Encoding: 0x4F098400
    // Test aarch64_vector_shift_right_narrow_logical special value Q = 1 (Size variant 1)
    // Fields: Rd=0, immb=1, Rn=0, Q=1, immh=1, op=0
    let encoding: u32 = 0x4F098400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_special_rn_31_stack_pointer_sp_may_require_alignment_33792_0f0987e0()
 {
    // Encoding: 0x0F0987E0
    // Test aarch64_vector_shift_right_narrow_logical special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, immh=1, op=0, Rn=31, Rd=0, immb=1
    let encoding: u32 = 0x0F0987E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_33792_0f09841f()
 {
    // Encoding: 0x0F09841F
    // Test aarch64_vector_shift_right_narrow_logical special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, immh=1, Q=0, immb=1, op=0, Rd=31
    let encoding: u32 = 0x0F09841F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_sp_rn_0f0087e0() {
    // Test aarch64_vector_shift_right_narrow_logical with Rn = SP (31)
    // Encoding: 0x0F0087E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F0087E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_right_narrow_logical
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_right_narrow_logical_zr_rd_0f00841f() {
    // Test aarch64_vector_shift_right_narrow_logical with Rd = ZR (31)
    // Encoding: 0x0F00841F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00841F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_shift_conv_int_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_u_0_min_e400_5f00e400() {
    // Encoding: 0x5F00E400
    // Test aarch64_vector_shift_conv_int_sisd field U = 0 (Min)
    // Fields: immh=0, Rd=0, Rn=0, immb=0, U=0
    let encoding: u32 = 0x5F00E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_u_1_max_e400_7f00e400() {
    // Encoding: 0x7F00E400
    // Test aarch64_vector_shift_conv_int_sisd field U = 1 (Max)
    // Fields: Rn=0, immb=0, U=1, immh=0, Rd=0
    let encoding: u32 = 0x7F00E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_immh_0_zero_e400_5f00e400() {
    // Encoding: 0x5F00E400
    // Test aarch64_vector_shift_conv_int_sisd field immh = 0 (Zero)
    // Fields: immh=0, immb=0, Rd=0, U=0, Rn=0
    let encoding: u32 = 0x5F00E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_immh_1_poweroftwo_e400_5f08e400() {
    // Encoding: 0x5F08E400
    // Test aarch64_vector_shift_conv_int_sisd field immh = 1 (PowerOfTwo)
    // Fields: Rd=0, immb=0, U=0, immh=1, Rn=0
    let encoding: u32 = 0x5F08E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_immh_3_poweroftwominusone_e400_5f18e400() {
    // Encoding: 0x5F18E400
    // Test aarch64_vector_shift_conv_int_sisd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: immh=3, Rd=0, U=0, immb=0, Rn=0
    let encoding: u32 = 0x5F18E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_immh_4_poweroftwo_e400_5f20e400() {
    // Encoding: 0x5F20E400
    // Test aarch64_vector_shift_conv_int_sisd field immh = 4 (PowerOfTwo)
    // Fields: Rd=0, Rn=0, U=0, immb=0, immh=4
    let encoding: u32 = 0x5F20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_immh_7_poweroftwominusone_e400_5f38e400() {
    // Encoding: 0x5F38E400
    // Test aarch64_vector_shift_conv_int_sisd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, immh=7, Rd=0, immb=0, U=0
    let encoding: u32 = 0x5F38E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_immh_8_poweroftwo_e400_5f40e400() {
    // Encoding: 0x5F40E400
    // Test aarch64_vector_shift_conv_int_sisd field immh = 8 (PowerOfTwo)
    // Fields: Rd=0, U=0, immh=8, Rn=0, immb=0
    let encoding: u32 = 0x5F40E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_immh_15_max_e400_5f78e400() {
    // Encoding: 0x5F78E400
    // Test aarch64_vector_shift_conv_int_sisd field immh = 15 (Max)
    // Fields: Rd=0, immb=0, U=0, Rn=0, immh=15
    let encoding: u32 = 0x5F78E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_immb_0_zero_e400_5f00e400() {
    // Encoding: 0x5F00E400
    // Test aarch64_vector_shift_conv_int_sisd field immb = 0 (Zero)
    // Fields: U=0, Rn=0, immh=0, Rd=0, immb=0
    let encoding: u32 = 0x5F00E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_immb_1_poweroftwo_e400_5f01e400() {
    // Encoding: 0x5F01E400
    // Test aarch64_vector_shift_conv_int_sisd field immb = 1 (PowerOfTwo)
    // Fields: immh=0, Rn=0, immb=1, U=0, Rd=0
    let encoding: u32 = 0x5F01E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_immb_3_poweroftwominusone_e400_5f03e400() {
    // Encoding: 0x5F03E400
    // Test aarch64_vector_shift_conv_int_sisd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: immb=3, U=0, Rn=0, immh=0, Rd=0
    let encoding: u32 = 0x5F03E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_immb_7_max_e400_5f07e400() {
    // Encoding: 0x5F07E400
    // Test aarch64_vector_shift_conv_int_sisd field immb = 7 (Max)
    // Fields: immh=0, Rd=0, Rn=0, U=0, immb=7
    let encoding: u32 = 0x5F07E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_rn_0_min_e400_5f00e400() {
    // Encoding: 0x5F00E400
    // Test aarch64_vector_shift_conv_int_sisd field Rn = 0 (Min)
    // Fields: U=0, immh=0, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x5F00E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_rn_1_poweroftwo_e400_5f00e420() {
    // Encoding: 0x5F00E420
    // Test aarch64_vector_shift_conv_int_sisd field Rn = 1 (PowerOfTwo)
    // Fields: immh=0, Rn=1, Rd=0, immb=0, U=0
    let encoding: u32 = 0x5F00E420;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_rn_30_poweroftwominusone_e400_5f00e7c0() {
    // Encoding: 0x5F00E7C0
    // Test aarch64_vector_shift_conv_int_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: immh=0, U=0, immb=0, Rd=0, Rn=30
    let encoding: u32 = 0x5F00E7C0;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_rn_31_max_e400_5f00e7e0() {
    // Encoding: 0x5F00E7E0
    // Test aarch64_vector_shift_conv_int_sisd field Rn = 31 (Max)
    // Fields: immb=0, Rn=31, Rd=0, U=0, immh=0
    let encoding: u32 = 0x5F00E7E0;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_rd_0_min_e400_5f00e400() {
    // Encoding: 0x5F00E400
    // Test aarch64_vector_shift_conv_int_sisd field Rd = 0 (Min)
    // Fields: Rd=0, immb=0, U=0, Rn=0, immh=0
    let encoding: u32 = 0x5F00E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_rd_1_poweroftwo_e400_5f00e401() {
    // Encoding: 0x5F00E401
    // Test aarch64_vector_shift_conv_int_sisd field Rd = 1 (PowerOfTwo)
    // Fields: U=0, immh=0, Rn=0, Rd=1, immb=0
    let encoding: u32 = 0x5F00E401;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_rd_30_poweroftwominusone_e400_5f00e41e() {
    // Encoding: 0x5F00E41E
    // Test aarch64_vector_shift_conv_int_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: U=0, Rn=0, Rd=30, immh=0, immb=0
    let encoding: u32 = 0x5F00E41E;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_field_rd_31_max_e400_5f00e41f() {
    // Encoding: 0x5F00E41F
    // Test aarch64_vector_shift_conv_int_sisd field Rd = 31 (Max)
    // Fields: immb=0, immh=0, U=0, Rn=0, Rd=31
    let encoding: u32 = 0x5F00E41F;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_0_e400_5f00e400() {
    // Encoding: 0x5F00E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: U=0, immh=0, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x5F00E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_1_e400_7f00e400() {
    // Encoding: 0x7F00E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=1, immh=0, immb=0, Rn=0, Rd=0
    // Fields: U=1, immh=0, Rd=0, immb=0, Rn=0
    let encoding: u32 = 0x7F00E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_2_e400_5f00e400() {
    // Encoding: 0x5F00E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: U=0, immh=0, Rn=0, Rd=0, immb=0
    let encoding: u32 = 0x5F00E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_3_e400_5f08e400() {
    // Encoding: 0x5F08E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=1, immb=0, Rn=0, Rd=0
    // Fields: immb=0, U=0, immh=1, Rd=0, Rn=0
    let encoding: u32 = 0x5F08E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_4_e400_5f18e400() {
    // Encoding: 0x5F18E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=3, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rd=0, U=0, immh=3, Rn=0
    let encoding: u32 = 0x5F18E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_5_e400_5f20e400() {
    // Encoding: 0x5F20E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=4, immb=0, Rn=0, Rd=0
    // Fields: U=0, immb=0, Rn=0, Rd=0, immh=4
    let encoding: u32 = 0x5F20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_6_e400_5f38e400() {
    // Encoding: 0x5F38E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=7, immb=0, Rn=0, Rd=0
    // Fields: U=0, Rd=0, immh=7, Rn=0, immb=0
    let encoding: u32 = 0x5F38E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_7_e400_5f40e400() {
    // Encoding: 0x5F40E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=8, immb=0, Rn=0, Rd=0
    // Fields: immh=8, U=0, Rd=0, immb=0, Rn=0
    let encoding: u32 = 0x5F40E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_8_e400_5f78e400() {
    // Encoding: 0x5F78E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=15, immb=0, Rn=0, Rd=0
    // Fields: immh=15, Rd=0, Rn=0, U=0, immb=0
    let encoding: u32 = 0x5F78E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_9_e400_5f00e400() {
    // Encoding: 0x5F00E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immh=0, immb=0, U=0, Rd=0
    let encoding: u32 = 0x5F00E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_10_e400_5f01e400() {
    // Encoding: 0x5F01E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=1, Rn=0, Rd=0
    // Fields: immb=1, Rn=0, U=0, Rd=0, immh=0
    let encoding: u32 = 0x5F01E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_11_e400_5f03e400() {
    // Encoding: 0x5F03E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=3, Rn=0, Rd=0
    // Fields: immh=0, U=0, Rn=0, Rd=0, immb=3
    let encoding: u32 = 0x5F03E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_12_e400_5f07e400() {
    // Encoding: 0x5F07E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=7, Rn=0, Rd=0
    // Fields: Rn=0, U=0, immb=7, Rd=0, immh=0
    let encoding: u32 = 0x5F07E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_13_e400_5f00e400() {
    // Encoding: 0x5F00E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, immb=0, U=0, immh=0, Rn=0
    let encoding: u32 = 0x5F00E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_14_e400_5f00e420() {
    // Encoding: 0x5F00E420
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=0, Rn=1, Rd=0
    // Fields: Rd=0, immb=0, U=0, Rn=1, immh=0
    let encoding: u32 = 0x5F00E420;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_15_e400_5f00e7c0() {
    // Encoding: 0x5F00E7C0
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=0, Rn=30, Rd=0
    // Fields: U=0, immb=0, immh=0, Rn=30, Rd=0
    let encoding: u32 = 0x5F00E7C0;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_16_e400_5f00e7e0() {
    // Encoding: 0x5F00E7E0
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=0, Rn=31, Rd=0
    // Fields: immb=0, U=0, immh=0, Rn=31, Rd=0
    let encoding: u32 = 0x5F00E7E0;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_17_e400_5f00e400() {
    // Encoding: 0x5F00E400
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: U=0, immb=0, immh=0, Rn=0, Rd=0
    let encoding: u32 = 0x5F00E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_18_e400_5f00e401() {
    // Encoding: 0x5F00E401
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=1
    // Fields: immh=0, Rn=0, Rd=1, immb=0, U=0
    let encoding: u32 = 0x5F00E401;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_19_e400_5f00e41e() {
    // Encoding: 0x5F00E41E
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=30
    // Fields: immh=0, immb=0, U=0, Rn=0, Rd=30
    let encoding: u32 = 0x5F00E41E;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_20_e400_5f00e41f() {
    // Encoding: 0x5F00E41F
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=0, Rn=0, Rd=31
    // Fields: U=0, immh=0, immb=0, Rd=31, Rn=0
    let encoding: u32 = 0x5F00E41F;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_21_e400_5f00e421() {
    // Encoding: 0x5F00E421
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=0, Rn=1, Rd=1
    // Fields: Rd=1, immh=0, U=0, Rn=1, immb=0
    let encoding: u32 = 0x5F00E421;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_combo_22_e400_5f00e7ff() {
    // Encoding: 0x5F00E7FF
    // Test aarch64_vector_shift_conv_int_sisd field combination: U=0, immh=0, immb=0, Rn=31, Rd=31
    // Fields: Rn=31, Rd=31, U=0, immb=0, immh=0
    let encoding: u32 = 0x5F00E7FF;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_58368_5f09e7e0()
 {
    // Encoding: 0x5F09E7E0
    // Test aarch64_vector_shift_conv_int_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: immh=1, U=0, immb=1, Rn=31, Rd=0
    let encoding: u32 = 0x5F09E7E0;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_58368_5f09e41f()
 {
    // Encoding: 0x5F09E41F
    // Test aarch64_vector_shift_conv_int_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: immb=1, Rd=31, immh=1, Rn=0, U=0
    let encoding: u32 = 0x5F09E41F;
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

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_q_0_min_e400_0f00e400() {
    // Encoding: 0x0F00E400
    // Test aarch64_vector_shift_conv_int_simd field Q = 0 (Min)
    // Fields: immb=0, U=0, immh=0, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x0F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_q_1_max_e400_4f00e400() {
    // Encoding: 0x4F00E400
    // Test aarch64_vector_shift_conv_int_simd field Q = 1 (Max)
    // Fields: immh=0, immb=0, Rd=0, U=0, Rn=0, Q=1
    let encoding: u32 = 0x4F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_u_0_min_e400_0f00e400() {
    // Encoding: 0x0F00E400
    // Test aarch64_vector_shift_conv_int_simd field U = 0 (Min)
    // Fields: U=0, immb=0, immh=0, Q=0, Rd=0, Rn=0
    let encoding: u32 = 0x0F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field U 29 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_u_1_max_e400_2f00e400() {
    // Encoding: 0x2F00E400
    // Test aarch64_vector_shift_conv_int_simd field U = 1 (Max)
    // Fields: Rn=0, immh=0, immb=0, U=1, Q=0, Rd=0
    let encoding: u32 = 0x2F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_immh_0_zero_e400_0f00e400() {
    // Encoding: 0x0F00E400
    // Test aarch64_vector_shift_conv_int_simd field immh = 0 (Zero)
    // Fields: immh=0, Rd=0, U=0, immb=0, Q=0, Rn=0
    let encoding: u32 = 0x0F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_immh_1_poweroftwo_e400_0f08e400() {
    // Encoding: 0x0F08E400
    // Test aarch64_vector_shift_conv_int_simd field immh = 1 (PowerOfTwo)
    // Fields: Q=0, Rd=0, immb=0, immh=1, U=0, Rn=0
    let encoding: u32 = 0x0F08E400;
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

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_immh_3_poweroftwominusone_e400_0f18e400() {
    // Encoding: 0x0F18E400
    // Test aarch64_vector_shift_conv_int_simd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Q=0, immb=0, Rn=0, U=0, immh=3
    let encoding: u32 = 0x0F18E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_immh_4_poweroftwo_e400_0f20e400() {
    // Encoding: 0x0F20E400
    // Test aarch64_vector_shift_conv_int_simd field immh = 4 (PowerOfTwo)
    // Fields: Q=0, immb=0, Rn=0, Rd=0, U=0, immh=4
    let encoding: u32 = 0x0F20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_immh_7_poweroftwominusone_e400_0f38e400() {
    // Encoding: 0x0F38E400
    // Test aarch64_vector_shift_conv_int_simd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: immh=7, immb=0, Rn=0, Rd=0, U=0, Q=0
    let encoding: u32 = 0x0F38E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_immh_8_poweroftwo_e400_0f40e400() {
    // Encoding: 0x0F40E400
    // Test aarch64_vector_shift_conv_int_simd field immh = 8 (PowerOfTwo)
    // Fields: Rd=0, immh=8, immb=0, Rn=0, Q=0, U=0
    let encoding: u32 = 0x0F40E400;
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

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_immh_15_max_e400_0f78e400() {
    // Encoding: 0x0F78E400
    // Test aarch64_vector_shift_conv_int_simd field immh = 15 (Max)
    // Fields: Rd=0, U=0, immh=15, Rn=0, Q=0, immb=0
    let encoding: u32 = 0x0F78E400;
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

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_immb_0_zero_e400_0f00e400() {
    // Encoding: 0x0F00E400
    // Test aarch64_vector_shift_conv_int_simd field immb = 0 (Zero)
    // Fields: Q=0, immh=0, immb=0, Rn=0, Rd=0, U=0
    let encoding: u32 = 0x0F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_immb_1_poweroftwo_e400_0f01e400() {
    // Encoding: 0x0F01E400
    // Test aarch64_vector_shift_conv_int_simd field immb = 1 (PowerOfTwo)
    // Fields: immh=0, U=0, Q=0, immb=1, Rd=0, Rn=0
    let encoding: u32 = 0x0F01E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_immb_3_poweroftwominusone_e400_0f03e400() {
    // Encoding: 0x0F03E400
    // Test aarch64_vector_shift_conv_int_simd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: U=0, immb=3, Rd=0, Q=0, immh=0, Rn=0
    let encoding: u32 = 0x0F03E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_immb_7_max_e400_0f07e400() {
    // Encoding: 0x0F07E400
    // Test aarch64_vector_shift_conv_int_simd field immb = 7 (Max)
    // Fields: immh=0, immb=7, Rn=0, U=0, Rd=0, Q=0
    let encoding: u32 = 0x0F07E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_rn_0_min_e400_0f00e400() {
    // Encoding: 0x0F00E400
    // Test aarch64_vector_shift_conv_int_simd field Rn = 0 (Min)
    // Fields: Rn=0, immh=0, U=0, Q=0, immb=0, Rd=0
    let encoding: u32 = 0x0F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_rn_1_poweroftwo_e400_0f00e420() {
    // Encoding: 0x0F00E420
    // Test aarch64_vector_shift_conv_int_simd field Rn = 1 (PowerOfTwo)
    // Fields: U=0, Rn=1, Q=0, immb=0, Rd=0, immh=0
    let encoding: u32 = 0x0F00E420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_rn_30_poweroftwominusone_e400_0f00e7c0() {
    // Encoding: 0x0F00E7C0
    // Test aarch64_vector_shift_conv_int_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Rn=30, immh=0, U=0, immb=0, Q=0
    let encoding: u32 = 0x0F00E7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_rn_31_max_e400_0f00e7e0() {
    // Encoding: 0x0F00E7E0
    // Test aarch64_vector_shift_conv_int_simd field Rn = 31 (Max)
    // Fields: Rd=0, immb=0, Rn=31, U=0, Q=0, immh=0
    let encoding: u32 = 0x0F00E7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_rd_0_min_e400_0f00e400() {
    // Encoding: 0x0F00E400
    // Test aarch64_vector_shift_conv_int_simd field Rd = 0 (Min)
    // Fields: Q=0, U=0, immb=0, Rd=0, Rn=0, immh=0
    let encoding: u32 = 0x0F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_rd_1_poweroftwo_e400_0f00e401() {
    // Encoding: 0x0F00E401
    // Test aarch64_vector_shift_conv_int_simd field Rd = 1 (PowerOfTwo)
    // Fields: immb=0, Rd=1, U=0, immh=0, Rn=0, Q=0
    let encoding: u32 = 0x0F00E401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_rd_30_poweroftwominusone_e400_0f00e41e() {
    // Encoding: 0x0F00E41E
    // Test aarch64_vector_shift_conv_int_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=30, immh=0, U=0, immb=0, Rn=0, Q=0
    let encoding: u32 = 0x0F00E41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_field_rd_31_max_e400_0f00e41f() {
    // Encoding: 0x0F00E41F
    // Test aarch64_vector_shift_conv_int_simd field Rd = 31 (Max)
    // Fields: Rn=0, immh=0, Q=0, immb=0, Rd=31, U=0
    let encoding: u32 = 0x0F00E41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_0_e400_0f00e400() {
    // Encoding: 0x0F00E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, immb=0, U=0, immh=0, Rn=0
    let encoding: u32 = 0x0F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_1_e400_4f00e400() {
    // Encoding: 0x4F00E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=1, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Q=1, immb=0, Rn=0, immh=0, U=0, Rd=0
    let encoding: u32 = 0x4F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_2_e400_0f00e400() {
    // Encoding: 0x0F00E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: U=0, Q=0, immb=0, Rd=0, immh=0, Rn=0
    let encoding: u32 = 0x0F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=1 (maximum value (1))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_3_e400_2f00e400() {
    // Encoding: 0x2F00E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=1, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, Rd=0, Q=0, U=1, immh=0
    let encoding: u32 = 0x2F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_4_e400_0f00e400() {
    // Encoding: 0x0F00E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, U=0, immh=0, immb=0, Rd=0, Q=0
    let encoding: u32 = 0x0F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_5_e400_0f08e400() {
    // Encoding: 0x0F08E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=1, immb=0, Rn=0, Rd=0
    // Fields: immh=1, Rn=0, immb=0, Q=0, Rd=0, U=0
    let encoding: u32 = 0x0F08E400;
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

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_6_e400_0f18e400() {
    // Encoding: 0x0F18E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=3, immb=0, Rn=0, Rd=0
    // Fields: U=0, immh=3, immb=0, Q=0, Rd=0, Rn=0
    let encoding: u32 = 0x0F18E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_7_e400_0f20e400() {
    // Encoding: 0x0F20E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=4, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, U=0, immh=4, Q=0, immb=0, Rd=0
    let encoding: u32 = 0x0F20E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_8_e400_0f38e400() {
    // Encoding: 0x0F38E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=7, immb=0, Rn=0, Rd=0
    // Fields: Q=0, U=0, immb=0, Rd=0, Rn=0, immh=7
    let encoding: u32 = 0x0F38E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_9_e400_0f40e400() {
    // Encoding: 0x0F40E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=8, immb=0, Rn=0, Rd=0
    // Fields: Q=0, immb=0, U=0, immh=8, Rn=0, Rd=0
    let encoding: u32 = 0x0F40E400;
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

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_10_e400_0f78e400() {
    // Encoding: 0x0F78E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=15, immb=0, Rn=0, Rd=0
    // Fields: Q=0, immb=0, Rd=0, Rn=0, immh=15, U=0
    let encoding: u32 = 0x0F78E400;
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

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_11_e400_0f00e400() {
    // Encoding: 0x0F00E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immh=0, Rd=0, U=0, Q=0, immb=0
    let encoding: u32 = 0x0F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_12_e400_0f01e400() {
    // Encoding: 0x0F01E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=1, Rn=0, Rd=0
    // Fields: immb=1, Q=0, Rd=0, U=0, immh=0, Rn=0
    let encoding: u32 = 0x0F01E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_13_e400_0f03e400() {
    // Encoding: 0x0F03E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=3, Rn=0, Rd=0
    // Fields: Rd=0, U=0, Q=0, immh=0, immb=3, Rn=0
    let encoding: u32 = 0x0F03E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_14_e400_0f07e400() {
    // Encoding: 0x0F07E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=7, Rn=0, Rd=0
    // Fields: immb=7, immh=0, Rn=0, Q=0, U=0, Rd=0
    let encoding: u32 = 0x0F07E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_15_e400_0f00e400() {
    // Encoding: 0x0F00E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immb=0, Q=0, Rd=0, immh=0, U=0
    let encoding: u32 = 0x0F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_16_e400_0f00e420() {
    // Encoding: 0x0F00E420
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=1, Rd=0
    // Fields: Q=0, immb=0, Rd=0, U=0, immh=0, Rn=1
    let encoding: u32 = 0x0F00E420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_17_e400_0f00e7c0() {
    // Encoding: 0x0F00E7C0
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=30, Rd=0
    // Fields: Rn=30, immh=0, Q=0, Rd=0, U=0, immb=0
    let encoding: u32 = 0x0F00E7C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_18_e400_0f00e7e0() {
    // Encoding: 0x0F00E7E0
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=31, Rd=0
    // Fields: immb=0, Rn=31, immh=0, Rd=0, U=0, Q=0
    let encoding: u32 = 0x0F00E7E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_19_e400_0f00e400() {
    // Encoding: 0x0F00E400
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, U=0, immb=0, Rd=0, Q=0, immh=0
    let encoding: u32 = 0x0F00E400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_20_e400_0f00e401() {
    // Encoding: 0x0F00E401
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, immh=0, U=0, Q=0, immb=0
    let encoding: u32 = 0x0F00E401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_21_e400_0f00e41e() {
    // Encoding: 0x0F00E41E
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=30
    // Fields: U=0, immh=0, immb=0, Rd=30, Rn=0, Q=0
    let encoding: u32 = 0x0F00E41E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_22_e400_0f00e41f() {
    // Encoding: 0x0F00E41F
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=0, Rd=31
    // Fields: Q=0, immh=0, immb=0, U=0, Rn=0, Rd=31
    let encoding: u32 = 0x0F00E41F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_23_e400_0f00e421() {
    // Encoding: 0x0F00E421
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=1, Rd=1
    // Fields: Rd=1, U=0, immh=0, immb=0, Q=0, Rn=1
    let encoding: u32 = 0x0F00E421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_conv_int_simd_combo_24_e400_0f00e7ff() {
    // Encoding: 0x0F00E7FF
    // Test aarch64_vector_shift_conv_int_simd field combination: Q=0, U=0, immh=0, immb=0, Rn=31, Rd=31
    // Fields: immb=0, immh=0, Rd=31, Q=0, U=0, Rn=31
    let encoding: u32 = 0x0F00E7FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_shift_conv_int_simd_special_q_0_size_variant_0_58368_0f09e400() {
    // Encoding: 0x0F09E400
    // Test aarch64_vector_shift_conv_int_simd special value Q = 0 (Size variant 0)
    // Fields: Q=0, U=0, Rn=0, Rd=0, immh=1, immb=1
    let encoding: u32 = 0x0F09E400;
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

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_shift_conv_int_simd_special_q_1_size_variant_1_58368_4f09e400() {
    // Encoding: 0x4F09E400
    // Test aarch64_vector_shift_conv_int_simd special value Q = 1 (Size variant 1)
    // Fields: U=0, Q=1, Rd=0, immh=1, immb=1, Rn=0
    let encoding: u32 = 0x4F09E400;
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

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_conv_int_simd_special_rn_31_stack_pointer_sp_may_require_alignment_58368_0f09e7e0()
 {
    // Encoding: 0x0F09E7E0
    // Test aarch64_vector_shift_conv_int_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: immh=1, Rn=31, immb=1, Q=0, U=0, Rd=0
    let encoding: u32 = 0x0F09E7E0;
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

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_conv_int_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_58368_0f09e41f()
 {
    // Encoding: 0x0F09E41F
    // Test aarch64_vector_shift_conv_int_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, U=0, Q=0, immh=1, immb=1, Rn=0
    let encoding: u32 = 0x0F09E41F;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_reg_write_0_5f00e400() {
    // Test aarch64_vector_shift_conv_int_sisd register write: SimdFromField("d")
    // Encoding: 0x5F00E400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F00E400;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_sp_rn_5f00e7e0() {
    // Test aarch64_vector_shift_conv_int_sisd with Rn = SP (31)
    // Encoding: 0x5F00E7E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F00E7E0;
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

/// Provenance: aarch64_vector_shift_conv_int_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_conv_int_sisd_zr_rd_5f00e41f() {
    // Test aarch64_vector_shift_conv_int_sisd with Rd = ZR (31)
    // Encoding: 0x5F00E41F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x5F00E41F;
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

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_conv_int_simd_reg_write_0_0f00e400() {
    // Test aarch64_vector_shift_conv_int_simd register write: SimdFromField("d")
    // Encoding: 0x0F00E400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00E400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_sp_rn_0f00e7e0() {
    // Test aarch64_vector_shift_conv_int_simd with Rn = SP (31)
    // Encoding: 0x0F00E7E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00E7E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_conv_int_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_conv_int_simd_zr_rd_0f00e41f() {
    // Test aarch64_vector_shift_conv_int_simd with Rd = ZR (31)
    // Encoding: 0x0F00E41F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00E41F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_shift_left_insert_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_immh_0_zero_5400_7f005400() {
    // Encoding: 0x7F005400
    // Test aarch64_vector_shift_left_insert_sisd field immh = 0 (Zero)
    // Fields: immh=0, Rd=0, Rn=0, immb=0
    let encoding: u32 = 0x7F005400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_immh_1_poweroftwo_5400_7f085400() {
    // Encoding: 0x7F085400
    // Test aarch64_vector_shift_left_insert_sisd field immh = 1 (PowerOfTwo)
    // Fields: Rn=0, Rd=0, immh=1, immb=0
    let encoding: u32 = 0x7F085400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_immh_3_poweroftwominusone_5400_7f185400() {
    // Encoding: 0x7F185400
    // Test aarch64_vector_shift_left_insert_sisd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: immh=3, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F185400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_immh_4_poweroftwo_5400_7f205400() {
    // Encoding: 0x7F205400
    // Test aarch64_vector_shift_left_insert_sisd field immh = 4 (PowerOfTwo)
    // Fields: immh=4, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F205400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_immh_7_poweroftwominusone_5400_7f385400() {
    // Encoding: 0x7F385400
    // Test aarch64_vector_shift_left_insert_sisd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: immh=7, Rd=0, immb=0, Rn=0
    let encoding: u32 = 0x7F385400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_immh_8_poweroftwo_5400_7f405400() {
    // Encoding: 0x7F405400
    // Test aarch64_vector_shift_left_insert_sisd field immh = 8 (PowerOfTwo)
    // Fields: immh=8, Rn=0, Rd=0, immb=0
    let encoding: u32 = 0x7F405400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_immh_15_max_5400_7f785400() {
    // Encoding: 0x7F785400
    // Test aarch64_vector_shift_left_insert_sisd field immh = 15 (Max)
    // Fields: immh=15, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F785400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_immb_0_zero_5400_7f005400() {
    // Encoding: 0x7F005400
    // Test aarch64_vector_shift_left_insert_sisd field immb = 0 (Zero)
    // Fields: Rd=0, immh=0, immb=0, Rn=0
    let encoding: u32 = 0x7F005400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_immb_1_poweroftwo_5400_7f015400() {
    // Encoding: 0x7F015400
    // Test aarch64_vector_shift_left_insert_sisd field immb = 1 (PowerOfTwo)
    // Fields: Rn=0, immb=1, immh=0, Rd=0
    let encoding: u32 = 0x7F015400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_immb_3_poweroftwominusone_5400_7f035400() {
    // Encoding: 0x7F035400
    // Test aarch64_vector_shift_left_insert_sisd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: immb=3, immh=0, Rd=0, Rn=0
    let encoding: u32 = 0x7F035400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_immb_7_max_5400_7f075400() {
    // Encoding: 0x7F075400
    // Test aarch64_vector_shift_left_insert_sisd field immb = 7 (Max)
    // Fields: immh=0, Rn=0, immb=7, Rd=0
    let encoding: u32 = 0x7F075400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_rn_0_min_5400_7f005400() {
    // Encoding: 0x7F005400
    // Test aarch64_vector_shift_left_insert_sisd field Rn = 0 (Min)
    // Fields: immh=0, Rn=0, Rd=0, immb=0
    let encoding: u32 = 0x7F005400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_rn_1_poweroftwo_5400_7f005420() {
    // Encoding: 0x7F005420
    // Test aarch64_vector_shift_left_insert_sisd field Rn = 1 (PowerOfTwo)
    // Fields: immh=0, Rn=1, immb=0, Rd=0
    let encoding: u32 = 0x7F005420;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_rn_30_poweroftwominusone_5400_7f0057c0() {
    // Encoding: 0x7F0057C0
    // Test aarch64_vector_shift_left_insert_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, immh=0, Rn=30, immb=0
    let encoding: u32 = 0x7F0057C0;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_rn_31_max_5400_7f0057e0() {
    // Encoding: 0x7F0057E0
    // Test aarch64_vector_shift_left_insert_sisd field Rn = 31 (Max)
    // Fields: immb=0, immh=0, Rd=0, Rn=31
    let encoding: u32 = 0x7F0057E0;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_rd_0_min_5400_7f005400() {
    // Encoding: 0x7F005400
    // Test aarch64_vector_shift_left_insert_sisd field Rd = 0 (Min)
    // Fields: immh=0, Rd=0, immb=0, Rn=0
    let encoding: u32 = 0x7F005400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_rd_1_poweroftwo_5400_7f005401() {
    // Encoding: 0x7F005401
    // Test aarch64_vector_shift_left_insert_sisd field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, immh=0, Rd=1, immb=0
    let encoding: u32 = 0x7F005401;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_rd_30_poweroftwominusone_5400_7f00541e() {
    // Encoding: 0x7F00541E
    // Test aarch64_vector_shift_left_insert_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: immb=0, immh=0, Rn=0, Rd=30
    let encoding: u32 = 0x7F00541E;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_field_rd_31_max_5400_7f00541f() {
    // Encoding: 0x7F00541F
    // Test aarch64_vector_shift_left_insert_sisd field Rd = 31 (Max)
    // Fields: immh=0, Rn=0, Rd=31, immb=0
    let encoding: u32 = 0x7F00541F;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_0_5400_7f005400() {
    // Encoding: 0x7F005400
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, immh=0, Rd=0
    let encoding: u32 = 0x7F005400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_1_5400_7f085400() {
    // Encoding: 0x7F085400
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=1, immb=0, Rn=0, Rd=0
    // Fields: immh=1, Rn=0, Rd=0, immb=0
    let encoding: u32 = 0x7F085400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_2_5400_7f185400() {
    // Encoding: 0x7F185400
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=3, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immh=3, immb=0, Rd=0
    let encoding: u32 = 0x7F185400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_3_5400_7f205400() {
    // Encoding: 0x7F205400
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=4, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, immh=4, Rd=0
    let encoding: u32 = 0x7F205400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_4_5400_7f385400() {
    // Encoding: 0x7F385400
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=7, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rd=0, Rn=0, immh=7
    let encoding: u32 = 0x7F385400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_5_5400_7f405400() {
    // Encoding: 0x7F405400
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=8, immb=0, Rn=0, Rd=0
    // Fields: immh=8, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x7F405400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_6_5400_7f785400() {
    // Encoding: 0x7F785400
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=15, immb=0, Rn=0, Rd=0
    // Fields: immh=15, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x7F785400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_7_5400_7f005400() {
    // Encoding: 0x7F005400
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immb=0, Rd=0, immh=0
    let encoding: u32 = 0x7F005400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_8_5400_7f015400() {
    // Encoding: 0x7F015400
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=1, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, immh=0, immb=1
    let encoding: u32 = 0x7F015400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_9_5400_7f035400() {
    // Encoding: 0x7F035400
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=3, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, immh=0, immb=3
    let encoding: u32 = 0x7F035400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_10_5400_7f075400() {
    // Encoding: 0x7F075400
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=7, Rn=0, Rd=0
    // Fields: immb=7, immh=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F075400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_11_5400_7f005400() {
    // Encoding: 0x7F005400
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, immh=0, immb=0
    let encoding: u32 = 0x7F005400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_12_5400_7f005420() {
    // Encoding: 0x7F005420
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=0, Rn=1, Rd=0
    // Fields: immh=0, immb=0, Rn=1, Rd=0
    let encoding: u32 = 0x7F005420;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_13_5400_7f0057c0() {
    // Encoding: 0x7F0057C0
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=0, Rn=30, Rd=0
    // Fields: Rn=30, immb=0, immh=0, Rd=0
    let encoding: u32 = 0x7F0057C0;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_14_5400_7f0057e0() {
    // Encoding: 0x7F0057E0
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=0, Rn=31, Rd=0
    // Fields: Rn=31, Rd=0, immb=0, immh=0
    let encoding: u32 = 0x7F0057E0;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_15_5400_7f005400() {
    // Encoding: 0x7F005400
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rd=0, immh=0, Rn=0
    let encoding: u32 = 0x7F005400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_16_5400_7f005401() {
    // Encoding: 0x7F005401
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=1
    // Fields: Rn=0, immh=0, immb=0, Rd=1
    let encoding: u32 = 0x7F005401;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_17_5400_7f00541e() {
    // Encoding: 0x7F00541E
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=30
    // Fields: Rd=30, immh=0, immb=0, Rn=0
    let encoding: u32 = 0x7F00541E;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_18_5400_7f00541f() {
    // Encoding: 0x7F00541F
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=31
    // Fields: immb=0, immh=0, Rd=31, Rn=0
    let encoding: u32 = 0x7F00541F;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_19_5400_7f005421() {
    // Encoding: 0x7F005421
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=0, Rn=1, Rd=1
    // Fields: immb=0, immh=0, Rn=1, Rd=1
    let encoding: u32 = 0x7F005421;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_combo_20_5400_7f0057ff() {
    // Encoding: 0x7F0057FF
    // Test aarch64_vector_shift_left_insert_sisd field combination: immh=0, immb=0, Rn=31, Rd=31
    // Fields: immb=0, Rn=31, immh=0, Rd=31
    let encoding: u32 = 0x7F0057FF;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_21504_7f0957e0()
 {
    // Encoding: 0x7F0957E0
    // Test aarch64_vector_shift_left_insert_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, immh=1, immb=1, Rd=0
    let encoding: u32 = 0x7F0957E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_21504_7f09541f()
 {
    // Encoding: 0x7F09541F
    // Test aarch64_vector_shift_left_insert_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, immh=1, immb=1, Rd=31
    let encoding: u32 = 0x7F09541F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_q_0_min_5400_2f005400() {
    // Encoding: 0x2F005400
    // Test aarch64_vector_shift_left_insert_simd field Q = 0 (Min)
    // Fields: Rn=0, Q=0, Rd=0, immb=0, immh=0
    let encoding: u32 = 0x2F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_q_1_max_5400_6f005400() {
    // Encoding: 0x6F005400
    // Test aarch64_vector_shift_left_insert_simd field Q = 1 (Max)
    // Fields: Q=1, immh=0, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x6F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_immh_0_zero_5400_2f005400() {
    // Encoding: 0x2F005400
    // Test aarch64_vector_shift_left_insert_simd field immh = 0 (Zero)
    // Fields: Rd=0, immh=0, Rn=0, immb=0, Q=0
    let encoding: u32 = 0x2F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_immh_1_poweroftwo_5400_2f085400() {
    // Encoding: 0x2F085400
    // Test aarch64_vector_shift_left_insert_simd field immh = 1 (PowerOfTwo)
    // Fields: immh=1, immb=0, Rd=0, Rn=0, Q=0
    let encoding: u32 = 0x2F085400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_immh_3_poweroftwominusone_5400_2f185400() {
    // Encoding: 0x2F185400
    // Test aarch64_vector_shift_left_insert_simd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: immh=3, Rn=0, Q=0, Rd=0, immb=0
    let encoding: u32 = 0x2F185400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_immh_4_poweroftwo_5400_2f205400() {
    // Encoding: 0x2F205400
    // Test aarch64_vector_shift_left_insert_simd field immh = 4 (PowerOfTwo)
    // Fields: Rd=0, Q=0, immb=0, Rn=0, immh=4
    let encoding: u32 = 0x2F205400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_immh_7_poweroftwominusone_5400_2f385400() {
    // Encoding: 0x2F385400
    // Test aarch64_vector_shift_left_insert_simd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rn=0, immh=7, immb=0, Rd=0
    let encoding: u32 = 0x2F385400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_immh_8_poweroftwo_5400_2f405400() {
    // Encoding: 0x2F405400
    // Test aarch64_vector_shift_left_insert_simd field immh = 8 (PowerOfTwo)
    // Fields: Q=0, Rn=0, Rd=0, immb=0, immh=8
    let encoding: u32 = 0x2F405400;
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

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_immh_15_max_5400_2f785400() {
    // Encoding: 0x2F785400
    // Test aarch64_vector_shift_left_insert_simd field immh = 15 (Max)
    // Fields: Rn=0, immh=15, Q=0, immb=0, Rd=0
    let encoding: u32 = 0x2F785400;
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

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_immb_0_zero_5400_2f005400() {
    // Encoding: 0x2F005400
    // Test aarch64_vector_shift_left_insert_simd field immb = 0 (Zero)
    // Fields: Rn=0, Q=0, immh=0, Rd=0, immb=0
    let encoding: u32 = 0x2F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_immb_1_poweroftwo_5400_2f015400() {
    // Encoding: 0x2F015400
    // Test aarch64_vector_shift_left_insert_simd field immb = 1 (PowerOfTwo)
    // Fields: Rn=0, immh=0, Q=0, Rd=0, immb=1
    let encoding: u32 = 0x2F015400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_immb_3_poweroftwominusone_5400_2f035400() {
    // Encoding: 0x2F035400
    // Test aarch64_vector_shift_left_insert_simd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: Q=0, immh=0, Rd=0, immb=3, Rn=0
    let encoding: u32 = 0x2F035400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_immb_7_max_5400_2f075400() {
    // Encoding: 0x2F075400
    // Test aarch64_vector_shift_left_insert_simd field immb = 7 (Max)
    // Fields: immb=7, Rn=0, immh=0, Q=0, Rd=0
    let encoding: u32 = 0x2F075400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_rn_0_min_5400_2f005400() {
    // Encoding: 0x2F005400
    // Test aarch64_vector_shift_left_insert_simd field Rn = 0 (Min)
    // Fields: Rn=0, Rd=0, immb=0, Q=0, immh=0
    let encoding: u32 = 0x2F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_rn_1_poweroftwo_5400_2f005420() {
    // Encoding: 0x2F005420
    // Test aarch64_vector_shift_left_insert_simd field Rn = 1 (PowerOfTwo)
    // Fields: immb=0, Q=0, Rd=0, Rn=1, immh=0
    let encoding: u32 = 0x2F005420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_rn_30_poweroftwominusone_5400_2f0057c0() {
    // Encoding: 0x2F0057C0
    // Test aarch64_vector_shift_left_insert_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rd=0, immb=0, Q=0, immh=0, Rn=30
    let encoding: u32 = 0x2F0057C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_rn_31_max_5400_2f0057e0() {
    // Encoding: 0x2F0057E0
    // Test aarch64_vector_shift_left_insert_simd field Rn = 31 (Max)
    // Fields: immh=0, immb=0, Rn=31, Q=0, Rd=0
    let encoding: u32 = 0x2F0057E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_rd_0_min_5400_2f005400() {
    // Encoding: 0x2F005400
    // Test aarch64_vector_shift_left_insert_simd field Rd = 0 (Min)
    // Fields: Q=0, immb=0, Rd=0, immh=0, Rn=0
    let encoding: u32 = 0x2F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_rd_1_poweroftwo_5400_2f005401() {
    // Encoding: 0x2F005401
    // Test aarch64_vector_shift_left_insert_simd field Rd = 1 (PowerOfTwo)
    // Fields: immh=0, Rn=0, Rd=1, Q=0, immb=0
    let encoding: u32 = 0x2F005401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_rd_30_poweroftwominusone_5400_2f00541e() {
    // Encoding: 0x2F00541E
    // Test aarch64_vector_shift_left_insert_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: immb=0, immh=0, Q=0, Rd=30, Rn=0
    let encoding: u32 = 0x2F00541E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_field_rd_31_max_5400_2f00541f() {
    // Encoding: 0x2F00541F
    // Test aarch64_vector_shift_left_insert_simd field Rd = 31 (Max)
    // Fields: immh=0, Rd=31, Q=0, immb=0, Rn=0
    let encoding: u32 = 0x2F00541F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_0_5400_2f005400() {
    // Encoding: 0x2F005400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Q=0, immb=0, Rd=0, immh=0, Rn=0
    let encoding: u32 = 0x2F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_1_5400_6f005400() {
    // Encoding: 0x6F005400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=1, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, Rd=0, Q=1, immh=0
    let encoding: u32 = 0x6F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_2_5400_2f005400() {
    // Encoding: 0x2F005400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Q=0, immh=0, Rn=0, Rd=0
    let encoding: u32 = 0x2F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_3_5400_2f085400() {
    // Encoding: 0x2F085400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=1, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, immh=1, immb=0, Q=0, Rn=0
    let encoding: u32 = 0x2F085400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_4_5400_2f185400() {
    // Encoding: 0x2F185400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=3, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immb=0, Rd=0, immh=3, Q=0
    let encoding: u32 = 0x2F185400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_5_5400_2f205400() {
    // Encoding: 0x2F205400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=4, immb=0, Rn=0, Rd=0
    // Fields: immb=0, immh=4, Q=0, Rn=0, Rd=0
    let encoding: u32 = 0x2F205400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_6_5400_2f385400() {
    // Encoding: 0x2F385400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=7, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, immb=0, Rn=0, Q=0, immh=7
    let encoding: u32 = 0x2F385400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_7_5400_2f405400() {
    // Encoding: 0x2F405400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=8, immb=0, Rn=0, Rd=0
    // Fields: Q=0, Rd=0, immb=0, immh=8, Rn=0
    let encoding: u32 = 0x2F405400;
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

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_8_5400_2f785400() {
    // Encoding: 0x2F785400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=15, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, Rn=0, immb=0, immh=15
    let encoding: u32 = 0x2F785400;
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

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_9_5400_2f005400() {
    // Encoding: 0x2F005400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immb=0, immh=0, Rn=0, Rd=0, Q=0
    let encoding: u32 = 0x2F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_10_5400_2f015400() {
    // Encoding: 0x2F015400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=1, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, immb=1, Rn=0, immh=0
    let encoding: u32 = 0x2F015400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_11_5400_2f035400() {
    // Encoding: 0x2F035400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=3, Rn=0, Rd=0
    // Fields: immh=0, Q=0, immb=3, Rn=0, Rd=0
    let encoding: u32 = 0x2F035400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_12_5400_2f075400() {
    // Encoding: 0x2F075400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=7, Rn=0, Rd=0
    // Fields: immh=0, Rd=0, Rn=0, Q=0, immb=7
    let encoding: u32 = 0x2F075400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_13_5400_2f005400() {
    // Encoding: 0x2F005400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, immh=0, Rn=0, immb=0
    let encoding: u32 = 0x2F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_14_5400_2f005420() {
    // Encoding: 0x2F005420
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=0, Rn=1, Rd=0
    // Fields: immb=0, Rn=1, immh=0, Q=0, Rd=0
    let encoding: u32 = 0x2F005420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_15_5400_2f0057c0() {
    // Encoding: 0x2F0057C0
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=0, Rn=30, Rd=0
    // Fields: Rd=0, Q=0, immb=0, immh=0, Rn=30
    let encoding: u32 = 0x2F0057C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_16_5400_2f0057e0() {
    // Encoding: 0x2F0057E0
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=0, Rn=31, Rd=0
    // Fields: Rd=0, Q=0, immh=0, Rn=31, immb=0
    let encoding: u32 = 0x2F0057E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_17_5400_2f005400() {
    // Encoding: 0x2F005400
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, Q=0, Rn=0, immh=0, immb=0
    let encoding: u32 = 0x2F005400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_18_5400_2f005401() {
    // Encoding: 0x2F005401
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=1
    // Fields: immh=0, immb=0, Rd=1, Rn=0, Q=0
    let encoding: u32 = 0x2F005401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_19_5400_2f00541e() {
    // Encoding: 0x2F00541E
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=30
    // Fields: Rd=30, Q=0, immb=0, immh=0, Rn=0
    let encoding: u32 = 0x2F00541E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_20_5400_2f00541f() {
    // Encoding: 0x2F00541F
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=31
    // Fields: Q=0, immh=0, immb=0, Rd=31, Rn=0
    let encoding: u32 = 0x2F00541F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_21_5400_2f005421() {
    // Encoding: 0x2F005421
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=0, Rn=1, Rd=1
    // Fields: immh=0, Rn=1, Rd=1, immb=0, Q=0
    let encoding: u32 = 0x2F005421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_left_insert_simd_combo_22_5400_2f0057ff() {
    // Encoding: 0x2F0057FF
    // Test aarch64_vector_shift_left_insert_simd field combination: Q=0, immh=0, immb=0, Rn=31, Rd=31
    // Fields: Rn=31, immh=0, immb=0, Rd=31, Q=0
    let encoding: u32 = 0x2F0057FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_shift_left_insert_simd_special_q_0_size_variant_0_21504_2f095400() {
    // Encoding: 0x2F095400
    // Test aarch64_vector_shift_left_insert_simd special value Q = 0 (Size variant 0)
    // Fields: Q=0, Rn=0, immb=1, Rd=0, immh=1
    let encoding: u32 = 0x2F095400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_shift_left_insert_simd_special_q_1_size_variant_1_21504_6f095400() {
    // Encoding: 0x6F095400
    // Test aarch64_vector_shift_left_insert_simd special value Q = 1 (Size variant 1)
    // Fields: Rd=0, Q=1, immb=1, immh=1, Rn=0
    let encoding: u32 = 0x6F095400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_left_insert_simd_special_rn_31_stack_pointer_sp_may_require_alignment_21504_2f0957e0()
 {
    // Encoding: 0x2F0957E0
    // Test aarch64_vector_shift_left_insert_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Rd=0, immh=1, Q=0, immb=1
    let encoding: u32 = 0x2F0957E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_left_insert_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_21504_2f09541f()
 {
    // Encoding: 0x2F09541F
    // Test aarch64_vector_shift_left_insert_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rd=31, Rn=0, Q=0, immh=1, immb=1
    let encoding: u32 = 0x2F09541F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_reg_write_0_7f005400() {
    // Test aarch64_vector_shift_left_insert_sisd register write: SimdFromField("d")
    // Encoding: 0x7F005400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7F005400;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_sp_rn_7f0057e0() {
    // Test aarch64_vector_shift_left_insert_sisd with Rn = SP (31)
    // Encoding: 0x7F0057E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7F0057E0;
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

/// Provenance: aarch64_vector_shift_left_insert_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_left_insert_sisd_zr_rd_7f00541f() {
    // Test aarch64_vector_shift_left_insert_sisd with Rd = ZR (31)
    // Encoding: 0x7F00541F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7F00541F;
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

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_left_insert_simd_reg_write_0_2f005400() {
    // Test aarch64_vector_shift_left_insert_simd register write: SimdFromField("d")
    // Encoding: 0x2F005400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2F005400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_sp_rn_2f0057e0() {
    // Test aarch64_vector_shift_left_insert_simd with Rn = SP (31)
    // Encoding: 0x2F0057E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2F0057E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_left_insert_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_left_insert_simd_zr_rd_2f00541f() {
    // Test aarch64_vector_shift_left_insert_simd with Rd = ZR (31)
    // Encoding: 0x2F00541F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2F00541F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_shift_right_insert_sisd Tests
// ============================================================================

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_immh_0_zero_4400_7f004400() {
    // Encoding: 0x7F004400
    // Test aarch64_vector_shift_right_insert_sisd field immh = 0 (Zero)
    // Fields: Rn=0, immh=0, immb=0, Rd=0
    let encoding: u32 = 0x7F004400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_immh_1_poweroftwo_4400_7f084400() {
    // Encoding: 0x7F084400
    // Test aarch64_vector_shift_right_insert_sisd field immh = 1 (PowerOfTwo)
    // Fields: immh=1, Rn=0, immb=0, Rd=0
    let encoding: u32 = 0x7F084400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_immh_3_poweroftwominusone_4400_7f184400() {
    // Encoding: 0x7F184400
    // Test aarch64_vector_shift_right_insert_sisd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: Rn=0, immh=3, Rd=0, immb=0
    let encoding: u32 = 0x7F184400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_immh_4_poweroftwo_4400_7f204400() {
    // Encoding: 0x7F204400
    // Test aarch64_vector_shift_right_insert_sisd field immh = 4 (PowerOfTwo)
    // Fields: Rd=0, immb=0, Rn=0, immh=4
    let encoding: u32 = 0x7F204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_immh_7_poweroftwominusone_4400_7f384400() {
    // Encoding: 0x7F384400
    // Test aarch64_vector_shift_right_insert_sisd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: immh=7, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x7F384400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_immh_8_poweroftwo_4400_7f404400() {
    // Encoding: 0x7F404400
    // Test aarch64_vector_shift_right_insert_sisd field immh = 8 (PowerOfTwo)
    // Fields: immb=0, Rd=0, immh=8, Rn=0
    let encoding: u32 = 0x7F404400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_immh_15_max_4400_7f784400() {
    // Encoding: 0x7F784400
    // Test aarch64_vector_shift_right_insert_sisd field immh = 15 (Max)
    // Fields: immh=15, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F784400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_immb_0_zero_4400_7f004400() {
    // Encoding: 0x7F004400
    // Test aarch64_vector_shift_right_insert_sisd field immb = 0 (Zero)
    // Fields: immh=0, Rn=0, Rd=0, immb=0
    let encoding: u32 = 0x7F004400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_immb_1_poweroftwo_4400_7f014400() {
    // Encoding: 0x7F014400
    // Test aarch64_vector_shift_right_insert_sisd field immb = 1 (PowerOfTwo)
    // Fields: immh=0, immb=1, Rn=0, Rd=0
    let encoding: u32 = 0x7F014400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_immb_3_poweroftwominusone_4400_7f034400() {
    // Encoding: 0x7F034400
    // Test aarch64_vector_shift_right_insert_sisd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: immh=0, Rd=0, immb=3, Rn=0
    let encoding: u32 = 0x7F034400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_immb_7_max_4400_7f074400() {
    // Encoding: 0x7F074400
    // Test aarch64_vector_shift_right_insert_sisd field immb = 7 (Max)
    // Fields: immh=0, Rn=0, immb=7, Rd=0
    let encoding: u32 = 0x7F074400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_rn_0_min_4400_7f004400() {
    // Encoding: 0x7F004400
    // Test aarch64_vector_shift_right_insert_sisd field Rn = 0 (Min)
    // Fields: immh=0, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x7F004400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_rn_1_poweroftwo_4400_7f004420() {
    // Encoding: 0x7F004420
    // Test aarch64_vector_shift_right_insert_sisd field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, immb=0, immh=0, Rn=1
    let encoding: u32 = 0x7F004420;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_rn_30_poweroftwominusone_4400_7f0047c0() {
    // Encoding: 0x7F0047C0
    // Test aarch64_vector_shift_right_insert_sisd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: immb=0, Rd=0, Rn=30, immh=0
    let encoding: u32 = 0x7F0047C0;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_rn_31_max_4400_7f0047e0() {
    // Encoding: 0x7F0047E0
    // Test aarch64_vector_shift_right_insert_sisd field Rn = 31 (Max)
    // Fields: immb=0, Rd=0, Rn=31, immh=0
    let encoding: u32 = 0x7F0047E0;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_rd_0_min_4400_7f004400() {
    // Encoding: 0x7F004400
    // Test aarch64_vector_shift_right_insert_sisd field Rd = 0 (Min)
    // Fields: immh=0, Rd=0, Rn=0, immb=0
    let encoding: u32 = 0x7F004400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_rd_1_poweroftwo_4400_7f004401() {
    // Encoding: 0x7F004401
    // Test aarch64_vector_shift_right_insert_sisd field Rd = 1 (PowerOfTwo)
    // Fields: immh=0, Rd=1, Rn=0, immb=0
    let encoding: u32 = 0x7F004401;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_rd_30_poweroftwominusone_4400_7f00441e() {
    // Encoding: 0x7F00441E
    // Test aarch64_vector_shift_right_insert_sisd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, immb=0, Rd=30, immh=0
    let encoding: u32 = 0x7F00441E;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_field_rd_31_max_4400_7f00441f() {
    // Encoding: 0x7F00441F
    // Test aarch64_vector_shift_right_insert_sisd field Rd = 31 (Max)
    // Fields: Rn=0, Rd=31, immb=0, immh=0
    let encoding: u32 = 0x7F00441F;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_0_4400_7f004400() {
    // Encoding: 0x7F004400
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=0
    // Fields: immh=0, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F004400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_1_4400_7f084400() {
    // Encoding: 0x7F084400
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=1, immb=0, Rn=0, Rd=0
    // Fields: immh=1, immb=0, Rn=0, Rd=0
    let encoding: u32 = 0x7F084400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_2_4400_7f184400() {
    // Encoding: 0x7F184400
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=3, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, immb=0, Rn=0, immh=3
    let encoding: u32 = 0x7F184400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_3_4400_7f204400() {
    // Encoding: 0x7F204400
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=4, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, immb=0, immh=4
    let encoding: u32 = 0x7F204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_4_4400_7f384400() {
    // Encoding: 0x7F384400
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=7, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, immh=7, immb=0
    let encoding: u32 = 0x7F384400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_5_4400_7f404400() {
    // Encoding: 0x7F404400
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=8, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immh=8, immb=0, Rd=0
    let encoding: u32 = 0x7F404400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_6_4400_7f784400() {
    // Encoding: 0x7F784400
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=15, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, immb=0, immh=15, Rd=0
    let encoding: u32 = 0x7F784400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_7_4400_7f004400() {
    // Encoding: 0x7F004400
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, immh=0, Rd=0
    let encoding: u32 = 0x7F004400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_8_4400_7f014400() {
    // Encoding: 0x7F014400
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=1, Rn=0, Rd=0
    // Fields: immh=0, Rd=0, Rn=0, immb=1
    let encoding: u32 = 0x7F014400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_9_4400_7f034400() {
    // Encoding: 0x7F034400
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=3, Rn=0, Rd=0
    // Fields: immh=0, Rn=0, immb=3, Rd=0
    let encoding: u32 = 0x7F034400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_10_4400_7f074400() {
    // Encoding: 0x7F074400
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=7, Rn=0, Rd=0
    // Fields: immh=0, immb=7, Rn=0, Rd=0
    let encoding: u32 = 0x7F074400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_11_4400_7f004400() {
    // Encoding: 0x7F004400
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, immh=0, Rd=0
    let encoding: u32 = 0x7F004400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_12_4400_7f004420() {
    // Encoding: 0x7F004420
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=0, Rn=1, Rd=0
    // Fields: Rn=1, immh=0, Rd=0, immb=0
    let encoding: u32 = 0x7F004420;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_13_4400_7f0047c0() {
    // Encoding: 0x7F0047C0
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=0, Rn=30, Rd=0
    // Fields: immb=0, Rn=30, Rd=0, immh=0
    let encoding: u32 = 0x7F0047C0;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_14_4400_7f0047e0() {
    // Encoding: 0x7F0047E0
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=0, Rn=31, Rd=0
    // Fields: immh=0, Rd=0, Rn=31, immb=0
    let encoding: u32 = 0x7F0047E0;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_15_4400_7f004400() {
    // Encoding: 0x7F004400
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, immb=0, immh=0, Rn=0
    let encoding: u32 = 0x7F004400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_16_4400_7f004401() {
    // Encoding: 0x7F004401
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=1
    // Fields: Rn=0, immh=0, Rd=1, immb=0
    let encoding: u32 = 0x7F004401;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_17_4400_7f00441e() {
    // Encoding: 0x7F00441E
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=30
    // Fields: immb=0, Rn=0, immh=0, Rd=30
    let encoding: u32 = 0x7F00441E;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_18_4400_7f00441f() {
    // Encoding: 0x7F00441F
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=0, Rn=0, Rd=31
    // Fields: Rn=0, immb=0, immh=0, Rd=31
    let encoding: u32 = 0x7F00441F;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_19_4400_7f004421() {
    // Encoding: 0x7F004421
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=0, Rn=1, Rd=1
    // Fields: immh=0, immb=0, Rd=1, Rn=1
    let encoding: u32 = 0x7F004421;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_combo_20_4400_7f0047ff() {
    // Encoding: 0x7F0047FF
    // Test aarch64_vector_shift_right_insert_sisd field combination: immh=0, immb=0, Rn=31, Rd=31
    // Fields: Rd=31, Rn=31, immh=0, immb=0
    let encoding: u32 = 0x7F0047FF;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_special_rn_31_stack_pointer_sp_may_require_alignment_17408_7f0947e0()
 {
    // Encoding: 0x7F0947E0
    // Test aarch64_vector_shift_right_insert_sisd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, immh=1, Rd=0, immb=1
    let encoding: u32 = 0x7F0947E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_17408_7f09441f()
 {
    // Encoding: 0x7F09441F
    // Test aarch64_vector_shift_right_insert_sisd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: immb=1, immh=1, Rn=0, Rd=31
    let encoding: u32 = 0x7F09441F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_q_0_min_4400_2f004400() {
    // Encoding: 0x2F004400
    // Test aarch64_vector_shift_right_insert_simd field Q = 0 (Min)
    // Fields: Rn=0, Rd=0, immh=0, Q=0, immb=0
    let encoding: u32 = 0x2F004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_q_1_max_4400_6f004400() {
    // Encoding: 0x6F004400
    // Test aarch64_vector_shift_right_insert_simd field Q = 1 (Max)
    // Fields: Rn=0, immh=0, immb=0, Rd=0, Q=1
    let encoding: u32 = 0x6F004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_immh_0_zero_4400_2f004400() {
    // Encoding: 0x2F004400
    // Test aarch64_vector_shift_right_insert_simd field immh = 0 (Zero)
    // Fields: immh=0, Q=0, Rd=0, immb=0, Rn=0
    let encoding: u32 = 0x2F004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_immh_1_poweroftwo_4400_2f084400() {
    // Encoding: 0x2F084400
    // Test aarch64_vector_shift_right_insert_simd field immh = 1 (PowerOfTwo)
    // Fields: Rd=0, immb=0, Q=0, immh=1, Rn=0
    let encoding: u32 = 0x2F084400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_immh_3_poweroftwominusone_4400_2f184400() {
    // Encoding: 0x2F184400
    // Test aarch64_vector_shift_right_insert_simd field immh = 3 (PowerOfTwoMinusOne)
    // Fields: Rd=0, immh=3, Q=0, immb=0, Rn=0
    let encoding: u32 = 0x2F184400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_immh_4_poweroftwo_4400_2f204400() {
    // Encoding: 0x2F204400
    // Test aarch64_vector_shift_right_insert_simd field immh = 4 (PowerOfTwo)
    // Fields: Rd=0, immh=4, immb=0, Rn=0, Q=0
    let encoding: u32 = 0x2F204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_immh_7_poweroftwominusone_4400_2f384400() {
    // Encoding: 0x2F384400
    // Test aarch64_vector_shift_right_insert_simd field immh = 7 (PowerOfTwoMinusOne)
    // Fields: immh=7, Rn=0, Q=0, Rd=0, immb=0
    let encoding: u32 = 0x2F384400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_immh_8_poweroftwo_4400_2f404400() {
    // Encoding: 0x2F404400
    // Test aarch64_vector_shift_right_insert_simd field immh = 8 (PowerOfTwo)
    // Fields: immb=0, Q=0, Rd=0, Rn=0, immh=8
    let encoding: u32 = 0x2F404400;
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

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field immh 19 +: 4`
/// Requirement: FieldBoundary { field: "immh", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_immh_15_max_4400_2f784400() {
    // Encoding: 0x2F784400
    // Test aarch64_vector_shift_right_insert_simd field immh = 15 (Max)
    // Fields: Rd=0, Rn=0, immb=0, Q=0, immh=15
    let encoding: u32 = 0x2F784400;
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

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_immb_0_zero_4400_2f004400() {
    // Encoding: 0x2F004400
    // Test aarch64_vector_shift_right_insert_simd field immb = 0 (Zero)
    // Fields: immb=0, Rn=0, immh=0, Rd=0, Q=0
    let encoding: u32 = 0x2F004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_immb_1_poweroftwo_4400_2f014400() {
    // Encoding: 0x2F014400
    // Test aarch64_vector_shift_right_insert_simd field immb = 1 (PowerOfTwo)
    // Fields: immb=1, Q=0, immh=0, Rn=0, Rd=0
    let encoding: u32 = 0x2F014400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 3, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (3)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_immb_3_poweroftwominusone_4400_2f034400() {
    // Encoding: 0x2F034400
    // Test aarch64_vector_shift_right_insert_simd field immb = 3 (PowerOfTwoMinusOne)
    // Fields: immh=0, Q=0, Rn=0, Rd=0, immb=3
    let encoding: u32 = 0x2F034400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field immb 16 +: 3`
/// Requirement: FieldBoundary { field: "immb", value: 7, boundary: Max }
/// maximum immediate (7)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_immb_7_max_4400_2f074400() {
    // Encoding: 0x2F074400
    // Test aarch64_vector_shift_right_insert_simd field immb = 7 (Max)
    // Fields: Rn=0, immh=0, Q=0, Rd=0, immb=7
    let encoding: u32 = 0x2F074400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_rn_0_min_4400_2f004400() {
    // Encoding: 0x2F004400
    // Test aarch64_vector_shift_right_insert_simd field Rn = 0 (Min)
    // Fields: Rd=0, immb=0, immh=0, Rn=0, Q=0
    let encoding: u32 = 0x2F004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_rn_1_poweroftwo_4400_2f004420() {
    // Encoding: 0x2F004420
    // Test aarch64_vector_shift_right_insert_simd field Rn = 1 (PowerOfTwo)
    // Fields: immb=0, Rn=1, Q=0, immh=0, Rd=0
    let encoding: u32 = 0x2F004420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_rn_30_poweroftwominusone_4400_2f0047c0() {
    // Encoding: 0x2F0047C0
    // Test aarch64_vector_shift_right_insert_simd field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Q=0, Rd=0, immb=0, immh=0
    let encoding: u32 = 0x2F0047C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_rn_31_max_4400_2f0047e0() {
    // Encoding: 0x2F0047E0
    // Test aarch64_vector_shift_right_insert_simd field Rn = 31 (Max)
    // Fields: Rd=0, immh=0, immb=0, Q=0, Rn=31
    let encoding: u32 = 0x2F0047E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_rd_0_min_4400_2f004400() {
    // Encoding: 0x2F004400
    // Test aarch64_vector_shift_right_insert_simd field Rd = 0 (Min)
    // Fields: Q=0, immh=0, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x2F004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_rd_1_poweroftwo_4400_2f004401() {
    // Encoding: 0x2F004401
    // Test aarch64_vector_shift_right_insert_simd field Rd = 1 (PowerOfTwo)
    // Fields: Q=0, immb=0, Rd=1, Rn=0, immh=0
    let encoding: u32 = 0x2F004401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_rd_30_poweroftwominusone_4400_2f00441e() {
    // Encoding: 0x2F00441E
    // Test aarch64_vector_shift_right_insert_simd field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, immb=0, Rd=30, Rn=0, immh=0
    let encoding: u32 = 0x2F00441E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_field_rd_31_max_4400_2f00441f() {
    // Encoding: 0x2F00441F
    // Test aarch64_vector_shift_right_insert_simd field Rd = 31 (Max)
    // Fields: Rn=0, Rd=31, Q=0, immh=0, immb=0
    let encoding: u32 = 0x2F00441F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_0_4400_2f004400() {
    // Encoding: 0x2F004400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Q=0, immh=0, Rd=0, immb=0, Rn=0
    let encoding: u32 = 0x2F004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_1_4400_6f004400() {
    // Encoding: 0x6F004400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=1, immh=0, immb=0, Rn=0, Rd=0
    // Fields: immh=0, Q=1, immb=0, Rd=0, Rn=0
    let encoding: u32 = 0x6F004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_2_4400_2f004400() {
    // Encoding: 0x2F004400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Rd=0, immb=0, immh=0, Q=0, Rn=0
    let encoding: u32 = 0x2F004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_3_4400_2f084400() {
    // Encoding: 0x2F084400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=1, immb=0, Rn=0, Rd=0
    // Fields: immh=1, Rn=0, Q=0, Rd=0, immb=0
    let encoding: u32 = 0x2F084400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_4_4400_2f184400() {
    // Encoding: 0x2F184400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=3, immb=0, Rn=0, Rd=0
    // Fields: Rn=0, Q=0, Rd=0, immb=0, immh=3
    let encoding: u32 = 0x2F184400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_5_4400_2f204400() {
    // Encoding: 0x2F204400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=4, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Rn=0, Rd=0, Q=0, immh=4
    let encoding: u32 = 0x2F204400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=7 (immediate midpoint (7))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_6_4400_2f384400() {
    // Encoding: 0x2F384400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=7, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Q=0, immh=7, Rd=0, Rn=0
    let encoding: u32 = 0x2F384400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_7_4400_2f404400() {
    // Encoding: 0x2F404400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=8, immb=0, Rn=0, Rd=0
    // Fields: immb=0, Q=0, immh=8, Rn=0, Rd=0
    let encoding: u32 = 0x2F404400;
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

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immh=15 (maximum immediate (15))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_8_4400_2f784400() {
    // Encoding: 0x2F784400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=15, immb=0, Rn=0, Rd=0
    // Fields: immh=15, immb=0, Rn=0, Q=0, Rd=0
    let encoding: u32 = 0x2F784400;
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

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=0 (immediate value 0)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_9_4400_2f004400() {
    // Encoding: 0x2F004400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, immh=0, immb=0, Rd=0
    let encoding: u32 = 0x2F004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=1 (immediate value 1)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_10_4400_2f014400() {
    // Encoding: 0x2F014400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=1, Rn=0, Rd=0
    // Fields: immh=0, Rd=0, Q=0, Rn=0, immb=1
    let encoding: u32 = 0x2F014400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=3 (immediate midpoint (3))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_11_4400_2f034400() {
    // Encoding: 0x2F034400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=3, Rn=0, Rd=0
    // Fields: Rn=0, immb=3, immh=0, Q=0, Rd=0
    let encoding: u32 = 0x2F034400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// immb=7 (maximum immediate (7))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_12_4400_2f074400() {
    // Encoding: 0x2F074400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=7, Rn=0, Rd=0
    // Fields: Q=0, immb=7, immh=0, Rd=0, Rn=0
    let encoding: u32 = 0x2F074400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_13_4400_2f004400() {
    // Encoding: 0x2F004400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Q=0, immb=0, immh=0, Rn=0, Rd=0
    let encoding: u32 = 0x2F004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_14_4400_2f004420() {
    // Encoding: 0x2F004420
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=0, Rn=1, Rd=0
    // Fields: Rd=0, immb=0, Rn=1, Q=0, immh=0
    let encoding: u32 = 0x2F004420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_15_4400_2f0047c0() {
    // Encoding: 0x2F0047C0
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=0, Rn=30, Rd=0
    // Fields: Rn=30, immh=0, Q=0, Rd=0, immb=0
    let encoding: u32 = 0x2F0047C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_16_4400_2f0047e0() {
    // Encoding: 0x2F0047E0
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=0, Rn=31, Rd=0
    // Fields: immh=0, immb=0, Q=0, Rd=0, Rn=31
    let encoding: u32 = 0x2F0047E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_17_4400_2f004400() {
    // Encoding: 0x2F004400
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=0
    // Fields: Q=0, Rn=0, Rd=0, immh=0, immb=0
    let encoding: u32 = 0x2F004400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_18_4400_2f004401() {
    // Encoding: 0x2F004401
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=1
    // Fields: immb=0, Rd=1, Q=0, immh=0, Rn=0
    let encoding: u32 = 0x2F004401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_19_4400_2f00441e() {
    // Encoding: 0x2F00441E
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=30
    // Fields: Rn=0, Rd=30, immh=0, immb=0, Q=0
    let encoding: u32 = 0x2F00441E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_20_4400_2f00441f() {
    // Encoding: 0x2F00441F
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=0, Rn=0, Rd=31
    // Fields: immb=0, Q=0, immh=0, Rn=0, Rd=31
    let encoding: u32 = 0x2F00441F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_21_4400_2f004421() {
    // Encoding: 0x2F004421
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=0, Rn=1, Rd=1
    // Fields: Rd=1, Q=0, immb=0, immh=0, Rn=1
    let encoding: u32 = 0x2F004421;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_vector_shift_right_insert_simd_combo_22_4400_2f0047ff() {
    // Encoding: 0x2F0047FF
    // Test aarch64_vector_shift_right_insert_simd field combination: Q=0, immh=0, immb=0, Rn=31, Rd=31
    // Fields: Q=0, Rn=31, Rd=31, immh=0, immb=0
    let encoding: u32 = 0x2F0047FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_shift_right_insert_simd_special_q_0_size_variant_0_17408_2f094400() {
    // Encoding: 0x2F094400
    // Test aarch64_vector_shift_right_insert_simd special value Q = 0 (Size variant 0)
    // Fields: immb=1, Rn=0, immh=1, Q=0, Rd=0
    let encoding: u32 = 0x2F094400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_shift_right_insert_simd_special_q_1_size_variant_1_17408_6f094400() {
    // Encoding: 0x6F094400
    // Test aarch64_vector_shift_right_insert_simd special value Q = 1 (Size variant 1)
    // Fields: immb=1, immh=1, Q=1, Rn=0, Rd=0
    let encoding: u32 = 0x6F094400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_vector_shift_right_insert_simd_special_rn_31_stack_pointer_sp_may_require_alignment_17408_2f0947e0()
 {
    // Encoding: 0x2F0947E0
    // Test aarch64_vector_shift_right_insert_simd special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: immb=1, Q=0, Rn=31, immh=1, Rd=0
    let encoding: u32 = 0x2F0947E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_shift_right_insert_simd_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_17408_2f09441f()
 {
    // Encoding: 0x2F09441F
    // Test aarch64_vector_shift_right_insert_simd special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Q=0, immh=1, Rd=31, immb=1
    let encoding: u32 = 0x2F09441F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_reg_write_0_7f004400() {
    // Test aarch64_vector_shift_right_insert_sisd register write: SimdFromField("d")
    // Encoding: 0x7F004400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7F004400;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_sp_rn_7f0047e0() {
    // Test aarch64_vector_shift_right_insert_sisd with Rn = SP (31)
    // Encoding: 0x7F0047E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7F0047E0;
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

/// Provenance: aarch64_vector_shift_right_insert_sisd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_right_insert_sisd_zr_rd_7f00441f() {
    // Test aarch64_vector_shift_right_insert_sisd with Rd = ZR (31)
    // Encoding: 0x7F00441F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x7F00441F;
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

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `SimdFromField("d") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("d")
#[test]
fn test_aarch64_vector_shift_right_insert_simd_reg_write_0_2f004400() {
    // Test aarch64_vector_shift_right_insert_simd register write: SimdFromField("d")
    // Encoding: 0x2F004400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2F004400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_sp_rn_2f0047e0() {
    // Test aarch64_vector_shift_right_insert_simd with Rn = SP (31)
    // Encoding: 0x2F0047E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2F0047E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_shift_right_insert_simd
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_shift_right_insert_simd_zr_rd_2f00441f() {
    // Test aarch64_vector_shift_right_insert_simd with Rd = ZR (31)
    // Encoding: 0x2F00441F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2F00441F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}
