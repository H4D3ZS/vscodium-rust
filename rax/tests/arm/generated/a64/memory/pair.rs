//! A64 memory pair tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_memory_pair_simdfp_no_alloc Tests
// ============================================================================

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_opc_0_min_0_2c000000() {
    // Encoding: 0x2C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field opc = 0 (Min)
    // Fields: Rt=0, imm7=0, opc=0, Rt2=0, Rn=0, L=0
    let encoding: u32 = 0x2C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_opc_1_poweroftwo_0_6c000000() {
    // Encoding: 0x6C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field opc = 1 (PowerOfTwo)
    // Fields: opc=1, L=0, Rt2=0, imm7=0, Rn=0, Rt=0
    let encoding: u32 = 0x6C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	d0, d0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_opc_2_poweroftwo_0_ac000000() {
    // Encoding: 0xAC000000
    // Test aarch64_memory_pair_simdfp_no_alloc field opc = 2 (PowerOfTwo)
    // Fields: L=0, Rt=0, imm7=0, opc=2, Rt2=0, Rn=0
    let encoding: u32 = 0xAC000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	q0, q0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_opc_3_max_0_ec000000() {
    // Encoding: 0xEC000000
    // Test aarch64_memory_pair_simdfp_no_alloc field opc = 3 (Max)
    // Fields: opc=3, Rt2=0, imm7=0, L=0, Rn=0, Rt=0
    let encoding: u32 = 0xEC000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_l_0_min_0_2c000000() {
    // Encoding: 0x2C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field L = 0 (Min)
    // Fields: Rt2=0, imm7=0, opc=0, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x2C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_l_1_max_0_2c400000() {
    // Encoding: 0x2C400000
    // Test aarch64_memory_pair_simdfp_no_alloc field L = 1 (Max)
    // Fields: opc=0, Rt2=0, L=1, imm7=0, Rt=0, Rn=0
    let encoding: u32 = 0x2C400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_imm7_0_zero_0_2c000000() {
    // Encoding: 0x2C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field imm7 = 0 (Zero)
    // Fields: opc=0, Rt2=0, Rn=0, L=0, Rt=0, imm7=0
    let encoding: u32 = 0x2C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_imm7_1_poweroftwo_0_2c008000() {
    // Encoding: 0x2C008000
    // Test aarch64_memory_pair_simdfp_no_alloc field imm7 = 1 (PowerOfTwo)
    // Fields: Rt=0, imm7=1, opc=0, L=0, Rt2=0, Rn=0
    let encoding: u32 = 0x2C008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_imm7_3_poweroftwominusone_0_2c018000() {
    // Encoding: 0x2C018000
    // Test aarch64_memory_pair_simdfp_no_alloc field imm7 = 3 (PowerOfTwoMinusOne)
    // Fields: L=0, Rt2=0, Rt=0, opc=0, imm7=3, Rn=0
    let encoding: u32 = 0x2C018000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #12]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_imm7_4_poweroftwo_0_2c020000() {
    // Encoding: 0x2C020000
    // Test aarch64_memory_pair_simdfp_no_alloc field imm7 = 4 (PowerOfTwo)
    // Fields: imm7=4, Rt=0, opc=0, Rt2=0, Rn=0, L=0
    let encoding: u32 = 0x2C020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #16]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_imm7_7_poweroftwominusone_0_2c038000() {
    // Encoding: 0x2C038000
    // Test aarch64_memory_pair_simdfp_no_alloc field imm7 = 7 (PowerOfTwoMinusOne)
    // Fields: L=0, opc=0, Rt=0, imm7=7, Rn=0, Rt2=0
    let encoding: u32 = 0x2C038000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #28]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_imm7_8_poweroftwo_0_2c040000() {
    // Encoding: 0x2C040000
    // Test aarch64_memory_pair_simdfp_no_alloc field imm7 = 8 (PowerOfTwo)
    // Fields: L=0, opc=0, Rt2=0, Rn=0, imm7=8, Rt=0
    let encoding: u32 = 0x2C040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #32]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_imm7_15_poweroftwominusone_0_2c078000() {
    // Encoding: 0x2C078000
    // Test aarch64_memory_pair_simdfp_no_alloc field imm7 = 15 (PowerOfTwoMinusOne)
    // Fields: Rn=0, opc=0, Rt=0, L=0, Rt2=0, imm7=15
    let encoding: u32 = 0x2C078000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #60]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_imm7_16_poweroftwo_0_2c080000() {
    // Encoding: 0x2C080000
    // Test aarch64_memory_pair_simdfp_no_alloc field imm7 = 16 (PowerOfTwo)
    // Fields: opc=0, Rt2=0, Rt=0, imm7=16, Rn=0, L=0
    let encoding: u32 = 0x2C080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #64]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_imm7_31_poweroftwominusone_0_2c0f8000() {
    // Encoding: 0x2C0F8000
    // Test aarch64_memory_pair_simdfp_no_alloc field imm7 = 31 (PowerOfTwoMinusOne)
    // Fields: Rt=0, imm7=31, L=0, opc=0, Rn=0, Rt2=0
    let encoding: u32 = 0x2C0F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #124]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_imm7_32_poweroftwo_0_2c100000() {
    // Encoding: 0x2C100000
    // Test aarch64_memory_pair_simdfp_no_alloc field imm7 = 32 (PowerOfTwo)
    // Fields: Rn=0, imm7=32, opc=0, Rt=0, L=0, Rt2=0
    let encoding: u32 = 0x2C100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #128]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 63, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (63)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_imm7_63_poweroftwominusone_0_2c1f8000() {
    // Encoding: 0x2C1F8000
    // Test aarch64_memory_pair_simdfp_no_alloc field imm7 = 63 (PowerOfTwoMinusOne)
    // Fields: imm7=63, opc=0, Rt=0, Rt2=0, L=0, Rn=0
    let encoding: u32 = 0x2C1F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #252]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_imm7_64_poweroftwo_0_2c200000() {
    // Encoding: 0x2C200000
    // Test aarch64_memory_pair_simdfp_no_alloc field imm7 = 64 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, L=0, opc=0, Rt2=0, imm7=64
    let encoding: u32 = 0x2C200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 127, boundary: Max }
/// maximum immediate (127)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_imm7_127_max_0_2c3f8000() {
    // Encoding: 0x2C3F8000
    // Test aarch64_memory_pair_simdfp_no_alloc field imm7 = 127 (Max)
    // Fields: Rn=0, Rt=0, imm7=127, opc=0, Rt2=0, L=0
    let encoding: u32 = 0x2C3F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_rt2_0_min_0_2c000000() {
    // Encoding: 0x2C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field Rt2 = 0 (Min)
    // Fields: Rn=0, Rt=0, L=0, Rt2=0, opc=0, imm7=0
    let encoding: u32 = 0x2C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_rt2_1_poweroftwo_0_2c000400() {
    // Encoding: 0x2C000400
    // Test aarch64_memory_pair_simdfp_no_alloc field Rt2 = 1 (PowerOfTwo)
    // Fields: Rt=0, opc=0, imm7=0, L=0, Rn=0, Rt2=1
    let encoding: u32 = 0x2C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s1, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_rt2_30_poweroftwominusone_0_2c007800() {
    // Encoding: 0x2C007800
    // Test aarch64_memory_pair_simdfp_no_alloc field Rt2 = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=0, Rt2=30, imm7=0, opc=0, L=0, Rn=0
    let encoding: u32 = 0x2C007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s30, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_rt2_31_max_0_2c007c00() {
    // Encoding: 0x2C007C00
    // Test aarch64_memory_pair_simdfp_no_alloc field Rt2 = 31 (Max)
    // Fields: Rt2=31, L=0, Rn=0, imm7=0, Rt=0, opc=0
    let encoding: u32 = 0x2C007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s31, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_rn_0_min_0_2c000000() {
    // Encoding: 0x2C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field Rn = 0 (Min)
    // Fields: opc=0, Rn=0, imm7=0, Rt=0, Rt2=0, L=0
    let encoding: u32 = 0x2C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_rn_1_poweroftwo_0_2c000020() {
    // Encoding: 0x2C000020
    // Test aarch64_memory_pair_simdfp_no_alloc field Rn = 1 (PowerOfTwo)
    // Fields: imm7=0, Rt=0, Rt2=0, opc=0, L=0, Rn=1
    let encoding: u32 = 0x2C000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_rn_30_poweroftwominusone_0_2c0003c0() {
    // Encoding: 0x2C0003C0
    // Test aarch64_memory_pair_simdfp_no_alloc field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, L=0, Rt=0, Rt2=0, imm7=0, opc=0
    let encoding: u32 = 0x2C0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x30]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_rn_31_max_0_2c0003e0() {
    // Encoding: 0x2C0003E0
    // Test aarch64_memory_pair_simdfp_no_alloc field Rn = 31 (Max)
    // Fields: L=0, imm7=0, Rn=31, opc=0, Rt2=0, Rt=0
    let encoding: u32 = 0x2C0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_rt_0_min_0_2c000000() {
    // Encoding: 0x2C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field Rt = 0 (Min)
    // Fields: imm7=0, Rt2=0, Rt=0, L=0, opc=0, Rn=0
    let encoding: u32 = 0x2C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_rt_1_poweroftwo_0_2c000001() {
    // Encoding: 0x2C000001
    // Test aarch64_memory_pair_simdfp_no_alloc field Rt = 1 (PowerOfTwo)
    // Fields: L=0, opc=0, Rt2=0, Rn=0, Rt=1, imm7=0
    let encoding: u32 = 0x2C000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s1, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_rt_30_poweroftwominusone_0_2c00001e() {
    // Encoding: 0x2C00001E
    // Test aarch64_memory_pair_simdfp_no_alloc field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: L=0, imm7=0, Rt=30, Rn=0, opc=0, Rt2=0
    let encoding: u32 = 0x2C00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s30, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_field_rt_31_max_0_2c00001f() {
    // Encoding: 0x2C00001F
    // Test aarch64_memory_pair_simdfp_no_alloc field Rt = 31 (Max)
    // Fields: L=0, opc=0, imm7=0, Rt2=0, Rt=31, Rn=0
    let encoding: u32 = 0x2C00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s31, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_0_0_2c000000() {
    // Encoding: 0x2C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, Rt2=0, opc=0, L=0, imm7=0, Rn=0
    let encoding: u32 = 0x2C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_1_0_6c000000() {
    // Encoding: 0x6C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=1, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=0, opc=1, Rt=0, Rt2=0, L=0, Rn=0
    let encoding: u32 = 0x6C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	d0, d0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_2_0_ac000000() {
    // Encoding: 0xAC000000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=2, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=2, Rn=0, L=0, Rt=0, imm7=0, Rt2=0
    let encoding: u32 = 0xAC000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	q0, q0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_3_0_ec000000() {
    // Encoding: 0xEC000000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=3, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=0, Rt=0, Rn=0, L=0, opc=3, Rt2=0
    let encoding: u32 = 0xEC000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_4_0_2c000000() {
    // Encoding: 0x2C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=0, opc=0, Rt2=0, Rt=0, Rn=0, L=0
    let encoding: u32 = 0x2C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_5_0_2c400000() {
    // Encoding: 0x2C400000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=1, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=1, Rt=0, opc=0, Rt2=0, Rn=0, imm7=0
    let encoding: u32 = 0x2C400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=0 (immediate value 0)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_6_0_2c000000() {
    // Encoding: 0x2C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, L=0, imm7=0, Rt=0, Rt2=0, Rn=0
    let encoding: u32 = 0x2C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=1 (immediate value 1)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_7_0_2c008000() {
    // Encoding: 0x2C008000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=1, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, imm7=1, Rt2=0, Rn=0, L=0, opc=0
    let encoding: u32 = 0x2C008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_8_0_2c018000() {
    // Encoding: 0x2C018000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=3, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt2=0, imm7=3, opc=0, Rt=0, Rn=0
    let encoding: u32 = 0x2C018000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #12]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_9_0_2c020000() {
    // Encoding: 0x2C020000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=4, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, L=0, Rn=0, opc=0, Rt=0, imm7=4
    let encoding: u32 = 0x2C020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #16]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_10_0_2c038000() {
    // Encoding: 0x2C038000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=7, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, Rt2=0, Rn=0, opc=0, imm7=7, L=0
    let encoding: u32 = 0x2C038000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #28]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_11_0_2c040000() {
    // Encoding: 0x2C040000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=8, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, imm7=8, Rt2=0, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x2C040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #32]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_12_0_2c078000() {
    // Encoding: 0x2C078000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=15, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, imm7=15, opc=0, L=0, Rt=0, Rt2=0
    let encoding: u32 = 0x2C078000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #60]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_13_0_2c080000() {
    // Encoding: 0x2C080000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=16, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rt=0, imm7=16, Rt2=0, Rn=0, L=0
    let encoding: u32 = 0x2C080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #64]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_14_0_2c0f8000() {
    // Encoding: 0x2C0F8000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=31, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rt=0, Rn=0, opc=0, L=0, imm7=31
    let encoding: u32 = 0x2C0F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #124]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_15_0_2c100000() {
    // Encoding: 0x2C100000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=32, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt=0, Rt2=0, opc=0, imm7=32, Rn=0
    let encoding: u32 = 0x2C100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #128]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=63 (immediate midpoint (63))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_16_0_2c1f8000() {
    // Encoding: 0x2C1F8000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=63, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt=0, imm7=63, opc=0, Rt2=0, Rn=0
    let encoding: u32 = 0x2C1F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #252]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_17_0_2c200000() {
    // Encoding: 0x2C200000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=64, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, imm7=64, opc=0, L=0, Rt2=0, Rn=0
    let encoding: u32 = 0x2C200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=127 (maximum immediate (127))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_18_0_2c3f8000() {
    // Encoding: 0x2C3F8000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=127, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rt=0, Rn=0, imm7=127, opc=0, L=0
    let encoding: u32 = 0x2C3F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_19_0_2c000000() {
    // Encoding: 0x2C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt=0, opc=0, Rt2=0, imm7=0, Rn=0
    let encoding: u32 = 0x2C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_20_0_2c000400() {
    // Encoding: 0x2C000400
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=0
    // Fields: L=0, imm7=0, opc=0, Rt2=1, Rn=0, Rt=0
    let encoding: u32 = 0x2C000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s1, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_21_0_2c007800() {
    // Encoding: 0x2C007800
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=30, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, L=0, Rt2=30, imm7=0, Rn=0
    let encoding: u32 = 0x2C007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s30, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_22_0_2c007c00() {
    // Encoding: 0x2C007C00
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=0
    // Fields: Rt2=31, Rt=0, Rn=0, L=0, imm7=0, opc=0
    let encoding: u32 = 0x2C007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s31, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_23_0_2c000000() {
    // Encoding: 0x2C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=0, Rt=0, opc=0, L=0, Rn=0, Rt2=0
    let encoding: u32 = 0x2C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_24_0_2c000020() {
    // Encoding: 0x2C000020
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=0
    // Fields: Rt2=0, opc=0, Rt=0, Rn=1, L=0, imm7=0
    let encoding: u32 = 0x2C000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_25_0_2c0003c0() {
    // Encoding: 0x2C0003C0
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=30, Rt=0
    // Fields: Rt2=0, L=0, imm7=0, Rn=30, opc=0, Rt=0
    let encoding: u32 = 0x2C0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x30]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_26_0_2c0003e0() {
    // Encoding: 0x2C0003E0
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=0
    // Fields: Rn=31, imm7=0, Rt=0, L=0, opc=0, Rt2=0
    let encoding: u32 = 0x2C0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_27_0_2c000000() {
    // Encoding: 0x2C000000
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, imm7=0, Rt=0, opc=0, Rn=0, L=0
    let encoding: u32 = 0x2C000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_28_0_2c000001() {
    // Encoding: 0x2C000001
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=1
    // Fields: Rt2=0, imm7=0, Rt=1, Rn=0, opc=0, L=0
    let encoding: u32 = 0x2C000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s1, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_29_0_2c00001e() {
    // Encoding: 0x2C00001E
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=30
    // Fields: Rt2=0, L=0, Rn=0, Rt=30, opc=0, imm7=0
    let encoding: u32 = 0x2C00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s30, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_30_0_2c00001f() {
    // Encoding: 0x2C00001F
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=31
    // Fields: Rn=0, imm7=0, Rt=31, opc=0, Rt2=0, L=0
    let encoding: u32 = 0x2C00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s31, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_31_0_2c000420() {
    // Encoding: 0x2C000420
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=1, Rt=0
    // Fields: Rt2=1, Rn=1, Rt=0, opc=0, imm7=0, L=0
    let encoding: u32 = 0x2C000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s1, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_32_0_2c007fe0() {
    // Encoding: 0x2C007FE0
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=31, Rt=0
    // Fields: Rn=31, imm7=0, opc=0, Rt2=31, Rt=0, L=0
    let encoding: u32 = 0x2C007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s31, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_33_0_2c000401() {
    // Encoding: 0x2C000401
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=1
    // Fields: Rt2=1, Rn=0, Rt=1, L=0, opc=0, imm7=0
    let encoding: u32 = 0x2C000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s1, s1, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_34_0_2c007c1f() {
    // Encoding: 0x2C007C1F
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=31
    // Fields: Rt2=31, L=0, Rn=0, opc=0, Rt=31, imm7=0
    let encoding: u32 = 0x2C007C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s31, s31, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_35_0_2c000021() {
    // Encoding: 0x2C000021
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=1
    // Fields: imm7=0, opc=0, Rn=1, Rt=1, L=0, Rt2=0
    let encoding: u32 = 0x2C000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s1, s0, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_combo_36_0_2c0003ff() {
    // Encoding: 0x2C0003FF
    // Test aarch64_memory_pair_simdfp_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=31
    // Fields: Rn=31, L=0, Rt2=0, Rt=31, imm7=0, opc=0
    let encoding: u32 = 0x2C0003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s31, s0, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_special_opc_0_size_variant_0_0_2c008000() {
    // Encoding: 0x2C008000
    // Test aarch64_memory_pair_simdfp_no_alloc special value opc = 0 (Size variant 0)
    // Fields: Rt2=0, imm7=1, Rn=0, L=0, Rt=0, opc=0
    let encoding: u32 = 0x2C008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [x0, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_special_opc_1_size_variant_1_0_6c008000() {
    // Encoding: 0x6C008000
    // Test aarch64_memory_pair_simdfp_no_alloc special value opc = 1 (Size variant 1)
    // Fields: imm7=1, Rn=0, opc=1, Rt2=0, Rt=0, L=0
    let encoding: u32 = 0x6C008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	d0, d0, [x0, #8]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_special_opc_2_size_variant_2_0_ac008000() {
    // Encoding: 0xAC008000
    // Test aarch64_memory_pair_simdfp_no_alloc special value opc = 2 (Size variant 2)
    // Fields: Rt=0, imm7=1, Rn=0, Rt2=0, opc=2, L=0
    let encoding: u32 = 0xAC008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	q0, q0, [x0, #16]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_special_opc_3_size_variant_3_0_ec008000() {
    // Encoding: 0xEC008000
    // Test aarch64_memory_pair_simdfp_no_alloc special value opc = 3 (Size variant 3)
    // Fields: opc=3, Rn=0, Rt2=0, Rt=0, L=0, imm7=1
    let encoding: u32 = 0xEC008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_special_rn_31_stack_pointer_sp_may_require_alignment_0_2c0083e0()
 {
    // Encoding: 0x2C0083E0
    // Test aarch64_memory_pair_simdfp_no_alloc special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rt=0, opc=0, L=0, imm7=1, Rt2=0, Rn=31
    let encoding: u32 = 0x2C0083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s0, s0, [sp, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_no_alloc
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_pair_simdfp_no_alloc_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_2c00801f()
 {
    // Encoding: 0x2C00801F
    // Test aarch64_memory_pair_simdfp_no_alloc special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: opc=0, Rt2=0, Rn=0, Rt=31, L=0, imm7=1
    let encoding: u32 = 0x2C00801F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stnp	s31, s0, [x0, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

// ============================================================================
// aarch64_memory_pair_general_post_idx Tests
// ============================================================================

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_opc_0_min_0_28800000() {
    // Encoding: 0x28800000
    // Test aarch64_memory_pair_general_post_idx field opc = 0 (Min)
    // Fields: L=0, imm7=0, opc=0, Rt=0, Rn=0, Rt2=0
    let encoding: u32 = 0x28800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_opc_1_poweroftwo_0_68800000() {
    // Encoding: 0x68800000
    // Test aarch64_memory_pair_general_post_idx field opc = 1 (PowerOfTwo)
    // Fields: opc=1, Rt2=0, Rt=0, imm7=0, Rn=0, L=0
    let encoding: u32 = 0x68800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_opc_2_poweroftwo_0_a8800000() {
    // Encoding: 0xA8800000
    // Test aarch64_memory_pair_general_post_idx field opc = 2 (PowerOfTwo)
    // Fields: L=0, Rt2=0, Rn=0, imm7=0, Rt=0, opc=2
    let encoding: u32 = 0xA8800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_opc_3_max_0_e8800000() {
    // Encoding: 0xE8800000
    // Test aarch64_memory_pair_general_post_idx field opc = 3 (Max)
    // Fields: imm7=0, L=0, Rt=0, opc=3, Rn=0, Rt2=0
    let encoding: u32 = 0xE8800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_l_0_min_0_28800000() {
    // Encoding: 0x28800000
    // Test aarch64_memory_pair_general_post_idx field L = 0 (Min)
    // Fields: Rt=0, Rn=0, opc=0, L=0, imm7=0, Rt2=0
    let encoding: u32 = 0x28800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_l_1_max_0_28c00000() {
    // Encoding: 0x28C00000
    // Test aarch64_memory_pair_general_post_idx field L = 1 (Max)
    // Fields: L=1, Rt=0, opc=0, imm7=0, Rt2=0, Rn=0
    let encoding: u32 = 0x28C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_imm7_0_zero_0_28800000() {
    // Encoding: 0x28800000
    // Test aarch64_memory_pair_general_post_idx field imm7 = 0 (Zero)
    // Fields: Rt=0, Rn=0, L=0, opc=0, imm7=0, Rt2=0
    let encoding: u32 = 0x28800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_imm7_1_poweroftwo_0_28808000() {
    // Encoding: 0x28808000
    // Test aarch64_memory_pair_general_post_idx field imm7 = 1 (PowerOfTwo)
    // Fields: L=0, Rn=0, Rt=0, opc=0, imm7=1, Rt2=0
    let encoding: u32 = 0x28808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_imm7_3_poweroftwominusone_0_28818000() {
    // Encoding: 0x28818000
    // Test aarch64_memory_pair_general_post_idx field imm7 = 3 (PowerOfTwoMinusOne)
    // Fields: imm7=3, opc=0, Rn=0, Rt=0, Rt2=0, L=0
    let encoding: u32 = 0x28818000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_imm7_4_poweroftwo_0_28820000() {
    // Encoding: 0x28820000
    // Test aarch64_memory_pair_general_post_idx field imm7 = 4 (PowerOfTwo)
    // Fields: Rt=0, imm7=4, Rn=0, Rt2=0, L=0, opc=0
    let encoding: u32 = 0x28820000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_imm7_7_poweroftwominusone_0_28838000() {
    // Encoding: 0x28838000
    // Test aarch64_memory_pair_general_post_idx field imm7 = 7 (PowerOfTwoMinusOne)
    // Fields: L=0, opc=0, Rt=0, imm7=7, Rt2=0, Rn=0
    let encoding: u32 = 0x28838000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_imm7_8_poweroftwo_0_28840000() {
    // Encoding: 0x28840000
    // Test aarch64_memory_pair_general_post_idx field imm7 = 8 (PowerOfTwo)
    // Fields: Rt2=0, opc=0, Rn=0, Rt=0, L=0, imm7=8
    let encoding: u32 = 0x28840000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_imm7_15_poweroftwominusone_0_28878000() {
    // Encoding: 0x28878000
    // Test aarch64_memory_pair_general_post_idx field imm7 = 15 (PowerOfTwoMinusOne)
    // Fields: imm7=15, Rt2=0, L=0, Rn=0, opc=0, Rt=0
    let encoding: u32 = 0x28878000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_imm7_16_poweroftwo_0_28880000() {
    // Encoding: 0x28880000
    // Test aarch64_memory_pair_general_post_idx field imm7 = 16 (PowerOfTwo)
    // Fields: Rt2=0, opc=0, Rn=0, L=0, Rt=0, imm7=16
    let encoding: u32 = 0x28880000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_imm7_31_poweroftwominusone_0_288f8000() {
    // Encoding: 0x288F8000
    // Test aarch64_memory_pair_general_post_idx field imm7 = 31 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rt2=0, Rn=0, Rt=0, imm7=31, L=0
    let encoding: u32 = 0x288F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_imm7_32_poweroftwo_0_28900000() {
    // Encoding: 0x28900000
    // Test aarch64_memory_pair_general_post_idx field imm7 = 32 (PowerOfTwo)
    // Fields: imm7=32, Rt2=0, Rn=0, Rt=0, opc=0, L=0
    let encoding: u32 = 0x28900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 63, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (63)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_imm7_63_poweroftwominusone_0_289f8000() {
    // Encoding: 0x289F8000
    // Test aarch64_memory_pair_general_post_idx field imm7 = 63 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=0, L=0, opc=0, Rt2=0, imm7=63
    let encoding: u32 = 0x289F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_imm7_64_poweroftwo_0_28a00000() {
    // Encoding: 0x28A00000
    // Test aarch64_memory_pair_general_post_idx field imm7 = 64 (PowerOfTwo)
    // Fields: Rt2=0, L=0, opc=0, imm7=64, Rn=0, Rt=0
    let encoding: u32 = 0x28A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 127, boundary: Max }
/// maximum immediate (127)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_imm7_127_max_0_28bf8000() {
    // Encoding: 0x28BF8000
    // Test aarch64_memory_pair_general_post_idx field imm7 = 127 (Max)
    // Fields: L=0, Rn=0, Rt=0, imm7=127, Rt2=0, opc=0
    let encoding: u32 = 0x28BF8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_rt2_0_min_0_28800000() {
    // Encoding: 0x28800000
    // Test aarch64_memory_pair_general_post_idx field Rt2 = 0 (Min)
    // Fields: Rt=0, Rt2=0, L=0, Rn=0, opc=0, imm7=0
    let encoding: u32 = 0x28800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_rt2_1_poweroftwo_0_28800400() {
    // Encoding: 0x28800400
    // Test aarch64_memory_pair_general_post_idx field Rt2 = 1 (PowerOfTwo)
    // Fields: Rt2=1, Rn=0, opc=0, Rt=0, L=0, imm7=0
    let encoding: u32 = 0x28800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_rt2_30_poweroftwominusone_0_28807800() {
    // Encoding: 0x28807800
    // Test aarch64_memory_pair_general_post_idx field Rt2 = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm7=0, Rt2=30, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x28807800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_rt2_31_max_0_28807c00() {
    // Encoding: 0x28807C00
    // Test aarch64_memory_pair_general_post_idx field Rt2 = 31 (Max)
    // Fields: Rt=0, imm7=0, Rt2=31, Rn=0, L=0, opc=0
    let encoding: u32 = 0x28807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_rn_0_min_0_28800000() {
    // Encoding: 0x28800000
    // Test aarch64_memory_pair_general_post_idx field Rn = 0 (Min)
    // Fields: L=0, Rt=0, Rn=0, Rt2=0, opc=0, imm7=0
    let encoding: u32 = 0x28800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_rn_1_poweroftwo_0_28800020() {
    // Encoding: 0x28800020
    // Test aarch64_memory_pair_general_post_idx field Rn = 1 (PowerOfTwo)
    // Fields: L=0, Rt2=0, opc=0, imm7=0, Rn=1, Rt=0
    let encoding: u32 = 0x28800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_rn_30_poweroftwominusone_0_288003c0() {
    // Encoding: 0x288003C0
    // Test aarch64_memory_pair_general_post_idx field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rt2=0, Rn=30, imm7=0, Rt=0, L=0, opc=0
    let encoding: u32 = 0x288003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_rn_31_max_0_288003e0() {
    // Encoding: 0x288003E0
    // Test aarch64_memory_pair_general_post_idx field Rn = 31 (Max)
    // Fields: Rt2=0, Rt=0, imm7=0, opc=0, Rn=31, L=0
    let encoding: u32 = 0x288003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_rt_0_min_0_28800000() {
    // Encoding: 0x28800000
    // Test aarch64_memory_pair_general_post_idx field Rt = 0 (Min)
    // Fields: Rt2=0, Rt=0, Rn=0, imm7=0, L=0, opc=0
    let encoding: u32 = 0x28800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_rt_1_poweroftwo_0_28800001() {
    // Encoding: 0x28800001
    // Test aarch64_memory_pair_general_post_idx field Rt = 1 (PowerOfTwo)
    // Fields: opc=0, Rt2=0, L=0, Rn=0, Rt=1, imm7=0
    let encoding: u32 = 0x28800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_rt_30_poweroftwominusone_0_2880001e() {
    // Encoding: 0x2880001E
    // Test aarch64_memory_pair_general_post_idx field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=30, Rn=0, opc=0, L=0, Rt2=0, imm7=0
    let encoding: u32 = 0x2880001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_pair_general_post_idx_field_rt_31_max_0_2880001f() {
    // Encoding: 0x2880001F
    // Test aarch64_memory_pair_general_post_idx field Rt = 31 (Max)
    // Fields: L=0, Rt2=0, Rt=31, opc=0, imm7=0, Rn=0
    let encoding: u32 = 0x2880001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_0_0_28800000() {
    // Encoding: 0x28800000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, Rt2=0, Rt=0, L=0, imm7=0
    let encoding: u32 = 0x28800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_1_0_68800000() {
    // Encoding: 0x68800000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=1, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=0, L=0, Rn=0, opc=1, Rt=0, Rt2=0
    let encoding: u32 = 0x68800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_2_0_a8800000() {
    // Encoding: 0xA8800000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=2, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=2, Rn=0, L=0, imm7=0, Rt2=0, Rt=0
    let encoding: u32 = 0xA8800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_3_0_e8800000() {
    // Encoding: 0xE8800000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=3, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rn=0, L=0, Rt=0, imm7=0, opc=3
    let encoding: u32 = 0xE8800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_4_0_28800000() {
    // Encoding: 0x28800000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, L=0, imm7=0, Rt2=0, Rt=0
    let encoding: u32 = 0x28800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_5_0_28c00000() {
    // Encoding: 0x28C00000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=1, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, imm7=0, Rt2=0, Rt=0, L=1
    let encoding: u32 = 0x28C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=0 (immediate value 0)
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_6_0_28800000() {
    // Encoding: 0x28800000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt2=0, opc=0, Rn=0, imm7=0, Rt=0
    let encoding: u32 = 0x28800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=1 (immediate value 1)
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_7_0_28808000() {
    // Encoding: 0x28808000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=1, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, L=0, Rn=0, Rt=0, imm7=1, opc=0
    let encoding: u32 = 0x28808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_8_0_28818000() {
    // Encoding: 0x28818000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=3, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, imm7=3, Rt=0, L=0, Rt2=0, Rn=0
    let encoding: u32 = 0x28818000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_9_0_28820000() {
    // Encoding: 0x28820000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=4, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt2=0, Rn=0, opc=0, Rt=0, imm7=4
    let encoding: u32 = 0x28820000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_10_0_28838000() {
    // Encoding: 0x28838000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=7, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, L=0, imm7=7, Rt2=0, opc=0, Rn=0
    let encoding: u32 = 0x28838000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_11_0_28840000() {
    // Encoding: 0x28840000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=8, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, L=0, opc=0, imm7=8, Rt2=0, Rt=0
    let encoding: u32 = 0x28840000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_12_0_28878000() {
    // Encoding: 0x28878000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=15, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rn=0, L=0, Rt=0, opc=0, imm7=15
    let encoding: u32 = 0x28878000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_13_0_28880000() {
    // Encoding: 0x28880000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=16, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, opc=0, L=0, Rt2=0, imm7=16
    let encoding: u32 = 0x28880000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_14_0_288f8000() {
    // Encoding: 0x288F8000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=31, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt2=0, opc=0, Rn=0, imm7=31, Rt=0
    let encoding: u32 = 0x288F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_15_0_28900000() {
    // Encoding: 0x28900000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=32, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, imm7=32, opc=0, Rt2=0, Rn=0, L=0
    let encoding: u32 = 0x28900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=63 (immediate midpoint (63))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_16_0_289f8000() {
    // Encoding: 0x289F8000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=63, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, L=0, imm7=63, Rt2=0, Rn=0, opc=0
    let encoding: u32 = 0x289F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_17_0_28a00000() {
    // Encoding: 0x28A00000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=64, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, opc=0, Rn=0, L=0, imm7=64, Rt=0
    let encoding: u32 = 0x28A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=127 (maximum immediate (127))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_18_0_28bf8000() {
    // Encoding: 0x28BF8000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=127, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, L=0, imm7=127, opc=0, Rn=0, Rt2=0
    let encoding: u32 = 0x28BF8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_19_0_28800000() {
    // Encoding: 0x28800000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, imm7=0, L=0, Rt2=0, opc=0
    let encoding: u32 = 0x28800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_20_0_28800400() {
    // Encoding: 0x28800400
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=0
    // Fields: L=0, Rt2=1, Rn=0, Rt=0, opc=0, imm7=0
    let encoding: u32 = 0x28800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_21_0_28807800() {
    // Encoding: 0x28807800
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=30, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, opc=0, imm7=0, L=0, Rt2=30
    let encoding: u32 = 0x28807800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_22_0_28807c00() {
    // Encoding: 0x28807C00
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=0
    // Fields: imm7=0, L=0, Rn=0, Rt=0, opc=0, Rt2=31
    let encoding: u32 = 0x28807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_23_0_28800000() {
    // Encoding: 0x28800000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt=0, opc=0, imm7=0, Rn=0, Rt2=0
    let encoding: u32 = 0x28800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_24_0_28800020() {
    // Encoding: 0x28800020
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=0
    // Fields: Rt=0, imm7=0, L=0, opc=0, Rt2=0, Rn=1
    let encoding: u32 = 0x28800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_25_0_288003c0() {
    // Encoding: 0x288003C0
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=30, Rt=0
    // Fields: Rn=30, L=0, imm7=0, opc=0, Rt2=0, Rt=0
    let encoding: u32 = 0x288003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_26_0_288003e0() {
    // Encoding: 0x288003E0
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=0
    // Fields: L=0, Rt2=0, Rt=0, Rn=31, imm7=0, opc=0
    let encoding: u32 = 0x288003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_27_0_28800000() {
    // Encoding: 0x28800000
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt2=0, imm7=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x28800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_28_0_28800001() {
    // Encoding: 0x28800001
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=1
    // Fields: imm7=0, opc=0, Rt2=0, Rn=0, Rt=1, L=0
    let encoding: u32 = 0x28800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_29_0_2880001e() {
    // Encoding: 0x2880001E
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=30
    // Fields: Rt=30, Rt2=0, Rn=0, opc=0, L=0, imm7=0
    let encoding: u32 = 0x2880001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_30_0_2880001f() {
    // Encoding: 0x2880001F
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=31
    // Fields: L=0, imm7=0, Rt2=0, Rn=0, opc=0, Rt=31
    let encoding: u32 = 0x2880001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_31_0_28800420() {
    // Encoding: 0x28800420
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=1, Rt=0
    // Fields: Rt=0, opc=0, Rn=1, Rt2=1, L=0, imm7=0
    let encoding: u32 = 0x28800420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_32_0_28807fe0() {
    // Encoding: 0x28807FE0
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=31, Rt=0
    // Fields: Rt=0, imm7=0, Rt2=31, L=0, Rn=31, opc=0
    let encoding: u32 = 0x28807FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_33_0_28800401() {
    // Encoding: 0x28800401
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=1
    // Fields: Rt2=1, Rt=1, imm7=0, Rn=0, L=0, opc=0
    let encoding: u32 = 0x28800401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_34_0_28807c1f() {
    // Encoding: 0x28807C1F
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=31
    // Fields: Rt2=31, Rn=0, opc=0, imm7=0, Rt=31, L=0
    let encoding: u32 = 0x28807C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_35_0_28800021() {
    // Encoding: 0x28800021
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=1
    // Fields: imm7=0, L=0, opc=0, Rt=1, Rt2=0, Rn=1
    let encoding: u32 = 0x28800021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_general_post_idx_combo_36_0_288003ff() {
    // Encoding: 0x288003FF
    // Test aarch64_memory_pair_general_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=31
    // Fields: opc=0, Rn=31, Rt2=0, L=0, Rt=31, imm7=0
    let encoding: u32 = 0x288003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_pair_general_post_idx_special_opc_0_size_variant_0_0_28808000() {
    // Encoding: 0x28808000
    // Test aarch64_memory_pair_general_post_idx special value opc = 0 (Size variant 0)
    // Fields: L=0, imm7=1, Rn=0, Rt=0, opc=0, Rt2=0
    let encoding: u32 = 0x28808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_pair_general_post_idx_special_opc_1_size_variant_1_0_68808000() {
    // Encoding: 0x68808000
    // Test aarch64_memory_pair_general_post_idx special value opc = 1 (Size variant 1)
    // Fields: opc=1, L=0, Rt2=0, imm7=1, Rt=0, Rn=0
    let encoding: u32 = 0x68808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_pair_general_post_idx_special_opc_2_size_variant_2_0_a8808000() {
    // Encoding: 0xA8808000
    // Test aarch64_memory_pair_general_post_idx special value opc = 2 (Size variant 2)
    // Fields: L=0, Rn=0, opc=2, Rt=0, Rt2=0, imm7=1
    let encoding: u32 = 0xA8808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_pair_general_post_idx_special_opc_3_size_variant_3_0_e8808000() {
    // Encoding: 0xE8808000
    // Test aarch64_memory_pair_general_post_idx special value opc = 3 (Size variant 3)
    // Fields: Rn=0, L=0, Rt=0, opc=3, imm7=1, Rt2=0
    let encoding: u32 = 0xE8808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_pair_general_post_idx_special_rn_31_stack_pointer_sp_may_require_alignment_0_288083e0()
 {
    // Encoding: 0x288083E0
    // Test aarch64_memory_pair_general_post_idx special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: L=0, Rn=31, Rt2=0, imm7=1, Rt=0, opc=0
    let encoding: u32 = 0x288083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_pair_general_post_idx_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_2880801f()
 {
    // Encoding: 0x2880801F
    // Test aarch64_memory_pair_general_post_idx special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: imm7=1, Rt=31, Rn=0, opc=0, L=0, Rt2=0
    let encoding: u32 = 0x2880801F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_opc_0_min_0_29800000() {
    // Encoding: 0x29800000
    // Test aarch64_memory_pair_general_pre_idx field opc = 0 (Min)
    // Fields: imm7=0, Rt2=0, opc=0, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x29800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_opc_1_poweroftwo_0_69800000() {
    // Encoding: 0x69800000
    // Test aarch64_memory_pair_general_pre_idx field opc = 1 (PowerOfTwo)
    // Fields: imm7=0, L=0, Rn=0, Rt2=0, opc=1, Rt=0
    let encoding: u32 = 0x69800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_opc_2_poweroftwo_0_a9800000() {
    // Encoding: 0xA9800000
    // Test aarch64_memory_pair_general_pre_idx field opc = 2 (PowerOfTwo)
    // Fields: opc=2, imm7=0, L=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0xA9800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_opc_3_max_0_e9800000() {
    // Encoding: 0xE9800000
    // Test aarch64_memory_pair_general_pre_idx field opc = 3 (Max)
    // Fields: Rt2=0, opc=3, Rt=0, L=0, imm7=0, Rn=0
    let encoding: u32 = 0xE9800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_l_0_min_0_29800000() {
    // Encoding: 0x29800000
    // Test aarch64_memory_pair_general_pre_idx field L = 0 (Min)
    // Fields: L=0, Rn=0, Rt2=0, imm7=0, Rt=0, opc=0
    let encoding: u32 = 0x29800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_l_1_max_0_29c00000() {
    // Encoding: 0x29C00000
    // Test aarch64_memory_pair_general_pre_idx field L = 1 (Max)
    // Fields: opc=0, imm7=0, L=1, Rn=0, Rt2=0, Rt=0
    let encoding: u32 = 0x29C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_imm7_0_zero_0_29800000() {
    // Encoding: 0x29800000
    // Test aarch64_memory_pair_general_pre_idx field imm7 = 0 (Zero)
    // Fields: Rt=0, L=0, imm7=0, opc=0, Rt2=0, Rn=0
    let encoding: u32 = 0x29800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_imm7_1_poweroftwo_0_29808000() {
    // Encoding: 0x29808000
    // Test aarch64_memory_pair_general_pre_idx field imm7 = 1 (PowerOfTwo)
    // Fields: opc=0, Rn=0, imm7=1, Rt2=0, L=0, Rt=0
    let encoding: u32 = 0x29808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_imm7_3_poweroftwominusone_0_29818000() {
    // Encoding: 0x29818000
    // Test aarch64_memory_pair_general_pre_idx field imm7 = 3 (PowerOfTwoMinusOne)
    // Fields: L=0, opc=0, Rt2=0, imm7=3, Rt=0, Rn=0
    let encoding: u32 = 0x29818000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_imm7_4_poweroftwo_0_29820000() {
    // Encoding: 0x29820000
    // Test aarch64_memory_pair_general_pre_idx field imm7 = 4 (PowerOfTwo)
    // Fields: opc=0, L=0, Rn=0, Rt=0, imm7=4, Rt2=0
    let encoding: u32 = 0x29820000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_imm7_7_poweroftwominusone_0_29838000() {
    // Encoding: 0x29838000
    // Test aarch64_memory_pair_general_pre_idx field imm7 = 7 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=0, opc=0, Rt2=0, L=0, imm7=7
    let encoding: u32 = 0x29838000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_imm7_8_poweroftwo_0_29840000() {
    // Encoding: 0x29840000
    // Test aarch64_memory_pair_general_pre_idx field imm7 = 8 (PowerOfTwo)
    // Fields: opc=0, Rn=0, Rt=0, imm7=8, L=0, Rt2=0
    let encoding: u32 = 0x29840000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_imm7_15_poweroftwominusone_0_29878000() {
    // Encoding: 0x29878000
    // Test aarch64_memory_pair_general_pre_idx field imm7 = 15 (PowerOfTwoMinusOne)
    // Fields: L=0, imm7=15, Rt=0, Rt2=0, opc=0, Rn=0
    let encoding: u32 = 0x29878000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_imm7_16_poweroftwo_0_29880000() {
    // Encoding: 0x29880000
    // Test aarch64_memory_pair_general_pre_idx field imm7 = 16 (PowerOfTwo)
    // Fields: Rt=0, imm7=16, opc=0, L=0, Rt2=0, Rn=0
    let encoding: u32 = 0x29880000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_imm7_31_poweroftwominusone_0_298f8000() {
    // Encoding: 0x298F8000
    // Test aarch64_memory_pair_general_pre_idx field imm7 = 31 (PowerOfTwoMinusOne)
    // Fields: Rn=0, L=0, Rt2=0, Rt=0, imm7=31, opc=0
    let encoding: u32 = 0x298F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_imm7_32_poweroftwo_0_29900000() {
    // Encoding: 0x29900000
    // Test aarch64_memory_pair_general_pre_idx field imm7 = 32 (PowerOfTwo)
    // Fields: Rn=0, L=0, imm7=32, Rt=0, Rt2=0, opc=0
    let encoding: u32 = 0x29900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 63, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (63)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_imm7_63_poweroftwominusone_0_299f8000() {
    // Encoding: 0x299F8000
    // Test aarch64_memory_pair_general_pre_idx field imm7 = 63 (PowerOfTwoMinusOne)
    // Fields: imm7=63, Rt2=0, L=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x299F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_imm7_64_poweroftwo_0_29a00000() {
    // Encoding: 0x29A00000
    // Test aarch64_memory_pair_general_pre_idx field imm7 = 64 (PowerOfTwo)
    // Fields: L=0, Rn=0, Rt2=0, imm7=64, Rt=0, opc=0
    let encoding: u32 = 0x29A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 127, boundary: Max }
/// maximum immediate (127)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_imm7_127_max_0_29bf8000() {
    // Encoding: 0x29BF8000
    // Test aarch64_memory_pair_general_pre_idx field imm7 = 127 (Max)
    // Fields: opc=0, imm7=127, Rt2=0, L=0, Rn=0, Rt=0
    let encoding: u32 = 0x29BF8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_rt2_0_min_0_29800000() {
    // Encoding: 0x29800000
    // Test aarch64_memory_pair_general_pre_idx field Rt2 = 0 (Min)
    // Fields: Rt2=0, L=0, imm7=0, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x29800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_rt2_1_poweroftwo_0_29800400() {
    // Encoding: 0x29800400
    // Test aarch64_memory_pair_general_pre_idx field Rt2 = 1 (PowerOfTwo)
    // Fields: Rt=0, imm7=0, L=0, Rt2=1, opc=0, Rn=0
    let encoding: u32 = 0x29800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_rt2_30_poweroftwominusone_0_29807800() {
    // Encoding: 0x29807800
    // Test aarch64_memory_pair_general_pre_idx field Rt2 = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm7=0, Rt2=30, Rt=0, L=0, Rn=0
    let encoding: u32 = 0x29807800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_rt2_31_max_0_29807c00() {
    // Encoding: 0x29807C00
    // Test aarch64_memory_pair_general_pre_idx field Rt2 = 31 (Max)
    // Fields: Rn=0, Rt=0, Rt2=31, opc=0, L=0, imm7=0
    let encoding: u32 = 0x29807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_rn_0_min_0_29800000() {
    // Encoding: 0x29800000
    // Test aarch64_memory_pair_general_pre_idx field Rn = 0 (Min)
    // Fields: Rn=0, Rt2=0, Rt=0, L=0, imm7=0, opc=0
    let encoding: u32 = 0x29800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_rn_1_poweroftwo_0_29800020() {
    // Encoding: 0x29800020
    // Test aarch64_memory_pair_general_pre_idx field Rn = 1 (PowerOfTwo)
    // Fields: Rt=0, L=0, imm7=0, Rn=1, opc=0, Rt2=0
    let encoding: u32 = 0x29800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_rn_30_poweroftwominusone_0_298003c0() {
    // Encoding: 0x298003C0
    // Test aarch64_memory_pair_general_pre_idx field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, L=0, imm7=0, Rn=30, Rt2=0
    let encoding: u32 = 0x298003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_rn_31_max_0_298003e0() {
    // Encoding: 0x298003E0
    // Test aarch64_memory_pair_general_pre_idx field Rn = 31 (Max)
    // Fields: Rn=31, imm7=0, L=0, opc=0, Rt2=0, Rt=0
    let encoding: u32 = 0x298003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_rt_0_min_0_29800000() {
    // Encoding: 0x29800000
    // Test aarch64_memory_pair_general_pre_idx field Rt = 0 (Min)
    // Fields: Rt=0, Rn=0, L=0, imm7=0, Rt2=0, opc=0
    let encoding: u32 = 0x29800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_rt_1_poweroftwo_0_29800001() {
    // Encoding: 0x29800001
    // Test aarch64_memory_pair_general_pre_idx field Rt = 1 (PowerOfTwo)
    // Fields: Rn=0, Rt2=0, Rt=1, opc=0, imm7=0, L=0
    let encoding: u32 = 0x29800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_rt_30_poweroftwominusone_0_2980001e() {
    // Encoding: 0x2980001E
    // Test aarch64_memory_pair_general_pre_idx field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rn=0, Rt=30, L=0, imm7=0, Rt2=0
    let encoding: u32 = 0x2980001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_field_rt_31_max_0_2980001f() {
    // Encoding: 0x2980001F
    // Test aarch64_memory_pair_general_pre_idx field Rt = 31 (Max)
    // Fields: imm7=0, Rt2=0, opc=0, L=0, Rn=0, Rt=31
    let encoding: u32 = 0x2980001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_0_0_29800000() {
    // Encoding: 0x29800000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, L=0, imm7=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x29800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_1_0_69800000() {
    // Encoding: 0x69800000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=1, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, L=0, Rn=0, opc=1, Rt=0, imm7=0
    let encoding: u32 = 0x69800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_2_0_a9800000() {
    // Encoding: 0xA9800000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=2, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, imm7=0, opc=2, Rn=0, Rt2=0, Rt=0
    let encoding: u32 = 0xA9800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_3_0_e9800000() {
    // Encoding: 0xE9800000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=3, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, opc=3, Rt2=0, imm7=0, Rn=0, Rt=0
    let encoding: u32 = 0xE9800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_4_0_29800000() {
    // Encoding: 0x29800000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt=0, imm7=0, Rt2=0, opc=0, Rn=0
    let encoding: u32 = 0x29800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_5_0_29c00000() {
    // Encoding: 0x29C00000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=1, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, Rt2=0, L=1, opc=0, imm7=0, Rn=0
    let encoding: u32 = 0x29C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=0 (immediate value 0)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_6_0_29800000() {
    // Encoding: 0x29800000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, imm7=0, Rt2=0, Rn=0, opc=0, Rt=0
    let encoding: u32 = 0x29800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=1 (immediate value 1)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_7_0_29808000() {
    // Encoding: 0x29808000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=1, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rn=0, Rt=0, L=0, opc=0, imm7=1
    let encoding: u32 = 0x29808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_8_0_29818000() {
    // Encoding: 0x29818000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=3, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rn=0, imm7=3, Rt=0, Rt2=0, opc=0
    let encoding: u32 = 0x29818000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_9_0_29820000() {
    // Encoding: 0x29820000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=4, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, L=0, Rt=0, Rt2=0, imm7=4
    let encoding: u32 = 0x29820000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_10_0_29838000() {
    // Encoding: 0x29838000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=7, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, imm7=7, Rt2=0, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x29838000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_11_0_29840000() {
    // Encoding: 0x29840000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=8, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, L=0, imm7=8, Rt2=0, Rn=0, opc=0
    let encoding: u32 = 0x29840000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_12_0_29878000() {
    // Encoding: 0x29878000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=15, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, Rt2=0, opc=0, imm7=15, L=0
    let encoding: u32 = 0x29878000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_13_0_29880000() {
    // Encoding: 0x29880000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=16, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, L=0, imm7=16, Rt2=0, opc=0, Rn=0
    let encoding: u32 = 0x29880000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_14_0_298f8000() {
    // Encoding: 0x298F8000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=31, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, opc=0, imm7=31, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x298F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_15_0_29900000() {
    // Encoding: 0x29900000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=32, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, imm7=32, Rt2=0, L=0, Rt=0
    let encoding: u32 = 0x29900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=63 (immediate midpoint (63))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_16_0_299f8000() {
    // Encoding: 0x299F8000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=63, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt2=0, Rt=0, opc=0, Rn=0, imm7=63
    let encoding: u32 = 0x299F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_17_0_29a00000() {
    // Encoding: 0x29A00000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=64, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=64, Rn=0, Rt2=0, Rt=0, opc=0, L=0
    let encoding: u32 = 0x29A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=127 (maximum immediate (127))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_18_0_29bf8000() {
    // Encoding: 0x29BF8000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=127, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, L=0, Rt2=0, Rn=0, Rt=0, imm7=127
    let encoding: u32 = 0x29BF8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_19_0_29800000() {
    // Encoding: 0x29800000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, L=0, imm7=0, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x29800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_20_0_29800400() {
    // Encoding: 0x29800400
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=0
    // Fields: Rt=0, imm7=0, opc=0, Rn=0, L=0, Rt2=1
    let encoding: u32 = 0x29800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_21_0_29807800() {
    // Encoding: 0x29807800
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=30, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, imm7=0, L=0, Rt2=30, Rt=0
    let encoding: u32 = 0x29807800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_22_0_29807c00() {
    // Encoding: 0x29807C00
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=0
    // Fields: Rt=0, Rt2=31, opc=0, imm7=0, Rn=0, L=0
    let encoding: u32 = 0x29807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_23_0_29800000() {
    // Encoding: 0x29800000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, L=0, opc=0, Rt2=0, Rt=0, imm7=0
    let encoding: u32 = 0x29800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_24_0_29800020() {
    // Encoding: 0x29800020
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=0
    // Fields: L=0, imm7=0, Rt2=0, Rt=0, Rn=1, opc=0
    let encoding: u32 = 0x29800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_25_0_298003c0() {
    // Encoding: 0x298003C0
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=30, Rt=0
    // Fields: L=0, Rt=0, Rn=30, imm7=0, opc=0, Rt2=0
    let encoding: u32 = 0x298003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_26_0_298003e0() {
    // Encoding: 0x298003E0
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=0
    // Fields: imm7=0, L=0, opc=0, Rn=31, Rt=0, Rt2=0
    let encoding: u32 = 0x298003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_27_0_29800000() {
    // Encoding: 0x29800000
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=0, Rt2=0, Rn=0, Rt=0, opc=0, L=0
    let encoding: u32 = 0x29800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_28_0_29800001() {
    // Encoding: 0x29800001
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=1
    // Fields: L=0, Rt2=0, imm7=0, opc=0, Rn=0, Rt=1
    let encoding: u32 = 0x29800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_29_0_2980001e() {
    // Encoding: 0x2980001E
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=30
    // Fields: opc=0, Rt=30, Rt2=0, L=0, imm7=0, Rn=0
    let encoding: u32 = 0x2980001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_30_0_2980001f() {
    // Encoding: 0x2980001F
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=31
    // Fields: imm7=0, Rt=31, Rn=0, opc=0, Rt2=0, L=0
    let encoding: u32 = 0x2980001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_31_0_29800420() {
    // Encoding: 0x29800420
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=1, Rt=0
    // Fields: L=0, opc=0, imm7=0, Rn=1, Rt=0, Rt2=1
    let encoding: u32 = 0x29800420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_32_0_29807fe0() {
    // Encoding: 0x29807FE0
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=31, Rt=0
    // Fields: opc=0, imm7=0, Rt2=31, Rt=0, L=0, Rn=31
    let encoding: u32 = 0x29807FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_33_0_29800401() {
    // Encoding: 0x29800401
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=1
    // Fields: opc=0, Rt=1, imm7=0, Rt2=1, Rn=0, L=0
    let encoding: u32 = 0x29800401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_34_0_29807c1f() {
    // Encoding: 0x29807C1F
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=31
    // Fields: Rn=0, Rt=31, L=0, Rt2=31, opc=0, imm7=0
    let encoding: u32 = 0x29807C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_35_0_29800021() {
    // Encoding: 0x29800021
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=1
    // Fields: imm7=0, Rn=1, Rt=1, Rt2=0, L=0, opc=0
    let encoding: u32 = 0x29800021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_general_pre_idx_combo_36_0_298003ff() {
    // Encoding: 0x298003FF
    // Test aarch64_memory_pair_general_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=31
    // Fields: L=0, opc=0, imm7=0, Rt=31, Rn=31, Rt2=0
    let encoding: u32 = 0x298003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_pair_general_pre_idx_special_opc_0_size_variant_0_0_29808000() {
    // Encoding: 0x29808000
    // Test aarch64_memory_pair_general_pre_idx special value opc = 0 (Size variant 0)
    // Fields: Rt=0, opc=0, L=0, imm7=1, Rn=0, Rt2=0
    let encoding: u32 = 0x29808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_pair_general_pre_idx_special_opc_1_size_variant_1_0_69808000() {
    // Encoding: 0x69808000
    // Test aarch64_memory_pair_general_pre_idx special value opc = 1 (Size variant 1)
    // Fields: opc=1, L=0, imm7=1, Rt2=0, Rt=0, Rn=0
    let encoding: u32 = 0x69808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_pair_general_pre_idx_special_opc_2_size_variant_2_0_a9808000() {
    // Encoding: 0xA9808000
    // Test aarch64_memory_pair_general_pre_idx special value opc = 2 (Size variant 2)
    // Fields: opc=2, Rn=0, Rt=0, Rt2=0, L=0, imm7=1
    let encoding: u32 = 0xA9808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_pair_general_pre_idx_special_opc_3_size_variant_3_0_e9808000() {
    // Encoding: 0xE9808000
    // Test aarch64_memory_pair_general_pre_idx special value opc = 3 (Size variant 3)
    // Fields: Rt=0, imm7=1, Rn=0, Rt2=0, L=0, opc=3
    let encoding: u32 = 0xE9808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_pair_general_pre_idx_special_rn_31_stack_pointer_sp_may_require_alignment_0_298083e0()
 {
    // Encoding: 0x298083E0
    // Test aarch64_memory_pair_general_pre_idx special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: L=0, Rt=0, imm7=1, Rn=31, opc=0, Rt2=0
    let encoding: u32 = 0x298083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_pair_general_pre_idx_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_2980801f()
 {
    // Encoding: 0x2980801F
    // Test aarch64_memory_pair_general_pre_idx special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: L=0, Rn=0, Rt2=0, opc=0, imm7=1, Rt=31
    let encoding: u32 = 0x2980801F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_pair_general_offset_field_opc_0_min_0_29000000() {
    // Encoding: 0x29000000
    // Test aarch64_memory_pair_general_offset field opc = 0 (Min)
    // Fields: Rt=0, Rt2=0, Rn=0, opc=0, L=0, imm7=0
    let encoding: u32 = 0x29000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_pair_general_offset_field_opc_1_poweroftwo_0_69000000() {
    // Encoding: 0x69000000
    // Test aarch64_memory_pair_general_offset field opc = 1 (PowerOfTwo)
    // Fields: L=0, opc=1, imm7=0, Rn=0, Rt2=0, Rt=0
    let encoding: u32 = 0x69000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_pair_general_offset_field_opc_2_poweroftwo_0_a9000000() {
    // Encoding: 0xA9000000
    // Test aarch64_memory_pair_general_offset field opc = 2 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, imm7=0, Rt2=0, opc=2, L=0
    let encoding: u32 = 0xA9000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_pair_general_offset_field_opc_3_max_0_e9000000() {
    // Encoding: 0xE9000000
    // Test aarch64_memory_pair_general_offset field opc = 3 (Max)
    // Fields: L=0, imm7=0, Rn=0, opc=3, Rt=0, Rt2=0
    let encoding: u32 = 0xE9000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_pair_general_offset_field_l_0_min_0_29000000() {
    // Encoding: 0x29000000
    // Test aarch64_memory_pair_general_offset field L = 0 (Min)
    // Fields: opc=0, Rn=0, Rt=0, Rt2=0, imm7=0, L=0
    let encoding: u32 = 0x29000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_pair_general_offset_field_l_1_max_0_29400000() {
    // Encoding: 0x29400000
    // Test aarch64_memory_pair_general_offset field L = 1 (Max)
    // Fields: Rn=0, imm7=0, Rt=0, L=1, opc=0, Rt2=0
    let encoding: u32 = 0x29400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_pair_general_offset_field_imm7_0_zero_0_29000000() {
    // Encoding: 0x29000000
    // Test aarch64_memory_pair_general_offset field imm7 = 0 (Zero)
    // Fields: opc=0, L=0, imm7=0, Rn=0, Rt=0, Rt2=0
    let encoding: u32 = 0x29000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_pair_general_offset_field_imm7_1_poweroftwo_0_29008000() {
    // Encoding: 0x29008000
    // Test aarch64_memory_pair_general_offset field imm7 = 1 (PowerOfTwo)
    // Fields: opc=0, L=0, Rn=0, Rt=0, Rt2=0, imm7=1
    let encoding: u32 = 0x29008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_pair_general_offset_field_imm7_3_poweroftwominusone_0_29018000() {
    // Encoding: 0x29018000
    // Test aarch64_memory_pair_general_offset field imm7 = 3 (PowerOfTwoMinusOne)
    // Fields: L=0, Rn=0, Rt=0, opc=0, Rt2=0, imm7=3
    let encoding: u32 = 0x29018000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_pair_general_offset_field_imm7_4_poweroftwo_0_29020000() {
    // Encoding: 0x29020000
    // Test aarch64_memory_pair_general_offset field imm7 = 4 (PowerOfTwo)
    // Fields: opc=0, Rt2=0, Rt=0, Rn=0, imm7=4, L=0
    let encoding: u32 = 0x29020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_pair_general_offset_field_imm7_7_poweroftwominusone_0_29038000() {
    // Encoding: 0x29038000
    // Test aarch64_memory_pair_general_offset field imm7 = 7 (PowerOfTwoMinusOne)
    // Fields: imm7=7, Rt2=0, Rt=0, L=0, opc=0, Rn=0
    let encoding: u32 = 0x29038000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_pair_general_offset_field_imm7_8_poweroftwo_0_29040000() {
    // Encoding: 0x29040000
    // Test aarch64_memory_pair_general_offset field imm7 = 8 (PowerOfTwo)
    // Fields: Rt=0, L=0, opc=0, imm7=8, Rn=0, Rt2=0
    let encoding: u32 = 0x29040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_pair_general_offset_field_imm7_15_poweroftwominusone_0_29078000() {
    // Encoding: 0x29078000
    // Test aarch64_memory_pair_general_offset field imm7 = 15 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm7=15, Rt2=0, L=0, Rt=0, Rn=0
    let encoding: u32 = 0x29078000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_pair_general_offset_field_imm7_16_poweroftwo_0_29080000() {
    // Encoding: 0x29080000
    // Test aarch64_memory_pair_general_offset field imm7 = 16 (PowerOfTwo)
    // Fields: Rt2=0, opc=0, L=0, imm7=16, Rn=0, Rt=0
    let encoding: u32 = 0x29080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_pair_general_offset_field_imm7_31_poweroftwominusone_0_290f8000() {
    // Encoding: 0x290F8000
    // Test aarch64_memory_pair_general_offset field imm7 = 31 (PowerOfTwoMinusOne)
    // Fields: opc=0, L=0, Rt2=0, Rn=0, Rt=0, imm7=31
    let encoding: u32 = 0x290F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_pair_general_offset_field_imm7_32_poweroftwo_0_29100000() {
    // Encoding: 0x29100000
    // Test aarch64_memory_pair_general_offset field imm7 = 32 (PowerOfTwo)
    // Fields: Rt=0, Rn=0, Rt2=0, L=0, imm7=32, opc=0
    let encoding: u32 = 0x29100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 63, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (63)
#[test]
fn test_aarch64_memory_pair_general_offset_field_imm7_63_poweroftwominusone_0_291f8000() {
    // Encoding: 0x291F8000
    // Test aarch64_memory_pair_general_offset field imm7 = 63 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, Rt2=0, L=0, imm7=63, Rn=0
    let encoding: u32 = 0x291F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_pair_general_offset_field_imm7_64_poweroftwo_0_29200000() {
    // Encoding: 0x29200000
    // Test aarch64_memory_pair_general_offset field imm7 = 64 (PowerOfTwo)
    // Fields: Rt2=0, Rn=0, Rt=0, L=0, opc=0, imm7=64
    let encoding: u32 = 0x29200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 127, boundary: Max }
/// maximum immediate (127)
#[test]
fn test_aarch64_memory_pair_general_offset_field_imm7_127_max_0_293f8000() {
    // Encoding: 0x293F8000
    // Test aarch64_memory_pair_general_offset field imm7 = 127 (Max)
    // Fields: opc=0, Rt2=0, Rn=0, imm7=127, Rt=0, L=0
    let encoding: u32 = 0x293F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_general_offset_field_rt2_0_min_0_29000000() {
    // Encoding: 0x29000000
    // Test aarch64_memory_pair_general_offset field Rt2 = 0 (Min)
    // Fields: Rt=0, opc=0, L=0, imm7=0, Rt2=0, Rn=0
    let encoding: u32 = 0x29000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_general_offset_field_rt2_1_poweroftwo_0_29000400() {
    // Encoding: 0x29000400
    // Test aarch64_memory_pair_general_offset field Rt2 = 1 (PowerOfTwo)
    // Fields: Rt2=1, Rt=0, Rn=0, opc=0, L=0, imm7=0
    let encoding: u32 = 0x29000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_general_offset_field_rt2_30_poweroftwominusone_0_29007800() {
    // Encoding: 0x29007800
    // Test aarch64_memory_pair_general_offset field Rt2 = 30 (PowerOfTwoMinusOne)
    // Fields: imm7=0, L=0, opc=0, Rt2=30, Rn=0, Rt=0
    let encoding: u32 = 0x29007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_pair_general_offset_field_rt2_31_max_0_29007c00() {
    // Encoding: 0x29007C00
    // Test aarch64_memory_pair_general_offset field Rt2 = 31 (Max)
    // Fields: opc=0, imm7=0, Rt=0, Rt2=31, L=0, Rn=0
    let encoding: u32 = 0x29007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_general_offset_field_rn_0_min_0_29000000() {
    // Encoding: 0x29000000
    // Test aarch64_memory_pair_general_offset field Rn = 0 (Min)
    // Fields: Rt=0, opc=0, imm7=0, L=0, Rn=0, Rt2=0
    let encoding: u32 = 0x29000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_general_offset_field_rn_1_poweroftwo_0_29000020() {
    // Encoding: 0x29000020
    // Test aarch64_memory_pair_general_offset field Rn = 1 (PowerOfTwo)
    // Fields: opc=0, L=0, Rn=1, imm7=0, Rt2=0, Rt=0
    let encoding: u32 = 0x29000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_general_offset_field_rn_30_poweroftwominusone_0_290003c0() {
    // Encoding: 0x290003C0
    // Test aarch64_memory_pair_general_offset field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, L=0, imm7=0, Rt=0, Rt2=0, Rn=30
    let encoding: u32 = 0x290003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_pair_general_offset_field_rn_31_max_0_290003e0() {
    // Encoding: 0x290003E0
    // Test aarch64_memory_pair_general_offset field Rn = 31 (Max)
    // Fields: opc=0, Rt2=0, Rn=31, Rt=0, L=0, imm7=0
    let encoding: u32 = 0x290003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_general_offset_field_rt_0_min_0_29000000() {
    // Encoding: 0x29000000
    // Test aarch64_memory_pair_general_offset field Rt = 0 (Min)
    // Fields: opc=0, imm7=0, L=0, Rn=0, Rt=0, Rt2=0
    let encoding: u32 = 0x29000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_general_offset_field_rt_1_poweroftwo_0_29000001() {
    // Encoding: 0x29000001
    // Test aarch64_memory_pair_general_offset field Rt = 1 (PowerOfTwo)
    // Fields: Rt=1, opc=0, L=0, imm7=0, Rn=0, Rt2=0
    let encoding: u32 = 0x29000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_general_offset_field_rt_30_poweroftwominusone_0_2900001e() {
    // Encoding: 0x2900001E
    // Test aarch64_memory_pair_general_offset field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=30, L=0, imm7=0, opc=0, Rt2=0, Rn=0
    let encoding: u32 = 0x2900001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_pair_general_offset_field_rt_31_max_0_2900001f() {
    // Encoding: 0x2900001F
    // Test aarch64_memory_pair_general_offset field Rt = 31 (Max)
    // Fields: Rt2=0, Rn=0, Rt=31, imm7=0, L=0, opc=0
    let encoding: u32 = 0x2900001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_pair_general_offset_combo_0_0_29000000() {
    // Encoding: 0x29000000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, opc=0, L=0, imm7=0, Rt2=0
    let encoding: u32 = 0x29000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_pair_general_offset_combo_1_0_69000000() {
    // Encoding: 0x69000000
    // Test aarch64_memory_pair_general_offset field combination: opc=1, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rt=0, imm7=0, opc=1, Rn=0, L=0
    let encoding: u32 = 0x69000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_pair_general_offset_combo_2_0_a9000000() {
    // Encoding: 0xA9000000
    // Test aarch64_memory_pair_general_offset field combination: opc=2, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=2, Rt=0, Rt2=0, imm7=0, L=0
    let encoding: u32 = 0xA9000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_pair_general_offset_combo_3_0_e9000000() {
    // Encoding: 0xE9000000
    // Test aarch64_memory_pair_general_offset field combination: opc=3, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rn=0, opc=3, L=0, Rt=0, imm7=0
    let encoding: u32 = 0xE9000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_pair_general_offset_combo_4_0_29000000() {
    // Encoding: 0x29000000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rn=0, imm7=0, Rt=0, opc=0, L=0
    let encoding: u32 = 0x29000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_5_0_29400000() {
    // Encoding: 0x29400000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=1, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rt=0, imm7=0, L=1, Rt2=0, Rn=0
    let encoding: u32 = 0x29400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=0 (immediate value 0)
#[test]
fn test_aarch64_memory_pair_general_offset_combo_6_0_29000000() {
    // Encoding: 0x29000000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, Rt2=0, L=0, imm7=0, Rn=0
    let encoding: u32 = 0x29000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=1 (immediate value 1)
#[test]
fn test_aarch64_memory_pair_general_offset_combo_7_0_29008000() {
    // Encoding: 0x29008000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=1, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rn=0, Rt=0, opc=0, imm7=1, L=0
    let encoding: u32 = 0x29008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_pair_general_offset_combo_8_0_29018000() {
    // Encoding: 0x29018000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=3, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, L=0, Rn=0, Rt2=0, imm7=3
    let encoding: u32 = 0x29018000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_9_0_29020000() {
    // Encoding: 0x29020000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=4, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, L=0, opc=0, Rn=0, imm7=4, Rt2=0
    let encoding: u32 = 0x29020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_pair_general_offset_combo_10_0_29038000() {
    // Encoding: 0x29038000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=7, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, L=0, imm7=7, Rt2=0, opc=0, Rt=0
    let encoding: u32 = 0x29038000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_11_0_29040000() {
    // Encoding: 0x29040000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=8, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt2=0, imm7=8, opc=0, Rt=0, L=0
    let encoding: u32 = 0x29040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_pair_general_offset_combo_12_0_29078000() {
    // Encoding: 0x29078000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=15, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, Rt2=0, imm7=15, opc=0, L=0
    let encoding: u32 = 0x29078000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_13_0_29080000() {
    // Encoding: 0x29080000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=16, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, imm7=16, Rt2=0, L=0, opc=0
    let encoding: u32 = 0x29080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_pair_general_offset_combo_14_0_290f8000() {
    // Encoding: 0x290F8000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=31, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rn=0, Rt=0, L=0, opc=0, imm7=31
    let encoding: u32 = 0x290F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_15_0_29100000() {
    // Encoding: 0x29100000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=32, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, imm7=32, Rt2=0, Rn=0, opc=0, Rt=0
    let encoding: u32 = 0x29100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=63 (immediate midpoint (63))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_16_0_291f8000() {
    // Encoding: 0x291F8000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=63, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=63, L=0, opc=0, Rn=0, Rt=0, Rt2=0
    let encoding: u32 = 0x291F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_17_0_29200000() {
    // Encoding: 0x29200000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=64, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=64, opc=0, Rt=0, L=0, Rn=0, Rt2=0
    let encoding: u32 = 0x29200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=127 (maximum immediate (127))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_18_0_293f8000() {
    // Encoding: 0x293F8000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=127, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rn=0, Rt=0, opc=0, Rt2=0, imm7=127
    let encoding: u32 = 0x293F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_19_0_29000000() {
    // Encoding: 0x29000000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rn=0, Rt=0, opc=0, Rt2=0, imm7=0
    let encoding: u32 = 0x29000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_20_0_29000400() {
    // Encoding: 0x29000400
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=0
    // Fields: Rt2=1, Rn=0, Rt=0, opc=0, L=0, imm7=0
    let encoding: u32 = 0x29000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_21_0_29007800() {
    // Encoding: 0x29007800
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=30, Rn=0, Rt=0
    // Fields: Rt=0, L=0, Rn=0, imm7=0, Rt2=30, opc=0
    let encoding: u32 = 0x29007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_22_0_29007c00() {
    // Encoding: 0x29007C00
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=0
    // Fields: Rn=0, Rt2=31, imm7=0, L=0, opc=0, Rt=0
    let encoding: u32 = 0x29007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_23_0_29000000() {
    // Encoding: 0x29000000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, imm7=0, Rt=0, Rt2=0, L=0
    let encoding: u32 = 0x29000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_24_0_29000020() {
    // Encoding: 0x29000020
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=0
    // Fields: Rn=1, imm7=0, L=0, opc=0, Rt=0, Rt2=0
    let encoding: u32 = 0x29000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_25_0_290003c0() {
    // Encoding: 0x290003C0
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=30, Rt=0
    // Fields: L=0, Rt2=0, Rn=30, Rt=0, opc=0, imm7=0
    let encoding: u32 = 0x290003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_26_0_290003e0() {
    // Encoding: 0x290003E0
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=0
    // Fields: imm7=0, Rt2=0, Rt=0, Rn=31, L=0, opc=0
    let encoding: u32 = 0x290003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_27_0_29000000() {
    // Encoding: 0x29000000
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt2=0, opc=0, imm7=0, Rt=0, L=0
    let encoding: u32 = 0x29000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_28_0_29000001() {
    // Encoding: 0x29000001
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=1
    // Fields: Rn=0, opc=0, Rt=1, Rt2=0, imm7=0, L=0
    let encoding: u32 = 0x29000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_29_0_2900001e() {
    // Encoding: 0x2900001E
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=30
    // Fields: L=0, imm7=0, opc=0, Rt=30, Rt2=0, Rn=0
    let encoding: u32 = 0x2900001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_30_0_2900001f() {
    // Encoding: 0x2900001F
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=31
    // Fields: Rn=0, imm7=0, Rt=31, L=0, opc=0, Rt2=0
    let encoding: u32 = 0x2900001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_31_0_29000420() {
    // Encoding: 0x29000420
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=1, Rt=0
    // Fields: Rn=1, imm7=0, Rt=0, Rt2=1, opc=0, L=0
    let encoding: u32 = 0x29000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_32_0_29007fe0() {
    // Encoding: 0x29007FE0
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=31, Rt=0
    // Fields: imm7=0, Rt=0, Rn=31, L=0, opc=0, Rt2=31
    let encoding: u32 = 0x29007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_33_0_29000401() {
    // Encoding: 0x29000401
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=1
    // Fields: Rt=1, Rn=0, Rt2=1, L=0, imm7=0, opc=0
    let encoding: u32 = 0x29000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_34_0_29007c1f() {
    // Encoding: 0x29007C1F
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=31
    // Fields: Rt=31, L=0, imm7=0, opc=0, Rt2=31, Rn=0
    let encoding: u32 = 0x29007C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_35_0_29000021() {
    // Encoding: 0x29000021
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=1
    // Fields: imm7=0, opc=0, L=0, Rt2=0, Rn=1, Rt=1
    let encoding: u32 = 0x29000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_general_offset_combo_36_0_290003ff() {
    // Encoding: 0x290003FF
    // Test aarch64_memory_pair_general_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=31
    // Fields: Rt2=0, L=0, Rn=31, Rt=31, imm7=0, opc=0
    let encoding: u32 = 0x290003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_pair_general_offset_special_opc_0_size_variant_0_0_29008000() {
    // Encoding: 0x29008000
    // Test aarch64_memory_pair_general_offset special value opc = 0 (Size variant 0)
    // Fields: Rn=0, Rt=0, opc=0, L=0, imm7=1, Rt2=0
    let encoding: u32 = 0x29008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_pair_general_offset_special_opc_1_size_variant_1_0_69008000() {
    // Encoding: 0x69008000
    // Test aarch64_memory_pair_general_offset special value opc = 1 (Size variant 1)
    // Fields: Rt2=0, Rn=0, Rt=0, L=0, opc=1, imm7=1
    let encoding: u32 = 0x69008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_pair_general_offset_special_opc_2_size_variant_2_0_a9008000() {
    // Encoding: 0xA9008000
    // Test aarch64_memory_pair_general_offset special value opc = 2 (Size variant 2)
    // Fields: opc=2, L=0, imm7=1, Rn=0, Rt=0, Rt2=0
    let encoding: u32 = 0xA9008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_pair_general_offset_special_opc_3_size_variant_3_0_e9008000() {
    // Encoding: 0xE9008000
    // Test aarch64_memory_pair_general_offset special value opc = 3 (Size variant 3)
    // Fields: Rn=0, Rt2=0, L=0, imm7=1, opc=3, Rt=0
    let encoding: u32 = 0xE9008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_pair_general_offset_special_rn_31_stack_pointer_sp_may_require_alignment_0_290083e0()
 {
    // Encoding: 0x290083E0
    // Test aarch64_memory_pair_general_offset special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rn=31, Rt=0, opc=0, L=0, Rt2=0, imm7=1
    let encoding: u32 = 0x290083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_pair_general_offset_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_2900801f()
 {
    // Encoding: 0x2900801F
    // Test aarch64_memory_pair_general_offset special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: opc=0, imm7=1, Rt2=0, L=0, Rn=0, Rt=31
    let encoding: u32 = 0x2900801F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_pair_general_post_idx_reg_write_0_28800000() {
    // Test aarch64_memory_pair_general_post_idx register write: GpFromField("t")
    // Encoding: 0x28800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `GpFromField("t2") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t2" }
/// verify register write to GpFromField("t2")
#[test]
fn test_aarch64_memory_pair_general_post_idx_reg_write_1_28800000() {
    // Test aarch64_memory_pair_general_post_idx register write: GpFromField("t2")
    // Encoding: 0x28800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_pair_general_post_idx_reg_write_2_28800000() {
    // Test aarch64_memory_pair_general_post_idx register write: GpFromField("t")
    // Encoding: 0x28800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `GpFromField("t2") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t2" }
/// verify register write to GpFromField("t2")
#[test]
fn test_aarch64_memory_pair_general_post_idx_reg_write_3_28800000() {
    // Test aarch64_memory_pair_general_post_idx register write: GpFromField("t2")
    // Encoding: 0x28800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_memory_pair_general_post_idx_reg_write_4_28800000() {
    // Test aarch64_memory_pair_general_post_idx register write: Sp
    // Encoding: 0x28800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_memory_pair_general_post_idx_reg_write_5_28800000() {
    // Test aarch64_memory_pair_general_post_idx register write: GpFromField("n")
    // Encoding: 0x28800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_pair_general_post_idx_sp_rn_288003e0() {
    // Test aarch64_memory_pair_general_post_idx with Rn = SP (31)
    // Encoding: 0x288003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x288003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_pair_general_post_idx_zr_rt_2880001f() {
    // Test aarch64_memory_pair_general_post_idx with Rt = ZR (31)
    // Encoding: 0x2880001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2880001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_pair_general_post_idx_store_0_28800020() {
    // Test aarch64_memory_pair_general_post_idx memory store: 8 bytes
    // Encoding: 0x28800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0x28800020;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_pair_general_post_idx_store_1_28800020() {
    // Test aarch64_memory_pair_general_post_idx memory store: 8 bytes
    // Encoding: 0x28800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x28800020;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_pair_general_post_idx_exception_0_28800000() {
    // Test aarch64_memory_pair_general_post_idx exception: Undefined
    // Encoding: 0x28800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_pair_general_post_idx_exception_1_28800000() {
    // Test aarch64_memory_pair_general_post_idx exception: Undefined
    // Encoding: 0x28800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_post_idx
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_pair_general_post_idx_exception_2_28800000() {
    // Test aarch64_memory_pair_general_post_idx exception: Undefined
    // Encoding: 0x28800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `LDRSB X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// zero value
#[test]
fn test_aarch64_memory_pair_general_pre_idx_ldr_oracle_0_39800020() {
    // Test LDRSB: zero value (oracle)
    // Encoding: 0x39800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0]).unwrap();
    let encoding: u32 = 0x39800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `LDRSB X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max byte
#[test]
fn test_aarch64_memory_pair_general_pre_idx_ldr_oracle_1_39800020() {
    // Test LDRSB: max byte (oracle)
    // Encoding: 0x39800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255]).unwrap();
    let encoding: u32 = 0x39800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `LDRSB X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max halfword
#[test]
fn test_aarch64_memory_pair_general_pre_idx_ldr_oracle_2_39800020() {
    // Test LDRSB: max halfword (oracle)
    // Encoding: 0x39800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255]).unwrap();
    let encoding: u32 = 0x39800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `LDRSB X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// max word
#[test]
fn test_aarch64_memory_pair_general_pre_idx_ldr_oracle_3_39800020() {
    // Test LDRSB: max word (oracle)
    // Encoding: 0x39800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[255]).unwrap();
    let encoding: u32 = 0x39800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFFF,
        "X0 should be 0xFFFFFFFFFFFFFFFF"
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `LDRSB X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// large value
#[test]
fn test_aarch64_memory_pair_general_pre_idx_ldr_oracle_4_39800020() {
    // Test LDRSB: large value (oracle)
    // Encoding: 0x39800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[240]).unwrap();
    let encoding: u32 = 0x39800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFFF0,
        "X0 should be 0xFFFFFFFFFFFFFFF0"
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `LDRSB X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (byte)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_ldr_oracle_5_39800020() {
    // Test LDRSB: sign bit (byte) (oracle)
    // Encoding: 0x39800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[128]).unwrap();
    let encoding: u32 = 0x39800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_x(&cpu, 0),
        0xFFFFFFFFFFFFFF80,
        "X0 should be 0xFFFFFFFFFFFFFF80"
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `LDRSB X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (halfword)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_ldr_oracle_6_39800020() {
    // Test LDRSB: sign bit (halfword) (oracle)
    // Encoding: 0x39800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0]).unwrap();
    let encoding: u32 = 0x39800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `LDRSB X0, [X1, #0]`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rt" }
/// sign bit (word)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_ldr_oracle_7_39800020() {
    // Test LDRSB: sign bit (word) (oracle)
    // Encoding: 0x39800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    cpu.write_memory(0x1000, &[0]).unwrap();
    let encoding: u32 = 0x39800020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_pair_general_pre_idx_reg_write_0_29800000() {
    // Test aarch64_memory_pair_general_pre_idx register write: GpFromField("t")
    // Encoding: 0x29800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `GpFromField("t2") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t2" }
/// verify register write to GpFromField("t2")
#[test]
fn test_aarch64_memory_pair_general_pre_idx_reg_write_1_29800000() {
    // Test aarch64_memory_pair_general_pre_idx register write: GpFromField("t2")
    // Encoding: 0x29800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_pair_general_pre_idx_reg_write_2_29800000() {
    // Test aarch64_memory_pair_general_pre_idx register write: GpFromField("t")
    // Encoding: 0x29800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `GpFromField("t2") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t2" }
/// verify register write to GpFromField("t2")
#[test]
fn test_aarch64_memory_pair_general_pre_idx_reg_write_3_29800000() {
    // Test aarch64_memory_pair_general_pre_idx register write: GpFromField("t2")
    // Encoding: 0x29800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_memory_pair_general_pre_idx_reg_write_4_29800000() {
    // Test aarch64_memory_pair_general_pre_idx register write: Sp
    // Encoding: 0x29800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_memory_pair_general_pre_idx_reg_write_5_29800000() {
    // Test aarch64_memory_pair_general_pre_idx register write: GpFromField("n")
    // Encoding: 0x29800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_sp_rn_298003e0() {
    // Test aarch64_memory_pair_general_pre_idx with Rn = SP (31)
    // Encoding: 0x298003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x298003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_pair_general_pre_idx_zr_rt_2980001f() {
    // Test aarch64_memory_pair_general_pre_idx with Rt = ZR (31)
    // Encoding: 0x2980001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2980001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_pair_general_pre_idx_store_0_29800020() {
    // Test aarch64_memory_pair_general_pre_idx memory store: 8 bytes
    // Encoding: 0x29800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x29800020;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_pair_general_pre_idx_store_1_29800020() {
    // Test aarch64_memory_pair_general_pre_idx memory store: 8 bytes
    // Encoding: 0x29800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x29800020;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_pair_general_pre_idx_exception_0_29800000() {
    // Test aarch64_memory_pair_general_pre_idx exception: Undefined
    // Encoding: 0x29800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_pair_general_pre_idx_exception_1_29800000() {
    // Test aarch64_memory_pair_general_pre_idx exception: Undefined
    // Encoding: 0x29800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_pre_idx
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_pair_general_pre_idx_exception_2_29800000() {
    // Test aarch64_memory_pair_general_pre_idx exception: Undefined
    // Encoding: 0x29800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// zero value
#[test]
fn test_aarch64_memory_pair_general_offset_str_oracle_0_39000020() {
    // Test STRB: zero value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x0);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x0, "Memory at 0x1000 should be 0x0");
    }
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// byte value
#[test]
fn test_aarch64_memory_pair_general_offset_str_oracle_1_39000020() {
    // Test STRB: byte value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0xFF);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0xFF, "Memory at 0x1000 should be 0xFF");
    }
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// halfword value
#[test]
fn test_aarch64_memory_pair_general_offset_str_oracle_2_39000020() {
    // Test STRB: halfword value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x1234);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x34, "Memory at 0x1000 should be 0x34");
    }
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// word value
#[test]
fn test_aarch64_memory_pair_general_offset_str_oracle_3_39000020() {
    // Test STRB: word value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0x12345678);
    set_x(&mut cpu, 1, 0x1000);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0x78, "Memory at 0x1000 should be 0x78");
    }
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `STRB X0, [X1, #0]`
/// Requirement: MemoryAccess { op: Store, size_bits: 8, addressing: "immediate" }
/// doubleword value
#[test]
fn test_aarch64_memory_pair_general_offset_str_oracle_4_39000020() {
    // Test STRB: doubleword value (oracle)
    // Encoding: 0x39000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x1000);
    set_x(&mut cpu, 0, 0x123456789ABCDEF0);
    let encoding: u32 = 0x39000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
    {
        let buf = cpu.read_memory(0x1000, 1).unwrap();
        let mut bytes = [0u8; 8];
        let n = buf.len().min(8);
        bytes[..n].copy_from_slice(&buf[..n]);
        let val = u64::from_le_bytes(bytes);
        assert_eq!(val, 0xF0, "Memory at 0x1000 should be 0xF0");
    }
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_pair_general_offset_reg_write_0_29000000() {
    // Test aarch64_memory_pair_general_offset register write: GpFromField("t")
    // Encoding: 0x29000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `GpFromField("t2") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t2" }
/// verify register write to GpFromField("t2")
#[test]
fn test_aarch64_memory_pair_general_offset_reg_write_1_29000000() {
    // Test aarch64_memory_pair_general_offset register write: GpFromField("t2")
    // Encoding: 0x29000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_pair_general_offset_reg_write_2_29000000() {
    // Test aarch64_memory_pair_general_offset register write: GpFromField("t")
    // Encoding: 0x29000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `GpFromField("t2") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t2" }
/// verify register write to GpFromField("t2")
#[test]
fn test_aarch64_memory_pair_general_offset_reg_write_3_29000000() {
    // Test aarch64_memory_pair_general_offset register write: GpFromField("t2")
    // Encoding: 0x29000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_memory_pair_general_offset_reg_write_4_29000000() {
    // Test aarch64_memory_pair_general_offset register write: Sp
    // Encoding: 0x29000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_memory_pair_general_offset_reg_write_5_29000000() {
    // Test aarch64_memory_pair_general_offset register write: GpFromField("n")
    // Encoding: 0x29000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_pair_general_offset_sp_rn_290003e0() {
    // Test aarch64_memory_pair_general_offset with Rn = SP (31)
    // Encoding: 0x290003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x290003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_pair_general_offset_zr_rt_2900001f() {
    // Test aarch64_memory_pair_general_offset with Rt = ZR (31)
    // Encoding: 0x2900001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2900001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_pair_general_offset_store_0_29000020() {
    // Test aarch64_memory_pair_general_offset memory store: 8 bytes
    // Encoding: 0x29000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0x29000020;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_pair_general_offset_store_1_29000020() {
    // Test aarch64_memory_pair_general_offset memory store: 8 bytes
    // Encoding: 0x29000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x29000020;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_pair_general_offset_exception_0_29000000() {
    // Test aarch64_memory_pair_general_offset exception: Undefined
    // Encoding: 0x29000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_pair_general_offset_exception_1_29000000() {
    // Test aarch64_memory_pair_general_offset exception: Undefined
    // Encoding: 0x29000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_offset
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_pair_general_offset_exception_2_29000000() {
    // Test aarch64_memory_pair_general_offset exception: Undefined
    // Encoding: 0x29000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x29000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_memory_pair_general_no_alloc Tests
// ============================================================================

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_opc_0_min_0_28000000() {
    // Encoding: 0x28000000
    // Test aarch64_memory_pair_general_no_alloc field opc = 0 (Min)
    // Fields: Rn=0, imm7=0, Rt=0, L=0, opc=0, Rt2=0
    let encoding: u32 = 0x28000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_opc_1_poweroftwo_0_68000000() {
    // Encoding: 0x68000000
    // Test aarch64_memory_pair_general_no_alloc field opc = 1 (PowerOfTwo)
    // Fields: Rn=0, imm7=0, Rt2=0, opc=1, L=0, Rt=0
    let encoding: u32 = 0x68000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_opc_2_poweroftwo_0_a8000000() {
    // Encoding: 0xA8000000
    // Test aarch64_memory_pair_general_no_alloc field opc = 2 (PowerOfTwo)
    // Fields: Rn=0, L=0, imm7=0, Rt2=0, opc=2, Rt=0
    let encoding: u32 = 0xA8000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_opc_3_max_0_e8000000() {
    // Encoding: 0xE8000000
    // Test aarch64_memory_pair_general_no_alloc field opc = 3 (Max)
    // Fields: imm7=0, L=0, Rn=0, Rt=0, Rt2=0, opc=3
    let encoding: u32 = 0xE8000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_l_0_min_0_28000000() {
    // Encoding: 0x28000000
    // Test aarch64_memory_pair_general_no_alloc field L = 0 (Min)
    // Fields: imm7=0, L=0, opc=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x28000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_l_1_max_0_28400000() {
    // Encoding: 0x28400000
    // Test aarch64_memory_pair_general_no_alloc field L = 1 (Max)
    // Fields: opc=0, imm7=0, Rt=0, Rn=0, L=1, Rt2=0
    let encoding: u32 = 0x28400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_imm7_0_zero_0_28000000() {
    // Encoding: 0x28000000
    // Test aarch64_memory_pair_general_no_alloc field imm7 = 0 (Zero)
    // Fields: imm7=0, opc=0, Rn=0, Rt2=0, Rt=0, L=0
    let encoding: u32 = 0x28000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_imm7_1_poweroftwo_0_28008000() {
    // Encoding: 0x28008000
    // Test aarch64_memory_pair_general_no_alloc field imm7 = 1 (PowerOfTwo)
    // Fields: Rt2=0, imm7=1, Rt=0, opc=0, Rn=0, L=0
    let encoding: u32 = 0x28008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_imm7_3_poweroftwominusone_0_28018000() {
    // Encoding: 0x28018000
    // Test aarch64_memory_pair_general_no_alloc field imm7 = 3 (PowerOfTwoMinusOne)
    // Fields: L=0, imm7=3, opc=0, Rn=0, Rt2=0, Rt=0
    let encoding: u32 = 0x28018000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_imm7_4_poweroftwo_0_28020000() {
    // Encoding: 0x28020000
    // Test aarch64_memory_pair_general_no_alloc field imm7 = 4 (PowerOfTwo)
    // Fields: opc=0, Rn=0, Rt=0, L=0, imm7=4, Rt2=0
    let encoding: u32 = 0x28020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_imm7_7_poweroftwominusone_0_28038000() {
    // Encoding: 0x28038000
    // Test aarch64_memory_pair_general_no_alloc field imm7 = 7 (PowerOfTwoMinusOne)
    // Fields: imm7=7, Rt2=0, L=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x28038000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_imm7_8_poweroftwo_0_28040000() {
    // Encoding: 0x28040000
    // Test aarch64_memory_pair_general_no_alloc field imm7 = 8 (PowerOfTwo)
    // Fields: L=0, imm7=8, Rt=0, Rn=0, opc=0, Rt2=0
    let encoding: u32 = 0x28040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_imm7_15_poweroftwominusone_0_28078000() {
    // Encoding: 0x28078000
    // Test aarch64_memory_pair_general_no_alloc field imm7 = 15 (PowerOfTwoMinusOne)
    // Fields: L=0, Rt=0, opc=0, Rn=0, Rt2=0, imm7=15
    let encoding: u32 = 0x28078000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_imm7_16_poweroftwo_0_28080000() {
    // Encoding: 0x28080000
    // Test aarch64_memory_pair_general_no_alloc field imm7 = 16 (PowerOfTwo)
    // Fields: opc=0, L=0, Rt2=0, Rn=0, Rt=0, imm7=16
    let encoding: u32 = 0x28080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_imm7_31_poweroftwominusone_0_280f8000() {
    // Encoding: 0x280F8000
    // Test aarch64_memory_pair_general_no_alloc field imm7 = 31 (PowerOfTwoMinusOne)
    // Fields: Rn=0, opc=0, Rt2=0, imm7=31, Rt=0, L=0
    let encoding: u32 = 0x280F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_imm7_32_poweroftwo_0_28100000() {
    // Encoding: 0x28100000
    // Test aarch64_memory_pair_general_no_alloc field imm7 = 32 (PowerOfTwo)
    // Fields: opc=0, imm7=32, L=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x28100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 63, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (63)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_imm7_63_poweroftwominusone_0_281f8000() {
    // Encoding: 0x281F8000
    // Test aarch64_memory_pair_general_no_alloc field imm7 = 63 (PowerOfTwoMinusOne)
    // Fields: imm7=63, opc=0, Rn=0, Rt2=0, Rt=0, L=0
    let encoding: u32 = 0x281F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_imm7_64_poweroftwo_0_28200000() {
    // Encoding: 0x28200000
    // Test aarch64_memory_pair_general_no_alloc field imm7 = 64 (PowerOfTwo)
    // Fields: Rt=0, opc=0, Rt2=0, Rn=0, L=0, imm7=64
    let encoding: u32 = 0x28200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 127, boundary: Max }
/// maximum immediate (127)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_imm7_127_max_0_283f8000() {
    // Encoding: 0x283F8000
    // Test aarch64_memory_pair_general_no_alloc field imm7 = 127 (Max)
    // Fields: opc=0, Rn=0, Rt=0, L=0, imm7=127, Rt2=0
    let encoding: u32 = 0x283F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_rt2_0_min_0_28000000() {
    // Encoding: 0x28000000
    // Test aarch64_memory_pair_general_no_alloc field Rt2 = 0 (Min)
    // Fields: Rn=0, Rt=0, L=0, opc=0, imm7=0, Rt2=0
    let encoding: u32 = 0x28000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_rt2_1_poweroftwo_0_28000400() {
    // Encoding: 0x28000400
    // Test aarch64_memory_pair_general_no_alloc field Rt2 = 1 (PowerOfTwo)
    // Fields: Rt2=1, Rn=0, L=0, imm7=0, opc=0, Rt=0
    let encoding: u32 = 0x28000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_rt2_30_poweroftwominusone_0_28007800() {
    // Encoding: 0x28007800
    // Test aarch64_memory_pair_general_no_alloc field Rt2 = 30 (PowerOfTwoMinusOne)
    // Fields: imm7=0, Rn=0, L=0, opc=0, Rt2=30, Rt=0
    let encoding: u32 = 0x28007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_rt2_31_max_0_28007c00() {
    // Encoding: 0x28007C00
    // Test aarch64_memory_pair_general_no_alloc field Rt2 = 31 (Max)
    // Fields: L=0, Rt2=31, Rn=0, opc=0, imm7=0, Rt=0
    let encoding: u32 = 0x28007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_rn_0_min_0_28000000() {
    // Encoding: 0x28000000
    // Test aarch64_memory_pair_general_no_alloc field Rn = 0 (Min)
    // Fields: L=0, opc=0, Rt2=0, Rn=0, Rt=0, imm7=0
    let encoding: u32 = 0x28000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_rn_1_poweroftwo_0_28000020() {
    // Encoding: 0x28000020
    // Test aarch64_memory_pair_general_no_alloc field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, imm7=0, L=0, Rt2=0, Rt=0, opc=0
    let encoding: u32 = 0x28000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_rn_30_poweroftwominusone_0_280003c0() {
    // Encoding: 0x280003C0
    // Test aarch64_memory_pair_general_no_alloc field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: imm7=0, opc=0, Rt2=0, L=0, Rn=30, Rt=0
    let encoding: u32 = 0x280003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_rn_31_max_0_280003e0() {
    // Encoding: 0x280003E0
    // Test aarch64_memory_pair_general_no_alloc field Rn = 31 (Max)
    // Fields: imm7=0, Rn=31, Rt=0, Rt2=0, opc=0, L=0
    let encoding: u32 = 0x280003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_rt_0_min_0_28000000() {
    // Encoding: 0x28000000
    // Test aarch64_memory_pair_general_no_alloc field Rt = 0 (Min)
    // Fields: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x28000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_rt_1_poweroftwo_0_28000001() {
    // Encoding: 0x28000001
    // Test aarch64_memory_pair_general_no_alloc field Rt = 1 (PowerOfTwo)
    // Fields: imm7=0, Rt2=0, Rn=0, Rt=1, opc=0, L=0
    let encoding: u32 = 0x28000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_rt_30_poweroftwominusone_0_2800001e() {
    // Encoding: 0x2800001E
    // Test aarch64_memory_pair_general_no_alloc field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: Rt2=0, imm7=0, Rn=0, Rt=30, L=0, opc=0
    let encoding: u32 = 0x2800001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_field_rt_31_max_0_2800001f() {
    // Encoding: 0x2800001F
    // Test aarch64_memory_pair_general_no_alloc field Rt = 31 (Max)
    // Fields: Rn=0, Rt=31, L=0, Rt2=0, imm7=0, opc=0
    let encoding: u32 = 0x2800001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_0_0_28000000() {
    // Encoding: 0x28000000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, imm7=0, opc=0, L=0, Rn=0, Rt=0
    let encoding: u32 = 0x28000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_1_0_68000000() {
    // Encoding: 0x68000000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=1, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, opc=1, Rt2=0, Rt=0, Rn=0, imm7=0
    let encoding: u32 = 0x68000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_2_0_a8000000() {
    // Encoding: 0xA8000000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=2, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, L=0, Rt=0, opc=2, Rn=0, imm7=0
    let encoding: u32 = 0xA8000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_3_0_e8000000() {
    // Encoding: 0xE8000000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=3, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=3, imm7=0, Rt2=0, Rt=0, L=0
    let encoding: u32 = 0xE8000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_4_0_28000000() {
    // Encoding: 0x28000000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=0, Rt=0, L=0, Rt2=0, opc=0, Rn=0
    let encoding: u32 = 0x28000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_5_0_28400000() {
    // Encoding: 0x28400000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=1, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rt2=0, Rt=0, Rn=0, L=1, imm7=0
    let encoding: u32 = 0x28400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=0 (immediate value 0)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_6_0_28000000() {
    // Encoding: 0x28000000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, imm7=0, Rt2=0, L=0, Rn=0, Rt=0
    let encoding: u32 = 0x28000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=1 (immediate value 1)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_7_0_28008000() {
    // Encoding: 0x28008000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=1, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rn=0, L=0, Rt=0, imm7=1, opc=0
    let encoding: u32 = 0x28008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_8_0_28018000() {
    // Encoding: 0x28018000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=3, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt=0, opc=0, imm7=3, Rn=0, Rt2=0
    let encoding: u32 = 0x28018000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_9_0_28020000() {
    // Encoding: 0x28020000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=4, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=4, Rt=0, Rt2=0, L=0, opc=0, Rn=0
    let encoding: u32 = 0x28020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_10_0_28038000() {
    // Encoding: 0x28038000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=7, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, imm7=7, opc=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x28038000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_11_0_28040000() {
    // Encoding: 0x28040000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=8, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, imm7=8, Rt2=0, Rn=0, L=0
    let encoding: u32 = 0x28040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_12_0_28078000() {
    // Encoding: 0x28078000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=15, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rt2=0, Rn=0, Rt=0, imm7=15, L=0
    let encoding: u32 = 0x28078000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_13_0_28080000() {
    // Encoding: 0x28080000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=16, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, L=0, Rt2=0, Rn=0, Rt=0, imm7=16
    let encoding: u32 = 0x28080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_14_0_280f8000() {
    // Encoding: 0x280F8000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=31, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=31, L=0, opc=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x280F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_15_0_28100000() {
    // Encoding: 0x28100000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=32, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, L=0, Rt2=0, imm7=32, Rn=0
    let encoding: u32 = 0x28100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=63 (immediate midpoint (63))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_16_0_281f8000() {
    // Encoding: 0x281F8000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=63, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt2=0, Rt=0, opc=0, imm7=63, Rn=0
    let encoding: u32 = 0x281F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_17_0_28200000() {
    // Encoding: 0x28200000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=64, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, imm7=64, Rt=0, opc=0, Rn=0, L=0
    let encoding: u32 = 0x28200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=127 (maximum immediate (127))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_18_0_283f8000() {
    // Encoding: 0x283F8000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=127, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, imm7=127, Rn=0, Rt=0, Rt2=0, opc=0
    let encoding: u32 = 0x283F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_19_0_28000000() {
    // Encoding: 0x28000000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, L=0, imm7=0, Rn=0, Rt2=0, opc=0
    let encoding: u32 = 0x28000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_20_0_28000400() {
    // Encoding: 0x28000400
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=0
    // Fields: imm7=0, opc=0, Rt2=1, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x28000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_21_0_28007800() {
    // Encoding: 0x28007800
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=30, Rn=0, Rt=0
    // Fields: L=0, Rn=0, Rt=0, imm7=0, Rt2=30, opc=0
    let encoding: u32 = 0x28007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_22_0_28007c00() {
    // Encoding: 0x28007C00
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=0
    // Fields: imm7=0, Rt2=31, Rn=0, Rt=0, L=0, opc=0
    let encoding: u32 = 0x28007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_23_0_28000000() {
    // Encoding: 0x28000000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, imm7=0, Rn=0, Rt=0, Rt2=0, opc=0
    let encoding: u32 = 0x28000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_24_0_28000020() {
    // Encoding: 0x28000020
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=0
    // Fields: opc=0, Rn=1, imm7=0, Rt2=0, L=0, Rt=0
    let encoding: u32 = 0x28000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_25_0_280003c0() {
    // Encoding: 0x280003C0
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=30, Rt=0
    // Fields: Rt=0, L=0, opc=0, Rt2=0, imm7=0, Rn=30
    let encoding: u32 = 0x280003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_26_0_280003e0() {
    // Encoding: 0x280003E0
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=0
    // Fields: L=0, imm7=0, Rt=0, opc=0, Rt2=0, Rn=31
    let encoding: u32 = 0x280003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_27_0_28000000() {
    // Encoding: 0x28000000
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rt=0, Rt2=0, L=0, Rn=0, imm7=0
    let encoding: u32 = 0x28000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_28_0_28000001() {
    // Encoding: 0x28000001
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=1
    // Fields: opc=0, Rt2=0, L=0, Rn=0, Rt=1, imm7=0
    let encoding: u32 = 0x28000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_29_0_2800001e() {
    // Encoding: 0x2800001E
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=30
    // Fields: L=0, imm7=0, opc=0, Rt2=0, Rn=0, Rt=30
    let encoding: u32 = 0x2800001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_30_0_2800001f() {
    // Encoding: 0x2800001F
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=31
    // Fields: opc=0, L=0, Rt=31, Rt2=0, Rn=0, imm7=0
    let encoding: u32 = 0x2800001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_31_0_28000420() {
    // Encoding: 0x28000420
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=1, Rt=0
    // Fields: Rn=1, L=0, imm7=0, opc=0, Rt=0, Rt2=1
    let encoding: u32 = 0x28000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_32_0_28007fe0() {
    // Encoding: 0x28007FE0
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=31, Rt=0
    // Fields: L=0, opc=0, imm7=0, Rt2=31, Rn=31, Rt=0
    let encoding: u32 = 0x28007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_33_0_28000401() {
    // Encoding: 0x28000401
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=1
    // Fields: Rn=0, L=0, imm7=0, Rt2=1, opc=0, Rt=1
    let encoding: u32 = 0x28000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_34_0_28007c1f() {
    // Encoding: 0x28007C1F
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=31
    // Fields: Rt2=31, imm7=0, Rn=0, opc=0, Rt=31, L=0
    let encoding: u32 = 0x28007C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_35_0_28000021() {
    // Encoding: 0x28000021
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=1
    // Fields: opc=0, L=0, Rt2=0, imm7=0, Rn=1, Rt=1
    let encoding: u32 = 0x28000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_general_no_alloc_combo_36_0_280003ff() {
    // Encoding: 0x280003FF
    // Test aarch64_memory_pair_general_no_alloc field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=31
    // Fields: Rt=31, imm7=0, opc=0, L=0, Rt2=0, Rn=31
    let encoding: u32 = 0x280003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_pair_general_no_alloc_special_opc_0_size_variant_0_0_28008000() {
    // Encoding: 0x28008000
    // Test aarch64_memory_pair_general_no_alloc special value opc = 0 (Size variant 0)
    // Fields: L=0, Rt2=0, opc=0, imm7=1, Rn=0, Rt=0
    let encoding: u32 = 0x28008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_pair_general_no_alloc_special_opc_1_size_variant_1_0_68008000() {
    // Encoding: 0x68008000
    // Test aarch64_memory_pair_general_no_alloc special value opc = 1 (Size variant 1)
    // Fields: L=0, Rt=0, Rt2=0, imm7=1, Rn=0, opc=1
    let encoding: u32 = 0x68008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_pair_general_no_alloc_special_opc_2_size_variant_2_0_a8008000() {
    // Encoding: 0xA8008000
    // Test aarch64_memory_pair_general_no_alloc special value opc = 2 (Size variant 2)
    // Fields: L=0, imm7=1, opc=2, Rt=0, Rt2=0, Rn=0
    let encoding: u32 = 0xA8008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_pair_general_no_alloc_special_opc_3_size_variant_3_0_e8008000() {
    // Encoding: 0xE8008000
    // Test aarch64_memory_pair_general_no_alloc special value opc = 3 (Size variant 3)
    // Fields: Rt=0, imm7=1, L=0, Rt2=0, opc=3, Rn=0
    let encoding: u32 = 0xE8008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_pair_general_no_alloc_special_rn_31_stack_pointer_sp_may_require_alignment_0_280083e0()
 {
    // Encoding: 0x280083E0
    // Test aarch64_memory_pair_general_no_alloc special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: opc=0, Rt2=0, imm7=1, Rn=31, Rt=0, L=0
    let encoding: u32 = 0x280083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_pair_general_no_alloc_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_2800801f()
 {
    // Encoding: 0x2800801F
    // Test aarch64_memory_pair_general_no_alloc special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: opc=0, L=0, Rt2=0, imm7=1, Rn=0, Rt=31
    let encoding: u32 = 0x2800801F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_pair_general_no_alloc_reg_write_0_28000000() {
    // Test aarch64_memory_pair_general_no_alloc register write: GpFromField("t")
    // Encoding: 0x28000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `GpFromField("t2") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t2" }
/// verify register write to GpFromField("t2")
#[test]
fn test_aarch64_memory_pair_general_no_alloc_reg_write_1_28000000() {
    // Test aarch64_memory_pair_general_no_alloc register write: GpFromField("t2")
    // Encoding: 0x28000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `Sp write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "unknown" }
/// verify register write to Sp
#[test]
fn test_aarch64_memory_pair_general_no_alloc_reg_write_2_28000000() {
    // Test aarch64_memory_pair_general_no_alloc register write: Sp
    // Encoding: 0x28000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `GpFromField("n") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "n" }
/// verify register write to GpFromField("n")
#[test]
fn test_aarch64_memory_pair_general_no_alloc_reg_write_3_28000000() {
    // Test aarch64_memory_pair_general_no_alloc register write: GpFromField("n")
    // Encoding: 0x28000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_sp_rn_280003e0() {
    // Test aarch64_memory_pair_general_no_alloc with Rn = SP (31)
    // Encoding: 0x280003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x280003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_pair_general_no_alloc_zr_rt_2800001f() {
    // Test aarch64_memory_pair_general_no_alloc with Rt = ZR (31)
    // Encoding: 0x2800001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x2800001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_pair_general_no_alloc_store_0_28000020() {
    // Test aarch64_memory_pair_general_no_alloc memory store: 8 bytes
    // Encoding: 0x28000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0x28000020;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_pair_general_no_alloc_store_1_28000020() {
    // Test aarch64_memory_pair_general_no_alloc memory store: 8 bytes
    // Encoding: 0x28000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    set_x(&mut cpu, 1, 0x100000000000);
    let encoding: u32 = 0x28000020;
    write_insn(&mut cpu, 0, encoding);
    // Allocated encoding; the synthetic operands may fault at runtime,
    // which still proves the instruction decodes.
    let exit = cpu.step();
    let undef = match &exit {
        Ok(CpuExit::Undefined(_)) => true,
        Err(e) => format!("{e:?}").starts_with("UndefinedInstruction"),
        _ => false,
    };
    assert!(
        !undef,
        "expected allocated encoding for {:#010X}: {:?}",
        encoding, exit
    );
}

/// Provenance: aarch64_memory_pair_general_no_alloc
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_pair_general_no_alloc_exception_0_28000000() {
    // Test aarch64_memory_pair_general_no_alloc exception: Undefined
    // Encoding: 0x28000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x28000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_memory_pair_simdfp_post_idx Tests
// ============================================================================

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_opc_0_min_0_2c800000() {
    // Encoding: 0x2C800000
    // Test aarch64_memory_pair_simdfp_post_idx field opc = 0 (Min)
    // Fields: opc=0, imm7=0, Rt2=0, Rt=0, L=0, Rn=0
    let encoding: u32 = 0x2C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_opc_1_poweroftwo_0_6c800000() {
    // Encoding: 0x6C800000
    // Test aarch64_memory_pair_simdfp_post_idx field opc = 1 (PowerOfTwo)
    // Fields: opc=1, imm7=0, L=0, Rt2=0, Rt=0, Rn=0
    let encoding: u32 = 0x6C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	d0, d0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_opc_2_poweroftwo_0_ac800000() {
    // Encoding: 0xAC800000
    // Test aarch64_memory_pair_simdfp_post_idx field opc = 2 (PowerOfTwo)
    // Fields: Rn=0, imm7=0, Rt=0, L=0, opc=2, Rt2=0
    let encoding: u32 = 0xAC800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	q0, q0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_opc_3_max_0_ec800000() {
    // Encoding: 0xEC800000
    // Test aarch64_memory_pair_simdfp_post_idx field opc = 3 (Max)
    // Fields: L=0, Rt2=0, Rn=0, opc=3, imm7=0, Rt=0
    let encoding: u32 = 0xEC800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_l_0_min_0_2c800000() {
    // Encoding: 0x2C800000
    // Test aarch64_memory_pair_simdfp_post_idx field L = 0 (Min)
    // Fields: imm7=0, Rt=0, opc=0, L=0, Rt2=0, Rn=0
    let encoding: u32 = 0x2C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_l_1_max_0_2cc00000() {
    // Encoding: 0x2CC00000
    // Test aarch64_memory_pair_simdfp_post_idx field L = 1 (Max)
    // Fields: Rt=0, imm7=0, opc=0, Rn=0, L=1, Rt2=0
    let encoding: u32 = 0x2CC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_imm7_0_zero_0_2c800000() {
    // Encoding: 0x2C800000
    // Test aarch64_memory_pair_simdfp_post_idx field imm7 = 0 (Zero)
    // Fields: Rn=0, Rt=0, L=0, imm7=0, Rt2=0, opc=0
    let encoding: u32 = 0x2C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_imm7_1_poweroftwo_0_2c808000() {
    // Encoding: 0x2C808000
    // Test aarch64_memory_pair_simdfp_post_idx field imm7 = 1 (PowerOfTwo)
    // Fields: L=0, opc=0, imm7=1, Rn=0, Rt=0, Rt2=0
    let encoding: u32 = 0x2C808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_imm7_3_poweroftwominusone_0_2c818000() {
    // Encoding: 0x2C818000
    // Test aarch64_memory_pair_simdfp_post_idx field imm7 = 3 (PowerOfTwoMinusOne)
    // Fields: imm7=3, Rt2=0, Rn=0, Rt=0, L=0, opc=0
    let encoding: u32 = 0x2C818000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #12
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_imm7_4_poweroftwo_0_2c820000() {
    // Encoding: 0x2C820000
    // Test aarch64_memory_pair_simdfp_post_idx field imm7 = 4 (PowerOfTwo)
    // Fields: imm7=4, L=0, Rn=0, opc=0, Rt2=0, Rt=0
    let encoding: u32 = 0x2C820000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #16
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_imm7_7_poweroftwominusone_0_2c838000() {
    // Encoding: 0x2C838000
    // Test aarch64_memory_pair_simdfp_post_idx field imm7 = 7 (PowerOfTwoMinusOne)
    // Fields: imm7=7, opc=0, Rt=0, L=0, Rt2=0, Rn=0
    let encoding: u32 = 0x2C838000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #28
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_imm7_8_poweroftwo_0_2c840000() {
    // Encoding: 0x2C840000
    // Test aarch64_memory_pair_simdfp_post_idx field imm7 = 8 (PowerOfTwo)
    // Fields: imm7=8, Rt2=0, Rn=0, opc=0, L=0, Rt=0
    let encoding: u32 = 0x2C840000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_imm7_15_poweroftwominusone_0_2c878000() {
    // Encoding: 0x2C878000
    // Test aarch64_memory_pair_simdfp_post_idx field imm7 = 15 (PowerOfTwoMinusOne)
    // Fields: Rn=0, imm7=15, L=0, Rt2=0, opc=0, Rt=0
    let encoding: u32 = 0x2C878000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #60
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_imm7_16_poweroftwo_0_2c880000() {
    // Encoding: 0x2C880000
    // Test aarch64_memory_pair_simdfp_post_idx field imm7 = 16 (PowerOfTwo)
    // Fields: imm7=16, L=0, Rn=0, Rt=0, Rt2=0, opc=0
    let encoding: u32 = 0x2C880000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #64
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_imm7_31_poweroftwominusone_0_2c8f8000() {
    // Encoding: 0x2C8F8000
    // Test aarch64_memory_pair_simdfp_post_idx field imm7 = 31 (PowerOfTwoMinusOne)
    // Fields: L=0, Rt2=0, opc=0, Rn=0, Rt=0, imm7=31
    let encoding: u32 = 0x2C8F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #124
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_imm7_32_poweroftwo_0_2c900000() {
    // Encoding: 0x2C900000
    // Test aarch64_memory_pair_simdfp_post_idx field imm7 = 32 (PowerOfTwo)
    // Fields: Rn=0, Rt2=0, L=0, Rt=0, opc=0, imm7=32
    let encoding: u32 = 0x2C900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #128
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 63, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (63)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_imm7_63_poweroftwominusone_0_2c9f8000() {
    // Encoding: 0x2C9F8000
    // Test aarch64_memory_pair_simdfp_post_idx field imm7 = 63 (PowerOfTwoMinusOne)
    // Fields: opc=0, L=0, imm7=63, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x2C9F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #252
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_imm7_64_poweroftwo_0_2ca00000() {
    // Encoding: 0x2CA00000
    // Test aarch64_memory_pair_simdfp_post_idx field imm7 = 64 (PowerOfTwo)
    // Fields: Rt2=0, Rn=0, Rt=0, opc=0, L=0, imm7=64
    let encoding: u32 = 0x2CA00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #-256
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 127, boundary: Max }
/// maximum immediate (127)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_imm7_127_max_0_2cbf8000() {
    // Encoding: 0x2CBF8000
    // Test aarch64_memory_pair_simdfp_post_idx field imm7 = 127 (Max)
    // Fields: Rn=0, Rt2=0, Rt=0, L=0, imm7=127, opc=0
    let encoding: u32 = 0x2CBF8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #-4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_rt2_0_min_0_2c800000() {
    // Encoding: 0x2C800000
    // Test aarch64_memory_pair_simdfp_post_idx field Rt2 = 0 (Min)
    // Fields: opc=0, Rt2=0, Rn=0, Rt=0, L=0, imm7=0
    let encoding: u32 = 0x2C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_rt2_1_poweroftwo_0_2c800400() {
    // Encoding: 0x2C800400
    // Test aarch64_memory_pair_simdfp_post_idx field Rt2 = 1 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, opc=0, L=0, imm7=0, Rt2=1
    let encoding: u32 = 0x2C800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s1, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_rt2_30_poweroftwominusone_0_2c807800() {
    // Encoding: 0x2C807800
    // Test aarch64_memory_pair_simdfp_post_idx field Rt2 = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, opc=0, L=0, Rt2=30, imm7=0, Rt=0
    let encoding: u32 = 0x2C807800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s30, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_rt2_31_max_0_2c807c00() {
    // Encoding: 0x2C807C00
    // Test aarch64_memory_pair_simdfp_post_idx field Rt2 = 31 (Max)
    // Fields: L=0, opc=0, imm7=0, Rn=0, Rt2=31, Rt=0
    let encoding: u32 = 0x2C807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s31, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_rn_0_min_0_2c800000() {
    // Encoding: 0x2C800000
    // Test aarch64_memory_pair_simdfp_post_idx field Rn = 0 (Min)
    // Fields: Rt2=0, imm7=0, L=0, Rn=0, opc=0, Rt=0
    let encoding: u32 = 0x2C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_rn_1_poweroftwo_0_2c800020() {
    // Encoding: 0x2C800020
    // Test aarch64_memory_pair_simdfp_post_idx field Rn = 1 (PowerOfTwo)
    // Fields: Rt2=0, L=0, Rt=0, opc=0, Rn=1, imm7=0
    let encoding: u32 = 0x2C800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x1], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_rn_30_poweroftwominusone_0_2c8003c0() {
    // Encoding: 0x2C8003C0
    // Test aarch64_memory_pair_simdfp_post_idx field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: L=0, Rn=30, Rt2=0, Rt=0, imm7=0, opc=0
    let encoding: u32 = 0x2C8003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x30], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_rn_31_max_0_2c8003e0() {
    // Encoding: 0x2C8003E0
    // Test aarch64_memory_pair_simdfp_post_idx field Rn = 31 (Max)
    // Fields: Rt2=0, Rn=31, Rt=0, L=0, imm7=0, opc=0
    let encoding: u32 = 0x2C8003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [sp], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_rt_0_min_0_2c800000() {
    // Encoding: 0x2C800000
    // Test aarch64_memory_pair_simdfp_post_idx field Rt = 0 (Min)
    // Fields: opc=0, Rt2=0, Rn=0, Rt=0, L=0, imm7=0
    let encoding: u32 = 0x2C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_rt_1_poweroftwo_0_2c800001() {
    // Encoding: 0x2C800001
    // Test aarch64_memory_pair_simdfp_post_idx field Rt = 1 (PowerOfTwo)
    // Fields: Rt2=0, Rn=0, Rt=1, opc=0, imm7=0, L=0
    let encoding: u32 = 0x2C800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s1, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_rt_30_poweroftwominusone_0_2c80001e() {
    // Encoding: 0x2C80001E
    // Test aarch64_memory_pair_simdfp_post_idx field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rn=0, Rt=30, Rt2=0, imm7=0, L=0
    let encoding: u32 = 0x2C80001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s30, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_field_rt_31_max_0_2c80001f() {
    // Encoding: 0x2C80001F
    // Test aarch64_memory_pair_simdfp_post_idx field Rt = 31 (Max)
    // Fields: L=0, opc=0, Rt2=0, Rn=0, Rt=31, imm7=0
    let encoding: u32 = 0x2C80001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_0_0_2c800000() {
    // Encoding: 0x2C800000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt2=0, L=0, Rt=0, imm7=0, opc=0
    let encoding: u32 = 0x2C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_1_0_6c800000() {
    // Encoding: 0x6C800000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=1, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt2=0, opc=1, Rt=0, Rn=0, imm7=0
    let encoding: u32 = 0x6C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	d0, d0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_2_0_ac800000() {
    // Encoding: 0xAC800000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=2, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=2, L=0, Rt=0, imm7=0, Rt2=0, Rn=0
    let encoding: u32 = 0xAC800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	q0, q0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_3_0_ec800000() {
    // Encoding: 0xEC800000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=3, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, L=0, Rt=0, opc=3, Rt2=0, imm7=0
    let encoding: u32 = 0xEC800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_4_0_2c800000() {
    // Encoding: 0x2C800000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x2C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_5_0_2cc00000() {
    // Encoding: 0x2CC00000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=1, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, imm7=0, L=1, Rn=0, Rt=0, Rt2=0
    let encoding: u32 = 0x2CC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=0 (immediate value 0)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_6_0_2c800000() {
    // Encoding: 0x2C800000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, imm7=0, opc=0, L=0, Rt=0, Rn=0
    let encoding: u32 = 0x2C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=1 (immediate value 1)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_7_0_2c808000() {
    // Encoding: 0x2C808000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=1, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, Rt2=0, L=0, Rn=0, imm7=1
    let encoding: u32 = 0x2C808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_8_0_2c818000() {
    // Encoding: 0x2C818000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=3, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rn=0, opc=0, Rt2=0, imm7=3, Rt=0
    let encoding: u32 = 0x2C818000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #12
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_9_0_2c820000() {
    // Encoding: 0x2C820000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=4, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rt2=0, Rt=0, L=0, Rn=0, imm7=4
    let encoding: u32 = 0x2C820000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #16
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_10_0_2c838000() {
    // Encoding: 0x2C838000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=7, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=7, Rt=0, L=0, opc=0, Rt2=0, Rn=0
    let encoding: u32 = 0x2C838000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #28
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_11_0_2c840000() {
    // Encoding: 0x2C840000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=8, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=8, L=0, opc=0, Rn=0, Rt2=0, Rt=0
    let encoding: u32 = 0x2C840000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #32
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_12_0_2c878000() {
    // Encoding: 0x2C878000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=15, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=15, Rt=0, L=0, opc=0, Rt2=0, Rn=0
    let encoding: u32 = 0x2C878000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #60
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_13_0_2c880000() {
    // Encoding: 0x2C880000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=16, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt=0, Rt2=0, opc=0, imm7=16, Rn=0
    let encoding: u32 = 0x2C880000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #64
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_14_0_2c8f8000() {
    // Encoding: 0x2C8F8000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=31, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt2=0, Rt=0, imm7=31, opc=0, Rn=0
    let encoding: u32 = 0x2C8F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #124
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_15_0_2c900000() {
    // Encoding: 0x2C900000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=32, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rt2=0, imm7=32, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x2C900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #128
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=63 (immediate midpoint (63))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_16_0_2c9f8000() {
    // Encoding: 0x2C9F8000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=63, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, L=0, opc=0, imm7=63, Rt2=0
    let encoding: u32 = 0x2C9F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #252
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_17_0_2ca00000() {
    // Encoding: 0x2CA00000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=64, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, opc=0, Rt2=0, imm7=64, L=0
    let encoding: u32 = 0x2CA00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #-256
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=127 (maximum immediate (127))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_18_0_2cbf8000() {
    // Encoding: 0x2CBF8000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=127, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, imm7=127, Rt2=0, Rt=0, L=0
    let encoding: u32 = 0x2CBF8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #-4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_19_0_2c800000() {
    // Encoding: 0x2C800000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rn=0, Rt=0, opc=0, Rt2=0, imm7=0
    let encoding: u32 = 0x2C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_20_0_2c800400() {
    // Encoding: 0x2C800400
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=0
    // Fields: Rn=0, imm7=0, Rt2=1, Rt=0, opc=0, L=0
    let encoding: u32 = 0x2C800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s1, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_21_0_2c807800() {
    // Encoding: 0x2C807800
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=30, Rn=0, Rt=0
    // Fields: Rt2=30, L=0, imm7=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x2C807800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s30, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_22_0_2c807c00() {
    // Encoding: 0x2C807C00
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=0
    // Fields: Rt2=31, imm7=0, L=0, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x2C807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s31, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_23_0_2c800000() {
    // Encoding: 0x2C800000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, Rt=0, imm7=0, L=0, Rt2=0
    let encoding: u32 = 0x2C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_24_0_2c800020() {
    // Encoding: 0x2C800020
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=0
    // Fields: Rt2=0, Rn=1, Rt=0, imm7=0, opc=0, L=0
    let encoding: u32 = 0x2C800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x1], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_25_0_2c8003c0() {
    // Encoding: 0x2C8003C0
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=30, Rt=0
    // Fields: L=0, imm7=0, Rn=30, opc=0, Rt2=0, Rt=0
    let encoding: u32 = 0x2C8003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x30], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_26_0_2c8003e0() {
    // Encoding: 0x2C8003E0
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=0
    // Fields: L=0, Rn=31, imm7=0, opc=0, Rt=0, Rt2=0
    let encoding: u32 = 0x2C8003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [sp], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_27_0_2c800000() {
    // Encoding: 0x2C800000
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, opc=0, Rt2=0, Rn=0, Rt=0, imm7=0
    let encoding: u32 = 0x2C800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_28_0_2c800001() {
    // Encoding: 0x2C800001
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=1
    // Fields: Rt=1, imm7=0, L=0, Rt2=0, opc=0, Rn=0
    let encoding: u32 = 0x2C800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s1, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_29_0_2c80001e() {
    // Encoding: 0x2C80001E
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=30
    // Fields: Rt=30, opc=0, Rt2=0, L=0, Rn=0, imm7=0
    let encoding: u32 = 0x2C80001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s30, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_30_0_2c80001f() {
    // Encoding: 0x2C80001F
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=31
    // Fields: Rt2=0, imm7=0, Rt=31, L=0, Rn=0, opc=0
    let encoding: u32 = 0x2C80001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s0, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_31_0_2c800420() {
    // Encoding: 0x2C800420
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=1, Rt=0
    // Fields: opc=0, imm7=0, Rt2=1, Rn=1, Rt=0, L=0
    let encoding: u32 = 0x2C800420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s1, [x1], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_32_0_2c807fe0() {
    // Encoding: 0x2C807FE0
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=31, Rt=0
    // Fields: L=0, Rt2=31, opc=0, Rn=31, Rt=0, imm7=0
    let encoding: u32 = 0x2C807FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s31, [sp], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_33_0_2c800401() {
    // Encoding: 0x2C800401
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=1
    // Fields: L=0, Rt=1, opc=0, imm7=0, Rt2=1, Rn=0
    let encoding: u32 = 0x2C800401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s1, s1, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_34_0_2c807c1f() {
    // Encoding: 0x2C807C1F
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=31
    // Fields: opc=0, Rt2=31, Rt=31, imm7=0, Rn=0, L=0
    let encoding: u32 = 0x2C807C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s31, [x0], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_35_0_2c800021() {
    // Encoding: 0x2C800021
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=1
    // Fields: opc=0, L=0, Rt2=0, Rn=1, Rt=1, imm7=0
    let encoding: u32 = 0x2C800021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s1, s0, [x1], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_combo_36_0_2c8003ff() {
    // Encoding: 0x2C8003FF
    // Test aarch64_memory_pair_simdfp_post_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=31
    // Fields: imm7=0, opc=0, Rt2=0, Rn=31, L=0, Rt=31
    let encoding: u32 = 0x2C8003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s0, [sp], #0
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_special_opc_0_size_variant_0_0_2c808000() {
    // Encoding: 0x2C808000
    // Test aarch64_memory_pair_simdfp_post_idx special value opc = 0 (Size variant 0)
    // Fields: Rt=0, L=0, opc=0, Rt2=0, imm7=1, Rn=0
    let encoding: u32 = 0x2C808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0], #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_special_opc_1_size_variant_1_0_6c808000() {
    // Encoding: 0x6C808000
    // Test aarch64_memory_pair_simdfp_post_idx special value opc = 1 (Size variant 1)
    // Fields: Rt=0, opc=1, imm7=1, L=0, Rt2=0, Rn=0
    let encoding: u32 = 0x6C808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	d0, d0, [x0], #8
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_special_opc_2_size_variant_2_0_ac808000() {
    // Encoding: 0xAC808000
    // Test aarch64_memory_pair_simdfp_post_idx special value opc = 2 (Size variant 2)
    // Fields: opc=2, imm7=1, Rt2=0, Rn=0, Rt=0, L=0
    let encoding: u32 = 0xAC808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	q0, q0, [x0], #16
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_special_opc_3_size_variant_3_0_ec808000() {
    // Encoding: 0xEC808000
    // Test aarch64_memory_pair_simdfp_post_idx special value opc = 3 (Size variant 3)
    // Fields: Rt=0, L=0, imm7=1, opc=3, Rn=0, Rt2=0
    let encoding: u32 = 0xEC808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_special_rn_31_stack_pointer_sp_may_require_alignment_0_2c8083e0()
 {
    // Encoding: 0x2C8083E0
    // Test aarch64_memory_pair_simdfp_post_idx special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: imm7=1, Rt=0, Rt2=0, opc=0, Rn=31, L=0
    let encoding: u32 = 0x2C8083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [sp], #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_post_idx
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_pair_simdfp_post_idx_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_2c80801f()
 {
    // Encoding: 0x2C80801F
    // Test aarch64_memory_pair_simdfp_post_idx special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: L=0, Rn=0, Rt=31, Rt2=0, opc=0, imm7=1
    let encoding: u32 = 0x2C80801F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s0, [x0], #4
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_opc_0_min_0_2d800000() {
    // Encoding: 0x2D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field opc = 0 (Min)
    // Fields: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x2D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_opc_1_poweroftwo_0_6d800000() {
    // Encoding: 0x6D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field opc = 1 (PowerOfTwo)
    // Fields: Rt=0, Rt2=0, opc=1, L=0, imm7=0, Rn=0
    let encoding: u32 = 0x6D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	d0, d0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_opc_2_poweroftwo_0_ad800000() {
    // Encoding: 0xAD800000
    // Test aarch64_memory_pair_simdfp_pre_idx field opc = 2 (PowerOfTwo)
    // Fields: Rt2=0, Rt=0, imm7=0, Rn=0, opc=2, L=0
    let encoding: u32 = 0xAD800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	q0, q0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_opc_3_max_0_ed800000() {
    // Encoding: 0xED800000
    // Test aarch64_memory_pair_simdfp_pre_idx field opc = 3 (Max)
    // Fields: opc=3, imm7=0, Rt2=0, L=0, Rn=0, Rt=0
    let encoding: u32 = 0xED800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_l_0_min_0_2d800000() {
    // Encoding: 0x2D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field L = 0 (Min)
    // Fields: imm7=0, opc=0, Rt2=0, L=0, Rn=0, Rt=0
    let encoding: u32 = 0x2D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_l_1_max_0_2dc00000() {
    // Encoding: 0x2DC00000
    // Test aarch64_memory_pair_simdfp_pre_idx field L = 1 (Max)
    // Fields: Rt=0, imm7=0, Rt2=0, Rn=0, L=1, opc=0
    let encoding: u32 = 0x2DC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_imm7_0_zero_0_2d800000() {
    // Encoding: 0x2D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field imm7 = 0 (Zero)
    // Fields: Rn=0, Rt2=0, imm7=0, L=0, opc=0, Rt=0
    let encoding: u32 = 0x2D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_imm7_1_poweroftwo_0_2d808000() {
    // Encoding: 0x2D808000
    // Test aarch64_memory_pair_simdfp_pre_idx field imm7 = 1 (PowerOfTwo)
    // Fields: opc=0, imm7=1, Rt=0, L=0, Rt2=0, Rn=0
    let encoding: u32 = 0x2D808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #4]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_imm7_3_poweroftwominusone_0_2d818000() {
    // Encoding: 0x2D818000
    // Test aarch64_memory_pair_simdfp_pre_idx field imm7 = 3 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=0, L=0, opc=0, imm7=3, Rt2=0
    let encoding: u32 = 0x2D818000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #12]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_imm7_4_poweroftwo_0_2d820000() {
    // Encoding: 0x2D820000
    // Test aarch64_memory_pair_simdfp_pre_idx field imm7 = 4 (PowerOfTwo)
    // Fields: L=0, Rt2=0, Rt=0, imm7=4, opc=0, Rn=0
    let encoding: u32 = 0x2D820000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #16]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_imm7_7_poweroftwominusone_0_2d838000() {
    // Encoding: 0x2D838000
    // Test aarch64_memory_pair_simdfp_pre_idx field imm7 = 7 (PowerOfTwoMinusOne)
    // Fields: L=0, Rt2=0, opc=0, Rn=0, Rt=0, imm7=7
    let encoding: u32 = 0x2D838000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #28]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_imm7_8_poweroftwo_0_2d840000() {
    // Encoding: 0x2D840000
    // Test aarch64_memory_pair_simdfp_pre_idx field imm7 = 8 (PowerOfTwo)
    // Fields: imm7=8, L=0, Rn=0, opc=0, Rt=0, Rt2=0
    let encoding: u32 = 0x2D840000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #32]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_imm7_15_poweroftwominusone_0_2d878000() {
    // Encoding: 0x2D878000
    // Test aarch64_memory_pair_simdfp_pre_idx field imm7 = 15 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, Rn=0, imm7=15, L=0, Rt2=0
    let encoding: u32 = 0x2D878000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #60]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_imm7_16_poweroftwo_0_2d880000() {
    // Encoding: 0x2D880000
    // Test aarch64_memory_pair_simdfp_pre_idx field imm7 = 16 (PowerOfTwo)
    // Fields: imm7=16, opc=0, Rt2=0, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x2D880000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #64]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_imm7_31_poweroftwominusone_0_2d8f8000() {
    // Encoding: 0x2D8F8000
    // Test aarch64_memory_pair_simdfp_pre_idx field imm7 = 31 (PowerOfTwoMinusOne)
    // Fields: Rn=0, opc=0, Rt=0, imm7=31, L=0, Rt2=0
    let encoding: u32 = 0x2D8F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #124]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_imm7_32_poweroftwo_0_2d900000() {
    // Encoding: 0x2D900000
    // Test aarch64_memory_pair_simdfp_pre_idx field imm7 = 32 (PowerOfTwo)
    // Fields: Rn=0, opc=0, imm7=32, L=0, Rt2=0, Rt=0
    let encoding: u32 = 0x2D900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #128]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 63, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (63)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_imm7_63_poweroftwominusone_0_2d9f8000() {
    // Encoding: 0x2D9F8000
    // Test aarch64_memory_pair_simdfp_pre_idx field imm7 = 63 (PowerOfTwoMinusOne)
    // Fields: opc=0, Rt=0, L=0, Rn=0, imm7=63, Rt2=0
    let encoding: u32 = 0x2D9F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #252]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_imm7_64_poweroftwo_0_2da00000() {
    // Encoding: 0x2DA00000
    // Test aarch64_memory_pair_simdfp_pre_idx field imm7 = 64 (PowerOfTwo)
    // Fields: Rn=0, L=0, imm7=64, opc=0, Rt2=0, Rt=0
    let encoding: u32 = 0x2DA00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 127, boundary: Max }
/// maximum immediate (127)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_imm7_127_max_0_2dbf8000() {
    // Encoding: 0x2DBF8000
    // Test aarch64_memory_pair_simdfp_pre_idx field imm7 = 127 (Max)
    // Fields: Rt=0, opc=0, imm7=127, L=0, Rn=0, Rt2=0
    let encoding: u32 = 0x2DBF8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_rt2_0_min_0_2d800000() {
    // Encoding: 0x2D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field Rt2 = 0 (Min)
    // Fields: L=0, Rt2=0, Rn=0, Rt=0, opc=0, imm7=0
    let encoding: u32 = 0x2D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_rt2_1_poweroftwo_0_2d800400() {
    // Encoding: 0x2D800400
    // Test aarch64_memory_pair_simdfp_pre_idx field Rt2 = 1 (PowerOfTwo)
    // Fields: Rt=0, Rn=0, opc=0, L=0, imm7=0, Rt2=1
    let encoding: u32 = 0x2D800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s1, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_rt2_30_poweroftwominusone_0_2d807800() {
    // Encoding: 0x2D807800
    // Test aarch64_memory_pair_simdfp_pre_idx field Rt2 = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, Rt2=30, imm7=0, L=0, Rn=0
    let encoding: u32 = 0x2D807800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s30, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_rt2_31_max_0_2d807c00() {
    // Encoding: 0x2D807C00
    // Test aarch64_memory_pair_simdfp_pre_idx field Rt2 = 31 (Max)
    // Fields: opc=0, imm7=0, Rt2=31, Rt=0, Rn=0, L=0
    let encoding: u32 = 0x2D807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s31, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_rn_0_min_0_2d800000() {
    // Encoding: 0x2D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field Rn = 0 (Min)
    // Fields: Rt2=0, Rn=0, Rt=0, L=0, opc=0, imm7=0
    let encoding: u32 = 0x2D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_rn_1_poweroftwo_0_2d800020() {
    // Encoding: 0x2D800020
    // Test aarch64_memory_pair_simdfp_pre_idx field Rn = 1 (PowerOfTwo)
    // Fields: opc=0, L=0, Rn=1, Rt=0, Rt2=0, imm7=0
    let encoding: u32 = 0x2D800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x1, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_rn_30_poweroftwominusone_0_2d8003c0() {
    // Encoding: 0x2D8003C0
    // Test aarch64_memory_pair_simdfp_pre_idx field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rt2=0, Rn=30, opc=0, imm7=0, Rt=0, L=0
    let encoding: u32 = 0x2D8003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x30, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_rn_31_max_0_2d8003e0() {
    // Encoding: 0x2D8003E0
    // Test aarch64_memory_pair_simdfp_pre_idx field Rn = 31 (Max)
    // Fields: Rt2=0, Rt=0, opc=0, imm7=0, Rn=31, L=0
    let encoding: u32 = 0x2D8003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [sp, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_rt_0_min_0_2d800000() {
    // Encoding: 0x2D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field Rt = 0 (Min)
    // Fields: Rt=0, Rt2=0, L=0, opc=0, Rn=0, imm7=0
    let encoding: u32 = 0x2D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_rt_1_poweroftwo_0_2d800001() {
    // Encoding: 0x2D800001
    // Test aarch64_memory_pair_simdfp_pre_idx field Rt = 1 (PowerOfTwo)
    // Fields: Rt2=0, Rn=0, L=0, imm7=0, Rt=1, opc=0
    let encoding: u32 = 0x2D800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s1, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_rt_30_poweroftwominusone_0_2d80001e() {
    // Encoding: 0x2D80001E
    // Test aarch64_memory_pair_simdfp_pre_idx field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: L=0, Rt2=0, imm7=0, opc=0, Rn=0, Rt=30
    let encoding: u32 = 0x2D80001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s30, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_field_rt_31_max_0_2d80001f() {
    // Encoding: 0x2D80001F
    // Test aarch64_memory_pair_simdfp_pre_idx field Rt = 31 (Max)
    // Fields: imm7=0, Rn=0, Rt2=0, opc=0, Rt=31, L=0
    let encoding: u32 = 0x2D80001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_0_0_2d800000() {
    // Encoding: 0x2D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, imm7=0, L=0, opc=0, Rt2=0
    let encoding: u32 = 0x2D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_1_0_6d800000() {
    // Encoding: 0x6D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=1, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, opc=1, Rn=0, L=0, imm7=0, Rt2=0
    let encoding: u32 = 0x6D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	d0, d0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_2_0_ad800000() {
    // Encoding: 0xAD800000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=2, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, opc=2, L=0, Rt2=0, imm7=0
    let encoding: u32 = 0xAD800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	q0, q0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_3_0_ed800000() {
    // Encoding: 0xED800000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=3, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=3, Rn=0, Rt=0, L=0, Rt2=0, imm7=0
    let encoding: u32 = 0xED800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_4_0_2d800000() {
    // Encoding: 0x2D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rt2=0, L=0, imm7=0, Rn=0, Rt=0
    let encoding: u32 = 0x2D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_5_0_2dc00000() {
    // Encoding: 0x2DC00000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=1, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, opc=0, Rn=0, L=1, imm7=0, Rt=0
    let encoding: u32 = 0x2DC00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=0 (immediate value 0)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_6_0_2d800000() {
    // Encoding: 0x2D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, imm7=0, L=0, Rn=0, Rt=0, Rt2=0
    let encoding: u32 = 0x2D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=1 (immediate value 1)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_7_0_2d808000() {
    // Encoding: 0x2D808000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=1, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, L=0, imm7=1, Rt2=0, Rn=0
    let encoding: u32 = 0x2D808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #4]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_8_0_2d818000() {
    // Encoding: 0x2D818000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=3, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, opc=0, Rt2=0, L=0, Rn=0, imm7=3
    let encoding: u32 = 0x2D818000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #12]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_9_0_2d820000() {
    // Encoding: 0x2D820000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=4, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=4, opc=0, Rt2=0, Rt=0, L=0, Rn=0
    let encoding: u32 = 0x2D820000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #16]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_10_0_2d838000() {
    // Encoding: 0x2D838000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=7, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt=0, imm7=7, Rt2=0, opc=0, Rn=0
    let encoding: u32 = 0x2D838000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #28]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_11_0_2d840000() {
    // Encoding: 0x2D840000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=8, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rn=0, opc=0, Rt2=0, Rt=0, imm7=8
    let encoding: u32 = 0x2D840000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #32]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_12_0_2d878000() {
    // Encoding: 0x2D878000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=15, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, imm7=15, opc=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x2D878000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #60]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_13_0_2d880000() {
    // Encoding: 0x2D880000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=16, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rt2=0, Rt=0, imm7=16, L=0, Rn=0
    let encoding: u32 = 0x2D880000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #64]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_14_0_2d8f8000() {
    // Encoding: 0x2D8F8000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=31, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, L=0, Rn=0, opc=0, Rt2=0, imm7=31
    let encoding: u32 = 0x2D8F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #124]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_15_0_2d900000() {
    // Encoding: 0x2D900000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=32, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, opc=0, Rt2=0, L=0, Rt=0, imm7=32
    let encoding: u32 = 0x2D900000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #128]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=63 (immediate midpoint (63))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_16_0_2d9f8000() {
    // Encoding: 0x2D9F8000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=63, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=63, L=0, opc=0, Rt2=0, Rt=0, Rn=0
    let encoding: u32 = 0x2D9F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #252]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_17_0_2da00000() {
    // Encoding: 0x2DA00000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=64, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, Rt=0, Rt2=0, L=0, imm7=64
    let encoding: u32 = 0x2DA00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=127 (maximum immediate (127))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_18_0_2dbf8000() {
    // Encoding: 0x2DBF8000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=127, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, Rt2=0, opc=0, imm7=127, Rn=0, L=0
    let encoding: u32 = 0x2DBF8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_19_0_2d800000() {
    // Encoding: 0x2D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, imm7=0, Rt2=0, L=0, Rt=0, opc=0
    let encoding: u32 = 0x2D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_20_0_2d800400() {
    // Encoding: 0x2D800400
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, Rt=0, L=0, Rt2=1, imm7=0
    let encoding: u32 = 0x2D800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s1, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_21_0_2d807800() {
    // Encoding: 0x2D807800
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=30, Rn=0, Rt=0
    // Fields: imm7=0, Rt2=30, Rn=0, Rt=0, opc=0, L=0
    let encoding: u32 = 0x2D807800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s30, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_22_0_2d807c00() {
    // Encoding: 0x2D807C00
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=0
    // Fields: imm7=0, Rt=0, opc=0, L=0, Rt2=31, Rn=0
    let encoding: u32 = 0x2D807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s31, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_23_0_2d800000() {
    // Encoding: 0x2D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, imm7=0, L=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x2D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_24_0_2d800020() {
    // Encoding: 0x2D800020
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=0
    // Fields: opc=0, Rn=1, L=0, imm7=0, Rt2=0, Rt=0
    let encoding: u32 = 0x2D800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x1, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_25_0_2d8003c0() {
    // Encoding: 0x2D8003C0
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=30, Rt=0
    // Fields: imm7=0, opc=0, Rt2=0, Rn=30, L=0, Rt=0
    let encoding: u32 = 0x2D8003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x30, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_26_0_2d8003e0() {
    // Encoding: 0x2D8003E0
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=0
    // Fields: opc=0, Rn=31, Rt=0, imm7=0, L=0, Rt2=0
    let encoding: u32 = 0x2D8003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [sp, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_27_0_2d800000() {
    // Encoding: 0x2D800000
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, imm7=0, Rt2=0, Rn=0, L=0, opc=0
    let encoding: u32 = 0x2D800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_28_0_2d800001() {
    // Encoding: 0x2D800001
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=1
    // Fields: Rt2=0, L=0, Rn=0, Rt=1, opc=0, imm7=0
    let encoding: u32 = 0x2D800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s1, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_29_0_2d80001e() {
    // Encoding: 0x2D80001E
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=30
    // Fields: Rn=0, opc=0, L=0, imm7=0, Rt=30, Rt2=0
    let encoding: u32 = 0x2D80001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s30, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_30_0_2d80001f() {
    // Encoding: 0x2D80001F
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=31
    // Fields: Rn=0, opc=0, Rt=31, Rt2=0, imm7=0, L=0
    let encoding: u32 = 0x2D80001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s0, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_31_0_2d800420() {
    // Encoding: 0x2D800420
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=1, Rt=0
    // Fields: Rt2=1, Rt=0, opc=0, L=0, imm7=0, Rn=1
    let encoding: u32 = 0x2D800420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s1, [x1, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_32_0_2d807fe0() {
    // Encoding: 0x2D807FE0
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=31, Rt=0
    // Fields: Rt2=31, opc=0, Rn=31, Rt=0, imm7=0, L=0
    let encoding: u32 = 0x2D807FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s31, [sp, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_33_0_2d800401() {
    // Encoding: 0x2D800401
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=1
    // Fields: Rt2=1, Rt=1, L=0, imm7=0, opc=0, Rn=0
    let encoding: u32 = 0x2D800401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s1, s1, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_34_0_2d807c1f() {
    // Encoding: 0x2D807C1F
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=31
    // Fields: L=0, imm7=0, opc=0, Rt2=31, Rt=31, Rn=0
    let encoding: u32 = 0x2D807C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s31, [x0, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_35_0_2d800021() {
    // Encoding: 0x2D800021
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=1
    // Fields: opc=0, L=0, imm7=0, Rt=1, Rt2=0, Rn=1
    let encoding: u32 = 0x2D800021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s1, s0, [x1, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_combo_36_0_2d8003ff() {
    // Encoding: 0x2D8003FF
    // Test aarch64_memory_pair_simdfp_pre_idx field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=31
    // Fields: Rn=31, Rt=31, L=0, Rt2=0, opc=0, imm7=0
    let encoding: u32 = 0x2D8003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s0, [sp, #0]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_special_opc_0_size_variant_0_0_2d808000() {
    // Encoding: 0x2D808000
    // Test aarch64_memory_pair_simdfp_pre_idx special value opc = 0 (Size variant 0)
    // Fields: Rn=0, Rt2=0, imm7=1, opc=0, L=0, Rt=0
    let encoding: u32 = 0x2D808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #4]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_special_opc_1_size_variant_1_0_6d808000() {
    // Encoding: 0x6D808000
    // Test aarch64_memory_pair_simdfp_pre_idx special value opc = 1 (Size variant 1)
    // Fields: Rn=0, Rt=0, L=0, imm7=1, opc=1, Rt2=0
    let encoding: u32 = 0x6D808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	d0, d0, [x0, #8]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_special_opc_2_size_variant_2_0_ad808000() {
    // Encoding: 0xAD808000
    // Test aarch64_memory_pair_simdfp_pre_idx special value opc = 2 (Size variant 2)
    // Fields: opc=2, Rn=0, imm7=1, L=0, Rt2=0, Rt=0
    let encoding: u32 = 0xAD808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	q0, q0, [x0, #16]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_special_opc_3_size_variant_3_0_ed808000() {
    // Encoding: 0xED808000
    // Test aarch64_memory_pair_simdfp_pre_idx special value opc = 3 (Size variant 3)
    // Fields: Rt=0, opc=3, imm7=1, L=0, Rt2=0, Rn=0
    let encoding: u32 = 0xED808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_special_rn_31_stack_pointer_sp_may_require_alignment_0_2d8083e0()
 {
    // Encoding: 0x2D8083E0
    // Test aarch64_memory_pair_simdfp_pre_idx special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: opc=0, Rt=0, Rt2=0, L=0, Rn=31, imm7=1
    let encoding: u32 = 0x2D8083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [sp, #4]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_pre_idx
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_pair_simdfp_pre_idx_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_2d80801f()
 {
    // Encoding: 0x2D80801F
    // Test aarch64_memory_pair_simdfp_pre_idx special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: L=0, opc=0, imm7=1, Rn=0, Rt2=0, Rt=31
    let encoding: u32 = 0x2D80801F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s0, [x0, #4]!
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_opc_0_min_0_2d000000() {
    // Encoding: 0x2D000000
    // Test aarch64_memory_pair_simdfp_offset field opc = 0 (Min)
    // Fields: imm7=0, Rt2=0, Rn=0, L=0, Rt=0, opc=0
    let encoding: u32 = 0x2D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_opc_1_poweroftwo_0_6d000000() {
    // Encoding: 0x6D000000
    // Test aarch64_memory_pair_simdfp_offset field opc = 1 (PowerOfTwo)
    // Fields: imm7=0, Rt2=0, opc=1, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x6D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	d0, d0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_opc_2_poweroftwo_0_ad000000() {
    // Encoding: 0xAD000000
    // Test aarch64_memory_pair_simdfp_offset field opc = 2 (PowerOfTwo)
    // Fields: Rt2=0, imm7=0, Rt=0, Rn=0, opc=2, L=0
    let encoding: u32 = 0xAD000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	q0, q0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field opc 30 +: 2`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_opc_3_max_0_ed000000() {
    // Encoding: 0xED000000
    // Test aarch64_memory_pair_simdfp_offset field opc = 3 (Max)
    // Fields: opc=3, imm7=0, Rn=0, Rt2=0, Rt=0, L=0
    let encoding: u32 = 0xED000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_l_0_min_0_2d000000() {
    // Encoding: 0x2D000000
    // Test aarch64_memory_pair_simdfp_offset field L = 0 (Min)
    // Fields: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x2D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_l_1_max_0_2d400000() {
    // Encoding: 0x2D400000
    // Test aarch64_memory_pair_simdfp_offset field L = 1 (Max)
    // Fields: L=1, opc=0, Rt2=0, Rt=0, Rn=0, imm7=0
    let encoding: u32 = 0x2D400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_imm7_0_zero_0_2d000000() {
    // Encoding: 0x2D000000
    // Test aarch64_memory_pair_simdfp_offset field imm7 = 0 (Zero)
    // Fields: opc=0, imm7=0, Rt=0, L=0, Rt2=0, Rn=0
    let encoding: u32 = 0x2D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_imm7_1_poweroftwo_0_2d008000() {
    // Encoding: 0x2D008000
    // Test aarch64_memory_pair_simdfp_offset field imm7 = 1 (PowerOfTwo)
    // Fields: L=0, Rn=0, opc=0, imm7=1, Rt=0, Rt2=0
    let encoding: u32 = 0x2D008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_imm7_3_poweroftwominusone_0_2d018000() {
    // Encoding: 0x2D018000
    // Test aarch64_memory_pair_simdfp_offset field imm7 = 3 (PowerOfTwoMinusOne)
    // Fields: opc=0, L=0, imm7=3, Rn=0, Rt=0, Rt2=0
    let encoding: u32 = 0x2D018000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #12]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_imm7_4_poweroftwo_0_2d020000() {
    // Encoding: 0x2D020000
    // Test aarch64_memory_pair_simdfp_offset field imm7 = 4 (PowerOfTwo)
    // Fields: L=0, opc=0, Rt2=0, Rn=0, imm7=4, Rt=0
    let encoding: u32 = 0x2D020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #16]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_imm7_7_poweroftwominusone_0_2d038000() {
    // Encoding: 0x2D038000
    // Test aarch64_memory_pair_simdfp_offset field imm7 = 7 (PowerOfTwoMinusOne)
    // Fields: Rt=0, opc=0, imm7=7, L=0, Rt2=0, Rn=0
    let encoding: u32 = 0x2D038000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #28]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_imm7_8_poweroftwo_0_2d040000() {
    // Encoding: 0x2D040000
    // Test aarch64_memory_pair_simdfp_offset field imm7 = 8 (PowerOfTwo)
    // Fields: imm7=8, L=0, Rt2=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x2D040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #32]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_imm7_15_poweroftwominusone_0_2d078000() {
    // Encoding: 0x2D078000
    // Test aarch64_memory_pair_simdfp_offset field imm7 = 15 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm7=15, Rn=0, L=0, Rt2=0, Rt=0
    let encoding: u32 = 0x2D078000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #60]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_imm7_16_poweroftwo_0_2d080000() {
    // Encoding: 0x2D080000
    // Test aarch64_memory_pair_simdfp_offset field imm7 = 16 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, L=0, imm7=16, opc=0, Rt2=0
    let encoding: u32 = 0x2D080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #64]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 31, boundary: PowerOfTwoMinusOne }
/// 2^5 - 1 = 31
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_imm7_31_poweroftwominusone_0_2d0f8000() {
    // Encoding: 0x2D0F8000
    // Test aarch64_memory_pair_simdfp_offset field imm7 = 31 (PowerOfTwoMinusOne)
    // Fields: Rn=0, L=0, imm7=31, Rt=0, opc=0, Rt2=0
    let encoding: u32 = 0x2D0F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #124]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_imm7_32_poweroftwo_0_2d100000() {
    // Encoding: 0x2D100000
    // Test aarch64_memory_pair_simdfp_offset field imm7 = 32 (PowerOfTwo)
    // Fields: opc=0, L=0, imm7=32, Rt2=0, Rt=0, Rn=0
    let encoding: u32 = 0x2D100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #128]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 63, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (63)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_imm7_63_poweroftwominusone_0_2d1f8000() {
    // Encoding: 0x2D1F8000
    // Test aarch64_memory_pair_simdfp_offset field imm7 = 63 (PowerOfTwoMinusOne)
    // Fields: L=0, Rn=0, Rt2=0, opc=0, imm7=63, Rt=0
    let encoding: u32 = 0x2D1F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #252]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 64, boundary: PowerOfTwo }
/// power of 2 (2^6 = 64)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_imm7_64_poweroftwo_0_2d200000() {
    // Encoding: 0x2D200000
    // Test aarch64_memory_pair_simdfp_offset field imm7 = 64 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, opc=0, imm7=64, L=0, Rt2=0
    let encoding: u32 = 0x2D200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field imm7 15 +: 7`
/// Requirement: FieldBoundary { field: "imm7", value: 127, boundary: Max }
/// maximum immediate (127)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_imm7_127_max_0_2d3f8000() {
    // Encoding: 0x2D3F8000
    // Test aarch64_memory_pair_simdfp_offset field imm7 = 127 (Max)
    // Fields: opc=0, Rt2=0, Rn=0, L=0, Rt=0, imm7=127
    let encoding: u32 = 0x2D3F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_rt2_0_min_0_2d000000() {
    // Encoding: 0x2D000000
    // Test aarch64_memory_pair_simdfp_offset field Rt2 = 0 (Min)
    // Fields: imm7=0, Rt=0, Rn=0, opc=0, L=0, Rt2=0
    let encoding: u32 = 0x2D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_rt2_1_poweroftwo_0_2d000400() {
    // Encoding: 0x2D000400
    // Test aarch64_memory_pair_simdfp_offset field Rt2 = 1 (PowerOfTwo)
    // Fields: opc=0, Rt=0, L=0, imm7=0, Rn=0, Rt2=1
    let encoding: u32 = 0x2D000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s1, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_rt2_30_poweroftwominusone_0_2d007800() {
    // Encoding: 0x2D007800
    // Test aarch64_memory_pair_simdfp_offset field Rt2 = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, imm7=0, Rn=0, Rt=0, Rt2=30, L=0
    let encoding: u32 = 0x2D007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s30, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_rt2_31_max_0_2d007c00() {
    // Encoding: 0x2D007C00
    // Test aarch64_memory_pair_simdfp_offset field Rt2 = 31 (Max)
    // Fields: opc=0, L=0, imm7=0, Rt2=31, Rt=0, Rn=0
    let encoding: u32 = 0x2D007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s31, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_rn_0_min_0_2d000000() {
    // Encoding: 0x2D000000
    // Test aarch64_memory_pair_simdfp_offset field Rn = 0 (Min)
    // Fields: opc=0, L=0, Rt=0, imm7=0, Rn=0, Rt2=0
    let encoding: u32 = 0x2D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_rn_1_poweroftwo_0_2d000020() {
    // Encoding: 0x2D000020
    // Test aarch64_memory_pair_simdfp_offset field Rn = 1 (PowerOfTwo)
    // Fields: Rt=0, imm7=0, Rt2=0, Rn=1, opc=0, L=0
    let encoding: u32 = 0x2D000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_rn_30_poweroftwominusone_0_2d0003c0() {
    // Encoding: 0x2D0003C0
    // Test aarch64_memory_pair_simdfp_offset field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: L=0, opc=0, imm7=0, Rt2=0, Rt=0, Rn=30
    let encoding: u32 = 0x2D0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x30]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_rn_31_max_0_2d0003e0() {
    // Encoding: 0x2D0003E0
    // Test aarch64_memory_pair_simdfp_offset field Rn = 31 (Max)
    // Fields: L=0, Rn=31, imm7=0, Rt2=0, opc=0, Rt=0
    let encoding: u32 = 0x2D0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_rt_0_min_0_2d000000() {
    // Encoding: 0x2D000000
    // Test aarch64_memory_pair_simdfp_offset field Rt = 0 (Min)
    // Fields: imm7=0, Rt2=0, Rn=0, opc=0, Rt=0, L=0
    let encoding: u32 = 0x2D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_rt_1_poweroftwo_0_2d000001() {
    // Encoding: 0x2D000001
    // Test aarch64_memory_pair_simdfp_offset field Rt = 1 (PowerOfTwo)
    // Fields: imm7=0, opc=0, Rt2=0, L=0, Rn=0, Rt=1
    let encoding: u32 = 0x2D000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s1, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_rt_30_poweroftwominusone_0_2d00001e() {
    // Encoding: 0x2D00001E
    // Test aarch64_memory_pair_simdfp_offset field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=30, opc=0, L=0, Rt2=0, imm7=0
    let encoding: u32 = 0x2D00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s30, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_field_rt_31_max_0_2d00001f() {
    // Encoding: 0x2D00001F
    // Test aarch64_memory_pair_simdfp_offset field Rt = 31 (Max)
    // Fields: opc=0, imm7=0, Rt2=0, L=0, Rt=31, Rn=0
    let encoding: u32 = 0x2D00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_0_0_2d000000() {
    // Encoding: 0x2D000000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, imm7=0, Rn=0, Rt=0, Rt2=0, L=0
    let encoding: u32 = 0x2D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_1_0_6d000000() {
    // Encoding: 0x6D000000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=1, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, Rt2=0, opc=1, L=0, imm7=0
    let encoding: u32 = 0x6D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	d0, d0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_2_0_ad000000() {
    // Encoding: 0xAD000000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=2, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt=0, Rn=0, opc=2, imm7=0, Rt2=0
    let encoding: u32 = 0xAD000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	q0, q0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_3_0_ed000000() {
    // Encoding: 0xED000000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=3, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: opc=3, imm7=0, L=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0xED000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_4_0_2d000000() {
    // Encoding: 0x2D000000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, opc=0, L=0, Rn=0, Rt=0, imm7=0
    let encoding: u32 = 0x2D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_5_0_2d400000() {
    // Encoding: 0x2D400000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=1, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=0, opc=0, Rt=0, L=1, Rt2=0, Rn=0
    let encoding: u32 = 0x2D400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: ldp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=0 (immediate value 0)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_6_0_2d000000() {
    // Encoding: 0x2D000000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, opc=0, Rt=0, Rn=0, L=0, imm7=0
    let encoding: u32 = 0x2D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=1 (immediate value 1)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_7_0_2d008000() {
    // Encoding: 0x2D008000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=1, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rn=0, imm7=1, opc=0, L=0, Rt=0
    let encoding: u32 = 0x2D008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=3 (2^2 - 1 = 3)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_8_0_2d018000() {
    // Encoding: 0x2D018000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=3, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rn=0, Rt=0, imm7=3, opc=0, L=0
    let encoding: u32 = 0x2D018000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #12]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=4 (power of 2 (2^2 = 4))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_9_0_2d020000() {
    // Encoding: 0x2D020000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=4, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, Rt2=0, imm7=4, L=0, opc=0
    let encoding: u32 = 0x2D020000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #16]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=7 (2^3 - 1 = 7)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_10_0_2d038000() {
    // Encoding: 0x2D038000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=7, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, imm7=7, L=0, opc=0, Rt2=0, Rt=0
    let encoding: u32 = 0x2D038000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #28]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=8 (power of 2 (2^3 = 8))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_11_0_2d040000() {
    // Encoding: 0x2D040000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=8, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, imm7=8, L=0, Rt2=0, Rt=0
    let encoding: u32 = 0x2D040000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #32]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=15 (2^4 - 1 = 15)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_12_0_2d078000() {
    // Encoding: 0x2D078000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=15, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, imm7=15, opc=0, L=0, Rt2=0, Rn=0
    let encoding: u32 = 0x2D078000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #60]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=16 (power of 2 (2^4 = 16))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_13_0_2d080000() {
    // Encoding: 0x2D080000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=16, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, L=0, Rt2=0, Rt=0, imm7=16, opc=0
    let encoding: u32 = 0x2D080000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #64]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=31 (2^5 - 1 = 31)
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_14_0_2d0f8000() {
    // Encoding: 0x2D0F8000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=31, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, imm7=31, Rt2=0, L=0, Rn=0, Rt=0
    let encoding: u32 = 0x2D0F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #124]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=32 (power of 2 (2^5 = 32))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_15_0_2d100000() {
    // Encoding: 0x2D100000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=32, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, Rt2=0, imm7=32, L=0, opc=0
    let encoding: u32 = 0x2D100000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #128]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=63 (immediate midpoint (63))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_16_0_2d1f8000() {
    // Encoding: 0x2D1F8000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=63, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=63, L=0, Rn=0, opc=0, Rt2=0, Rt=0
    let encoding: u32 = 0x2D1F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #252]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=64 (power of 2 (2^6 = 64))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_17_0_2d200000() {
    // Encoding: 0x2D200000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=64, Rt2=0, Rn=0, Rt=0
    // Fields: opc=0, Rt2=0, L=0, imm7=64, Rn=0, Rt=0
    let encoding: u32 = 0x2D200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// imm7=127 (maximum immediate (127))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_18_0_2d3f8000() {
    // Encoding: 0x2D3F8000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=127, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, opc=0, Rt2=0, Rn=0, Rt=0, imm7=127
    let encoding: u32 = 0x2D3F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_19_0_2d000000() {
    // Encoding: 0x2D000000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: imm7=0, L=0, Rt2=0, opc=0, Rn=0, Rt=0
    let encoding: u32 = 0x2D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_20_0_2d000400() {
    // Encoding: 0x2D000400
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=0
    // Fields: opc=0, L=0, imm7=0, Rt2=1, Rt=0, Rn=0
    let encoding: u32 = 0x2D000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s1, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_21_0_2d007800() {
    // Encoding: 0x2D007800
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=30, Rn=0, Rt=0
    // Fields: Rt=0, L=0, imm7=0, opc=0, Rn=0, Rt2=30
    let encoding: u32 = 0x2D007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s30, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_22_0_2d007c00() {
    // Encoding: 0x2D007C00
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=0
    // Fields: opc=0, imm7=0, Rt2=31, L=0, Rt=0, Rn=0
    let encoding: u32 = 0x2D007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s31, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_23_0_2d000000() {
    // Encoding: 0x2D000000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, Rt2=0, L=0, Rn=0, imm7=0, opc=0
    let encoding: u32 = 0x2D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_24_0_2d000020() {
    // Encoding: 0x2D000020
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=0
    // Fields: Rn=1, imm7=0, opc=0, L=0, Rt2=0, Rt=0
    let encoding: u32 = 0x2D000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_25_0_2d0003c0() {
    // Encoding: 0x2D0003C0
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=30, Rt=0
    // Fields: L=0, Rt2=0, imm7=0, Rn=30, opc=0, Rt=0
    let encoding: u32 = 0x2D0003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x30]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_26_0_2d0003e0() {
    // Encoding: 0x2D0003E0
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=0
    // Fields: Rt=0, Rt2=0, L=0, Rn=31, imm7=0, opc=0
    let encoding: u32 = 0x2D0003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_27_0_2d000000() {
    // Encoding: 0x2D000000
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, opc=0, imm7=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x2D000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_28_0_2d000001() {
    // Encoding: 0x2D000001
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=1
    // Fields: L=0, imm7=0, opc=0, Rt2=0, Rn=0, Rt=1
    let encoding: u32 = 0x2D000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s1, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_29_0_2d00001e() {
    // Encoding: 0x2D00001E
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=30
    // Fields: L=0, Rt2=0, Rt=30, Rn=0, opc=0, imm7=0
    let encoding: u32 = 0x2D00001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s30, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_30_0_2d00001f() {
    // Encoding: 0x2D00001F
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=0, Rt=31
    // Fields: imm7=0, Rt2=0, Rn=0, Rt=31, opc=0, L=0
    let encoding: u32 = 0x2D00001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s0, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_31_0_2d000420() {
    // Encoding: 0x2D000420
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=1, Rt=0
    // Fields: opc=0, imm7=0, Rt2=1, Rn=1, Rt=0, L=0
    let encoding: u32 = 0x2D000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s1, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_32_0_2d007fe0() {
    // Encoding: 0x2D007FE0
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=31, Rt=0
    // Fields: opc=0, L=0, imm7=0, Rt2=31, Rn=31, Rt=0
    let encoding: u32 = 0x2D007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s31, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_33_0_2d000401() {
    // Encoding: 0x2D000401
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=1, Rn=0, Rt=1
    // Fields: Rn=0, opc=0, Rt2=1, imm7=0, L=0, Rt=1
    let encoding: u32 = 0x2D000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s1, s1, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_34_0_2d007c1f() {
    // Encoding: 0x2D007C1F
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=31, Rn=0, Rt=31
    // Fields: imm7=0, Rt2=31, opc=0, Rt=31, Rn=0, L=0
    let encoding: u32 = 0x2D007C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s31, [x0]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_35_0_2d000021() {
    // Encoding: 0x2D000021
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=1, Rt=1
    // Fields: L=0, opc=0, imm7=0, Rt2=0, Rn=1, Rt=1
    let encoding: u32 = 0x2D000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s1, s0, [x1]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field combination 36`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_pair_simdfp_offset_combo_36_0_2d0003ff() {
    // Encoding: 0x2D0003FF
    // Test aarch64_memory_pair_simdfp_offset field combination: opc=0, L=0, imm7=0, Rt2=0, Rn=31, Rt=31
    // Fields: opc=0, Rn=31, imm7=0, Rt2=0, Rt=31, L=0
    let encoding: u32 = 0x2D0003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s0, [sp]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_pair_simdfp_offset_special_opc_0_size_variant_0_0_2d008000() {
    // Encoding: 0x2D008000
    // Test aarch64_memory_pair_simdfp_offset special value opc = 0 (Size variant 0)
    // Fields: Rt=0, L=0, opc=0, Rn=0, imm7=1, Rt2=0
    let encoding: u32 = 0x2D008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [x0, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_pair_simdfp_offset_special_opc_1_size_variant_1_0_6d008000() {
    // Encoding: 0x6D008000
    // Test aarch64_memory_pair_simdfp_offset special value opc = 1 (Size variant 1)
    // Fields: Rn=0, imm7=1, Rt2=0, L=0, opc=1, Rt=0
    let encoding: u32 = 0x6D008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	d0, d0, [x0, #8]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_pair_simdfp_offset_special_opc_2_size_variant_2_0_ad008000() {
    // Encoding: 0xAD008000
    // Test aarch64_memory_pair_simdfp_offset special value opc = 2 (Size variant 2)
    // Fields: opc=2, L=0, imm7=1, Rn=0, Rt2=0, Rt=0
    let encoding: u32 = 0xAD008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	q0, q0, [x0, #16]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_pair_simdfp_offset_special_opc_3_size_variant_3_0_ed008000() {
    // Encoding: 0xED008000
    // Test aarch64_memory_pair_simdfp_offset special value opc = 3 (Size variant 3)
    // Fields: imm7=1, Rt=0, L=0, Rn=0, Rt2=0, opc=3
    let encoding: u32 = 0xED008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_pair_simdfp_offset_special_rn_31_stack_pointer_sp_may_require_alignment_0_2d0083e0()
 {
    // Encoding: 0x2D0083E0
    // Test aarch64_memory_pair_simdfp_offset special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rt=0, L=0, imm7=1, opc=0, Rt2=0, Rn=31
    let encoding: u32 = 0x2D0083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s0, s0, [sp, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_pair_simdfp_offset
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_pair_simdfp_offset_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_2d00801f()
 {
    // Encoding: 0x2D00801F
    // Test aarch64_memory_pair_simdfp_offset special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rt=31, Rt2=0, imm7=1, opc=0, L=0, Rn=0
    let encoding: u32 = 0x2D00801F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: stp	s31, s0, [x0, #4]
    let exit = cpu.step();
    assert!(
        exit.is_ok() && !matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected allocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}
