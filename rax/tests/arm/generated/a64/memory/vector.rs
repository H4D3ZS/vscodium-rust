//! A64 memory vector tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_memory_vector_single_no_wb Tests
// ============================================================================

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_q_0_min_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field Q = 0 (Min)
    // Fields: Q=0, Rt=0, opcode=0, L=0, Rn=0, S=0, size=0, R=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_q_1_max_0_4d000000() {
    // Encoding: 0x4D000000
    // Test aarch64_memory_vector_single_no_wb field Q = 1 (Max)
    // Fields: R=0, Q=1, Rn=0, opcode=0, S=0, L=0, Rt=0, size=0
    let encoding: u32 = 0x4D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[8], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_l_0_min_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field L = 0 (Min)
    // Fields: Q=0, R=0, Rn=0, S=0, size=0, opcode=0, L=0, Rt=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_l_1_max_0_0d400000() {
    // Encoding: 0x0D400000
    // Test aarch64_memory_vector_single_no_wb field L = 1 (Max)
    // Fields: size=0, R=0, L=1, opcode=0, Rn=0, Q=0, S=0, Rt=0
    let encoding: u32 = 0x0D400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ld1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field R 21 +: 1`
/// Requirement: FieldBoundary { field: "R", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_r_0_min_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field R = 0 (Min)
    // Fields: Rt=0, R=0, Q=0, L=0, opcode=0, Rn=0, S=0, size=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field R 21 +: 1`
/// Requirement: FieldBoundary { field: "R", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_r_1_max_0_0d200000() {
    // Encoding: 0x0D200000
    // Test aarch64_memory_vector_single_no_wb field R = 1 (Max)
    // Fields: Q=0, Rt=0, S=0, size=0, L=0, opcode=0, Rn=0, R=1
    let encoding: u32 = 0x0D200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st2	{ v0.b, v1.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field opcode 13 +: 3`
/// Requirement: FieldBoundary { field: "opcode", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_opcode_0_min_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field opcode = 0 (Min)
    // Fields: size=0, L=0, Rn=0, Rt=0, opcode=0, Q=0, R=0, S=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field opcode 13 +: 3`
/// Requirement: FieldBoundary { field: "opcode", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_opcode_1_poweroftwo_0_0d002000() {
    // Encoding: 0x0D002000
    // Test aarch64_memory_vector_single_no_wb field opcode = 1 (PowerOfTwo)
    // Fields: opcode=1, L=0, R=0, Q=0, size=0, Rn=0, Rt=0, S=0
    let encoding: u32 = 0x0D002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st3	{ v0.b, v1.b, v2.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field opcode 13 +: 3`
/// Requirement: FieldBoundary { field: "opcode", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_opcode_7_max_0_0d00e000() {
    // Encoding: 0x0D00E000
    // Test aarch64_memory_vector_single_no_wb field opcode = 7 (Max)
    // Fields: L=0, opcode=7, Rt=0, Q=0, S=0, R=0, Rn=0, size=0
    let encoding: u32 = 0x0D00E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field S 12 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_s_0_min_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field S = 0 (Min)
    // Fields: Rt=0, size=0, Q=0, Rn=0, L=0, S=0, R=0, opcode=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field S 12 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_s_1_max_0_0d001000() {
    // Encoding: 0x0D001000
    // Test aarch64_memory_vector_single_no_wb field S = 1 (Max)
    // Fields: Q=0, R=0, L=0, opcode=0, Rn=0, size=0, S=1, Rt=0
    let encoding: u32 = 0x0D001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[4], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_size_0_min_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field size = 0 (Min)
    // Fields: size=0, opcode=0, Q=0, S=0, Rt=0, L=0, R=0, Rn=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_size_1_poweroftwo_0_0d000400() {
    // Encoding: 0x0D000400
    // Test aarch64_memory_vector_single_no_wb field size = 1 (PowerOfTwo)
    // Fields: opcode=0, Q=0, Rt=0, size=1, L=0, Rn=0, S=0, R=0
    let encoding: u32 = 0x0D000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[1], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_size_2_poweroftwo_0_0d000800() {
    // Encoding: 0x0D000800
    // Test aarch64_memory_vector_single_no_wb field size = 2 (PowerOfTwo)
    // Fields: Rn=0, L=0, Q=0, R=0, opcode=0, size=2, Rt=0, S=0
    let encoding: u32 = 0x0D000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[2], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_size_3_max_0_0d000c00() {
    // Encoding: 0x0D000C00
    // Test aarch64_memory_vector_single_no_wb field size = 3 (Max)
    // Fields: R=0, S=0, opcode=0, size=3, Rn=0, L=0, Q=0, Rt=0
    let encoding: u32 = 0x0D000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[3], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_rn_0_min_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field Rn = 0 (Min)
    // Fields: Q=0, size=0, Rn=0, S=0, Rt=0, R=0, opcode=0, L=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_rn_1_poweroftwo_0_0d000020() {
    // Encoding: 0x0D000020
    // Test aarch64_memory_vector_single_no_wb field Rn = 1 (PowerOfTwo)
    // Fields: Rt=0, S=0, size=0, Q=0, Rn=1, opcode=0, R=0, L=0
    let encoding: u32 = 0x0D000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_rn_30_poweroftwominusone_0_0d0003c0() {
    // Encoding: 0x0D0003C0
    // Test aarch64_memory_vector_single_no_wb field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: S=0, Rn=30, L=0, size=0, opcode=0, R=0, Rt=0, Q=0
    let encoding: u32 = 0x0D0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x30]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_rn_31_max_0_0d0003e0() {
    // Encoding: 0x0D0003E0
    // Test aarch64_memory_vector_single_no_wb field Rn = 31 (Max)
    // Fields: S=0, R=0, Rn=31, opcode=0, Q=0, L=0, Rt=0, size=0
    let encoding: u32 = 0x0D0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_rt_0_min_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field Rt = 0 (Min)
    // Fields: size=0, Rt=0, S=0, R=0, L=0, Rn=0, opcode=0, Q=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_rt_1_poweroftwo_0_0d000001() {
    // Encoding: 0x0D000001
    // Test aarch64_memory_vector_single_no_wb field Rt = 1 (PowerOfTwo)
    // Fields: Q=0, S=0, size=0, L=0, Rt=1, opcode=0, Rn=0, R=0
    let encoding: u32 = 0x0D000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v1.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_rt_30_poweroftwominusone_0_0d00001e() {
    // Encoding: 0x0D00001E
    // Test aarch64_memory_vector_single_no_wb field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: R=0, L=0, Q=0, size=0, Rn=0, opcode=0, Rt=30, S=0
    let encoding: u32 = 0x0D00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v30.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_vector_single_no_wb_field_rt_31_max_0_0d00001f() {
    // Encoding: 0x0D00001F
    // Test aarch64_memory_vector_single_no_wb field Rt = 31 (Max)
    // Fields: Rn=0, R=0, Q=0, size=0, S=0, Rt=31, L=0, opcode=0
    let encoding: u32 = 0x0D00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v31.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_0_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Rn=0, size=0, R=0, opcode=0, S=0, L=0, Rt=0, Q=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_1_0_4d000000() {
    // Encoding: 0x4D000000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=1, L=0, R=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Q=1, R=0, size=0, L=0, opcode=0, Rn=0, Rt=0, S=0
    let encoding: u32 = 0x4D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[8], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_2_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: opcode=0, size=0, Rt=0, R=0, L=0, Rn=0, Q=0, S=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_3_0_0d400000() {
    // Encoding: 0x0D400000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=1, R=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: S=0, L=1, Rn=0, size=0, opcode=0, Rt=0, Q=0, R=0
    let encoding: u32 = 0x0D400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ld1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// R=0 (minimum value)
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_4_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Q=0, size=0, R=0, S=0, Rt=0, Rn=0, opcode=0, L=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// R=1 (maximum value (1))
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_5_0_0d200000() {
    // Encoding: 0x0D200000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=1, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: S=0, size=0, opcode=0, Q=0, Rt=0, R=1, Rn=0, L=0
    let encoding: u32 = 0x0D200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st2	{ v0.b, v1.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=0 (minimum value)
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_6_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: opcode=0, R=0, L=0, size=0, Rt=0, Rn=0, Q=0, S=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=1 (value 1)
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_7_0_0d002000() {
    // Encoding: 0x0D002000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=1, S=0, size=0, Rn=0, Rt=0
    // Fields: S=0, L=0, R=0, opcode=1, size=0, Rn=0, Rt=0, Q=0
    let encoding: u32 = 0x0D002000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st3	{ v0.b, v1.b, v2.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=7 (maximum value (7))
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_8_0_0d00e000() {
    // Encoding: 0x0D00E000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=7, S=0, size=0, Rn=0, Rt=0
    // Fields: R=0, Rt=0, size=0, Rn=0, opcode=7, S=0, Q=0, L=0
    let encoding: u32 = 0x0D00E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_9_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Q=0, opcode=0, S=0, Rt=0, size=0, Rn=0, L=0, R=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_10_0_0d001000() {
    // Encoding: 0x0D001000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=1, size=0, Rn=0, Rt=0
    // Fields: Rt=0, Q=0, Rn=0, R=0, L=0, size=0, opcode=0, S=1
    let encoding: u32 = 0x0D001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[4], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_11_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: opcode=0, S=0, L=0, Rn=0, Rt=0, Q=0, size=0, R=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_12_0_0d000400() {
    // Encoding: 0x0D000400
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=1, Rn=0, Rt=0
    // Fields: size=1, S=0, Rt=0, Rn=0, L=0, opcode=0, R=0, Q=0
    let encoding: u32 = 0x0D000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[1], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_13_0_0d000800() {
    // Encoding: 0x0D000800
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=2, Rn=0, Rt=0
    // Fields: Rn=0, R=0, S=0, L=0, size=2, Rt=0, opcode=0, Q=0
    let encoding: u32 = 0x0D000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[2], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_14_0_0d000c00() {
    // Encoding: 0x0D000C00
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=3, Rn=0, Rt=0
    // Fields: opcode=0, Rt=0, Rn=0, L=0, S=0, Q=0, R=0, size=3
    let encoding: u32 = 0x0D000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[3], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_15_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Rt=0, S=0, size=0, Rn=0, L=0, R=0, Q=0, opcode=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_16_0_0d000020() {
    // Encoding: 0x0D000020
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=1, Rt=0
    // Fields: size=0, opcode=0, Q=0, S=0, R=0, L=0, Rn=1, Rt=0
    let encoding: u32 = 0x0D000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_17_0_0d0003c0() {
    // Encoding: 0x0D0003C0
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=30, Rt=0
    // Fields: L=0, R=0, S=0, Rn=30, size=0, Q=0, opcode=0, Rt=0
    let encoding: u32 = 0x0D0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x30]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_18_0_0d0003e0() {
    // Encoding: 0x0D0003E0
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=31, Rt=0
    // Fields: L=0, size=0, R=0, Q=0, Rt=0, opcode=0, S=0, Rn=31
    let encoding: u32 = 0x0D0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_19_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: L=0, Rn=0, S=0, R=0, Rt=0, opcode=0, size=0, Q=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_20_0_0d000001() {
    // Encoding: 0x0D000001
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=0, Rt=1
    // Fields: Rn=0, opcode=0, L=0, size=0, S=0, R=0, Q=0, Rt=1
    let encoding: u32 = 0x0D000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v1.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_21_0_0d00001e() {
    // Encoding: 0x0D00001E
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=0, Rt=30
    // Fields: opcode=0, R=0, size=0, Rt=30, S=0, Rn=0, L=0, Q=0
    let encoding: u32 = 0x0D00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v30.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_22_0_0d00001f() {
    // Encoding: 0x0D00001F
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=0, Rt=31
    // Fields: Q=0, L=0, Rn=0, size=0, Rt=31, opcode=0, R=0, S=0
    let encoding: u32 = 0x0D00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v31.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_23_0_0d000021() {
    // Encoding: 0x0D000021
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=1, Rt=1
    // Fields: S=0, L=0, size=0, Rn=1, Rt=1, R=0, Q=0, opcode=0
    let encoding: u32 = 0x0D000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v1.b }[0], [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_vector_single_no_wb_combo_24_0_0d0003ff() {
    // Encoding: 0x0D0003FF
    // Test aarch64_memory_vector_single_no_wb field combination: Q=0, L=0, R=0, opcode=0, S=0, size=0, Rn=31, Rt=31
    // Fields: Rt=31, S=0, Rn=31, L=0, R=0, size=0, opcode=0, Q=0
    let encoding: u32 = 0x0D0003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v31.b }[0], [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_vector_single_no_wb_special_q_0_size_variant_0_0_0d000400() {
    // Encoding: 0x0D000400
    // Test aarch64_memory_vector_single_no_wb special value Q = 0 (Size variant 0)
    // Fields: R=0, Rn=0, opcode=0, Rt=0, Q=0, size=1, S=0, L=0
    let encoding: u32 = 0x0D000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[1], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_vector_single_no_wb_special_q_1_size_variant_1_0_4d000400() {
    // Encoding: 0x4D000400
    // Test aarch64_memory_vector_single_no_wb special value Q = 1 (Size variant 1)
    // Fields: Rn=0, L=0, Q=1, opcode=0, S=0, Rt=0, R=0, size=1
    let encoding: u32 = 0x4D000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[9], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_vector_single_no_wb_special_s_0_size_variant_0_0_0d000400() {
    // Encoding: 0x0D000400
    // Test aarch64_memory_vector_single_no_wb special value S = 0 (Size variant 0)
    // Fields: Q=0, opcode=0, Rt=0, L=0, Rn=0, S=0, R=0, size=1
    let encoding: u32 = 0x0D000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[1], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_vector_single_no_wb_special_s_1_size_variant_1_0_0d001400() {
    // Encoding: 0x0D001400
    // Test aarch64_memory_vector_single_no_wb special value S = 1 (Size variant 1)
    // Fields: S=1, size=1, Rn=0, R=0, opcode=0, Q=0, Rt=0, L=0
    let encoding: u32 = 0x0D001400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[5], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_vector_single_no_wb_special_size_0_size_variant_0_0_0d000000() {
    // Encoding: 0x0D000000
    // Test aarch64_memory_vector_single_no_wb special value size = 0 (Size variant 0)
    // Fields: L=0, R=0, Rt=0, Rn=0, size=0, Q=0, opcode=0, S=0
    let encoding: u32 = 0x0D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_vector_single_no_wb_special_size_1_size_variant_1_0_0d000400() {
    // Encoding: 0x0D000400
    // Test aarch64_memory_vector_single_no_wb special value size = 1 (Size variant 1)
    // Fields: Q=0, R=0, S=0, Rt=0, opcode=0, Rn=0, L=0, size=1
    let encoding: u32 = 0x0D000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[1], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_vector_single_no_wb_special_size_2_size_variant_2_0_0d000800() {
    // Encoding: 0x0D000800
    // Test aarch64_memory_vector_single_no_wb special value size = 2 (Size variant 2)
    // Fields: Rn=0, size=2, Q=0, L=0, opcode=0, R=0, S=0, Rt=0
    let encoding: u32 = 0x0D000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[2], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_vector_single_no_wb_special_size_3_size_variant_3_0_0d000c00() {
    // Encoding: 0x0D000C00
    // Test aarch64_memory_vector_single_no_wb special value size = 3 (Size variant 3)
    // Fields: R=0, opcode=0, S=0, Q=0, Rn=0, size=3, L=0, Rt=0
    let encoding: u32 = 0x0D000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[3], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_vector_single_no_wb_special_rn_31_stack_pointer_sp_may_require_alignment_0_0d0007e0()
 {
    // Encoding: 0x0D0007E0
    // Test aarch64_memory_vector_single_no_wb special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: R=0, size=1, L=0, opcode=0, Q=0, S=0, Rt=0, Rn=31
    let encoding: u32 = 0x0D0007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[1], [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_no_wb
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_vector_single_no_wb_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_0d00041f()
 {
    // Encoding: 0x0D00041F
    // Test aarch64_memory_vector_single_no_wb special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: R=0, opcode=0, Rn=0, S=0, size=1, L=0, Rt=31, Q=0
    let encoding: u32 = 0x0D00041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v31.b }[1], [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_q_0_min_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field Q = 0 (Min)
    // Fields: S=0, Rn=0, L=0, opcode=0, Rm=0, size=0, R=0, Rt=0, Q=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_q_1_max_0_4d800000() {
    // Encoding: 0x4D800000
    // Test aarch64_memory_vector_single_post_inc field Q = 1 (Max)
    // Fields: S=0, Rn=0, Rm=0, Rt=0, L=0, opcode=0, R=0, Q=1, size=0
    let encoding: u32 = 0x4D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[8], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_l_0_min_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field L = 0 (Min)
    // Fields: size=0, R=0, Q=0, opcode=0, S=0, Rt=0, Rm=0, Rn=0, L=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_l_1_max_0_0dc00000() {
    // Encoding: 0x0DC00000
    // Test aarch64_memory_vector_single_post_inc field L = 1 (Max)
    // Fields: opcode=0, Rn=0, Rm=0, Q=0, Rt=0, R=0, S=0, L=1, size=0
    let encoding: u32 = 0x0DC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ld1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field R 21 +: 1`
/// Requirement: FieldBoundary { field: "R", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_r_0_min_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field R = 0 (Min)
    // Fields: S=0, Rn=0, R=0, opcode=0, Q=0, Rt=0, size=0, L=0, Rm=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field R 21 +: 1`
/// Requirement: FieldBoundary { field: "R", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_r_1_max_0_0da00000() {
    // Encoding: 0x0DA00000
    // Test aarch64_memory_vector_single_post_inc field R = 1 (Max)
    // Fields: size=0, Q=0, Rm=0, opcode=0, R=1, L=0, S=0, Rn=0, Rt=0
    let encoding: u32 = 0x0DA00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st2	{ v0.b, v1.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_rm_0_min_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field Rm = 0 (Min)
    // Fields: opcode=0, Rn=0, S=0, R=0, Q=0, L=0, Rm=0, size=0, Rt=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_rm_1_poweroftwo_0_0d810000() {
    // Encoding: 0x0D810000
    // Test aarch64_memory_vector_single_post_inc field Rm = 1 (PowerOfTwo)
    // Fields: opcode=0, Rm=1, Q=0, L=0, size=0, Rn=0, Rt=0, S=0, R=0
    let encoding: u32 = 0x0D810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_rm_30_poweroftwominusone_0_0d9e0000() {
    // Encoding: 0x0D9E0000
    // Test aarch64_memory_vector_single_post_inc field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, L=0, R=0, Rn=0, Rt=0, S=0, size=0, opcode=0, Rm=30
    let encoding: u32 = 0x0D9E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_rm_31_max_0_0d9f0000() {
    // Encoding: 0x0D9F0000
    // Test aarch64_memory_vector_single_post_inc field Rm = 31 (Max)
    // Fields: size=0, Rt=0, Rn=0, S=0, Rm=31, R=0, Q=0, opcode=0, L=0
    let encoding: u32 = 0x0D9F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field opcode 13 +: 3`
/// Requirement: FieldBoundary { field: "opcode", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_opcode_0_min_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field opcode = 0 (Min)
    // Fields: Rn=0, Rm=0, opcode=0, S=0, R=0, L=0, Q=0, size=0, Rt=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field opcode 13 +: 3`
/// Requirement: FieldBoundary { field: "opcode", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_opcode_1_poweroftwo_0_0d802000() {
    // Encoding: 0x0D802000
    // Test aarch64_memory_vector_single_post_inc field opcode = 1 (PowerOfTwo)
    // Fields: Rn=0, S=0, opcode=1, Rt=0, Rm=0, R=0, size=0, L=0, Q=0
    let encoding: u32 = 0x0D802000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st3	{ v0.b, v1.b, v2.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field opcode 13 +: 3`
/// Requirement: FieldBoundary { field: "opcode", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_opcode_7_max_0_0d80e000() {
    // Encoding: 0x0D80E000
    // Test aarch64_memory_vector_single_post_inc field opcode = 7 (Max)
    // Fields: Q=0, S=0, Rm=0, Rn=0, R=0, size=0, L=0, opcode=7, Rt=0
    let encoding: u32 = 0x0D80E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field S 12 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_s_0_min_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field S = 0 (Min)
    // Fields: L=0, size=0, Q=0, S=0, opcode=0, Rm=0, Rt=0, R=0, Rn=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field S 12 +: 1`
/// Requirement: FieldBoundary { field: "S", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_s_1_max_0_0d801000() {
    // Encoding: 0x0D801000
    // Test aarch64_memory_vector_single_post_inc field S = 1 (Max)
    // Fields: L=0, Rt=0, R=0, opcode=0, S=1, Q=0, Rn=0, Rm=0, size=0
    let encoding: u32 = 0x0D801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[4], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_size_0_min_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field size = 0 (Min)
    // Fields: Rm=0, L=0, size=0, Q=0, opcode=0, S=0, R=0, Rn=0, Rt=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_size_1_poweroftwo_0_0d800400() {
    // Encoding: 0x0D800400
    // Test aarch64_memory_vector_single_post_inc field size = 1 (PowerOfTwo)
    // Fields: Rt=0, L=0, R=0, size=1, opcode=0, Rm=0, Rn=0, Q=0, S=0
    let encoding: u32 = 0x0D800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[1], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_size_2_poweroftwo_0_0d800800() {
    // Encoding: 0x0D800800
    // Test aarch64_memory_vector_single_post_inc field size = 2 (PowerOfTwo)
    // Fields: R=0, Rn=0, Rm=0, opcode=0, S=0, size=2, L=0, Q=0, Rt=0
    let encoding: u32 = 0x0D800800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[2], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_size_3_max_0_0d800c00() {
    // Encoding: 0x0D800C00
    // Test aarch64_memory_vector_single_post_inc field size = 3 (Max)
    // Fields: Rn=0, opcode=0, Rt=0, L=0, S=0, Q=0, size=3, R=0, Rm=0
    let encoding: u32 = 0x0D800C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[3], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_rn_0_min_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field Rn = 0 (Min)
    // Fields: L=0, size=0, Rm=0, Q=0, opcode=0, Rn=0, R=0, S=0, Rt=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_rn_1_poweroftwo_0_0d800020() {
    // Encoding: 0x0D800020
    // Test aarch64_memory_vector_single_post_inc field Rn = 1 (PowerOfTwo)
    // Fields: S=0, size=0, R=0, Rn=1, Rt=0, Q=0, Rm=0, L=0, opcode=0
    let encoding: u32 = 0x0D800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x1], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_rn_30_poweroftwominusone_0_0d8003c0() {
    // Encoding: 0x0D8003C0
    // Test aarch64_memory_vector_single_post_inc field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rm=0, L=0, Rn=30, opcode=0, Rt=0, S=0, size=0, R=0, Q=0
    let encoding: u32 = 0x0D8003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x30], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_rn_31_max_0_0d8003e0() {
    // Encoding: 0x0D8003E0
    // Test aarch64_memory_vector_single_post_inc field Rn = 31 (Max)
    // Fields: R=0, Rn=31, size=0, Rt=0, Q=0, L=0, opcode=0, S=0, Rm=0
    let encoding: u32 = 0x0D8003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [sp], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_rt_0_min_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field Rt = 0 (Min)
    // Fields: Rt=0, Rn=0, S=0, Q=0, Rm=0, L=0, size=0, R=0, opcode=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_rt_1_poweroftwo_0_0d800001() {
    // Encoding: 0x0D800001
    // Test aarch64_memory_vector_single_post_inc field Rt = 1 (PowerOfTwo)
    // Fields: L=0, Q=0, opcode=0, Rm=0, R=0, S=0, size=0, Rn=0, Rt=1
    let encoding: u32 = 0x0D800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v1.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_rt_30_poweroftwominusone_0_0d80001e() {
    // Encoding: 0x0D80001E
    // Test aarch64_memory_vector_single_post_inc field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: S=0, opcode=0, Rn=0, Rm=0, Q=0, L=0, size=0, Rt=30, R=0
    let encoding: u32 = 0x0D80001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v30.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_vector_single_post_inc_field_rt_31_max_0_0d80001f() {
    // Encoding: 0x0D80001F
    // Test aarch64_memory_vector_single_post_inc field Rt = 31 (Max)
    // Fields: R=0, Rm=0, S=0, Q=0, L=0, opcode=0, size=0, Rn=0, Rt=31
    let encoding: u32 = 0x0D80001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v31.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_0_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Rt=0, size=0, opcode=0, Rm=0, R=0, S=0, Rn=0, Q=0, L=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_1_0_4d800000() {
    // Encoding: 0x4D800000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=1, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: S=0, Rn=0, L=0, Q=1, R=0, Rm=0, Rt=0, size=0, opcode=0
    let encoding: u32 = 0x4D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[8], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_2_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Rm=0, opcode=0, Q=0, size=0, S=0, Rt=0, Rn=0, L=0, R=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_3_0_0dc00000() {
    // Encoding: 0x0DC00000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=1, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Rm=0, Q=0, R=0, Rn=0, Rt=0, size=0, L=1, S=0, opcode=0
    let encoding: u32 = 0x0DC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ld1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// R=0 (minimum value)
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_4_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: R=0, Rt=0, Rm=0, opcode=0, S=0, Rn=0, Q=0, size=0, L=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// R=1 (maximum value (1))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_5_0_0da00000() {
    // Encoding: 0x0DA00000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=1, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Rt=0, Q=0, Rn=0, L=0, opcode=0, R=1, S=0, size=0, Rm=0
    let encoding: u32 = 0x0DA00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st2	{ v0.b, v1.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_6_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: R=0, L=0, Rt=0, S=0, opcode=0, Rm=0, size=0, Rn=0, Q=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_7_0_0d810000() {
    // Encoding: 0x0D810000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=1, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Rt=0, S=0, size=0, Q=0, Rm=1, R=0, L=0, opcode=0, Rn=0
    let encoding: u32 = 0x0D810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_8_0_0d9e0000() {
    // Encoding: 0x0D9E0000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=30, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Q=0, L=0, Rn=0, S=0, R=0, Rt=0, size=0, opcode=0, Rm=30
    let encoding: u32 = 0x0D9E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_9_0_0d9f0000() {
    // Encoding: 0x0D9F0000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=31, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, Q=0, R=0, S=0, L=0, size=0, Rm=31, opcode=0
    let encoding: u32 = 0x0D9F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=0 (minimum value)
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_10_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: R=0, S=0, Rm=0, opcode=0, L=0, Rt=0, size=0, Rn=0, Q=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=1 (value 1)
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_11_0_0d802000() {
    // Encoding: 0x0D802000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=1, S=0, size=0, Rn=0, Rt=0
    // Fields: opcode=1, Q=0, S=0, Rm=0, R=0, Rn=0, size=0, Rt=0, L=0
    let encoding: u32 = 0x0D802000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st3	{ v0.b, v1.b, v2.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=7 (maximum value (7))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_12_0_0d80e000() {
    // Encoding: 0x0D80E000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=7, S=0, size=0, Rn=0, Rt=0
    // Fields: Rm=0, size=0, Q=0, Rn=0, Rt=0, opcode=7, S=0, L=0, R=0
    let encoding: u32 = 0x0D80E000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_13_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: L=0, R=0, size=0, S=0, Rn=0, opcode=0, Rt=0, Rm=0, Q=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// S=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_14_0_0d801000() {
    // Encoding: 0x0D801000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=1, size=0, Rn=0, Rt=0
    // Fields: opcode=0, size=0, R=0, Rm=0, L=0, S=1, Rn=0, Rt=0, Q=0
    let encoding: u32 = 0x0D801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[4], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_15_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Q=0, Rm=0, L=0, opcode=0, Rt=0, Rn=0, size=0, R=0, S=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_16_0_0d800400() {
    // Encoding: 0x0D800400
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=1, Rn=0, Rt=0
    // Fields: Rm=0, opcode=0, Q=0, size=1, Rt=0, L=0, R=0, S=0, Rn=0
    let encoding: u32 = 0x0D800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[1], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_17_0_0d800800() {
    // Encoding: 0x0D800800
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=2, Rn=0, Rt=0
    // Fields: Q=0, Rm=0, Rt=0, size=2, R=0, S=0, opcode=0, L=0, Rn=0
    let encoding: u32 = 0x0D800800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[2], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_18_0_0d800c00() {
    // Encoding: 0x0D800C00
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=3, Rn=0, Rt=0
    // Fields: Rm=0, Rt=0, size=3, opcode=0, Q=0, Rn=0, L=0, S=0, R=0
    let encoding: u32 = 0x0D800C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[3], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_19_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: Rn=0, R=0, Rm=0, Rt=0, Q=0, L=0, S=0, size=0, opcode=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_20_0_0d800020() {
    // Encoding: 0x0D800020
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=1, Rt=0
    // Fields: Rt=0, Q=0, R=0, Rm=0, opcode=0, S=0, Rn=1, size=0, L=0
    let encoding: u32 = 0x0D800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x1], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_21_0_0d8003c0() {
    // Encoding: 0x0D8003C0
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=30, Rt=0
    // Fields: Rn=30, R=0, Q=0, S=0, L=0, opcode=0, size=0, Rm=0, Rt=0
    let encoding: u32 = 0x0D8003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x30], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_22_0_0d8003e0() {
    // Encoding: 0x0D8003E0
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=31, Rt=0
    // Fields: Rn=31, R=0, size=0, Q=0, opcode=0, S=0, Rt=0, L=0, Rm=0
    let encoding: u32 = 0x0D8003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [sp], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_23_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=0
    // Fields: S=0, Rt=0, L=0, R=0, size=0, Q=0, opcode=0, Rm=0, Rn=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_24_0_0d800001() {
    // Encoding: 0x0D800001
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=1
    // Fields: S=0, Rn=0, opcode=0, size=0, L=0, R=0, Q=0, Rm=0, Rt=1
    let encoding: u32 = 0x0D800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v1.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_25_0_0d80001e() {
    // Encoding: 0x0D80001E
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=30
    // Fields: Rt=30, size=0, opcode=0, Q=0, R=0, S=0, L=0, Rn=0, Rm=0
    let encoding: u32 = 0x0D80001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v30.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_26_0_0d80001f() {
    // Encoding: 0x0D80001F
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=0, Rt=31
    // Fields: R=0, Rn=0, Rt=31, Rm=0, S=0, Q=0, L=0, opcode=0, size=0
    let encoding: u32 = 0x0D80001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v31.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_27_0_0d810020() {
    // Encoding: 0x0D810020
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=1, opcode=0, S=0, size=0, Rn=1, Rt=0
    // Fields: opcode=0, L=0, R=0, Rt=0, Rm=1, size=0, Q=0, Rn=1, S=0
    let encoding: u32 = 0x0D810020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x1], x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_28_0_0d9f03e0() {
    // Encoding: 0x0D9F03E0
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=31, opcode=0, S=0, size=0, Rn=31, Rt=0
    // Fields: opcode=0, Rt=0, R=0, Rn=31, Rm=31, Q=0, S=0, size=0, L=0
    let encoding: u32 = 0x0D9F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [sp], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_29_0_0d810001() {
    // Encoding: 0x0D810001
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=1, opcode=0, S=0, size=0, Rn=0, Rt=1
    // Fields: Rm=1, Rt=1, S=0, Rn=0, L=0, opcode=0, Q=0, size=0, R=0
    let encoding: u32 = 0x0D810001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v1.b }[0], [x0], x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_30_0_0d9f001f() {
    // Encoding: 0x0D9F001F
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=31, opcode=0, S=0, size=0, Rn=0, Rt=31
    // Fields: size=0, S=0, opcode=0, Rt=31, Rn=0, R=0, Rm=31, Q=0, L=0
    let encoding: u32 = 0x0D9F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v31.b }[0], [x0], #1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_31_0_0d800021() {
    // Encoding: 0x0D800021
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=1, Rt=1
    // Fields: R=0, L=0, S=0, Rn=1, Rt=1, Q=0, size=0, Rm=0, opcode=0
    let encoding: u32 = 0x0D800021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v1.b }[0], [x1], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_vector_single_post_inc_combo_32_0_0d8003ff() {
    // Encoding: 0x0D8003FF
    // Test aarch64_memory_vector_single_post_inc field combination: Q=0, L=0, R=0, Rm=0, opcode=0, S=0, size=0, Rn=31, Rt=31
    // Fields: S=0, L=0, opcode=0, Rt=31, size=0, Rm=0, R=0, Rn=31, Q=0
    let encoding: u32 = 0x0D8003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v31.b }[0], [sp], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_vector_single_post_inc_special_q_0_size_variant_0_0_0d800400() {
    // Encoding: 0x0D800400
    // Test aarch64_memory_vector_single_post_inc special value Q = 0 (Size variant 0)
    // Fields: Rm=0, R=0, Rt=0, S=0, opcode=0, Rn=0, size=1, Q=0, L=0
    let encoding: u32 = 0x0D800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[1], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_vector_single_post_inc_special_q_1_size_variant_1_0_4d800400() {
    // Encoding: 0x4D800400
    // Test aarch64_memory_vector_single_post_inc special value Q = 1 (Size variant 1)
    // Fields: Rm=0, size=1, Rt=0, L=0, R=0, opcode=0, Rn=0, Q=1, S=0
    let encoding: u32 = 0x4D800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[9], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field S = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "S", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_vector_single_post_inc_special_s_0_size_variant_0_0_0d800400() {
    // Encoding: 0x0D800400
    // Test aarch64_memory_vector_single_post_inc special value S = 0 (Size variant 0)
    // Fields: R=0, opcode=0, Q=0, L=0, S=0, size=1, Rm=0, Rt=0, Rn=0
    let encoding: u32 = 0x0D800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[1], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field S = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "S", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_vector_single_post_inc_special_s_1_size_variant_1_0_0d801400() {
    // Encoding: 0x0D801400
    // Test aarch64_memory_vector_single_post_inc special value S = 1 (Size variant 1)
    // Fields: Rn=0, opcode=0, size=1, L=0, Q=0, Rt=0, R=0, S=1, Rm=0
    let encoding: u32 = 0x0D801400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[5], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_vector_single_post_inc_special_size_0_size_variant_0_0_0d800000() {
    // Encoding: 0x0D800000
    // Test aarch64_memory_vector_single_post_inc special value size = 0 (Size variant 0)
    // Fields: R=0, size=0, Q=0, S=0, Rn=0, opcode=0, L=0, Rt=0, Rm=0
    let encoding: u32 = 0x0D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[0], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_vector_single_post_inc_special_size_1_size_variant_1_0_0d800400() {
    // Encoding: 0x0D800400
    // Test aarch64_memory_vector_single_post_inc special value size = 1 (Size variant 1)
    // Fields: Rn=0, L=0, S=0, R=0, Rt=0, opcode=0, Rm=0, size=1, Q=0
    let encoding: u32 = 0x0D800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[1], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_vector_single_post_inc_special_size_2_size_variant_2_0_0d800800() {
    // Encoding: 0x0D800800
    // Test aarch64_memory_vector_single_post_inc special value size = 2 (Size variant 2)
    // Fields: Rm=0, Rt=0, opcode=0, Rn=0, R=0, S=0, Q=0, size=2, L=0
    let encoding: u32 = 0x0D800800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[2], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_vector_single_post_inc_special_size_3_size_variant_3_0_0d800c00() {
    // Encoding: 0x0D800C00
    // Test aarch64_memory_vector_single_post_inc special value size = 3 (Size variant 3)
    // Fields: size=3, L=0, Rt=0, Rm=0, Q=0, R=0, opcode=0, S=0, Rn=0
    let encoding: u32 = 0x0D800C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[3], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_vector_single_post_inc_special_rn_31_stack_pointer_sp_may_require_alignment_0_0d8007e0()
 {
    // Encoding: 0x0D8007E0
    // Test aarch64_memory_vector_single_post_inc special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rt=0, Rm=0, Q=0, L=0, opcode=0, Rn=31, S=0, size=1, R=0
    let encoding: u32 = 0x0D8007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.b }[1], [sp], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_single_post_inc
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_vector_single_post_inc_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_0d80041f()
 {
    // Encoding: 0x0D80041F
    // Test aarch64_memory_vector_single_post_inc special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rt=31, Rn=0, Rm=0, R=0, Q=0, size=1, L=0, opcode=0, S=0
    let encoding: u32 = 0x0D80041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v31.b }[1], [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// aarch64_memory_vector_multiple_no_wb Tests
// ============================================================================

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_q_0_min_0_0c000000() {
    // Encoding: 0x0C000000
    // Test aarch64_memory_vector_multiple_no_wb field Q = 0 (Min)
    // Fields: Rt=0, L=0, Q=0, Rn=0, size=0, opcode=0
    let encoding: u32 = 0x0C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_q_1_max_0_4c000000() {
    // Encoding: 0x4C000000
    // Test aarch64_memory_vector_multiple_no_wb field Q = 1 (Max)
    // Fields: Rn=0, Rt=0, Q=1, size=0, opcode=0, L=0
    let encoding: u32 = 0x4C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.16b, v1.16b, v2.16b, v3.16b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_l_0_min_0_0c000000() {
    // Encoding: 0x0C000000
    // Test aarch64_memory_vector_multiple_no_wb field L = 0 (Min)
    // Fields: Q=0, size=0, opcode=0, Rt=0, L=0, Rn=0
    let encoding: u32 = 0x0C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_l_1_max_0_0c400000() {
    // Encoding: 0x0C400000
    // Test aarch64_memory_vector_multiple_no_wb field L = 1 (Max)
    // Fields: L=1, opcode=0, Rn=0, size=0, Rt=0, Q=0
    let encoding: u32 = 0x0C400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ld4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field opcode 12 +: 4`
/// Requirement: FieldBoundary { field: "opcode", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_opcode_0_min_0_0c000000() {
    // Encoding: 0x0C000000
    // Test aarch64_memory_vector_multiple_no_wb field opcode = 0 (Min)
    // Fields: Q=0, opcode=0, Rt=0, L=0, Rn=0, size=0
    let encoding: u32 = 0x0C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field opcode 12 +: 4`
/// Requirement: FieldBoundary { field: "opcode", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_opcode_1_poweroftwo_0_0c001000() {
    // Encoding: 0x0C001000
    // Test aarch64_memory_vector_multiple_no_wb field opcode = 1 (PowerOfTwo)
    // Fields: Rt=0, Q=0, opcode=1, size=0, Rn=0, L=0
    let encoding: u32 = 0x0C001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field opcode 12 +: 4`
/// Requirement: FieldBoundary { field: "opcode", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_opcode_7_poweroftwominusone_0_0c007000() {
    // Encoding: 0x0C007000
    // Test aarch64_memory_vector_multiple_no_wb field opcode = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=0, size=0, opcode=7, Q=0, L=0
    let encoding: u32 = 0x0C007000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field opcode 12 +: 4`
/// Requirement: FieldBoundary { field: "opcode", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_opcode_15_max_0_0c00f000() {
    // Encoding: 0x0C00F000
    // Test aarch64_memory_vector_multiple_no_wb field opcode = 15 (Max)
    // Fields: size=0, Rt=0, Q=0, opcode=15, Rn=0, L=0
    let encoding: u32 = 0x0C00F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_size_0_min_0_0c000000() {
    // Encoding: 0x0C000000
    // Test aarch64_memory_vector_multiple_no_wb field size = 0 (Min)
    // Fields: L=0, opcode=0, Rn=0, Q=0, size=0, Rt=0
    let encoding: u32 = 0x0C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_size_1_poweroftwo_0_0c000400() {
    // Encoding: 0x0C000400
    // Test aarch64_memory_vector_multiple_no_wb field size = 1 (PowerOfTwo)
    // Fields: Rt=0, opcode=0, size=1, Rn=0, Q=0, L=0
    let encoding: u32 = 0x0C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.4h, v1.4h, v2.4h, v3.4h }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_size_2_poweroftwo_0_0c000800() {
    // Encoding: 0x0C000800
    // Test aarch64_memory_vector_multiple_no_wb field size = 2 (PowerOfTwo)
    // Fields: L=0, Q=0, size=2, opcode=0, Rn=0, Rt=0
    let encoding: u32 = 0x0C000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.2s, v1.2s, v2.2s, v3.2s }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_size_3_max_0_0c000c00() {
    // Encoding: 0x0C000C00
    // Test aarch64_memory_vector_multiple_no_wb field size = 3 (Max)
    // Fields: L=0, opcode=0, size=3, Rt=0, Q=0, Rn=0
    let encoding: u32 = 0x0C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_rn_0_min_0_0c000000() {
    // Encoding: 0x0C000000
    // Test aarch64_memory_vector_multiple_no_wb field Rn = 0 (Min)
    // Fields: opcode=0, size=0, Q=0, L=0, Rt=0, Rn=0
    let encoding: u32 = 0x0C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_rn_1_poweroftwo_0_0c000020() {
    // Encoding: 0x0C000020
    // Test aarch64_memory_vector_multiple_no_wb field Rn = 1 (PowerOfTwo)
    // Fields: size=0, Rt=0, L=0, Rn=1, opcode=0, Q=0
    let encoding: u32 = 0x0C000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_rn_30_poweroftwominusone_0_0c0003c0() {
    // Encoding: 0x0C0003C0
    // Test aarch64_memory_vector_multiple_no_wb field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rt=0, opcode=0, Q=0, L=0, size=0
    let encoding: u32 = 0x0C0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x30]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_rn_31_max_0_0c0003e0() {
    // Encoding: 0x0C0003E0
    // Test aarch64_memory_vector_multiple_no_wb field Rn = 31 (Max)
    // Fields: Q=0, size=0, opcode=0, Rn=31, Rt=0, L=0
    let encoding: u32 = 0x0C0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_rt_0_min_0_0c000000() {
    // Encoding: 0x0C000000
    // Test aarch64_memory_vector_multiple_no_wb field Rt = 0 (Min)
    // Fields: Rt=0, opcode=0, Rn=0, size=0, L=0, Q=0
    let encoding: u32 = 0x0C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_rt_1_poweroftwo_0_0c000001() {
    // Encoding: 0x0C000001
    // Test aarch64_memory_vector_multiple_no_wb field Rt = 1 (PowerOfTwo)
    // Fields: Q=0, L=0, size=0, opcode=0, Rn=0, Rt=1
    let encoding: u32 = 0x0C000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v1.8b, v2.8b, v3.8b, v4.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_rt_30_poweroftwominusone_0_0c00001e() {
    // Encoding: 0x0C00001E
    // Test aarch64_memory_vector_multiple_no_wb field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: L=0, Rt=30, Q=0, size=0, Rn=0, opcode=0
    let encoding: u32 = 0x0C00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v30.8b, v31.8b, v0.8b, v1.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_field_rt_31_max_0_0c00001f() {
    // Encoding: 0x0C00001F
    // Test aarch64_memory_vector_multiple_no_wb field Rt = 31 (Max)
    // Fields: Q=0, opcode=0, size=0, L=0, Rn=0, Rt=31
    let encoding: u32 = 0x0C00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v31.8b, v0.8b, v1.8b, v2.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_0_0_0c000000() {
    // Encoding: 0x0C000000
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: Q=0, Rn=0, size=0, Rt=0, L=0, opcode=0
    let encoding: u32 = 0x0C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_1_0_4c000000() {
    // Encoding: 0x4C000000
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=1, L=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: Rt=0, L=0, Q=1, size=0, opcode=0, Rn=0
    let encoding: u32 = 0x4C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.16b, v1.16b, v2.16b, v3.16b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_2_0_0c000000() {
    // Encoding: 0x0C000000
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, L=0, opcode=0, Q=0, size=0
    let encoding: u32 = 0x0C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_3_0_0c400000() {
    // Encoding: 0x0C400000
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=1, opcode=0, size=0, Rn=0, Rt=0
    // Fields: size=0, Rn=0, L=1, Rt=0, opcode=0, Q=0
    let encoding: u32 = 0x0C400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ld4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=0 (minimum value)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_4_0_0c000000() {
    // Encoding: 0x0C000000
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: Q=0, Rt=0, L=0, opcode=0, size=0, Rn=0
    let encoding: u32 = 0x0C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=1 (value 1)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_5_0_0c001000() {
    // Encoding: 0x0C001000
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=1, size=0, Rn=0, Rt=0
    // Fields: Rt=0, size=0, Q=0, Rn=0, opcode=1, L=0
    let encoding: u32 = 0x0C001000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=7 (midpoint (7))
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_6_0_0c007000() {
    // Encoding: 0x0C007000
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=7, size=0, Rn=0, Rt=0
    // Fields: L=0, Rn=0, opcode=7, Q=0, size=0, Rt=0
    let encoding: u32 = 0x0C007000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=15 (maximum value (15))
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_7_0_0c00f000() {
    // Encoding: 0x0C00F000
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=15, size=0, Rn=0, Rt=0
    // Fields: size=0, Rt=0, opcode=15, Rn=0, L=0, Q=0
    let encoding: u32 = 0x0C00F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_8_0_0c000000() {
    // Encoding: 0x0C000000
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: size=0, Rn=0, opcode=0, Rt=0, Q=0, L=0
    let encoding: u32 = 0x0C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_9_0_0c000400() {
    // Encoding: 0x0C000400
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=1, Rn=0, Rt=0
    // Fields: Rt=0, Q=0, Rn=0, L=0, opcode=0, size=1
    let encoding: u32 = 0x0C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.4h, v1.4h, v2.4h, v3.4h }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_10_0_0c000800() {
    // Encoding: 0x0C000800
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=2, Rn=0, Rt=0
    // Fields: Rn=0, size=2, L=0, opcode=0, Q=0, Rt=0
    let encoding: u32 = 0x0C000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.2s, v1.2s, v2.2s, v3.2s }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_11_0_0c000c00() {
    // Encoding: 0x0C000C00
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=3, Rn=0, Rt=0
    // Fields: Q=0, L=0, opcode=0, size=3, Rn=0, Rt=0
    let encoding: u32 = 0x0C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_12_0_0c000000() {
    // Encoding: 0x0C000000
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: L=0, size=0, Rn=0, Rt=0, Q=0, opcode=0
    let encoding: u32 = 0x0C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_13_0_0c000020() {
    // Encoding: 0x0C000020
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=1, Rt=0
    // Fields: Rn=1, Rt=0, opcode=0, size=0, Q=0, L=0
    let encoding: u32 = 0x0C000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_14_0_0c0003c0() {
    // Encoding: 0x0C0003C0
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=30, Rt=0
    // Fields: Q=0, opcode=0, L=0, size=0, Rn=30, Rt=0
    let encoding: u32 = 0x0C0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x30]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_15_0_0c0003e0() {
    // Encoding: 0x0C0003E0
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=31, Rt=0
    // Fields: Q=0, opcode=0, size=0, Rn=31, Rt=0, L=0
    let encoding: u32 = 0x0C0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_16_0_0c000000() {
    // Encoding: 0x0C000000
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, size=0, Q=0, L=0, opcode=0
    let encoding: u32 = 0x0C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_17_0_0c000001() {
    // Encoding: 0x0C000001
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=0, Rt=1
    // Fields: Rt=1, opcode=0, Q=0, size=0, Rn=0, L=0
    let encoding: u32 = 0x0C000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v1.8b, v2.8b, v3.8b, v4.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_18_0_0c00001e() {
    // Encoding: 0x0C00001E
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=0, Rt=30
    // Fields: Q=0, Rn=0, L=0, Rt=30, size=0, opcode=0
    let encoding: u32 = 0x0C00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v30.8b, v31.8b, v0.8b, v1.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_19_0_0c00001f() {
    // Encoding: 0x0C00001F
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=0, Rt=31
    // Fields: Rn=0, Q=0, L=0, Rt=31, opcode=0, size=0
    let encoding: u32 = 0x0C00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v31.8b, v0.8b, v1.8b, v2.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_20_0_0c000021() {
    // Encoding: 0x0C000021
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=1, Rt=1
    // Fields: Rt=1, Q=0, Rn=1, opcode=0, L=0, size=0
    let encoding: u32 = 0x0C000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v1.8b, v2.8b, v3.8b, v4.8b }, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_combo_21_0_0c0003ff() {
    // Encoding: 0x0C0003FF
    // Test aarch64_memory_vector_multiple_no_wb field combination: Q=0, L=0, opcode=0, size=0, Rn=31, Rt=31
    // Fields: L=0, Q=0, opcode=0, Rn=31, size=0, Rt=31
    let encoding: u32 = 0x0C0003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v31.8b, v0.8b, v1.8b, v2.8b }, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_special_q_0_size_variant_0_0_0c000400() {
    // Encoding: 0x0C000400
    // Test aarch64_memory_vector_multiple_no_wb special value Q = 0 (Size variant 0)
    // Fields: Q=0, L=0, Rt=0, opcode=0, size=1, Rn=0
    let encoding: u32 = 0x0C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.4h, v1.4h, v2.4h, v3.4h }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_special_q_1_size_variant_1_0_4c000400() {
    // Encoding: 0x4C000400
    // Test aarch64_memory_vector_multiple_no_wb special value Q = 1 (Size variant 1)
    // Fields: Q=1, L=0, size=1, Rn=0, opcode=0, Rt=0
    let encoding: u32 = 0x4C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8h, v1.8h, v2.8h, v3.8h }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_special_size_0_size_variant_0_0_0c000000() {
    // Encoding: 0x0C000000
    // Test aarch64_memory_vector_multiple_no_wb special value size = 0 (Size variant 0)
    // Fields: L=0, size=0, Rt=0, Rn=0, Q=0, opcode=0
    let encoding: u32 = 0x0C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_special_size_1_size_variant_1_0_0c000400() {
    // Encoding: 0x0C000400
    // Test aarch64_memory_vector_multiple_no_wb special value size = 1 (Size variant 1)
    // Fields: Rn=0, opcode=0, Q=0, L=0, Rt=0, size=1
    let encoding: u32 = 0x0C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.4h, v1.4h, v2.4h, v3.4h }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_special_size_2_size_variant_2_0_0c000800() {
    // Encoding: 0x0C000800
    // Test aarch64_memory_vector_multiple_no_wb special value size = 2 (Size variant 2)
    // Fields: L=0, size=2, Q=0, opcode=0, Rt=0, Rn=0
    let encoding: u32 = 0x0C000800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.2s, v1.2s, v2.2s, v3.2s }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_special_size_3_size_variant_3_0_0c000c00() {
    // Encoding: 0x0C000C00
    // Test aarch64_memory_vector_multiple_no_wb special value size = 3 (Size variant 3)
    // Fields: Rt=0, Rn=0, opcode=0, Q=0, L=0, size=3
    let encoding: u32 = 0x0C000C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_special_rn_31_stack_pointer_sp_may_require_alignment_0_0c0007e0()
 {
    // Encoding: 0x0C0007E0
    // Test aarch64_memory_vector_multiple_no_wb special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Q=0, Rn=31, opcode=0, size=1, Rt=0, L=0
    let encoding: u32 = 0x0C0007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.4h, v1.4h, v2.4h, v3.4h }, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_no_wb
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_vector_multiple_no_wb_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_0c00041f()
 {
    // Encoding: 0x0C00041F
    // Test aarch64_memory_vector_multiple_no_wb special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, L=0, Q=0, opcode=0, size=1, Rt=31
    let encoding: u32 = 0x0C00041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v31.4h, v0.4h, v1.4h, v2.4h }, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_q_0_min_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field Q = 0 (Min)
    // Fields: Rm=0, size=0, Q=0, opcode=0, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_q_1_max_0_4c800000() {
    // Encoding: 0x4C800000
    // Test aarch64_memory_vector_multiple_post_inc field Q = 1 (Max)
    // Fields: Rt=0, L=0, Rn=0, opcode=0, size=0, Q=1, Rm=0
    let encoding: u32 = 0x4C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.16b, v1.16b, v2.16b, v3.16b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_l_0_min_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field L = 0 (Min)
    // Fields: Rm=0, Q=0, opcode=0, size=0, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_l_1_max_0_0cc00000() {
    // Encoding: 0x0CC00000
    // Test aarch64_memory_vector_multiple_post_inc field L = 1 (Max)
    // Fields: L=1, size=0, opcode=0, Rm=0, Q=0, Rn=0, Rt=0
    let encoding: u32 = 0x0CC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ld4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_rm_0_min_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field Rm = 0 (Min)
    // Fields: size=0, Rn=0, L=0, Rt=0, Q=0, Rm=0, opcode=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_rm_1_poweroftwo_0_0c810000() {
    // Encoding: 0x0C810000
    // Test aarch64_memory_vector_multiple_post_inc field Rm = 1 (PowerOfTwo)
    // Fields: Rt=0, Q=0, L=0, Rm=1, opcode=0, size=0, Rn=0
    let encoding: u32 = 0x0C810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_rm_30_poweroftwominusone_0_0c9e0000() {
    // Encoding: 0x0C9E0000
    // Test aarch64_memory_vector_multiple_post_inc field Rm = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, size=0, Rn=0, Rt=0, L=0, opcode=0, Rm=30
    let encoding: u32 = 0x0C9E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rm 16 +: 5`
/// Requirement: FieldBoundary { field: "Rm", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_rm_31_max_0_0c9f0000() {
    // Encoding: 0x0C9F0000
    // Test aarch64_memory_vector_multiple_post_inc field Rm = 31 (Max)
    // Fields: size=0, Rn=0, Rt=0, opcode=0, Rm=31, Q=0, L=0
    let encoding: u32 = 0x0C9F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], #32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field opcode 12 +: 4`
/// Requirement: FieldBoundary { field: "opcode", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_opcode_0_min_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field opcode = 0 (Min)
    // Fields: L=0, Rm=0, Rn=0, Q=0, opcode=0, Rt=0, size=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field opcode 12 +: 4`
/// Requirement: FieldBoundary { field: "opcode", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_opcode_1_poweroftwo_0_0c801000() {
    // Encoding: 0x0C801000
    // Test aarch64_memory_vector_multiple_post_inc field opcode = 1 (PowerOfTwo)
    // Fields: Q=0, L=0, opcode=1, Rn=0, size=0, Rt=0, Rm=0
    let encoding: u32 = 0x0C801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field opcode 12 +: 4`
/// Requirement: FieldBoundary { field: "opcode", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_opcode_7_poweroftwominusone_0_0c807000() {
    // Encoding: 0x0C807000
    // Test aarch64_memory_vector_multiple_post_inc field opcode = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, L=0, Q=0, Rt=0, Rm=0, size=0, opcode=7
    let encoding: u32 = 0x0C807000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field opcode 12 +: 4`
/// Requirement: FieldBoundary { field: "opcode", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_opcode_15_max_0_0c80f000() {
    // Encoding: 0x0C80F000
    // Test aarch64_memory_vector_multiple_post_inc field opcode = 15 (Max)
    // Fields: Q=0, Rm=0, size=0, L=0, opcode=15, Rt=0, Rn=0
    let encoding: u32 = 0x0C80F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_size_0_min_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field size = 0 (Min)
    // Fields: opcode=0, L=0, Rn=0, size=0, Rt=0, Rm=0, Q=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_size_1_poweroftwo_0_0c800400() {
    // Encoding: 0x0C800400
    // Test aarch64_memory_vector_multiple_post_inc field size = 1 (PowerOfTwo)
    // Fields: L=0, Rm=0, size=1, Rt=0, Rn=0, Q=0, opcode=0
    let encoding: u32 = 0x0C800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.4h, v1.4h, v2.4h, v3.4h }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_size_2_poweroftwo_0_0c800800() {
    // Encoding: 0x0C800800
    // Test aarch64_memory_vector_multiple_post_inc field size = 2 (PowerOfTwo)
    // Fields: Rt=0, opcode=0, size=2, Q=0, Rm=0, Rn=0, L=0
    let encoding: u32 = 0x0C800800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.2s, v1.2s, v2.2s, v3.2s }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field size 10 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_size_3_max_0_0c800c00() {
    // Encoding: 0x0C800C00
    // Test aarch64_memory_vector_multiple_post_inc field size = 3 (Max)
    // Fields: Rn=0, opcode=0, Rm=0, size=3, L=0, Q=0, Rt=0
    let encoding: u32 = 0x0C800C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_rn_0_min_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field Rn = 0 (Min)
    // Fields: opcode=0, Rn=0, L=0, Rt=0, Rm=0, size=0, Q=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_rn_1_poweroftwo_0_0c800020() {
    // Encoding: 0x0C800020
    // Test aarch64_memory_vector_multiple_post_inc field Rn = 1 (PowerOfTwo)
    // Fields: opcode=0, Rn=1, Rt=0, size=0, Q=0, L=0, Rm=0
    let encoding: u32 = 0x0C800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x1], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_rn_30_poweroftwominusone_0_0c8003c0() {
    // Encoding: 0x0C8003C0
    // Test aarch64_memory_vector_multiple_post_inc field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Q=0, Rm=0, size=0, L=0, Rt=0, opcode=0, Rn=30
    let encoding: u32 = 0x0C8003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x30], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_rn_31_max_0_0c8003e0() {
    // Encoding: 0x0C8003E0
    // Test aarch64_memory_vector_multiple_post_inc field Rn = 31 (Max)
    // Fields: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=31, Rt=0
    let encoding: u32 = 0x0C8003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [sp], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_rt_0_min_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field Rt = 0 (Min)
    // Fields: L=0, Q=0, Rm=0, size=0, Rn=0, opcode=0, Rt=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_rt_1_poweroftwo_0_0c800001() {
    // Encoding: 0x0C800001
    // Test aarch64_memory_vector_multiple_post_inc field Rt = 1 (PowerOfTwo)
    // Fields: Rt=1, size=0, Rn=0, Q=0, Rm=0, L=0, opcode=0
    let encoding: u32 = 0x0C800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v1.8b, v2.8b, v3.8b, v4.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_rt_30_poweroftwominusone_0_0c80001e() {
    // Encoding: 0x0C80001E
    // Test aarch64_memory_vector_multiple_post_inc field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=30, Rm=0, Q=0, L=0, opcode=0, size=0
    let encoding: u32 = 0x0C80001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v30.8b, v31.8b, v0.8b, v1.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_field_rt_31_max_0_0c80001f() {
    // Encoding: 0x0C80001F
    // Test aarch64_memory_vector_multiple_post_inc field Rt = 31 (Max)
    // Fields: Rm=0, opcode=0, size=0, Rn=0, Rt=31, Q=0, L=0
    let encoding: u32 = 0x0C80001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v31.8b, v0.8b, v1.8b, v2.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_0_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: Rm=0, L=0, Q=0, opcode=0, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_1_0_4c800000() {
    // Encoding: 0x4C800000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=1, L=0, Rm=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: opcode=0, size=0, Rn=0, Rm=0, Rt=0, Q=1, L=0
    let encoding: u32 = 0x4C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.16b, v1.16b, v2.16b, v3.16b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_2_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: Rn=0, L=0, Rm=0, Rt=0, opcode=0, Q=0, size=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_3_0_0cc00000() {
    // Encoding: 0x0CC00000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=1, Rm=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: Rt=0, L=1, Rn=0, Rm=0, Q=0, size=0, opcode=0
    let encoding: u32 = 0x0CC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ld4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_4_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: size=0, L=0, Rm=0, opcode=0, Rn=0, Rt=0, Q=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_5_0_0c810000() {
    // Encoding: 0x0C810000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=1, opcode=0, size=0, Rn=0, Rt=0
    // Fields: L=0, size=0, Q=0, Rn=0, opcode=0, Rt=0, Rm=1
    let encoding: u32 = 0x0C810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_6_0_0c9e0000() {
    // Encoding: 0x0C9E0000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=30, opcode=0, size=0, Rn=0, Rt=0
    // Fields: L=0, size=0, Q=0, Rn=0, Rt=0, Rm=30, opcode=0
    let encoding: u32 = 0x0C9E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x30
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_7_0_0c9f0000() {
    // Encoding: 0x0C9F0000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=31, opcode=0, size=0, Rn=0, Rt=0
    // Fields: size=0, Rt=0, Q=0, opcode=0, L=0, Rn=0, Rm=31
    let encoding: u32 = 0x0C9F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], #32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=0 (minimum value)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_8_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: L=0, opcode=0, size=0, Rn=0, Rt=0, Rm=0, Q=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=1 (value 1)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_9_0_0c801000() {
    // Encoding: 0x0C801000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=1, size=0, Rn=0, Rt=0
    // Fields: Rm=0, opcode=1, Q=0, L=0, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x0C801000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=7 (midpoint (7))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_10_0_0c807000() {
    // Encoding: 0x0C807000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=7, size=0, Rn=0, Rt=0
    // Fields: Rm=0, L=0, opcode=7, Rt=0, Q=0, size=0, Rn=0
    let encoding: u32 = 0x0C807000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st1	{ v0.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opcode=15 (maximum value (15))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_11_0_0c80f000() {
    // Encoding: 0x0C80F000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=15, size=0, Rn=0, Rt=0
    // Fields: Rm=0, size=0, Rt=0, L=0, Q=0, Rn=0, opcode=15
    let encoding: u32 = 0x0C80F000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_12_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: Rt=0, Rm=0, Rn=0, opcode=0, Q=0, size=0, L=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_13_0_0c800400() {
    // Encoding: 0x0C800400
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=1, Rn=0, Rt=0
    // Fields: opcode=0, L=0, Rm=0, Rn=0, Rt=0, size=1, Q=0
    let encoding: u32 = 0x0C800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.4h, v1.4h, v2.4h, v3.4h }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_14_0_0c800800() {
    // Encoding: 0x0C800800
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=2, Rn=0, Rt=0
    // Fields: size=2, Rt=0, Rm=0, Q=0, L=0, Rn=0, opcode=0
    let encoding: u32 = 0x0C800800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.2s, v1.2s, v2.2s, v3.2s }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_15_0_0c800c00() {
    // Encoding: 0x0C800C00
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=3, Rn=0, Rt=0
    // Fields: Q=0, Rn=0, L=0, opcode=0, size=3, Rm=0, Rt=0
    let encoding: u32 = 0x0C800C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_16_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: Rt=0, size=0, Rn=0, Q=0, L=0, Rm=0, opcode=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_17_0_0c800020() {
    // Encoding: 0x0C800020
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=1, Rt=0
    // Fields: size=0, Rn=1, Q=0, Rt=0, Rm=0, opcode=0, L=0
    let encoding: u32 = 0x0C800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x1], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_18_0_0c8003c0() {
    // Encoding: 0x0C8003C0
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=30, Rt=0
    // Fields: Rn=30, size=0, Q=0, L=0, Rm=0, opcode=0, Rt=0
    let encoding: u32 = 0x0C8003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x30], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_19_0_0c8003e0() {
    // Encoding: 0x0C8003E0
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=31, Rt=0
    // Fields: size=0, Rn=31, opcode=0, Rm=0, Rt=0, L=0, Q=0
    let encoding: u32 = 0x0C8003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [sp], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_20_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=0, Rt=0
    // Fields: Q=0, L=0, size=0, Rt=0, Rm=0, opcode=0, Rn=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_21_0_0c800001() {
    // Encoding: 0x0C800001
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=0, Rt=1
    // Fields: opcode=0, Rt=1, L=0, Q=0, Rm=0, size=0, Rn=0
    let encoding: u32 = 0x0C800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v1.8b, v2.8b, v3.8b, v4.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_22_0_0c80001e() {
    // Encoding: 0x0C80001E
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=0, Rt=30
    // Fields: Q=0, size=0, Rn=0, Rt=30, Rm=0, opcode=0, L=0
    let encoding: u32 = 0x0C80001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v30.8b, v31.8b, v0.8b, v1.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_23_0_0c80001f() {
    // Encoding: 0x0C80001F
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=0, Rt=31
    // Fields: Rt=31, Q=0, Rm=0, Rn=0, L=0, opcode=0, size=0
    let encoding: u32 = 0x0C80001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v31.8b, v0.8b, v1.8b, v2.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_24_0_0c810020() {
    // Encoding: 0x0C810020
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=1, opcode=0, size=0, Rn=1, Rt=0
    // Fields: Rn=1, Rt=0, L=0, Q=0, Rm=1, opcode=0, size=0
    let encoding: u32 = 0x0C810020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x1], x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_25_0_0c9f03e0() {
    // Encoding: 0x0C9F03E0
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=31, opcode=0, size=0, Rn=31, Rt=0
    // Fields: opcode=0, size=0, Rt=0, Rn=31, Q=0, L=0, Rm=31
    let encoding: u32 = 0x0C9F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [sp], #32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_26_0_0c810001() {
    // Encoding: 0x0C810001
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=1, opcode=0, size=0, Rn=0, Rt=1
    // Fields: size=0, Rm=1, Rt=1, Rn=0, L=0, Q=0, opcode=0
    let encoding: u32 = 0x0C810001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v1.8b, v2.8b, v3.8b, v4.8b }, [x0], x1
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rm=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_27_0_0c9f001f() {
    // Encoding: 0x0C9F001F
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=31, opcode=0, size=0, Rn=0, Rt=31
    // Fields: Rn=0, Rt=31, L=0, Q=0, Rm=31, size=0, opcode=0
    let encoding: u32 = 0x0C9F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v31.8b, v0.8b, v1.8b, v2.8b }, [x0], #32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_28_0_0c800021() {
    // Encoding: 0x0C800021
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=1, Rt=1
    // Fields: Rm=0, Q=0, opcode=0, size=0, Rn=1, L=0, Rt=1
    let encoding: u32 = 0x0C800021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v1.8b, v2.8b, v3.8b, v4.8b }, [x1], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_combo_29_0_0c8003ff() {
    // Encoding: 0x0C8003FF
    // Test aarch64_memory_vector_multiple_post_inc field combination: Q=0, L=0, Rm=0, opcode=0, size=0, Rn=31, Rt=31
    // Fields: Q=0, opcode=0, size=0, Rn=31, Rt=31, L=0, Rm=0
    let encoding: u32 = 0x0C8003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v31.8b, v0.8b, v1.8b, v2.8b }, [sp], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_special_q_0_size_variant_0_0_0c800400() {
    // Encoding: 0x0C800400
    // Test aarch64_memory_vector_multiple_post_inc special value Q = 0 (Size variant 0)
    // Fields: opcode=0, L=0, Q=0, size=1, Rn=0, Rt=0, Rm=0
    let encoding: u32 = 0x0C800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.4h, v1.4h, v2.4h, v3.4h }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_special_q_1_size_variant_1_0_4c800400() {
    // Encoding: 0x4C800400
    // Test aarch64_memory_vector_multiple_post_inc special value Q = 1 (Size variant 1)
    // Fields: opcode=0, size=1, Rn=0, Rt=0, L=0, Q=1, Rm=0
    let encoding: u32 = 0x4C800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8h, v1.8h, v2.8h, v3.8h }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_special_size_0_size_variant_0_0_0c800000() {
    // Encoding: 0x0C800000
    // Test aarch64_memory_vector_multiple_post_inc special value size = 0 (Size variant 0)
    // Fields: size=0, opcode=0, Rn=0, Rt=0, L=0, Q=0, Rm=0
    let encoding: u32 = 0x0C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.8b, v1.8b, v2.8b, v3.8b }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_special_size_1_size_variant_1_0_0c800400() {
    // Encoding: 0x0C800400
    // Test aarch64_memory_vector_multiple_post_inc special value size = 1 (Size variant 1)
    // Fields: opcode=0, Q=0, L=0, size=1, Rn=0, Rt=0, Rm=0
    let encoding: u32 = 0x0C800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.4h, v1.4h, v2.4h, v3.4h }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_special_size_2_size_variant_2_0_0c800800() {
    // Encoding: 0x0C800800
    // Test aarch64_memory_vector_multiple_post_inc special value size = 2 (Size variant 2)
    // Fields: Rn=0, L=0, Rt=0, Rm=0, opcode=0, size=2, Q=0
    let encoding: u32 = 0x0C800800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.2s, v1.2s, v2.2s, v3.2s }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_special_size_3_size_variant_3_0_0c800c00() {
    // Encoding: 0x0C800C00
    // Test aarch64_memory_vector_multiple_post_inc special value size = 3 (Size variant 3)
    // Fields: size=3, Rt=0, Rm=0, L=0, Q=0, opcode=0, Rn=0
    let encoding: u32 = 0x0C800C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_special_rn_31_stack_pointer_sp_may_require_alignment_0_0c8007e0()
 {
    // Encoding: 0x0C8007E0
    // Test aarch64_memory_vector_multiple_post_inc special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, Rm=0, opcode=0, Rn=31, Q=0, Rt=0, L=0
    let encoding: u32 = 0x0C8007E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v0.4h, v1.4h, v2.4h, v3.4h }, [sp], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_vector_multiple_post_inc
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_vector_multiple_post_inc_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_0c80041f()
 {
    // Encoding: 0x0C80041F
    // Test aarch64_memory_vector_multiple_post_inc special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rm=0, Rn=0, L=0, size=1, opcode=0, Q=0, Rt=31
    let encoding: u32 = 0x0C80041F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: st4	{ v31.4h, v0.4h, v1.4h, v2.4h }, [x0], x0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}
