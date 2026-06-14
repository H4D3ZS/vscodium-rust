//! A64 float convert tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_float_convert_int Tests
// ============================================================================

/// Provenance: aarch64_float_convert_int
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_float_convert_int_field_sf_0_min_0_1e200000() {
    // Encoding: 0x1E200000
    // Test aarch64_float_convert_int field sf = 0 (Min)
    // Fields: opcode=0, type1=0, sf=0, Rn=0, Rd=0, rmode=0
    let encoding: u32 = 0x1E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_float_convert_int_field_sf_1_max_0_9e200000() {
    // Encoding: 0x9E200000
    // Test aarch64_float_convert_int field sf = 1 (Max)
    // Fields: Rd=0, type1=0, rmode=0, opcode=0, sf=1, Rn=0
    let encoding: u32 = 0x9E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	x0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_convert_int_field_type1_0_min_0_1e200000() {
    // Encoding: 0x1E200000
    // Test aarch64_float_convert_int field type1 = 0 (Min)
    // Fields: Rn=0, type1=0, rmode=0, sf=0, opcode=0, Rd=0
    let encoding: u32 = 0x1E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_float_convert_int_field_type1_1_poweroftwo_0_1e600000() {
    // Encoding: 0x1E600000
    // Test aarch64_float_convert_int field type1 = 1 (PowerOfTwo)
    // Fields: Rd=0, type1=1, sf=0, opcode=0, rmode=0, Rn=0
    let encoding: u32 = 0x1E600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, d0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch64_float_convert_int_field_type1_3_max_0_1ee00000() {
    // Encoding: 0x1EE00000
    // Test aarch64_float_convert_int field type1 = 3 (Max)
    // Fields: opcode=0, type1=3, Rn=0, Rd=0, rmode=0, sf=0
    let encoding: u32 = 0x1EE00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, h0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field rmode 19 +: 2`
/// Requirement: FieldBoundary { field: "rmode", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_convert_int_field_rmode_0_min_0_1e200000() {
    // Encoding: 0x1E200000
    // Test aarch64_float_convert_int field rmode = 0 (Min)
    // Fields: sf=0, opcode=0, Rd=0, Rn=0, type1=0, rmode=0
    let encoding: u32 = 0x1E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field rmode 19 +: 2`
/// Requirement: FieldBoundary { field: "rmode", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_float_convert_int_field_rmode_1_poweroftwo_0_1e280000() {
    // Encoding: 0x1E280000
    // Test aarch64_float_convert_int field rmode = 1 (PowerOfTwo)
    // Fields: rmode=1, type1=0, sf=0, opcode=0, Rn=0, Rd=0
    let encoding: u32 = 0x1E280000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtps	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field rmode 19 +: 2`
/// Requirement: FieldBoundary { field: "rmode", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch64_float_convert_int_field_rmode_3_max_0_1e380000() {
    // Encoding: 0x1E380000
    // Test aarch64_float_convert_int field rmode = 3 (Max)
    // Fields: opcode=0, sf=0, rmode=3, Rn=0, type1=0, Rd=0
    let encoding: u32 = 0x1E380000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtzs	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field opcode 16 +: 3`
/// Requirement: FieldBoundary { field: "opcode", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_convert_int_field_opcode_0_min_0_1e200000() {
    // Encoding: 0x1E200000
    // Test aarch64_float_convert_int field opcode = 0 (Min)
    // Fields: Rn=0, Rd=0, opcode=0, sf=0, type1=0, rmode=0
    let encoding: u32 = 0x1E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field opcode 16 +: 3`
/// Requirement: FieldBoundary { field: "opcode", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_float_convert_int_field_opcode_1_poweroftwo_0_1e210000() {
    // Encoding: 0x1E210000
    // Test aarch64_float_convert_int field opcode = 1 (PowerOfTwo)
    // Fields: rmode=0, Rd=0, type1=0, sf=0, Rn=0, opcode=1
    let encoding: u32 = 0x1E210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtnu	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field opcode 16 +: 3`
/// Requirement: FieldBoundary { field: "opcode", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_float_convert_int_field_opcode_7_max_0_1e270000() {
    // Encoding: 0x1E270000
    // Test aarch64_float_convert_int field opcode = 7 (Max)
    // Fields: type1=0, opcode=7, rmode=0, sf=0, Rn=0, Rd=0
    let encoding: u32 = 0x1E270000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_convert_int_field_rn_0_min_0_1e200000() {
    // Encoding: 0x1E200000
    // Test aarch64_float_convert_int field Rn = 0 (Min)
    // Fields: rmode=0, type1=0, sf=0, Rn=0, opcode=0, Rd=0
    let encoding: u32 = 0x1E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_convert_int_field_rn_1_poweroftwo_0_1e200020() {
    // Encoding: 0x1E200020
    // Test aarch64_float_convert_int field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, Rn=1, sf=0, rmode=0, opcode=0, type1=0
    let encoding: u32 = 0x1E200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_convert_int_field_rn_30_poweroftwominusone_0_1e2003c0() {
    // Encoding: 0x1E2003C0
    // Test aarch64_float_convert_int field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: type1=0, rmode=0, Rn=30, sf=0, opcode=0, Rd=0
    let encoding: u32 = 0x1E2003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_float_convert_int_field_rn_31_max_0_1e2003e0() {
    // Encoding: 0x1E2003E0
    // Test aarch64_float_convert_int field Rn = 31 (Max)
    // Fields: Rn=31, Rd=0, rmode=0, sf=0, opcode=0, type1=0
    let encoding: u32 = 0x1E2003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s31
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_convert_int_field_rd_0_min_0_1e200000() {
    // Encoding: 0x1E200000
    // Test aarch64_float_convert_int field Rd = 0 (Min)
    // Fields: rmode=0, Rd=0, sf=0, opcode=0, Rn=0, type1=0
    let encoding: u32 = 0x1E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_convert_int_field_rd_1_poweroftwo_0_1e200001() {
    // Encoding: 0x1E200001
    // Test aarch64_float_convert_int field Rd = 1 (PowerOfTwo)
    // Fields: type1=0, rmode=0, sf=0, Rn=0, Rd=1, opcode=0
    let encoding: u32 = 0x1E200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w1, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_convert_int_field_rd_30_poweroftwominusone_0_1e20001e() {
    // Encoding: 0x1E20001E
    // Test aarch64_float_convert_int field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: rmode=0, type1=0, Rn=0, Rd=30, sf=0, opcode=0
    let encoding: u32 = 0x1E20001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w30, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_float_convert_int_field_rd_31_max_0_1e20001f() {
    // Encoding: 0x1E20001F
    // Test aarch64_float_convert_int field Rd = 31 (Max)
    // Fields: sf=0, rmode=0, opcode=0, type1=0, Rd=31, Rn=0
    let encoding: u32 = 0x1E20001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	wzr, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_float_convert_int_combo_0_0_1e200000() {
    // Encoding: 0x1E200000
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=0, Rd=0
    // Fields: Rn=0, Rd=0, sf=0, type1=0, opcode=0, rmode=0
    let encoding: u32 = 0x1E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_float_convert_int_combo_1_0_9e200000() {
    // Encoding: 0x9E200000
    // Test aarch64_float_convert_int field combination: sf=1, type1=0, rmode=0, opcode=0, Rn=0, Rd=0
    // Fields: sf=1, opcode=0, type1=0, rmode=0, Rn=0, Rd=0
    let encoding: u32 = 0x9E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	x0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=0 (minimum value)
#[test]
fn test_aarch64_float_convert_int_combo_2_0_1e200000() {
    // Encoding: 0x1E200000
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=0, Rd=0
    // Fields: Rd=0, Rn=0, rmode=0, opcode=0, sf=0, type1=0
    let encoding: u32 = 0x1E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=1 (value 1)
#[test]
fn test_aarch64_float_convert_int_combo_3_0_1e600000() {
    // Encoding: 0x1E600000
    // Test aarch64_float_convert_int field combination: sf=0, type1=1, rmode=0, opcode=0, Rn=0, Rd=0
    // Fields: Rd=0, type1=1, opcode=0, Rn=0, sf=0, rmode=0
    let encoding: u32 = 0x1E600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, d0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=3 (maximum value (3))
#[test]
fn test_aarch64_float_convert_int_combo_4_0_1ee00000() {
    // Encoding: 0x1EE00000
    // Test aarch64_float_convert_int field combination: sf=0, type1=3, rmode=0, opcode=0, Rn=0, Rd=0
    // Fields: Rd=0, sf=0, type1=3, Rn=0, rmode=0, opcode=0
    let encoding: u32 = 0x1EE00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, h0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// rmode=0 (minimum value)
#[test]
fn test_aarch64_float_convert_int_combo_5_0_1e200000() {
    // Encoding: 0x1E200000
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=0, Rd=0
    // Fields: Rd=0, rmode=0, opcode=0, sf=0, type1=0, Rn=0
    let encoding: u32 = 0x1E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// rmode=1 (value 1)
#[test]
fn test_aarch64_float_convert_int_combo_6_0_1e280000() {
    // Encoding: 0x1E280000
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=1, opcode=0, Rn=0, Rd=0
    // Fields: opcode=0, Rd=0, Rn=0, sf=0, type1=0, rmode=1
    let encoding: u32 = 0x1E280000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtps	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// rmode=3 (maximum value (3))
#[test]
fn test_aarch64_float_convert_int_combo_7_0_1e380000() {
    // Encoding: 0x1E380000
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=3, opcode=0, Rn=0, Rd=0
    // Fields: sf=0, rmode=3, Rn=0, Rd=0, type1=0, opcode=0
    let encoding: u32 = 0x1E380000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtzs	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=0 (minimum value)
#[test]
fn test_aarch64_float_convert_int_combo_8_0_1e200000() {
    // Encoding: 0x1E200000
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=0, Rd=0
    // Fields: opcode=0, Rn=0, type1=0, Rd=0, sf=0, rmode=0
    let encoding: u32 = 0x1E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=1 (value 1)
#[test]
fn test_aarch64_float_convert_int_combo_9_0_1e210000() {
    // Encoding: 0x1E210000
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=1, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, type1=0, opcode=1, Rd=0, rmode=0
    let encoding: u32 = 0x1E210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtnu	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=7 (maximum value (7))
#[test]
fn test_aarch64_float_convert_int_combo_10_0_1e270000() {
    // Encoding: 0x1E270000
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=7, Rn=0, Rd=0
    // Fields: Rn=0, rmode=0, type1=0, sf=0, opcode=7, Rd=0
    let encoding: u32 = 0x1E270000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fmov	s0, w0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_convert_int_combo_11_0_1e200000() {
    // Encoding: 0x1E200000
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=0, Rd=0
    // Fields: sf=0, type1=0, Rn=0, rmode=0, Rd=0, opcode=0
    let encoding: u32 = 0x1E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_convert_int_combo_12_0_1e200020() {
    // Encoding: 0x1E200020
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=1, Rd=0
    // Fields: sf=0, rmode=0, opcode=0, Rn=1, type1=0, Rd=0
    let encoding: u32 = 0x1E200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_convert_int_combo_13_0_1e2003c0() {
    // Encoding: 0x1E2003C0
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=30, Rd=0
    // Fields: sf=0, Rn=30, opcode=0, Rd=0, type1=0, rmode=0
    let encoding: u32 = 0x1E2003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_float_convert_int_combo_14_0_1e2003e0() {
    // Encoding: 0x1E2003E0
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=31, Rd=0
    // Fields: type1=0, opcode=0, Rn=31, rmode=0, sf=0, Rd=0
    let encoding: u32 = 0x1E2003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s31
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_convert_int_combo_15_0_1e200000() {
    // Encoding: 0x1E200000
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=0, Rd=0
    // Fields: opcode=0, Rd=0, sf=0, rmode=0, type1=0, Rn=0
    let encoding: u32 = 0x1E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_convert_int_combo_16_0_1e200001() {
    // Encoding: 0x1E200001
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=0, Rd=1
    // Fields: opcode=0, rmode=0, type1=0, sf=0, Rn=0, Rd=1
    let encoding: u32 = 0x1E200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w1, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_convert_int_combo_17_0_1e20001e() {
    // Encoding: 0x1E20001E
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=0, Rd=30
    // Fields: opcode=0, Rn=0, Rd=30, type1=0, sf=0, rmode=0
    let encoding: u32 = 0x1E20001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w30, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_float_convert_int_combo_18_0_1e20001f() {
    // Encoding: 0x1E20001F
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=0, Rd=31
    // Fields: sf=0, rmode=0, type1=0, Rd=31, Rn=0, opcode=0
    let encoding: u32 = 0x1E20001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	wzr, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_float_convert_int_combo_19_0_1e200021() {
    // Encoding: 0x1E200021
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=1, Rd=1
    // Fields: sf=0, opcode=0, Rd=1, type1=0, rmode=0, Rn=1
    let encoding: u32 = 0x1E200021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w1, s1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_float_convert_int_combo_20_0_1e2003ff() {
    // Encoding: 0x1E2003FF
    // Test aarch64_float_convert_int field combination: sf=0, type1=0, rmode=0, opcode=0, Rn=31, Rd=31
    // Fields: Rn=31, type1=0, rmode=0, Rd=31, sf=0, opcode=0
    let encoding: u32 = 0x1E2003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	wzr, s31
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_float_convert_int_special_sf_0_size_variant_0_0_1e200000() {
    // Encoding: 0x1E200000
    // Test aarch64_float_convert_int special value sf = 0 (Size variant 0)
    // Fields: opcode=0, Rn=0, Rd=0, type1=0, sf=0, rmode=0
    let encoding: u32 = 0x1E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_float_convert_int_special_sf_1_size_variant_1_0_9e200000() {
    // Encoding: 0x9E200000
    // Test aarch64_float_convert_int special value sf = 1 (Size variant 1)
    // Fields: rmode=0, opcode=0, Rd=0, sf=1, type1=0, Rn=0
    let encoding: u32 = 0x9E200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	x0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_float_convert_int_special_rn_31_stack_pointer_sp_may_require_alignment_0_1e2003e0()
{
    // Encoding: 0x1E2003E0
    // Test aarch64_float_convert_int special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: rmode=0, type1=0, sf=0, opcode=0, Rn=31, Rd=0
    let encoding: u32 = 0x1E2003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	w0, s31
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_int
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_float_convert_int_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_1e20001f()
 {
    // Encoding: 0x1E20001F
    // Test aarch64_float_convert_int special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: type1=0, Rd=31, sf=0, rmode=0, opcode=0, Rn=0
    let encoding: u32 = 0x1E20001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvtns	wzr, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// aarch64_float_convert_fix Tests
// ============================================================================

/// Provenance: aarch64_float_convert_fix
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_float_convert_fix_field_sf_0_min_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field sf = 0 (Min)
    // Fields: Rn=0, Rd=0, type1=0, rmode=0, opcode=0, scale=0, sf=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field sf 31 +: 1`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_float_convert_fix_field_sf_1_max_0_9e000000() {
    // Encoding: 0x9E000000
    // Test aarch64_float_convert_fix field sf = 1 (Max)
    // Fields: Rn=0, type1=0, rmode=0, sf=1, scale=0, Rd=0, opcode=0
    let encoding: u32 = 0x9E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_convert_fix_field_type1_0_min_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field type1 = 0 (Min)
    // Fields: Rn=0, rmode=0, type1=0, sf=0, scale=0, opcode=0, Rd=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_float_convert_fix_field_type1_1_poweroftwo_0_1e400000() {
    // Encoding: 0x1E400000
    // Test aarch64_float_convert_fix field type1 = 1 (PowerOfTwo)
    // Fields: opcode=0, scale=0, type1=1, rmode=0, sf=0, Rn=0, Rd=0
    let encoding: u32 = 0x1E400000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch64_float_convert_fix_field_type1_3_max_0_1ec00000() {
    // Encoding: 0x1EC00000
    // Test aarch64_float_convert_fix field type1 = 3 (Max)
    // Fields: sf=0, type1=3, opcode=0, scale=0, Rd=0, rmode=0, Rn=0
    let encoding: u32 = 0x1EC00000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field rmode 19 +: 2`
/// Requirement: FieldBoundary { field: "rmode", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_convert_fix_field_rmode_0_min_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field rmode = 0 (Min)
    // Fields: opcode=0, type1=0, rmode=0, scale=0, sf=0, Rd=0, Rn=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field rmode 19 +: 2`
/// Requirement: FieldBoundary { field: "rmode", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_float_convert_fix_field_rmode_1_poweroftwo_0_1e080000() {
    // Encoding: 0x1E080000
    // Test aarch64_float_convert_fix field rmode = 1 (PowerOfTwo)
    // Fields: rmode=1, Rd=0, sf=0, opcode=0, scale=0, Rn=0, type1=0
    let encoding: u32 = 0x1E080000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field rmode 19 +: 2`
/// Requirement: FieldBoundary { field: "rmode", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch64_float_convert_fix_field_rmode_3_max_0_1e180000() {
    // Encoding: 0x1E180000
    // Test aarch64_float_convert_fix field rmode = 3 (Max)
    // Fields: Rd=0, Rn=0, type1=0, sf=0, opcode=0, scale=0, rmode=3
    let encoding: u32 = 0x1E180000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field opcode 16 +: 3`
/// Requirement: FieldBoundary { field: "opcode", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_convert_fix_field_opcode_0_min_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field opcode = 0 (Min)
    // Fields: type1=0, scale=0, Rd=0, rmode=0, opcode=0, Rn=0, sf=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field opcode 16 +: 3`
/// Requirement: FieldBoundary { field: "opcode", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_float_convert_fix_field_opcode_1_poweroftwo_0_1e010000() {
    // Encoding: 0x1E010000
    // Test aarch64_float_convert_fix field opcode = 1 (PowerOfTwo)
    // Fields: Rd=0, scale=0, type1=0, sf=0, opcode=1, Rn=0, rmode=0
    let encoding: u32 = 0x1E010000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field opcode 16 +: 3`
/// Requirement: FieldBoundary { field: "opcode", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_float_convert_fix_field_opcode_7_max_0_1e070000() {
    // Encoding: 0x1E070000
    // Test aarch64_float_convert_fix field opcode = 7 (Max)
    // Fields: Rd=0, type1=0, Rn=0, sf=0, rmode=0, opcode=7, scale=0
    let encoding: u32 = 0x1E070000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fix
/// ASL: `field scale 10 +: 6`
/// Requirement: FieldBoundary { field: "scale", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_convert_fix_field_scale_0_min_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field scale = 0 (Min)
    // Fields: sf=0, rmode=0, opcode=0, type1=0, Rn=0, Rd=0, scale=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field scale 10 +: 6`
/// Requirement: FieldBoundary { field: "scale", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_float_convert_fix_field_scale_1_poweroftwo_0_1e000400() {
    // Encoding: 0x1E000400
    // Test aarch64_float_convert_fix field scale = 1 (PowerOfTwo)
    // Fields: sf=0, Rn=0, rmode=0, Rd=0, type1=0, opcode=0, scale=1
    let encoding: u32 = 0x1E000400;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field scale 10 +: 6`
/// Requirement: FieldBoundary { field: "scale", value: 31, boundary: PowerOfTwoMinusOne }
/// midpoint (31)
#[test]
fn test_aarch64_float_convert_fix_field_scale_31_poweroftwominusone_0_1e007c00() {
    // Encoding: 0x1E007C00
    // Test aarch64_float_convert_fix field scale = 31 (PowerOfTwoMinusOne)
    // Fields: type1=0, opcode=0, rmode=0, scale=31, sf=0, Rn=0, Rd=0
    let encoding: u32 = 0x1E007C00;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field scale 10 +: 6`
/// Requirement: FieldBoundary { field: "scale", value: 63, boundary: Max }
/// maximum value (63)
#[test]
fn test_aarch64_float_convert_fix_field_scale_63_max_0_1e00fc00() {
    // Encoding: 0x1E00FC00
    // Test aarch64_float_convert_fix field scale = 63 (Max)
    // Fields: type1=0, sf=0, scale=63, Rn=0, Rd=0, rmode=0, opcode=0
    let encoding: u32 = 0x1E00FC00;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_convert_fix_field_rn_0_min_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field Rn = 0 (Min)
    // Fields: sf=0, type1=0, scale=0, rmode=0, Rn=0, opcode=0, Rd=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_convert_fix_field_rn_1_poweroftwo_0_1e000020() {
    // Encoding: 0x1E000020
    // Test aarch64_float_convert_fix field Rn = 1 (PowerOfTwo)
    // Fields: scale=0, type1=0, opcode=0, Rn=1, sf=0, rmode=0, Rd=0
    let encoding: u32 = 0x1E000020;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_convert_fix_field_rn_30_poweroftwominusone_0_1e0003c0() {
    // Encoding: 0x1E0003C0
    // Test aarch64_float_convert_fix field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: type1=0, Rn=30, scale=0, Rd=0, opcode=0, rmode=0, sf=0
    let encoding: u32 = 0x1E0003C0;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_float_convert_fix_field_rn_31_max_0_1e0003e0() {
    // Encoding: 0x1E0003E0
    // Test aarch64_float_convert_fix field Rn = 31 (Max)
    // Fields: sf=0, Rd=0, Rn=31, scale=0, type1=0, opcode=0, rmode=0
    let encoding: u32 = 0x1E0003E0;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_convert_fix_field_rd_0_min_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field Rd = 0 (Min)
    // Fields: Rn=0, type1=0, rmode=0, opcode=0, sf=0, scale=0, Rd=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_convert_fix_field_rd_1_poweroftwo_0_1e000001() {
    // Encoding: 0x1E000001
    // Test aarch64_float_convert_fix field Rd = 1 (PowerOfTwo)
    // Fields: Rd=1, rmode=0, type1=0, scale=0, sf=0, Rn=0, opcode=0
    let encoding: u32 = 0x1E000001;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_convert_fix_field_rd_30_poweroftwominusone_0_1e00001e() {
    // Encoding: 0x1E00001E
    // Test aarch64_float_convert_fix field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: scale=0, Rd=30, type1=0, rmode=0, opcode=0, sf=0, Rn=0
    let encoding: u32 = 0x1E00001E;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_float_convert_fix_field_rd_31_max_0_1e00001f() {
    // Encoding: 0x1E00001F
    // Test aarch64_float_convert_fix field Rd = 31 (Max)
    // Fields: type1=0, sf=0, rmode=0, scale=0, Rn=0, opcode=0, Rd=31
    let encoding: u32 = 0x1E00001F;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=0 (8-bit / byte size)
#[test]
fn test_aarch64_float_convert_fix_combo_0_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=0, Rd=0
    // Fields: opcode=0, scale=0, sf=0, Rd=0, Rn=0, type1=0, rmode=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sf=1 (16-bit / halfword size)
#[test]
fn test_aarch64_float_convert_fix_combo_1_0_9e000000() {
    // Encoding: 0x9E000000
    // Test aarch64_float_convert_fix field combination: sf=1, type1=0, rmode=0, opcode=0, scale=0, Rn=0, Rd=0
    // Fields: Rn=0, scale=0, type1=0, rmode=0, Rd=0, opcode=0, sf=1
    let encoding: u32 = 0x9E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=0 (minimum value)
#[test]
fn test_aarch64_float_convert_fix_combo_2_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=0, Rd=0
    // Fields: scale=0, rmode=0, sf=0, type1=0, Rn=0, opcode=0, Rd=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=1 (value 1)
#[test]
fn test_aarch64_float_convert_fix_combo_3_0_1e400000() {
    // Encoding: 0x1E400000
    // Test aarch64_float_convert_fix field combination: sf=0, type1=1, rmode=0, opcode=0, scale=0, Rn=0, Rd=0
    // Fields: sf=0, Rn=0, type1=1, opcode=0, scale=0, rmode=0, Rd=0
    let encoding: u32 = 0x1E400000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=3 (maximum value (3))
#[test]
fn test_aarch64_float_convert_fix_combo_4_0_1ec00000() {
    // Encoding: 0x1EC00000
    // Test aarch64_float_convert_fix field combination: sf=0, type1=3, rmode=0, opcode=0, scale=0, Rn=0, Rd=0
    // Fields: scale=0, Rd=0, rmode=0, type1=3, sf=0, Rn=0, opcode=0
    let encoding: u32 = 0x1EC00000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// rmode=0 (minimum value)
#[test]
fn test_aarch64_float_convert_fix_combo_5_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=0, Rd=0
    // Fields: rmode=0, type1=0, opcode=0, sf=0, scale=0, Rn=0, Rd=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// rmode=1 (value 1)
#[test]
fn test_aarch64_float_convert_fix_combo_6_0_1e080000() {
    // Encoding: 0x1E080000
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=1, opcode=0, scale=0, Rn=0, Rd=0
    // Fields: Rd=0, type1=0, scale=0, sf=0, Rn=0, rmode=1, opcode=0
    let encoding: u32 = 0x1E080000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// rmode=3 (maximum value (3))
#[test]
fn test_aarch64_float_convert_fix_combo_7_0_1e180000() {
    // Encoding: 0x1E180000
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=3, opcode=0, scale=0, Rn=0, Rd=0
    // Fields: sf=0, opcode=0, Rn=0, Rd=0, type1=0, rmode=3, scale=0
    let encoding: u32 = 0x1E180000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=0 (minimum value)
#[test]
fn test_aarch64_float_convert_fix_combo_8_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=0, Rd=0
    // Fields: scale=0, Rn=0, Rd=0, type1=0, sf=0, rmode=0, opcode=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=1 (value 1)
#[test]
fn test_aarch64_float_convert_fix_combo_9_0_1e010000() {
    // Encoding: 0x1E010000
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=1, scale=0, Rn=0, Rd=0
    // Fields: rmode=0, scale=0, Rn=0, Rd=0, opcode=1, sf=0, type1=0
    let encoding: u32 = 0x1E010000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=7 (maximum value (7))
#[test]
fn test_aarch64_float_convert_fix_combo_10_0_1e070000() {
    // Encoding: 0x1E070000
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=7, scale=0, Rn=0, Rd=0
    // Fields: opcode=7, sf=0, rmode=0, scale=0, Rd=0, type1=0, Rn=0
    let encoding: u32 = 0x1E070000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// scale=0 (minimum value)
#[test]
fn test_aarch64_float_convert_fix_combo_11_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=0, Rd=0
    // Fields: scale=0, type1=0, sf=0, opcode=0, Rn=0, Rd=0, rmode=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// scale=1 (value 1)
#[test]
fn test_aarch64_float_convert_fix_combo_12_0_1e000400() {
    // Encoding: 0x1E000400
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=1, Rn=0, Rd=0
    // Fields: Rd=0, opcode=0, rmode=0, sf=0, type1=0, scale=1, Rn=0
    let encoding: u32 = 0x1E000400;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// scale=31 (midpoint (31))
#[test]
fn test_aarch64_float_convert_fix_combo_13_0_1e007c00() {
    // Encoding: 0x1E007C00
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=31, Rn=0, Rd=0
    // Fields: sf=0, Rd=0, opcode=0, Rn=0, scale=31, type1=0, rmode=0
    let encoding: u32 = 0x1E007C00;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// scale=63 (maximum value (63))
#[test]
fn test_aarch64_float_convert_fix_combo_14_0_1e00fc00() {
    // Encoding: 0x1E00FC00
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=63, Rn=0, Rd=0
    // Fields: scale=63, rmode=0, sf=0, type1=0, opcode=0, Rd=0, Rn=0
    let encoding: u32 = 0x1E00FC00;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_convert_fix_combo_15_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=0, Rd=0
    // Fields: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=0, Rd=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_convert_fix_combo_16_0_1e000020() {
    // Encoding: 0x1E000020
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=1, Rd=0
    // Fields: type1=0, rmode=0, scale=0, Rd=0, opcode=0, sf=0, Rn=1
    let encoding: u32 = 0x1E000020;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_convert_fix_combo_17_0_1e0003c0() {
    // Encoding: 0x1E0003C0
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=30, Rd=0
    // Fields: rmode=0, scale=0, type1=0, sf=0, opcode=0, Rn=30, Rd=0
    let encoding: u32 = 0x1E0003C0;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_float_convert_fix_combo_18_0_1e0003e0() {
    // Encoding: 0x1E0003E0
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=31, Rd=0
    // Fields: Rd=0, type1=0, sf=0, opcode=0, scale=0, rmode=0, Rn=31
    let encoding: u32 = 0x1E0003E0;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_convert_fix_combo_19_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=0, Rd=0
    // Fields: opcode=0, scale=0, Rn=0, Rd=0, sf=0, rmode=0, type1=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_convert_fix_combo_20_0_1e000001() {
    // Encoding: 0x1E000001
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=0, Rd=1
    // Fields: Rn=0, Rd=1, opcode=0, type1=0, scale=0, sf=0, rmode=0
    let encoding: u32 = 0x1E000001;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_convert_fix_combo_21_0_1e00001e() {
    // Encoding: 0x1E00001E
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=0, Rd=30
    // Fields: sf=0, Rd=30, opcode=0, scale=0, rmode=0, type1=0, Rn=0
    let encoding: u32 = 0x1E00001E;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_float_convert_fix_combo_22_0_1e00001f() {
    // Encoding: 0x1E00001F
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=0, Rd=31
    // Fields: Rd=31, type1=0, Rn=0, rmode=0, opcode=0, scale=0, sf=0
    let encoding: u32 = 0x1E00001F;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_float_convert_fix_combo_23_0_1e000021() {
    // Encoding: 0x1E000021
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=1, Rd=1
    // Fields: Rn=1, Rd=1, opcode=0, type1=0, sf=0, rmode=0, scale=0
    let encoding: u32 = 0x1E000021;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_float_convert_fix_combo_24_0_1e0003ff() {
    // Encoding: 0x1E0003FF
    // Test aarch64_float_convert_fix field combination: sf=0, type1=0, rmode=0, opcode=0, scale=0, Rn=31, Rd=31
    // Fields: opcode=0, scale=0, Rd=31, sf=0, Rn=31, rmode=0, type1=0
    let encoding: u32 = 0x1E0003FF;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_float_convert_fix_special_sf_0_size_variant_0_0_1e000000() {
    // Encoding: 0x1E000000
    // Test aarch64_float_convert_fix special value sf = 0 (Size variant 0)
    // Fields: rmode=0, scale=0, type1=0, Rn=0, sf=0, opcode=0, Rd=0
    let encoding: u32 = 0x1E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_float_convert_fix_special_sf_1_size_variant_1_0_9e000000() {
    // Encoding: 0x9E000000
    // Test aarch64_float_convert_fix special value sf = 1 (Size variant 1)
    // Fields: type1=0, sf=1, opcode=0, scale=0, Rd=0, rmode=0, Rn=0
    let encoding: u32 = 0x9E000000;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_float_convert_fix_special_rn_31_stack_pointer_sp_may_require_alignment_0_1e0003e0()
{
    // Encoding: 0x1E0003E0
    // Test aarch64_float_convert_fix special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: rmode=0, type1=0, opcode=0, sf=0, Rn=31, Rd=0, scale=0
    let encoding: u32 = 0x1E0003E0;
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

/// Provenance: aarch64_float_convert_fix
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_float_convert_fix_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_1e00001f()
 {
    // Encoding: 0x1E00001F
    // Test aarch64_float_convert_fix special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: rmode=0, type1=0, scale=0, sf=0, opcode=0, Rn=0, Rd=31
    let encoding: u32 = 0x1E00001F;
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

// ============================================================================
// aarch64_float_convert_fp Tests
// ============================================================================

/// Provenance: aarch64_float_convert_fp
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_float_convert_fp_field_type1_0_min_4000_1e224000() {
    // Encoding: 0x1E224000
    // Test aarch64_float_convert_fp field type1 = 0 (Min)
    // Fields: Rd=0, Rn=0, opc=0, type1=0
    let encoding: u32 = 0x1E224000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_float_convert_fp_field_type1_1_poweroftwo_4000_1e624000() {
    // Encoding: 0x1E624000
    // Test aarch64_float_convert_fp field type1 = 1 (PowerOfTwo)
    // Fields: opc=0, type1=1, Rn=0, Rd=0
    let encoding: u32 = 0x1E624000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvt	s0, d0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field type1 22 +: 2`
/// Requirement: FieldBoundary { field: "type1", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch64_float_convert_fp_field_type1_3_max_4000_1ee24000() {
    // Encoding: 0x1EE24000
    // Test aarch64_float_convert_fp field type1 = 3 (Max)
    // Fields: type1=3, Rn=0, Rd=0, opc=0
    let encoding: u32 = 0x1EE24000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvt	s0, h0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field opc 15 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_float_convert_fp_field_opc_0_min_4000_1e224000() {
    // Encoding: 0x1E224000
    // Test aarch64_float_convert_fp field opc = 0 (Min)
    // Fields: type1=0, Rd=0, opc=0, Rn=0
    let encoding: u32 = 0x1E224000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field opc 15 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_float_convert_fp_field_opc_1_poweroftwo_4000_1e22c000() {
    // Encoding: 0x1E22C000
    // Test aarch64_float_convert_fp field opc = 1 (PowerOfTwo)
    // Fields: opc=1, Rd=0, Rn=0, type1=0
    let encoding: u32 = 0x1E22C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvt	d0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field opc 15 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_float_convert_fp_field_opc_2_poweroftwo_4000_1e234000() {
    // Encoding: 0x1E234000
    // Test aarch64_float_convert_fp field opc = 2 (PowerOfTwo)
    // Fields: opc=2, Rd=0, type1=0, Rn=0
    let encoding: u32 = 0x1E234000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field opc 15 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_float_convert_fp_field_opc_3_max_4000_1e23c000() {
    // Encoding: 0x1E23C000
    // Test aarch64_float_convert_fp field opc = 3 (Max)
    // Fields: Rn=0, Rd=0, type1=0, opc=3
    let encoding: u32 = 0x1E23C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvt	h0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_convert_fp_field_rn_0_min_4000_1e224000() {
    // Encoding: 0x1E224000
    // Test aarch64_float_convert_fp field Rn = 0 (Min)
    // Fields: Rd=0, type1=0, opc=0, Rn=0
    let encoding: u32 = 0x1E224000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_convert_fp_field_rn_1_poweroftwo_4000_1e224020() {
    // Encoding: 0x1E224020
    // Test aarch64_float_convert_fp field Rn = 1 (PowerOfTwo)
    // Fields: Rd=0, type1=0, opc=0, Rn=1
    let encoding: u32 = 0x1E224020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_convert_fp_field_rn_30_poweroftwominusone_4000_1e2243c0() {
    // Encoding: 0x1E2243C0
    // Test aarch64_float_convert_fp field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: type1=0, Rd=0, opc=0, Rn=30
    let encoding: u32 = 0x1E2243C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_float_convert_fp_field_rn_31_max_4000_1e2243e0() {
    // Encoding: 0x1E2243E0
    // Test aarch64_float_convert_fp field Rn = 31 (Max)
    // Fields: type1=0, Rd=0, Rn=31, opc=0
    let encoding: u32 = 0x1E2243E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_float_convert_fp_field_rd_0_min_4000_1e224000() {
    // Encoding: 0x1E224000
    // Test aarch64_float_convert_fp field Rd = 0 (Min)
    // Fields: opc=0, Rn=0, type1=0, Rd=0
    let encoding: u32 = 0x1E224000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_float_convert_fp_field_rd_1_poweroftwo_4000_1e224001() {
    // Encoding: 0x1E224001
    // Test aarch64_float_convert_fp field Rd = 1 (PowerOfTwo)
    // Fields: Rn=0, opc=0, Rd=1, type1=0
    let encoding: u32 = 0x1E224001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_float_convert_fp_field_rd_30_poweroftwominusone_4000_1e22401e() {
    // Encoding: 0x1E22401E
    // Test aarch64_float_convert_fp field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rd=30, opc=0, type1=0
    let encoding: u32 = 0x1E22401E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_float_convert_fp_field_rd_31_max_4000_1e22401f() {
    // Encoding: 0x1E22401F
    // Test aarch64_float_convert_fp field Rd = 31 (Max)
    // Fields: Rn=0, Rd=31, opc=0, type1=0
    let encoding: u32 = 0x1E22401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=0 (minimum value)
#[test]
fn test_aarch64_float_convert_fp_combo_0_4000_1e224000() {
    // Encoding: 0x1E224000
    // Test aarch64_float_convert_fp field combination: type1=0, opc=0, Rn=0, Rd=0
    // Fields: Rn=0, type1=0, Rd=0, opc=0
    let encoding: u32 = 0x1E224000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=1 (value 1)
#[test]
fn test_aarch64_float_convert_fp_combo_1_4000_1e624000() {
    // Encoding: 0x1E624000
    // Test aarch64_float_convert_fp field combination: type1=1, opc=0, Rn=0, Rd=0
    // Fields: opc=0, Rn=0, type1=1, Rd=0
    let encoding: u32 = 0x1E624000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvt	s0, d0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// type1=3 (maximum value (3))
#[test]
fn test_aarch64_float_convert_fp_combo_2_4000_1ee24000() {
    // Encoding: 0x1EE24000
    // Test aarch64_float_convert_fp field combination: type1=3, opc=0, Rn=0, Rd=0
    // Fields: type1=3, Rn=0, opc=0, Rd=0
    let encoding: u32 = 0x1EE24000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvt	s0, h0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_float_convert_fp_combo_3_4000_1e224000() {
    // Encoding: 0x1E224000
    // Test aarch64_float_convert_fp field combination: type1=0, opc=0, Rn=0, Rd=0
    // Fields: Rd=0, type1=0, opc=0, Rn=0
    let encoding: u32 = 0x1E224000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_float_convert_fp_combo_4_4000_1e22c000() {
    // Encoding: 0x1E22C000
    // Test aarch64_float_convert_fp field combination: type1=0, opc=1, Rn=0, Rd=0
    // Fields: type1=0, Rn=0, Rd=0, opc=1
    let encoding: u32 = 0x1E22C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvt	d0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_float_convert_fp_combo_5_4000_1e234000() {
    // Encoding: 0x1E234000
    // Test aarch64_float_convert_fp field combination: type1=0, opc=2, Rn=0, Rd=0
    // Fields: Rd=0, type1=0, opc=2, Rn=0
    let encoding: u32 = 0x1E234000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_float_convert_fp_combo_6_4000_1e23c000() {
    // Encoding: 0x1E23C000
    // Test aarch64_float_convert_fp field combination: type1=0, opc=3, Rn=0, Rd=0
    // Fields: Rn=0, type1=0, opc=3, Rd=0
    let encoding: u32 = 0x1E23C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvt	h0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_convert_fp_combo_7_4000_1e224000() {
    // Encoding: 0x1E224000
    // Test aarch64_float_convert_fp field combination: type1=0, opc=0, Rn=0, Rd=0
    // Fields: Rn=0, opc=0, type1=0, Rd=0
    let encoding: u32 = 0x1E224000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_convert_fp_combo_8_4000_1e224020() {
    // Encoding: 0x1E224020
    // Test aarch64_float_convert_fp field combination: type1=0, opc=0, Rn=1, Rd=0
    // Fields: opc=0, type1=0, Rn=1, Rd=0
    let encoding: u32 = 0x1E224020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_convert_fp_combo_9_4000_1e2243c0() {
    // Encoding: 0x1E2243C0
    // Test aarch64_float_convert_fp field combination: type1=0, opc=0, Rn=30, Rd=0
    // Fields: type1=0, opc=0, Rd=0, Rn=30
    let encoding: u32 = 0x1E2243C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_float_convert_fp_combo_10_4000_1e2243e0() {
    // Encoding: 0x1E2243E0
    // Test aarch64_float_convert_fp field combination: type1=0, opc=0, Rn=31, Rd=0
    // Fields: Rd=0, type1=0, opc=0, Rn=31
    let encoding: u32 = 0x1E2243E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_float_convert_fp_combo_11_4000_1e224000() {
    // Encoding: 0x1E224000
    // Test aarch64_float_convert_fp field combination: type1=0, opc=0, Rn=0, Rd=0
    // Fields: Rd=0, type1=0, Rn=0, opc=0
    let encoding: u32 = 0x1E224000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_float_convert_fp_combo_12_4000_1e224001() {
    // Encoding: 0x1E224001
    // Test aarch64_float_convert_fp field combination: type1=0, opc=0, Rn=0, Rd=1
    // Fields: type1=0, opc=0, Rn=0, Rd=1
    let encoding: u32 = 0x1E224001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_float_convert_fp_combo_13_4000_1e22401e() {
    // Encoding: 0x1E22401E
    // Test aarch64_float_convert_fp field combination: type1=0, opc=0, Rn=0, Rd=30
    // Fields: Rd=30, opc=0, Rn=0, type1=0
    let encoding: u32 = 0x1E22401E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_float_convert_fp_combo_14_4000_1e22401f() {
    // Encoding: 0x1E22401F
    // Test aarch64_float_convert_fp field combination: type1=0, opc=0, Rn=0, Rd=31
    // Fields: type1=0, opc=0, Rn=0, Rd=31
    let encoding: u32 = 0x1E22401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rd=1 (same register test (reg=1))
#[test]
fn test_aarch64_float_convert_fp_combo_15_4000_1e224021() {
    // Encoding: 0x1E224021
    // Test aarch64_float_convert_fp field combination: type1=0, opc=0, Rn=1, Rd=1
    // Fields: Rd=1, type1=0, opc=0, Rn=1
    let encoding: u32 = 0x1E224021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rd=31 (same register test (reg=31))
#[test]
fn test_aarch64_float_convert_fp_combo_16_4000_1e2243ff() {
    // Encoding: 0x1E2243FF
    // Test aarch64_float_convert_fp field combination: type1=0, opc=0, Rn=31, Rd=31
    // Fields: Rd=31, opc=0, type1=0, Rn=31
    let encoding: u32 = 0x1E2243FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_float_convert_fp_special_opc_0_size_variant_0_16384_1e224000() {
    // Encoding: 0x1E224000
    // Test aarch64_float_convert_fp special value opc = 0 (Size variant 0)
    // Fields: type1=0, opc=0, Rd=0, Rn=0
    let encoding: u32 = 0x1E224000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_float_convert_fp_special_opc_1_size_variant_1_16384_1e22c000() {
    // Encoding: 0x1E22C000
    // Test aarch64_float_convert_fp special value opc = 1 (Size variant 1)
    // Fields: Rn=0, type1=0, Rd=0, opc=1
    let encoding: u32 = 0x1E22C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvt	d0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_float_convert_fp_special_opc_2_size_variant_2_16384_1e234000() {
    // Encoding: 0x1E234000
    // Test aarch64_float_convert_fp special value opc = 2 (Size variant 2)
    // Fields: opc=2, Rn=0, type1=0, Rd=0
    let encoding: u32 = 0x1E234000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_float_convert_fp_special_opc_3_size_variant_3_16384_1e23c000() {
    // Encoding: 0x1E23C000
    // Test aarch64_float_convert_fp special value opc = 3 (Size variant 3)
    // Fields: Rd=0, opc=3, type1=0, Rn=0
    let encoding: u32 = 0x1E23C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: fcvt	h0, s0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_float_convert_fp_special_rn_31_stack_pointer_sp_may_require_alignment_16384_1e2243e0()
 {
    // Encoding: 0x1E2243E0
    // Test aarch64_float_convert_fp special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rd=0, type1=0, Rn=31, opc=0
    let encoding: u32 = 0x1E2243E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_float_convert_fp
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_float_convert_fp_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_16384_1e22401f()
 {
    // Encoding: 0x1E22401F
    // Test aarch64_float_convert_fp special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: type1=0, Rn=0, opc=0, Rd=31
    let encoding: u32 = 0x1E22401F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}
