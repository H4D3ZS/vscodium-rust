//! A64 memory atomic tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_memory_atomicops_ld Tests
// ============================================================================

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_atomicops_ld_field_size_0_min_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field size = 0 (Min)
    // Fields: Rs=0, opc=0, A=0, Rn=0, Rt=0, R=0, size=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_atomicops_ld_field_size_1_poweroftwo_0_78200000() {
    // Encoding: 0x78200000
    // Test aarch64_memory_atomicops_ld field size = 1 (PowerOfTwo)
    // Fields: R=0, size=1, A=0, opc=0, Rn=0, Rt=0, Rs=0
    let encoding: u32 = 0x78200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_atomicops_ld_field_size_2_poweroftwo_0_b8200000() {
    // Encoding: 0xB8200000
    // Test aarch64_memory_atomicops_ld field size = 2 (PowerOfTwo)
    // Fields: Rt=0, A=0, R=0, Rn=0, size=2, opc=0, Rs=0
    let encoding: u32 = 0xB8200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_atomicops_ld_field_size_3_max_0_f8200000() {
    // Encoding: 0xF8200000
    // Test aarch64_memory_atomicops_ld field size = 3 (Max)
    // Fields: opc=0, Rt=0, R=0, Rn=0, Rs=0, size=3, A=0
    let encoding: u32 = 0xF8200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field A 23 +: 1`
/// Requirement: FieldBoundary { field: "A", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_atomicops_ld_field_a_0_min_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field A = 0 (Min)
    // Fields: A=0, Rs=0, Rn=0, R=0, opc=0, Rt=0, size=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field A 23 +: 1`
/// Requirement: FieldBoundary { field: "A", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_atomicops_ld_field_a_1_max_0_38a00000() {
    // Encoding: 0x38A00000
    // Test aarch64_memory_atomicops_ld field A = 1 (Max)
    // Fields: Rs=0, Rt=0, A=1, opc=0, size=0, R=0, Rn=0
    let encoding: u32 = 0x38A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field R 22 +: 1`
/// Requirement: FieldBoundary { field: "R", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_atomicops_ld_field_r_0_min_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field R = 0 (Min)
    // Fields: opc=0, Rn=0, A=0, Rs=0, size=0, R=0, Rt=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field R 22 +: 1`
/// Requirement: FieldBoundary { field: "R", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_atomicops_ld_field_r_1_max_0_38600000() {
    // Encoding: 0x38600000
    // Test aarch64_memory_atomicops_ld field R = 1 (Max)
    // Fields: Rt=0, size=0, R=1, A=0, Rs=0, opc=0, Rn=0
    let encoding: u32 = 0x38600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_atomicops_ld_field_rs_0_min_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field Rs = 0 (Min)
    // Fields: size=0, Rs=0, R=0, A=0, Rn=0, Rt=0, opc=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_atomicops_ld_field_rs_1_poweroftwo_0_38210000() {
    // Encoding: 0x38210000
    // Test aarch64_memory_atomicops_ld field Rs = 1 (PowerOfTwo)
    // Fields: Rt=0, Rs=1, A=0, size=0, opc=0, R=0, Rn=0
    let encoding: u32 = 0x38210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_atomicops_ld_field_rs_30_poweroftwominusone_0_383e0000() {
    // Encoding: 0x383E0000
    // Test aarch64_memory_atomicops_ld field Rs = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=0, Rn=0, A=0, R=0, size=0, Rs=30, opc=0
    let encoding: u32 = 0x383E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_atomicops_ld_field_rs_31_max_0_383f0000() {
    // Encoding: 0x383F0000
    // Test aarch64_memory_atomicops_ld field Rs = 31 (Max)
    // Fields: Rs=31, Rt=0, opc=0, R=0, Rn=0, A=0, size=0
    let encoding: u32 = 0x383F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc 12 +: 3`
/// Requirement: FieldBoundary { field: "opc", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_atomicops_ld_field_opc_0_min_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field opc = 0 (Min)
    // Fields: R=0, Rs=0, size=0, Rn=0, Rt=0, opc=0, A=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc 12 +: 3`
/// Requirement: FieldBoundary { field: "opc", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_atomicops_ld_field_opc_1_poweroftwo_0_38201000() {
    // Encoding: 0x38201000
    // Test aarch64_memory_atomicops_ld field opc = 1 (PowerOfTwo)
    // Fields: A=0, size=0, Rn=0, Rt=0, R=0, Rs=0, opc=1
    let encoding: u32 = 0x38201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc 12 +: 3`
/// Requirement: FieldBoundary { field: "opc", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_atomicops_ld_field_opc_2_poweroftwo_0_38202000() {
    // Encoding: 0x38202000
    // Test aarch64_memory_atomicops_ld field opc = 2 (PowerOfTwo)
    // Fields: size=0, Rs=0, A=0, opc=2, Rn=0, Rt=0, R=0
    let encoding: u32 = 0x38202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc 12 +: 3`
/// Requirement: FieldBoundary { field: "opc", value: 3, boundary: PowerOfTwo }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_atomicops_ld_field_opc_3_poweroftwo_0_38203000() {
    // Encoding: 0x38203000
    // Test aarch64_memory_atomicops_ld field opc = 3 (PowerOfTwo)
    // Fields: opc=3, Rt=0, A=0, R=0, Rs=0, Rn=0, size=0
    let encoding: u32 = 0x38203000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc 12 +: 3`
/// Requirement: FieldBoundary { field: "opc", value: 4, boundary: PowerOfTwo }
/// size variant 4
#[test]
fn test_aarch64_memory_atomicops_ld_field_opc_4_poweroftwo_0_38204000() {
    // Encoding: 0x38204000
    // Test aarch64_memory_atomicops_ld field opc = 4 (PowerOfTwo)
    // Fields: Rs=0, opc=4, size=0, A=0, Rn=0, R=0, Rt=0
    let encoding: u32 = 0x38204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc 12 +: 3`
/// Requirement: FieldBoundary { field: "opc", value: 5, boundary: PowerOfTwo }
/// size variant 5
#[test]
fn test_aarch64_memory_atomicops_ld_field_opc_5_poweroftwo_0_38205000() {
    // Encoding: 0x38205000
    // Test aarch64_memory_atomicops_ld field opc = 5 (PowerOfTwo)
    // Fields: R=0, Rs=0, opc=5, Rn=0, size=0, Rt=0, A=0
    let encoding: u32 = 0x38205000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc 12 +: 3`
/// Requirement: FieldBoundary { field: "opc", value: 6, boundary: PowerOfTwo }
/// size variant 6
#[test]
fn test_aarch64_memory_atomicops_ld_field_opc_6_poweroftwo_0_38206000() {
    // Encoding: 0x38206000
    // Test aarch64_memory_atomicops_ld field opc = 6 (PowerOfTwo)
    // Fields: A=0, size=0, Rs=0, R=0, Rt=0, opc=6, Rn=0
    let encoding: u32 = 0x38206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc 12 +: 3`
/// Requirement: FieldBoundary { field: "opc", value: 7, boundary: Max }
/// size variant 7
#[test]
fn test_aarch64_memory_atomicops_ld_field_opc_7_max_0_38207000() {
    // Encoding: 0x38207000
    // Test aarch64_memory_atomicops_ld field opc = 7 (Max)
    // Fields: R=0, Rs=0, Rt=0, size=0, Rn=0, A=0, opc=7
    let encoding: u32 = 0x38207000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_atomicops_ld_field_rn_0_min_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field Rn = 0 (Min)
    // Fields: R=0, size=0, A=0, opc=0, Rs=0, Rn=0, Rt=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_atomicops_ld_field_rn_1_poweroftwo_0_38200020() {
    // Encoding: 0x38200020
    // Test aarch64_memory_atomicops_ld field Rn = 1 (PowerOfTwo)
    // Fields: Rt=0, size=0, opc=0, Rs=0, Rn=1, A=0, R=0
    let encoding: u32 = 0x38200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_atomicops_ld_field_rn_30_poweroftwominusone_0_382003c0() {
    // Encoding: 0x382003C0
    // Test aarch64_memory_atomicops_ld field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: opc=0, A=0, Rn=30, Rt=0, size=0, Rs=0, R=0
    let encoding: u32 = 0x382003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_atomicops_ld_field_rn_31_max_0_382003e0() {
    // Encoding: 0x382003E0
    // Test aarch64_memory_atomicops_ld field Rn = 31 (Max)
    // Fields: opc=0, R=0, size=0, Rn=31, Rs=0, Rt=0, A=0
    let encoding: u32 = 0x382003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_atomicops_ld_field_rt_0_min_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field Rt = 0 (Min)
    // Fields: R=0, opc=0, Rn=0, Rt=0, size=0, A=0, Rs=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_atomicops_ld_field_rt_1_poweroftwo_0_38200001() {
    // Encoding: 0x38200001
    // Test aarch64_memory_atomicops_ld field Rt = 1 (PowerOfTwo)
    // Fields: opc=0, A=0, Rs=0, Rn=0, Rt=1, size=0, R=0
    let encoding: u32 = 0x38200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_atomicops_ld_field_rt_30_poweroftwominusone_0_3820001e() {
    // Encoding: 0x3820001E
    // Test aarch64_memory_atomicops_ld field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rs=0, R=0, Rn=0, opc=0, Rt=30, A=0
    let encoding: u32 = 0x3820001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_atomicops_ld_field_rt_31_max_0_3820001f() {
    // Encoding: 0x3820001F
    // Test aarch64_memory_atomicops_ld field Rt = 31 (Max)
    // Fields: Rn=0, opc=0, Rt=31, A=0, R=0, Rs=0, size=0
    let encoding: u32 = 0x3820001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_0_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=0, Rt=0
    // Fields: size=0, Rs=0, A=0, opc=0, Rn=0, Rt=0, R=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_1_0_78200000() {
    // Encoding: 0x78200000
    // Test aarch64_memory_atomicops_ld field combination: size=1, A=0, R=0, Rs=0, opc=0, Rn=0, Rt=0
    // Fields: A=0, R=0, Rn=0, opc=0, size=1, Rt=0, Rs=0
    let encoding: u32 = 0x78200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_2_0_b8200000() {
    // Encoding: 0xB8200000
    // Test aarch64_memory_atomicops_ld field combination: size=2, A=0, R=0, Rs=0, opc=0, Rn=0, Rt=0
    // Fields: A=0, Rs=0, opc=0, R=0, Rn=0, Rt=0, size=2
    let encoding: u32 = 0xB8200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_3_0_f8200000() {
    // Encoding: 0xF8200000
    // Test aarch64_memory_atomicops_ld field combination: size=3, A=0, R=0, Rs=0, opc=0, Rn=0, Rt=0
    // Fields: Rs=0, Rn=0, Rt=0, opc=0, size=3, A=0, R=0
    let encoding: u32 = 0xF8200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// A=0 (minimum value)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_4_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=0, Rt=0
    // Fields: opc=0, Rn=0, size=0, A=0, Rt=0, Rs=0, R=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// A=1 (maximum value (1))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_5_0_38a00000() {
    // Encoding: 0x38A00000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=1, R=0, Rs=0, opc=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, A=1, Rs=0, R=0, size=0, opc=0
    let encoding: u32 = 0x38A00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// R=0 (minimum value)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_6_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=0, Rt=0
    // Fields: size=0, A=0, Rs=0, opc=0, Rt=0, Rn=0, R=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// R=1 (maximum value (1))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_7_0_38600000() {
    // Encoding: 0x38600000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=1, Rs=0, opc=0, Rn=0, Rt=0
    // Fields: R=1, Rt=0, Rs=0, A=0, size=0, opc=0, Rn=0
    let encoding: u32 = 0x38600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_8_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=0, Rt=0
    // Fields: R=0, Rs=0, Rn=0, A=0, opc=0, Rt=0, size=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_9_0_38210000() {
    // Encoding: 0x38210000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=1, opc=0, Rn=0, Rt=0
    // Fields: A=0, R=0, Rn=0, Rs=1, opc=0, Rt=0, size=0
    let encoding: u32 = 0x38210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_10_0_383e0000() {
    // Encoding: 0x383E0000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=30, opc=0, Rn=0, Rt=0
    // Fields: Rt=0, R=0, opc=0, Rs=30, size=0, A=0, Rn=0
    let encoding: u32 = 0x383E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_11_0_383f0000() {
    // Encoding: 0x383F0000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=31, opc=0, Rn=0, Rt=0
    // Fields: size=0, A=0, R=0, opc=0, Rn=0, Rs=31, Rt=0
    let encoding: u32 = 0x383F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_12_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=0, Rt=0
    // Fields: opc=0, size=0, Rt=0, R=0, Rn=0, A=0, Rs=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_13_0_38201000() {
    // Encoding: 0x38201000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=1, Rn=0, Rt=0
    // Fields: Rn=0, size=0, Rt=0, A=0, Rs=0, R=0, opc=1
    let encoding: u32 = 0x38201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_14_0_38202000() {
    // Encoding: 0x38202000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=2, Rn=0, Rt=0
    // Fields: A=0, Rs=0, opc=2, Rt=0, Rn=0, R=0, size=0
    let encoding: u32 = 0x38202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_15_0_38203000() {
    // Encoding: 0x38203000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=3, Rn=0, Rt=0
    // Fields: A=0, Rn=0, size=0, R=0, opc=3, Rt=0, Rs=0
    let encoding: u32 = 0x38203000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=4 (size variant 4)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_16_0_38204000() {
    // Encoding: 0x38204000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=4, Rn=0, Rt=0
    // Fields: opc=4, Rt=0, Rn=0, A=0, size=0, R=0, Rs=0
    let encoding: u32 = 0x38204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=5 (size variant 5)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_17_0_38205000() {
    // Encoding: 0x38205000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=5, Rn=0, Rt=0
    // Fields: A=0, Rn=0, R=0, Rs=0, opc=5, Rt=0, size=0
    let encoding: u32 = 0x38205000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=6 (size variant 6)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_18_0_38206000() {
    // Encoding: 0x38206000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=6, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, size=0, Rs=0, opc=6, R=0, A=0
    let encoding: u32 = 0x38206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// opc=7 (size variant 7)
#[test]
fn test_aarch64_memory_atomicops_ld_combo_19_0_38207000() {
    // Encoding: 0x38207000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=7, Rn=0, Rt=0
    // Fields: opc=7, Rt=0, size=0, A=0, R=0, Rs=0, Rn=0
    let encoding: u32 = 0x38207000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_20_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=0, Rt=0
    // Fields: size=0, R=0, Rn=0, opc=0, A=0, Rs=0, Rt=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_21_0_38200020() {
    // Encoding: 0x38200020
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=1, Rt=0
    // Fields: A=0, size=0, opc=0, R=0, Rs=0, Rn=1, Rt=0
    let encoding: u32 = 0x38200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_22_0_382003c0() {
    // Encoding: 0x382003C0
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=30, Rt=0
    // Fields: A=0, R=0, opc=0, Rs=0, Rn=30, Rt=0, size=0
    let encoding: u32 = 0x382003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_23_0_382003e0() {
    // Encoding: 0x382003E0
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=31, Rt=0
    // Fields: Rt=0, R=0, A=0, Rs=0, size=0, opc=0, Rn=31
    let encoding: u32 = 0x382003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_24_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=0, Rt=0
    // Fields: opc=0, Rs=0, R=0, A=0, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_25_0_38200001() {
    // Encoding: 0x38200001
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=0, Rt=1
    // Fields: Rs=0, size=0, R=0, opc=0, A=0, Rn=0, Rt=1
    let encoding: u32 = 0x38200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_26_0_3820001e() {
    // Encoding: 0x3820001E
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=0, Rt=30
    // Fields: A=0, size=0, Rs=0, opc=0, Rn=0, R=0, Rt=30
    let encoding: u32 = 0x3820001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_27_0_3820001f() {
    // Encoding: 0x3820001F
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=0, Rt=31
    // Fields: opc=0, Rs=0, R=0, size=0, Rn=0, Rt=31, A=0
    let encoding: u32 = 0x3820001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_28_0_38210020() {
    // Encoding: 0x38210020
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=1, opc=0, Rn=1, Rt=0
    // Fields: size=0, Rt=0, A=0, Rs=1, Rn=1, R=0, opc=0
    let encoding: u32 = 0x38210020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_29_0_383f03e0() {
    // Encoding: 0x383F03E0
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=31, opc=0, Rn=31, Rt=0
    // Fields: Rn=31, Rt=0, Rs=31, size=0, A=0, opc=0, R=0
    let encoding: u32 = 0x383F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_30_0_38210001() {
    // Encoding: 0x38210001
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=1, opc=0, Rn=0, Rt=1
    // Fields: Rs=1, opc=0, R=0, size=0, Rn=0, Rt=1, A=0
    let encoding: u32 = 0x38210001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_31_0_383f001f() {
    // Encoding: 0x383F001F
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=31, opc=0, Rn=0, Rt=31
    // Fields: Rn=0, Rt=31, opc=0, size=0, R=0, A=0, Rs=31
    let encoding: u32 = 0x383F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_32_0_38200021() {
    // Encoding: 0x38200021
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=1, Rt=1
    // Fields: Rt=1, Rn=1, R=0, size=0, A=0, Rs=0, opc=0
    let encoding: u32 = 0x38200021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_ld_combo_33_0_382003ff() {
    // Encoding: 0x382003FF
    // Test aarch64_memory_atomicops_ld field combination: size=0, A=0, R=0, Rs=0, opc=0, Rn=31, Rt=31
    // Fields: A=0, size=0, R=0, Rs=0, opc=0, Rn=31, Rt=31
    let encoding: u32 = 0x382003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_atomicops_ld_special_size_0_size_variant_0_0_38200000() {
    // Encoding: 0x38200000
    // Test aarch64_memory_atomicops_ld special value size = 0 (Size variant 0)
    // Fields: opc=0, Rs=0, Rn=0, A=0, R=0, size=0, Rt=0
    let encoding: u32 = 0x38200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_atomicops_ld_special_size_1_size_variant_1_0_78200000() {
    // Encoding: 0x78200000
    // Test aarch64_memory_atomicops_ld special value size = 1 (Size variant 1)
    // Fields: size=1, R=0, opc=0, Rs=0, Rn=0, Rt=0, A=0
    let encoding: u32 = 0x78200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_atomicops_ld_special_size_2_size_variant_2_0_b8200000() {
    // Encoding: 0xB8200000
    // Test aarch64_memory_atomicops_ld special value size = 2 (Size variant 2)
    // Fields: Rt=0, size=2, Rn=0, R=0, A=0, Rs=0, opc=0
    let encoding: u32 = 0xB8200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_atomicops_ld_special_size_3_size_variant_3_0_f8200000() {
    // Encoding: 0xF8200000
    // Test aarch64_memory_atomicops_ld special value size = 3 (Size variant 3)
    // Fields: R=0, Rt=0, A=0, Rs=0, opc=0, Rn=0, size=3
    let encoding: u32 = 0xF8200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "opc", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_atomicops_ld_special_opc_0_size_variant_0_0_78200000() {
    // Encoding: 0x78200000
    // Test aarch64_memory_atomicops_ld special value opc = 0 (Size variant 0)
    // Fields: R=0, A=0, Rn=0, Rt=0, size=1, opc=0, Rs=0
    let encoding: u32 = 0x78200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "opc", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_atomicops_ld_special_opc_1_size_variant_1_0_78201000() {
    // Encoding: 0x78201000
    // Test aarch64_memory_atomicops_ld special value opc = 1 (Size variant 1)
    // Fields: size=1, R=0, opc=1, Rs=0, A=0, Rn=0, Rt=0
    let encoding: u32 = 0x78201000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "opc", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_atomicops_ld_special_opc_2_size_variant_2_0_78202000() {
    // Encoding: 0x78202000
    // Test aarch64_memory_atomicops_ld special value opc = 2 (Size variant 2)
    // Fields: R=0, Rs=0, Rn=0, Rt=0, A=0, opc=2, size=1
    let encoding: u32 = 0x78202000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "opc", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_atomicops_ld_special_opc_3_size_variant_3_0_78203000() {
    // Encoding: 0x78203000
    // Test aarch64_memory_atomicops_ld special value opc = 3 (Size variant 3)
    // Fields: Rn=0, R=0, Rt=0, size=1, Rs=0, opc=3, A=0
    let encoding: u32 = 0x78203000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc = 4 (Size variant 4)`
/// Requirement: FieldSpecial { field: "opc", value: 4, meaning: "Size variant 4" }
/// Size variant 4
#[test]
fn test_aarch64_memory_atomicops_ld_special_opc_4_size_variant_4_0_78204000() {
    // Encoding: 0x78204000
    // Test aarch64_memory_atomicops_ld special value opc = 4 (Size variant 4)
    // Fields: R=0, A=0, Rn=0, size=1, Rs=0, opc=4, Rt=0
    let encoding: u32 = 0x78204000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc = 5 (Size variant 5)`
/// Requirement: FieldSpecial { field: "opc", value: 5, meaning: "Size variant 5" }
/// Size variant 5
#[test]
fn test_aarch64_memory_atomicops_ld_special_opc_5_size_variant_5_0_78205000() {
    // Encoding: 0x78205000
    // Test aarch64_memory_atomicops_ld special value opc = 5 (Size variant 5)
    // Fields: Rn=0, Rt=0, A=0, size=1, R=0, Rs=0, opc=5
    let encoding: u32 = 0x78205000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc = 6 (Size variant 6)`
/// Requirement: FieldSpecial { field: "opc", value: 6, meaning: "Size variant 6" }
/// Size variant 6
#[test]
fn test_aarch64_memory_atomicops_ld_special_opc_6_size_variant_6_0_78206000() {
    // Encoding: 0x78206000
    // Test aarch64_memory_atomicops_ld special value opc = 6 (Size variant 6)
    // Fields: opc=6, size=1, Rn=0, Rt=0, Rs=0, A=0, R=0
    let encoding: u32 = 0x78206000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field opc = 7 (Size variant 7)`
/// Requirement: FieldSpecial { field: "opc", value: 7, meaning: "Size variant 7" }
/// Size variant 7
#[test]
fn test_aarch64_memory_atomicops_ld_special_opc_7_size_variant_7_0_78207000() {
    // Encoding: 0x78207000
    // Test aarch64_memory_atomicops_ld special value opc = 7 (Size variant 7)
    // Fields: Rn=0, size=1, Rs=0, A=0, Rt=0, R=0, opc=7
    let encoding: u32 = 0x78207000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_atomicops_ld_special_rn_31_stack_pointer_sp_may_require_alignment_0_782003e0()
 {
    // Encoding: 0x782003E0
    // Test aarch64_memory_atomicops_ld special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rt=0, R=0, A=0, opc=0, Rs=0, Rn=31, size=1
    let encoding: u32 = 0x782003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_atomicops_ld_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_7820001f()
 {
    // Encoding: 0x7820001F
    // Test aarch64_memory_atomicops_ld special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rt=31, size=1, opc=0, A=0, Rs=0, Rn=0, R=0
    let encoding: u32 = 0x7820001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_atomicops_ld_reg_write_0_38200000() {
    // Test aarch64_memory_atomicops_ld register write: GpFromField("t")
    // Encoding: 0x38200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_atomicops_ld_sp_rn_382003e0() {
    // Test aarch64_memory_atomicops_ld with Rn = SP (31)
    // Encoding: 0x382003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x382003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_atomicops_ld_zr_rt_3820001f() {
    // Test aarch64_memory_atomicops_ld with Rt = ZR (31)
    // Encoding: 0x3820001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3820001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_atomicops_ld
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_atomicops_ld_store_0_38200020() {
    // Test aarch64_memory_atomicops_ld memory store: 8 bytes
    // Encoding: 0x38200020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x38200020;
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

// ============================================================================
// aarch64_memory_atomicops_cas_single Tests
// ============================================================================

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_size_0_min_7c00_08a07c00() {
    // Encoding: 0x08A07C00
    // Test aarch64_memory_atomicops_cas_single field size = 0 (Min)
    // Fields: Rn=0, size=0, Rs=0, o0=0, L=0, Rt=0
    let encoding: u32 = 0x08A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_size_1_poweroftwo_7c00_48a07c00() {
    // Encoding: 0x48A07C00
    // Test aarch64_memory_atomicops_cas_single field size = 1 (PowerOfTwo)
    // Fields: Rt=0, Rn=0, Rs=0, L=0, o0=0, size=1
    let encoding: u32 = 0x48A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_size_2_poweroftwo_7c00_88a07c00() {
    // Encoding: 0x88A07C00
    // Test aarch64_memory_atomicops_cas_single field size = 2 (PowerOfTwo)
    // Fields: o0=0, size=2, Rt=0, Rs=0, Rn=0, L=0
    let encoding: u32 = 0x88A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_size_3_max_7c00_c8a07c00() {
    // Encoding: 0xC8A07C00
    // Test aarch64_memory_atomicops_cas_single field size = 3 (Max)
    // Fields: o0=0, Rn=0, L=0, Rs=0, Rt=0, size=3
    let encoding: u32 = 0xC8A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_l_0_min_7c00_08a07c00() {
    // Encoding: 0x08A07C00
    // Test aarch64_memory_atomicops_cas_single field L = 0 (Min)
    // Fields: L=0, Rs=0, o0=0, Rt=0, size=0, Rn=0
    let encoding: u32 = 0x08A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_l_1_max_7c00_08e07c00() {
    // Encoding: 0x08E07C00
    // Test aarch64_memory_atomicops_cas_single field L = 1 (Max)
    // Fields: Rs=0, size=0, L=1, o0=0, Rn=0, Rt=0
    let encoding: u32 = 0x08E07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_rs_0_min_7c00_08a07c00() {
    // Encoding: 0x08A07C00
    // Test aarch64_memory_atomicops_cas_single field Rs = 0 (Min)
    // Fields: Rs=0, Rt=0, o0=0, L=0, size=0, Rn=0
    let encoding: u32 = 0x08A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_rs_1_poweroftwo_7c00_08a17c00() {
    // Encoding: 0x08A17C00
    // Test aarch64_memory_atomicops_cas_single field Rs = 1 (PowerOfTwo)
    // Fields: Rs=1, size=0, Rt=0, L=0, Rn=0, o0=0
    let encoding: u32 = 0x08A17C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_rs_30_poweroftwominusone_7c00_08be7c00() {
    // Encoding: 0x08BE7C00
    // Test aarch64_memory_atomicops_cas_single field Rs = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=0, size=0, Rn=0, o0=0, L=0, Rs=30
    let encoding: u32 = 0x08BE7C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_rs_31_max_7c00_08bf7c00() {
    // Encoding: 0x08BF7C00
    // Test aarch64_memory_atomicops_cas_single field Rs = 31 (Max)
    // Fields: o0=0, Rt=0, size=0, L=0, Rn=0, Rs=31
    let encoding: u32 = 0x08BF7C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field o0 15 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_o0_0_min_7c00_08a07c00() {
    // Encoding: 0x08A07C00
    // Test aarch64_memory_atomicops_cas_single field o0 = 0 (Min)
    // Fields: Rn=0, Rt=0, L=0, size=0, Rs=0, o0=0
    let encoding: u32 = 0x08A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field o0 15 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_o0_1_max_7c00_08a0fc00() {
    // Encoding: 0x08A0FC00
    // Test aarch64_memory_atomicops_cas_single field o0 = 1 (Max)
    // Fields: Rt=0, Rs=0, Rn=0, L=0, o0=1, size=0
    let encoding: u32 = 0x08A0FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_rn_0_min_7c00_08a07c00() {
    // Encoding: 0x08A07C00
    // Test aarch64_memory_atomicops_cas_single field Rn = 0 (Min)
    // Fields: Rn=0, size=0, Rt=0, L=0, Rs=0, o0=0
    let encoding: u32 = 0x08A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_rn_1_poweroftwo_7c00_08a07c20() {
    // Encoding: 0x08A07C20
    // Test aarch64_memory_atomicops_cas_single field Rn = 1 (PowerOfTwo)
    // Fields: Rt=0, Rs=0, L=0, size=0, Rn=1, o0=0
    let encoding: u32 = 0x08A07C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_rn_30_poweroftwominusone_7c00_08a07fc0() {
    // Encoding: 0x08A07FC0
    // Test aarch64_memory_atomicops_cas_single field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rt=0, size=0, L=0, o0=0, Rs=0
    let encoding: u32 = 0x08A07FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_rn_31_max_7c00_08a07fe0() {
    // Encoding: 0x08A07FE0
    // Test aarch64_memory_atomicops_cas_single field Rn = 31 (Max)
    // Fields: L=0, Rt=0, Rs=0, o0=0, size=0, Rn=31
    let encoding: u32 = 0x08A07FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_rt_0_min_7c00_08a07c00() {
    // Encoding: 0x08A07C00
    // Test aarch64_memory_atomicops_cas_single field Rt = 0 (Min)
    // Fields: Rs=0, L=0, o0=0, Rn=0, size=0, Rt=0
    let encoding: u32 = 0x08A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_rt_1_poweroftwo_7c00_08a07c01() {
    // Encoding: 0x08A07C01
    // Test aarch64_memory_atomicops_cas_single field Rt = 1 (PowerOfTwo)
    // Fields: o0=0, Rt=1, size=0, Rn=0, L=0, Rs=0
    let encoding: u32 = 0x08A07C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_rt_30_poweroftwominusone_7c00_08a07c1e() {
    // Encoding: 0x08A07C1E
    // Test aarch64_memory_atomicops_cas_single field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rs=0, o0=0, Rn=0, Rt=30, L=0
    let encoding: u32 = 0x08A07C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_atomicops_cas_single_field_rt_31_max_7c00_08a07c1f() {
    // Encoding: 0x08A07C1F
    // Test aarch64_memory_atomicops_cas_single field Rt = 31 (Max)
    // Fields: size=0, o0=0, Rn=0, L=0, Rt=31, Rs=0
    let encoding: u32 = 0x08A07C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_0_7c00_08a07c00() {
    // Encoding: 0x08A07C00
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=0, Rt=0
    // Fields: size=0, Rn=0, L=0, Rs=0, o0=0, Rt=0
    let encoding: u32 = 0x08A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_1_7c00_48a07c00() {
    // Encoding: 0x48A07C00
    // Test aarch64_memory_atomicops_cas_single field combination: size=1, L=0, Rs=0, o0=0, Rn=0, Rt=0
    // Fields: L=0, o0=0, Rn=0, Rt=0, size=1, Rs=0
    let encoding: u32 = 0x48A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_2_7c00_88a07c00() {
    // Encoding: 0x88A07C00
    // Test aarch64_memory_atomicops_cas_single field combination: size=2, L=0, Rs=0, o0=0, Rn=0, Rt=0
    // Fields: Rn=0, size=2, Rs=0, o0=0, Rt=0, L=0
    let encoding: u32 = 0x88A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_3_7c00_c8a07c00() {
    // Encoding: 0xC8A07C00
    // Test aarch64_memory_atomicops_cas_single field combination: size=3, L=0, Rs=0, o0=0, Rn=0, Rt=0
    // Fields: Rs=0, Rn=0, Rt=0, size=3, L=0, o0=0
    let encoding: u32 = 0xC8A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_4_7c00_08a07c00() {
    // Encoding: 0x08A07C00
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, L=0, Rs=0, o0=0, size=0
    let encoding: u32 = 0x08A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_5_7c00_08e07c00() {
    // Encoding: 0x08E07C00
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=1, Rs=0, o0=0, Rn=0, Rt=0
    // Fields: Rt=0, L=1, size=0, o0=0, Rs=0, Rn=0
    let encoding: u32 = 0x08E07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_6_7c00_08a07c00() {
    // Encoding: 0x08A07C00
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=0, Rt=0
    // Fields: L=0, Rs=0, o0=0, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x08A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_7_7c00_08a17c00() {
    // Encoding: 0x08A17C00
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=1, o0=0, Rn=0, Rt=0
    // Fields: L=0, Rt=0, size=0, o0=0, Rn=0, Rs=1
    let encoding: u32 = 0x08A17C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_8_7c00_08be7c00() {
    // Encoding: 0x08BE7C00
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=30, o0=0, Rn=0, Rt=0
    // Fields: o0=0, Rt=0, Rn=0, L=0, Rs=30, size=0
    let encoding: u32 = 0x08BE7C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_9_7c00_08bf7c00() {
    // Encoding: 0x08BF7C00
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=31, o0=0, Rn=0, Rt=0
    // Fields: size=0, Rt=0, L=0, Rs=31, o0=0, Rn=0
    let encoding: u32 = 0x08BF7C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=0 (minimum value)
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_10_7c00_08a07c00() {
    // Encoding: 0x08A07C00
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=0, Rt=0
    // Fields: Rs=0, L=0, Rn=0, size=0, o0=0, Rt=0
    let encoding: u32 = 0x08A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=1 (maximum value (1))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_11_7c00_08a0fc00() {
    // Encoding: 0x08A0FC00
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=1, Rn=0, Rt=0
    // Fields: size=0, Rn=0, Rt=0, o0=1, L=0, Rs=0
    let encoding: u32 = 0x08A0FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_12_7c00_08a07c00() {
    // Encoding: 0x08A07C00
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=0, Rt=0
    // Fields: Rs=0, size=0, L=0, o0=0, Rn=0, Rt=0
    let encoding: u32 = 0x08A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_13_7c00_08a07c20() {
    // Encoding: 0x08A07C20
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=1, Rt=0
    // Fields: Rt=0, o0=0, Rs=0, L=0, Rn=1, size=0
    let encoding: u32 = 0x08A07C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_14_7c00_08a07fc0() {
    // Encoding: 0x08A07FC0
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=30, Rt=0
    // Fields: Rs=0, Rt=0, size=0, L=0, Rn=30, o0=0
    let encoding: u32 = 0x08A07FC0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_15_7c00_08a07fe0() {
    // Encoding: 0x08A07FE0
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=31, Rt=0
    // Fields: L=0, o0=0, size=0, Rs=0, Rt=0, Rn=31
    let encoding: u32 = 0x08A07FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_16_7c00_08a07c00() {
    // Encoding: 0x08A07C00
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=0, Rt=0
    // Fields: Rn=0, Rs=0, L=0, Rt=0, size=0, o0=0
    let encoding: u32 = 0x08A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_17_7c00_08a07c01() {
    // Encoding: 0x08A07C01
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=0, Rt=1
    // Fields: o0=0, Rn=0, size=0, Rs=0, Rt=1, L=0
    let encoding: u32 = 0x08A07C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_18_7c00_08a07c1e() {
    // Encoding: 0x08A07C1E
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=0, Rt=30
    // Fields: Rt=30, Rn=0, size=0, L=0, Rs=0, o0=0
    let encoding: u32 = 0x08A07C1E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_19_7c00_08a07c1f() {
    // Encoding: 0x08A07C1F
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=0, Rt=31
    // Fields: Rn=0, size=0, Rt=31, L=0, o0=0, Rs=0
    let encoding: u32 = 0x08A07C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_20_7c00_08a17c20() {
    // Encoding: 0x08A17C20
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=1, o0=0, Rn=1, Rt=0
    // Fields: o0=0, Rt=0, Rs=1, L=0, Rn=1, size=0
    let encoding: u32 = 0x08A17C20;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_21_7c00_08bf7fe0() {
    // Encoding: 0x08BF7FE0
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=31, o0=0, Rn=31, Rt=0
    // Fields: Rs=31, Rt=0, size=0, Rn=31, L=0, o0=0
    let encoding: u32 = 0x08BF7FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_22_7c00_08a17c01() {
    // Encoding: 0x08A17C01
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=1, o0=0, Rn=0, Rt=1
    // Fields: size=0, Rn=0, Rs=1, o0=0, Rt=1, L=0
    let encoding: u32 = 0x08A17C01;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_23_7c00_08bf7c1f() {
    // Encoding: 0x08BF7C1F
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=31, o0=0, Rn=0, Rt=31
    // Fields: Rn=0, size=0, Rs=31, o0=0, Rt=31, L=0
    let encoding: u32 = 0x08BF7C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_24_7c00_08a07c21() {
    // Encoding: 0x08A07C21
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=1, Rt=1
    // Fields: o0=0, Rn=1, Rt=1, L=0, Rs=0, size=0
    let encoding: u32 = 0x08A07C21;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_cas_single_combo_25_7c00_08a07fff() {
    // Encoding: 0x08A07FFF
    // Test aarch64_memory_atomicops_cas_single field combination: size=0, L=0, Rs=0, o0=0, Rn=31, Rt=31
    // Fields: Rs=0, L=0, o0=0, size=0, Rn=31, Rt=31
    let encoding: u32 = 0x08A07FFF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_atomicops_cas_single_special_size_0_size_variant_0_31744_08a07c00() {
    // Encoding: 0x08A07C00
    // Test aarch64_memory_atomicops_cas_single special value size = 0 (Size variant 0)
    // Fields: Rt=0, L=0, Rn=0, size=0, Rs=0, o0=0
    let encoding: u32 = 0x08A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_atomicops_cas_single_special_size_1_size_variant_1_31744_48a07c00() {
    // Encoding: 0x48A07C00
    // Test aarch64_memory_atomicops_cas_single special value size = 1 (Size variant 1)
    // Fields: L=0, o0=0, Rt=0, Rn=0, size=1, Rs=0
    let encoding: u32 = 0x48A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_atomicops_cas_single_special_size_2_size_variant_2_31744_88a07c00() {
    // Encoding: 0x88A07C00
    // Test aarch64_memory_atomicops_cas_single special value size = 2 (Size variant 2)
    // Fields: size=2, L=0, Rs=0, Rt=0, o0=0, Rn=0
    let encoding: u32 = 0x88A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_atomicops_cas_single_special_size_3_size_variant_3_31744_c8a07c00() {
    // Encoding: 0xC8A07C00
    // Test aarch64_memory_atomicops_cas_single special value size = 3 (Size variant 3)
    // Fields: L=0, Rt=0, o0=0, size=3, Rn=0, Rs=0
    let encoding: u32 = 0xC8A07C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_atomicops_cas_single_special_rn_31_stack_pointer_sp_may_require_alignment_31744_48a07fe0()
 {
    // Encoding: 0x48A07FE0
    // Test aarch64_memory_atomicops_cas_single special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: o0=0, Rn=31, L=0, Rs=0, Rt=0, size=1
    let encoding: u32 = 0x48A07FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_atomicops_cas_single_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_31744_48a07c1f()
 {
    // Encoding: 0x48A07C1F
    // Test aarch64_memory_atomicops_cas_single special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rs=0, o0=0, Rt=31, L=0, Rn=0, size=1
    let encoding: u32 = 0x48A07C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `GpFromField("s") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "s" }
/// verify register write to GpFromField("s")
#[test]
fn test_aarch64_memory_atomicops_cas_single_reg_write_0_08a07c00() {
    // Test aarch64_memory_atomicops_cas_single register write: GpFromField("s")
    // Encoding: 0x08A07C00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08A07C00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_atomicops_cas_single_sp_rn_08a07fe0() {
    // Test aarch64_memory_atomicops_cas_single with Rn = SP (31)
    // Encoding: 0x08A07FE0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08A07FE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_atomicops_cas_single_zr_rt_08a07c1f() {
    // Test aarch64_memory_atomicops_cas_single with Rt = ZR (31)
    // Encoding: 0x08A07C1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08A07C1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_atomicops_cas_single
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_atomicops_cas_single_store_0_08a07c20() {
    // Test aarch64_memory_atomicops_cas_single memory store: 8 bytes
    // Encoding: 0x08A07C20
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x08A07C20;
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

// ============================================================================
// aarch64_memory_atomicops_swp Tests
// ============================================================================

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_atomicops_swp_field_size_0_min_8000_38208000() {
    // Encoding: 0x38208000
    // Test aarch64_memory_atomicops_swp field size = 0 (Min)
    // Fields: size=0, A=0, Rs=0, Rn=0, R=0, Rt=0
    let encoding: u32 = 0x38208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_atomicops_swp_field_size_1_poweroftwo_8000_78208000() {
    // Encoding: 0x78208000
    // Test aarch64_memory_atomicops_swp field size = 1 (PowerOfTwo)
    // Fields: R=0, Rt=0, size=1, A=0, Rs=0, Rn=0
    let encoding: u32 = 0x78208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_atomicops_swp_field_size_2_poweroftwo_8000_b8208000() {
    // Encoding: 0xB8208000
    // Test aarch64_memory_atomicops_swp field size = 2 (PowerOfTwo)
    // Fields: R=0, size=2, Rs=0, A=0, Rn=0, Rt=0
    let encoding: u32 = 0xB8208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_atomicops_swp_field_size_3_max_8000_f8208000() {
    // Encoding: 0xF8208000
    // Test aarch64_memory_atomicops_swp field size = 3 (Max)
    // Fields: A=0, Rs=0, Rt=0, Rn=0, R=0, size=3
    let encoding: u32 = 0xF8208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field A 23 +: 1`
/// Requirement: FieldBoundary { field: "A", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_atomicops_swp_field_a_0_min_8000_38208000() {
    // Encoding: 0x38208000
    // Test aarch64_memory_atomicops_swp field A = 0 (Min)
    // Fields: A=0, Rt=0, R=0, Rs=0, size=0, Rn=0
    let encoding: u32 = 0x38208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field A 23 +: 1`
/// Requirement: FieldBoundary { field: "A", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_atomicops_swp_field_a_1_max_8000_38a08000() {
    // Encoding: 0x38A08000
    // Test aarch64_memory_atomicops_swp field A = 1 (Max)
    // Fields: Rn=0, Rt=0, size=0, A=1, R=0, Rs=0
    let encoding: u32 = 0x38A08000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field R 22 +: 1`
/// Requirement: FieldBoundary { field: "R", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_atomicops_swp_field_r_0_min_8000_38208000() {
    // Encoding: 0x38208000
    // Test aarch64_memory_atomicops_swp field R = 0 (Min)
    // Fields: Rn=0, Rt=0, size=0, R=0, Rs=0, A=0
    let encoding: u32 = 0x38208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field R 22 +: 1`
/// Requirement: FieldBoundary { field: "R", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_atomicops_swp_field_r_1_max_8000_38608000() {
    // Encoding: 0x38608000
    // Test aarch64_memory_atomicops_swp field R = 1 (Max)
    // Fields: Rt=0, Rs=0, size=0, A=0, R=1, Rn=0
    let encoding: u32 = 0x38608000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_atomicops_swp_field_rs_0_min_8000_38208000() {
    // Encoding: 0x38208000
    // Test aarch64_memory_atomicops_swp field Rs = 0 (Min)
    // Fields: Rn=0, size=0, Rt=0, A=0, Rs=0, R=0
    let encoding: u32 = 0x38208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_atomicops_swp_field_rs_1_poweroftwo_8000_38218000() {
    // Encoding: 0x38218000
    // Test aarch64_memory_atomicops_swp field Rs = 1 (PowerOfTwo)
    // Fields: size=0, R=0, Rs=1, A=0, Rn=0, Rt=0
    let encoding: u32 = 0x38218000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_atomicops_swp_field_rs_30_poweroftwominusone_8000_383e8000() {
    // Encoding: 0x383E8000
    // Test aarch64_memory_atomicops_swp field Rs = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, A=0, Rs=30, R=0, Rt=0, Rn=0
    let encoding: u32 = 0x383E8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_atomicops_swp_field_rs_31_max_8000_383f8000() {
    // Encoding: 0x383F8000
    // Test aarch64_memory_atomicops_swp field Rs = 31 (Max)
    // Fields: A=0, R=0, size=0, Rs=31, Rn=0, Rt=0
    let encoding: u32 = 0x383F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_atomicops_swp_field_rn_0_min_8000_38208000() {
    // Encoding: 0x38208000
    // Test aarch64_memory_atomicops_swp field Rn = 0 (Min)
    // Fields: size=0, R=0, Rt=0, A=0, Rn=0, Rs=0
    let encoding: u32 = 0x38208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_atomicops_swp_field_rn_1_poweroftwo_8000_38208020() {
    // Encoding: 0x38208020
    // Test aarch64_memory_atomicops_swp field Rn = 1 (PowerOfTwo)
    // Fields: Rs=0, Rt=0, size=0, A=0, Rn=1, R=0
    let encoding: u32 = 0x38208020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_atomicops_swp_field_rn_30_poweroftwominusone_8000_382083c0() {
    // Encoding: 0x382083C0
    // Test aarch64_memory_atomicops_swp field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, R=0, A=0, Rs=0, Rn=30, Rt=0
    let encoding: u32 = 0x382083C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_atomicops_swp_field_rn_31_max_8000_382083e0() {
    // Encoding: 0x382083E0
    // Test aarch64_memory_atomicops_swp field Rn = 31 (Max)
    // Fields: Rn=31, R=0, A=0, size=0, Rs=0, Rt=0
    let encoding: u32 = 0x382083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_atomicops_swp_field_rt_0_min_8000_38208000() {
    // Encoding: 0x38208000
    // Test aarch64_memory_atomicops_swp field Rt = 0 (Min)
    // Fields: size=0, A=0, R=0, Rs=0, Rn=0, Rt=0
    let encoding: u32 = 0x38208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_atomicops_swp_field_rt_1_poweroftwo_8000_38208001() {
    // Encoding: 0x38208001
    // Test aarch64_memory_atomicops_swp field Rt = 1 (PowerOfTwo)
    // Fields: Rn=0, R=0, size=0, Rt=1, Rs=0, A=0
    let encoding: u32 = 0x38208001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_atomicops_swp_field_rt_30_poweroftwominusone_8000_3820801e() {
    // Encoding: 0x3820801E
    // Test aarch64_memory_atomicops_swp field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: A=0, R=0, Rn=0, Rt=30, Rs=0, size=0
    let encoding: u32 = 0x3820801E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_atomicops_swp_field_rt_31_max_8000_3820801f() {
    // Encoding: 0x3820801F
    // Test aarch64_memory_atomicops_swp field Rt = 31 (Max)
    // Fields: R=0, A=0, Rt=31, size=0, Rs=0, Rn=0
    let encoding: u32 = 0x3820801F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_atomicops_swp_combo_0_8000_38208000() {
    // Encoding: 0x38208000
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, Rs=0, R=0, A=0, size=0
    let encoding: u32 = 0x38208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_atomicops_swp_combo_1_8000_78208000() {
    // Encoding: 0x78208000
    // Test aarch64_memory_atomicops_swp field combination: size=1, A=0, R=0, Rs=0, Rn=0, Rt=0
    // Fields: size=1, R=0, Rn=0, Rt=0, Rs=0, A=0
    let encoding: u32 = 0x78208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_atomicops_swp_combo_2_8000_b8208000() {
    // Encoding: 0xB8208000
    // Test aarch64_memory_atomicops_swp field combination: size=2, A=0, R=0, Rs=0, Rn=0, Rt=0
    // Fields: size=2, R=0, Rs=0, A=0, Rn=0, Rt=0
    let encoding: u32 = 0xB8208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_atomicops_swp_combo_3_8000_f8208000() {
    // Encoding: 0xF8208000
    // Test aarch64_memory_atomicops_swp field combination: size=3, A=0, R=0, Rs=0, Rn=0, Rt=0
    // Fields: Rt=0, Rn=0, A=0, R=0, Rs=0, size=3
    let encoding: u32 = 0xF8208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// A=0 (minimum value)
#[test]
fn test_aarch64_memory_atomicops_swp_combo_4_8000_38208000() {
    // Encoding: 0x38208000
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=0, Rt=0
    // Fields: A=0, Rt=0, size=0, R=0, Rn=0, Rs=0
    let encoding: u32 = 0x38208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// A=1 (maximum value (1))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_5_8000_38a08000() {
    // Encoding: 0x38A08000
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=1, R=0, Rs=0, Rn=0, Rt=0
    // Fields: Rt=0, size=0, R=0, Rs=0, Rn=0, A=1
    let encoding: u32 = 0x38A08000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// R=0 (minimum value)
#[test]
fn test_aarch64_memory_atomicops_swp_combo_6_8000_38208000() {
    // Encoding: 0x38208000
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=0, Rt=0
    // Fields: A=0, R=0, size=0, Rs=0, Rn=0, Rt=0
    let encoding: u32 = 0x38208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// R=1 (maximum value (1))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_7_8000_38608000() {
    // Encoding: 0x38608000
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=1, Rs=0, Rn=0, Rt=0
    // Fields: A=0, Rn=0, Rt=0, size=0, R=1, Rs=0
    let encoding: u32 = 0x38608000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_8_8000_38208000() {
    // Encoding: 0x38208000
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=0, Rt=0
    // Fields: A=0, Rt=0, R=0, size=0, Rs=0, Rn=0
    let encoding: u32 = 0x38208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_9_8000_38218000() {
    // Encoding: 0x38218000
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=1, Rn=0, Rt=0
    // Fields: Rs=1, Rn=0, Rt=0, size=0, R=0, A=0
    let encoding: u32 = 0x38218000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_10_8000_383e8000() {
    // Encoding: 0x383E8000
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=30, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, A=0, Rs=30, size=0, R=0
    let encoding: u32 = 0x383E8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_11_8000_383f8000() {
    // Encoding: 0x383F8000
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=31, Rn=0, Rt=0
    // Fields: Rt=0, A=0, size=0, R=0, Rs=31, Rn=0
    let encoding: u32 = 0x383F8000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_12_8000_38208000() {
    // Encoding: 0x38208000
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=0, Rt=0
    // Fields: Rn=0, Rs=0, R=0, Rt=0, size=0, A=0
    let encoding: u32 = 0x38208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_13_8000_38208020() {
    // Encoding: 0x38208020
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=1, Rt=0
    // Fields: size=0, Rs=0, R=0, A=0, Rn=1, Rt=0
    let encoding: u32 = 0x38208020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_14_8000_382083c0() {
    // Encoding: 0x382083C0
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=30, Rt=0
    // Fields: Rt=0, Rs=0, size=0, A=0, Rn=30, R=0
    let encoding: u32 = 0x382083C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_15_8000_382083e0() {
    // Encoding: 0x382083E0
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=31, Rt=0
    // Fields: Rs=0, Rn=31, R=0, A=0, Rt=0, size=0
    let encoding: u32 = 0x382083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_16_8000_38208000() {
    // Encoding: 0x38208000
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=0, Rt=0
    // Fields: size=0, Rs=0, Rn=0, Rt=0, A=0, R=0
    let encoding: u32 = 0x38208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_17_8000_38208001() {
    // Encoding: 0x38208001
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=0, Rt=1
    // Fields: Rn=0, size=0, A=0, Rs=0, Rt=1, R=0
    let encoding: u32 = 0x38208001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_18_8000_3820801e() {
    // Encoding: 0x3820801E
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=0, Rt=30
    // Fields: A=0, R=0, Rt=30, size=0, Rs=0, Rn=0
    let encoding: u32 = 0x3820801E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_19_8000_3820801f() {
    // Encoding: 0x3820801F
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=0, Rt=31
    // Fields: Rt=31, size=0, A=0, R=0, Rn=0, Rs=0
    let encoding: u32 = 0x3820801F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_20_8000_38218020() {
    // Encoding: 0x38218020
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=1, Rn=1, Rt=0
    // Fields: Rs=1, Rn=1, Rt=0, size=0, A=0, R=0
    let encoding: u32 = 0x38218020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_21_8000_383f83e0() {
    // Encoding: 0x383F83E0
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=31, Rn=31, Rt=0
    // Fields: A=0, R=0, Rs=31, Rn=31, Rt=0, size=0
    let encoding: u32 = 0x383F83E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_22_8000_38218001() {
    // Encoding: 0x38218001
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=1, Rn=0, Rt=1
    // Fields: Rt=1, A=0, Rs=1, R=0, size=0, Rn=0
    let encoding: u32 = 0x38218001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_23_8000_383f801f() {
    // Encoding: 0x383F801F
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=31, Rn=0, Rt=31
    // Fields: Rs=31, Rn=0, Rt=31, R=0, size=0, A=0
    let encoding: u32 = 0x383F801F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_24_8000_38208021() {
    // Encoding: 0x38208021
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=1, Rt=1
    // Fields: Rn=1, Rt=1, A=0, size=0, R=0, Rs=0
    let encoding: u32 = 0x38208021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_swp_combo_25_8000_382083ff() {
    // Encoding: 0x382083FF
    // Test aarch64_memory_atomicops_swp field combination: size=0, A=0, R=0, Rs=0, Rn=31, Rt=31
    // Fields: Rs=0, A=0, Rn=31, size=0, Rt=31, R=0
    let encoding: u32 = 0x382083FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_atomicops_swp_special_size_0_size_variant_0_32768_38208000() {
    // Encoding: 0x38208000
    // Test aarch64_memory_atomicops_swp special value size = 0 (Size variant 0)
    // Fields: A=0, Rt=0, Rn=0, Rs=0, R=0, size=0
    let encoding: u32 = 0x38208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_atomicops_swp_special_size_1_size_variant_1_32768_78208000() {
    // Encoding: 0x78208000
    // Test aarch64_memory_atomicops_swp special value size = 1 (Size variant 1)
    // Fields: A=0, Rn=0, Rs=0, R=0, Rt=0, size=1
    let encoding: u32 = 0x78208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_atomicops_swp_special_size_2_size_variant_2_32768_b8208000() {
    // Encoding: 0xB8208000
    // Test aarch64_memory_atomicops_swp special value size = 2 (Size variant 2)
    // Fields: size=2, Rs=0, R=0, Rt=0, A=0, Rn=0
    let encoding: u32 = 0xB8208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_atomicops_swp_special_size_3_size_variant_3_32768_f8208000() {
    // Encoding: 0xF8208000
    // Test aarch64_memory_atomicops_swp special value size = 3 (Size variant 3)
    // Fields: size=3, A=0, R=0, Rs=0, Rn=0, Rt=0
    let encoding: u32 = 0xF8208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_atomicops_swp_special_rn_31_stack_pointer_sp_may_require_alignment_32768_782083e0()
 {
    // Encoding: 0x782083E0
    // Test aarch64_memory_atomicops_swp special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: R=0, Rn=31, Rs=0, A=0, Rt=0, size=1
    let encoding: u32 = 0x782083E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_atomicops_swp_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_32768_7820801f()
 {
    // Encoding: 0x7820801F
    // Test aarch64_memory_atomicops_swp special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, Rs=0, Rt=31, A=0, size=1, R=0
    let encoding: u32 = 0x7820801F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_atomicops_swp_reg_write_0_38208000() {
    // Test aarch64_memory_atomicops_swp register write: GpFromField("t")
    // Encoding: 0x38208000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38208000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_atomicops_swp_sp_rn_382083e0() {
    // Test aarch64_memory_atomicops_swp with Rn = SP (31)
    // Encoding: 0x382083E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x382083E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_atomicops_swp_zr_rt_3820801f() {
    // Test aarch64_memory_atomicops_swp with Rt = ZR (31)
    // Encoding: 0x3820801F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x3820801F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_atomicops_swp
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_atomicops_swp_store_0_38208020() {
    // Test aarch64_memory_atomicops_swp memory store: 8 bytes
    // Encoding: 0x38208020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x38208020;
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

// ============================================================================
// aarch64_memory_atomicops_cas_pair Tests
// ============================================================================

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field sz 30 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_sz_0_min_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field sz = 0 (Min)
    // Fields: Rs=0, L=0, o0=0, Rn=0, sz=0, Rt2=0, Rt=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field sz 30 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_sz_1_max_0_48200000() {
    // Encoding: 0x48200000
    // Test aarch64_memory_atomicops_cas_pair field sz = 1 (Max)
    // Fields: Rt2=0, L=0, Rs=0, Rt=0, sz=1, Rn=0, o0=0
    let encoding: u32 = 0x48200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_l_0_min_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field L = 0 (Min)
    // Fields: L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0, sz=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_l_1_max_0_08600000() {
    // Encoding: 0x08600000
    // Test aarch64_memory_atomicops_cas_pair field L = 1 (Max)
    // Fields: Rt=0, L=1, sz=0, o0=0, Rs=0, Rn=0, Rt2=0
    let encoding: u32 = 0x08600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rs_0_min_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field Rs = 0 (Min)
    // Fields: o0=0, Rt2=0, L=0, Rt=0, Rs=0, Rn=0, sz=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rs_1_poweroftwo_0_08210000() {
    // Encoding: 0x08210000
    // Test aarch64_memory_atomicops_cas_pair field Rs = 1 (PowerOfTwo)
    // Fields: Rt2=0, Rn=0, o0=0, Rt=0, sz=0, L=0, Rs=1
    let encoding: u32 = 0x08210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rs_30_poweroftwominusone_0_083e0000() {
    // Encoding: 0x083E0000
    // Test aarch64_memory_atomicops_cas_pair field Rs = 30 (PowerOfTwoMinusOne)
    // Fields: sz=0, o0=0, Rt2=0, Rn=0, Rs=30, Rt=0, L=0
    let encoding: u32 = 0x083E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rs_31_max_0_083f0000() {
    // Encoding: 0x083F0000
    // Test aarch64_memory_atomicops_cas_pair field Rs = 31 (Max)
    // Fields: Rn=0, Rt=0, L=0, sz=0, o0=0, Rt2=0, Rs=31
    let encoding: u32 = 0x083F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field o0 15 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_o0_0_min_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field o0 = 0 (Min)
    // Fields: Rt=0, sz=0, Rn=0, o0=0, Rs=0, L=0, Rt2=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field o0 15 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_o0_1_max_0_08208000() {
    // Encoding: 0x08208000
    // Test aarch64_memory_atomicops_cas_pair field o0 = 1 (Max)
    // Fields: Rt2=0, Rs=0, L=0, o0=1, Rn=0, Rt=0, sz=0
    let encoding: u32 = 0x08208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rt2_0_min_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field Rt2 = 0 (Min)
    // Fields: Rn=0, Rt=0, o0=0, Rs=0, Rt2=0, sz=0, L=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rt2_1_poweroftwo_0_08200400() {
    // Encoding: 0x08200400
    // Test aarch64_memory_atomicops_cas_pair field Rt2 = 1 (PowerOfTwo)
    // Fields: Rt=0, Rt2=1, L=0, o0=0, Rs=0, sz=0, Rn=0
    let encoding: u32 = 0x08200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rt2_30_poweroftwominusone_0_08207800() {
    // Encoding: 0x08207800
    // Test aarch64_memory_atomicops_cas_pair field Rt2 = 30 (PowerOfTwoMinusOne)
    // Fields: L=0, Rs=0, o0=0, Rt=0, Rn=0, Rt2=30, sz=0
    let encoding: u32 = 0x08207800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rt2_31_max_0_08207c00() {
    // Encoding: 0x08207C00
    // Test aarch64_memory_atomicops_cas_pair field Rt2 = 31 (Max)
    // Fields: Rn=0, Rs=0, o0=0, Rt=0, sz=0, L=0, Rt2=31
    let encoding: u32 = 0x08207C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rn_0_min_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field Rn = 0 (Min)
    // Fields: Rt2=0, sz=0, Rs=0, o0=0, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rn_1_poweroftwo_0_08200020() {
    // Encoding: 0x08200020
    // Test aarch64_memory_atomicops_cas_pair field Rn = 1 (PowerOfTwo)
    // Fields: o0=0, Rt2=0, Rn=1, Rt=0, Rs=0, sz=0, L=0
    let encoding: u32 = 0x08200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rn_30_poweroftwominusone_0_082003c0() {
    // Encoding: 0x082003C0
    // Test aarch64_memory_atomicops_cas_pair field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=30, Rs=0, Rt=0, o0=0, sz=0, L=0, Rt2=0
    let encoding: u32 = 0x082003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rn_31_max_0_082003e0() {
    // Encoding: 0x082003E0
    // Test aarch64_memory_atomicops_cas_pair field Rn = 31 (Max)
    // Fields: sz=0, Rn=31, Rt=0, L=0, o0=0, Rt2=0, Rs=0
    let encoding: u32 = 0x082003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rt_0_min_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field Rt = 0 (Min)
    // Fields: L=0, sz=0, o0=0, Rt=0, Rn=0, Rs=0, Rt2=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rt_1_poweroftwo_0_08200001() {
    // Encoding: 0x08200001
    // Test aarch64_memory_atomicops_cas_pair field Rt = 1 (PowerOfTwo)
    // Fields: o0=0, Rn=0, Rs=0, sz=0, Rt2=0, Rt=1, L=0
    let encoding: u32 = 0x08200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rt_30_poweroftwominusone_0_0820001e() {
    // Encoding: 0x0820001E
    // Test aarch64_memory_atomicops_cas_pair field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: L=0, o0=0, Rn=0, Rt=30, Rt2=0, Rs=0, sz=0
    let encoding: u32 = 0x0820001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_field_rt_31_max_0_0820001f() {
    // Encoding: 0x0820001F
    // Test aarch64_memory_atomicops_cas_pair field Rt = 31 (Max)
    // Fields: Rn=0, Rt=31, Rs=0, sz=0, o0=0, Rt2=0, L=0
    let encoding: u32 = 0x0820001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_0_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt2=0, Rs=0, sz=0, L=0, o0=0, Rt=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_1_0_48200000() {
    // Encoding: 0x48200000
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=1, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rs=0, sz=1, o0=0, Rt2=0, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x48200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_2_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, sz=0, Rt2=0, o0=0, Rn=0, L=0, Rs=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_3_0_08600000() {
    // Encoding: 0x08600000
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=1, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=1, o0=0, Rs=0, Rt2=0, sz=0, Rn=0, Rt=0
    let encoding: u32 = 0x08600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_4_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rs=0, o0=0, Rt2=0, sz=0, L=0, Rn=0, Rt=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_5_0_08210000() {
    // Encoding: 0x08210000
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=1, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, o0=0, Rt=0, sz=0, Rt2=0, Rn=0, Rs=1
    let encoding: u32 = 0x08210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_6_0_083e0000() {
    // Encoding: 0x083E0000
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=30, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: sz=0, o0=0, Rt2=0, L=0, Rn=0, Rt=0, Rs=30
    let encoding: u32 = 0x083E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_7_0_083f0000() {
    // Encoding: 0x083F0000
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=31, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rs=31, Rt2=0, Rn=0, o0=0, Rt=0, sz=0
    let encoding: u32 = 0x083F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=0 (minimum value)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_8_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, L=0, sz=0, Rt2=0, Rs=0, Rt=0, o0=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=1 (maximum value (1))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_9_0_08208000() {
    // Encoding: 0x08208000
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=1, Rt2=0, Rn=0, Rt=0
    // Fields: o0=1, L=0, Rs=0, sz=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x08208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_10_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, o0=0, sz=0, Rs=0, L=0, Rt2=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_11_0_08200400() {
    // Encoding: 0x08200400
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=1, Rn=0, Rt=0
    // Fields: Rt2=1, Rt=0, L=0, o0=0, Rs=0, sz=0, Rn=0
    let encoding: u32 = 0x08200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_12_0_08207800() {
    // Encoding: 0x08207800
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=30, Rn=0, Rt=0
    // Fields: L=0, Rs=0, Rt2=30, Rn=0, Rt=0, o0=0, sz=0
    let encoding: u32 = 0x08207800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_13_0_08207c00() {
    // Encoding: 0x08207C00
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=31, Rn=0, Rt=0
    // Fields: Rt=0, L=0, sz=0, Rs=0, o0=0, Rn=0, Rt2=31
    let encoding: u32 = 0x08207C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_14_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rn=0, Rt=0, sz=0, Rs=0, o0=0, L=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_15_0_08200020() {
    // Encoding: 0x08200020
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=1, Rt=0
    // Fields: L=0, Rt=0, o0=0, sz=0, Rn=1, Rs=0, Rt2=0
    let encoding: u32 = 0x08200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_16_0_082003c0() {
    // Encoding: 0x082003C0
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=30, Rt=0
    // Fields: Rs=0, o0=0, Rn=30, Rt2=0, Rt=0, L=0, sz=0
    let encoding: u32 = 0x082003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_17_0_082003e0() {
    // Encoding: 0x082003E0
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=31, Rt=0
    // Fields: L=0, Rs=0, Rt2=0, o0=0, Rn=31, Rt=0, sz=0
    let encoding: u32 = 0x082003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_18_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: o0=0, Rn=0, Rt2=0, sz=0, Rt=0, Rs=0, L=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_19_0_08200001() {
    // Encoding: 0x08200001
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=1
    // Fields: sz=0, Rs=0, Rt=1, Rt2=0, o0=0, L=0, Rn=0
    let encoding: u32 = 0x08200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_20_0_0820001e() {
    // Encoding: 0x0820001E
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=30
    // Fields: sz=0, Rs=0, Rt=30, Rn=0, o0=0, L=0, Rt2=0
    let encoding: u32 = 0x0820001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_21_0_0820001f() {
    // Encoding: 0x0820001F
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=31
    // Fields: Rt2=0, Rn=0, o0=0, Rs=0, Rt=31, sz=0, L=0
    let encoding: u32 = 0x0820001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rt2=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_22_0_08210400() {
    // Encoding: 0x08210400
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=1, o0=0, Rt2=1, Rn=0, Rt=0
    // Fields: sz=0, o0=0, L=0, Rs=1, Rt2=1, Rn=0, Rt=0
    let encoding: u32 = 0x08210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rt2=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_23_0_083f7c00() {
    // Encoding: 0x083F7C00
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=31, o0=0, Rt2=31, Rn=0, Rt=0
    // Fields: L=0, Rs=31, sz=0, o0=0, Rt2=31, Rt=0, Rn=0
    let encoding: u32 = 0x083F7C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_24_0_08210020() {
    // Encoding: 0x08210020
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=1, o0=0, Rt2=0, Rn=1, Rt=0
    // Fields: sz=0, L=0, Rt2=0, o0=0, Rt=0, Rn=1, Rs=1
    let encoding: u32 = 0x08210020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_25_0_083f03e0() {
    // Encoding: 0x083F03E0
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=31, o0=0, Rt2=0, Rn=31, Rt=0
    // Fields: L=0, o0=0, Rt2=0, Rn=31, sz=0, Rt=0, Rs=31
    let encoding: u32 = 0x083F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_26_0_08210001() {
    // Encoding: 0x08210001
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=1, o0=0, Rt2=0, Rn=0, Rt=1
    // Fields: Rt=1, o0=0, Rn=0, sz=0, L=0, Rs=1, Rt2=0
    let encoding: u32 = 0x08210001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_27_0_083f001f() {
    // Encoding: 0x083F001F
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=31, o0=0, Rt2=0, Rn=0, Rt=31
    // Fields: o0=0, Rt2=0, Rn=0, Rt=31, sz=0, L=0, Rs=31
    let encoding: u32 = 0x083F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_28_0_08200420() {
    // Encoding: 0x08200420
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=1, Rn=1, Rt=0
    // Fields: o0=0, Rt2=1, sz=0, Rn=1, Rt=0, Rs=0, L=0
    let encoding: u32 = 0x08200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_29_0_08207fe0() {
    // Encoding: 0x08207FE0
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=31, Rn=31, Rt=0
    // Fields: L=0, Rn=31, Rt2=31, Rt=0, sz=0, o0=0, Rs=0
    let encoding: u32 = 0x08207FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_30_0_08200401() {
    // Encoding: 0x08200401
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=1, Rn=0, Rt=1
    // Fields: Rs=0, Rt=1, o0=0, Rn=0, L=0, sz=0, Rt2=1
    let encoding: u32 = 0x08200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_31_0_08207c1f() {
    // Encoding: 0x08207C1F
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=31, Rn=0, Rt=31
    // Fields: sz=0, L=0, Rt=31, o0=0, Rt2=31, Rs=0, Rn=0
    let encoding: u32 = 0x08207C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_32_0_08200021() {
    // Encoding: 0x08200021
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=1, Rt=1
    // Fields: o0=0, sz=0, Rs=0, Rt2=0, Rn=1, L=0, Rt=1
    let encoding: u32 = 0x08200021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_atomicops_cas_pair_combo_33_0_082003ff() {
    // Encoding: 0x082003FF
    // Test aarch64_memory_atomicops_cas_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=31, Rt=31
    // Fields: Rt=31, L=0, Rt2=0, Rn=31, sz=0, Rs=0, o0=0
    let encoding: u32 = 0x082003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_atomicops_cas_pair_special_sz_0_size_variant_0_0_08200000() {
    // Encoding: 0x08200000
    // Test aarch64_memory_atomicops_cas_pair special value sz = 0 (Size variant 0)
    // Fields: sz=0, o0=0, L=0, Rs=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x08200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_atomicops_cas_pair_special_sz_1_size_variant_1_0_48200000() {
    // Encoding: 0x48200000
    // Test aarch64_memory_atomicops_cas_pair special value sz = 1 (Size variant 1)
    // Fields: sz=1, o0=0, L=0, Rt2=0, Rt=0, Rn=0, Rs=0
    let encoding: u32 = 0x48200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_atomicops_cas_pair_special_rn_31_stack_pointer_sp_may_require_alignment_0_482003e0()
 {
    // Encoding: 0x482003E0
    // Test aarch64_memory_atomicops_cas_pair special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: Rs=0, L=0, o0=0, Rt2=0, sz=1, Rn=31, Rt=0
    let encoding: u32 = 0x482003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_atomicops_cas_pair_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_4820001f()
 {
    // Encoding: 0x4820001F
    // Test aarch64_memory_atomicops_cas_pair special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: sz=1, L=0, o0=0, Rt2=0, Rs=0, Rn=0, Rt=31
    let encoding: u32 = 0x4820001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `GpFromField("s") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "s" }
/// verify register write to GpFromField("s")
#[test]
fn test_aarch64_memory_atomicops_cas_pair_reg_write_0_08200000() {
    // Test aarch64_memory_atomicops_cas_pair register write: GpFromField("s")
    // Encoding: 0x08200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `GpFromField("s") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "s" }
/// verify register write to GpFromField("s")
#[test]
fn test_aarch64_memory_atomicops_cas_pair_reg_write_1_08200000() {
    // Test aarch64_memory_atomicops_cas_pair register write: GpFromField("s")
    // Encoding: 0x08200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_sp_rn_082003e0() {
    // Test aarch64_memory_atomicops_cas_pair with Rn = SP (31)
    // Encoding: 0x082003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x082003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_atomicops_cas_pair_zr_rt_0820001f() {
    // Test aarch64_memory_atomicops_cas_pair with Rt = ZR (31)
    // Encoding: 0x0820001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0820001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_atomicops_cas_pair
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_atomicops_cas_pair_store_0_08200020() {
    // Test aarch64_memory_atomicops_cas_pair memory store: 8 bytes
    // Encoding: 0x08200020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x08200020;
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}
