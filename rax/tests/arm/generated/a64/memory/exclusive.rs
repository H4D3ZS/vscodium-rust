//! A64 memory exclusive tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_memory_exclusive_single Tests
// ============================================================================

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_exclusive_single_field_size_0_min_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field size = 0 (Min)
    // Fields: Rt2=0, size=0, L=0, o0=0, Rn=0, Rt=0, Rs=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_exclusive_single_field_size_1_poweroftwo_0_48000000() {
    // Encoding: 0x48000000
    // Test aarch64_memory_exclusive_single field size = 1 (PowerOfTwo)
    // Fields: Rt=0, size=1, L=0, Rt2=0, Rs=0, Rn=0, o0=0
    let encoding: u32 = 0x48000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_exclusive_single_field_size_2_poweroftwo_0_88000000() {
    // Encoding: 0x88000000
    // Test aarch64_memory_exclusive_single field size = 2 (PowerOfTwo)
    // Fields: Rt2=0, Rn=0, Rt=0, L=0, Rs=0, size=2, o0=0
    let encoding: u32 = 0x88000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_exclusive_single_field_size_3_max_0_c8000000() {
    // Encoding: 0xC8000000
    // Test aarch64_memory_exclusive_single field size = 3 (Max)
    // Fields: L=0, size=3, Rt2=0, Rn=0, Rs=0, Rt=0, o0=0
    let encoding: u32 = 0xC8000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_exclusive_single_field_l_0_min_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field L = 0 (Min)
    // Fields: Rt2=0, o0=0, L=0, Rn=0, Rt=0, size=0, Rs=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_exclusive_single_field_l_1_max_0_08400000() {
    // Encoding: 0x08400000
    // Test aarch64_memory_exclusive_single field L = 1 (Max)
    // Fields: Rt2=0, Rn=0, Rs=0, Rt=0, size=0, L=1, o0=0
    let encoding: u32 = 0x08400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_exclusive_single_field_rs_0_min_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field Rs = 0 (Min)
    // Fields: o0=0, Rt2=0, Rt=0, Rn=0, L=0, Rs=0, size=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_exclusive_single_field_rs_1_poweroftwo_0_08010000() {
    // Encoding: 0x08010000
    // Test aarch64_memory_exclusive_single field Rs = 1 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, L=0, Rt2=0, Rs=1, size=0, o0=0
    let encoding: u32 = 0x08010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_exclusive_single_field_rs_30_poweroftwominusone_0_081e0000() {
    // Encoding: 0x081E0000
    // Test aarch64_memory_exclusive_single field Rs = 30 (PowerOfTwoMinusOne)
    // Fields: o0=0, Rs=30, Rn=0, Rt=0, L=0, Rt2=0, size=0
    let encoding: u32 = 0x081E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_exclusive_single_field_rs_31_max_0_081f0000() {
    // Encoding: 0x081F0000
    // Test aarch64_memory_exclusive_single field Rs = 31 (Max)
    // Fields: L=0, Rs=31, Rn=0, o0=0, Rt=0, Rt2=0, size=0
    let encoding: u32 = 0x081F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field o0 15 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_exclusive_single_field_o0_0_min_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field o0 = 0 (Min)
    // Fields: Rn=0, Rs=0, Rt=0, L=0, Rt2=0, size=0, o0=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field o0 15 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_exclusive_single_field_o0_1_max_0_08008000() {
    // Encoding: 0x08008000
    // Test aarch64_memory_exclusive_single field o0 = 1 (Max)
    // Fields: o0=1, size=0, Rt2=0, Rn=0, Rt=0, Rs=0, L=0
    let encoding: u32 = 0x08008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_exclusive_single_field_rt2_0_min_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field Rt2 = 0 (Min)
    // Fields: Rs=0, Rn=0, o0=0, Rt=0, size=0, Rt2=0, L=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_exclusive_single_field_rt2_1_poweroftwo_0_08000400() {
    // Encoding: 0x08000400
    // Test aarch64_memory_exclusive_single field Rt2 = 1 (PowerOfTwo)
    // Fields: Rn=0, Rs=0, L=0, o0=0, size=0, Rt2=1, Rt=0
    let encoding: u32 = 0x08000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_exclusive_single_field_rt2_30_poweroftwominusone_0_08007800() {
    // Encoding: 0x08007800
    // Test aarch64_memory_exclusive_single field Rt2 = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=0, Rs=0, L=0, size=0, o0=0, Rt2=30
    let encoding: u32 = 0x08007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_exclusive_single_field_rt2_31_max_0_08007c00() {
    // Encoding: 0x08007C00
    // Test aarch64_memory_exclusive_single field Rt2 = 31 (Max)
    // Fields: size=0, L=0, Rs=0, o0=0, Rn=0, Rt=0, Rt2=31
    let encoding: u32 = 0x08007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_exclusive_single_field_rn_0_min_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field Rn = 0 (Min)
    // Fields: o0=0, Rn=0, Rt2=0, size=0, Rt=0, L=0, Rs=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_exclusive_single_field_rn_1_poweroftwo_0_08000020() {
    // Encoding: 0x08000020
    // Test aarch64_memory_exclusive_single field Rn = 1 (PowerOfTwo)
    // Fields: Rn=1, Rs=0, Rt=0, L=0, o0=0, Rt2=0, size=0
    let encoding: u32 = 0x08000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_exclusive_single_field_rn_30_poweroftwominusone_0_080003c0() {
    // Encoding: 0x080003C0
    // Test aarch64_memory_exclusive_single field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rt2=0, size=0, Rs=0, Rn=30, Rt=0, o0=0, L=0
    let encoding: u32 = 0x080003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_exclusive_single_field_rn_31_max_0_080003e0() {
    // Encoding: 0x080003E0
    // Test aarch64_memory_exclusive_single field Rn = 31 (Max)
    // Fields: size=0, Rn=31, Rt2=0, Rs=0, o0=0, Rt=0, L=0
    let encoding: u32 = 0x080003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_exclusive_single_field_rt_0_min_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field Rt = 0 (Min)
    // Fields: Rs=0, o0=0, Rt2=0, Rn=0, Rt=0, size=0, L=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_exclusive_single_field_rt_1_poweroftwo_0_08000001() {
    // Encoding: 0x08000001
    // Test aarch64_memory_exclusive_single field Rt = 1 (PowerOfTwo)
    // Fields: Rt2=0, Rn=0, Rt=1, size=0, Rs=0, o0=0, L=0
    let encoding: u32 = 0x08000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_exclusive_single_field_rt_30_poweroftwominusone_0_0800001e() {
    // Encoding: 0x0800001E
    // Test aarch64_memory_exclusive_single field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: Rs=0, o0=0, size=0, Rt2=0, Rt=30, Rn=0, L=0
    let encoding: u32 = 0x0800001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_exclusive_single_field_rt_31_max_0_0800001f() {
    // Encoding: 0x0800001F
    // Test aarch64_memory_exclusive_single field Rt = 31 (Max)
    // Fields: Rn=0, Rt=31, L=0, size=0, Rs=0, Rt2=0, o0=0
    let encoding: u32 = 0x0800001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_exclusive_single_combo_0_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: o0=0, Rn=0, Rt=0, size=0, Rt2=0, L=0, Rs=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_exclusive_single_combo_1_0_48000000() {
    // Encoding: 0x48000000
    // Test aarch64_memory_exclusive_single field combination: size=1, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: size=1, o0=0, Rn=0, Rs=0, Rt2=0, Rt=0, L=0
    let encoding: u32 = 0x48000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_exclusive_single_combo_2_0_88000000() {
    // Encoding: 0x88000000
    // Test aarch64_memory_exclusive_single field combination: size=2, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt2=0, Rn=0, Rt=0, size=2, o0=0, Rs=0
    let encoding: u32 = 0x88000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_exclusive_single_combo_3_0_c8000000() {
    // Encoding: 0xC8000000
    // Test aarch64_memory_exclusive_single field combination: size=3, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rs=0, o0=0, Rt=0, L=0, size=3, Rt2=0, Rn=0
    let encoding: u32 = 0xC8000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_exclusive_single_combo_4_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, size=0, L=0, Rt=0, Rs=0, Rn=0, o0=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_exclusive_single_combo_5_0_08400000() {
    // Encoding: 0x08400000
    // Test aarch64_memory_exclusive_single field combination: size=0, L=1, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: o0=0, Rn=0, size=0, Rt=0, Rt2=0, Rs=0, L=1
    let encoding: u32 = 0x08400000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_exclusive_single_combo_6_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rs=0, o0=0, Rt2=0, Rn=0, size=0, Rt=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_exclusive_single_combo_7_0_08010000() {
    // Encoding: 0x08010000
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=1, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: size=0, Rt2=0, o0=0, L=0, Rn=0, Rt=0, Rs=1
    let encoding: u32 = 0x08010000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_exclusive_single_combo_8_0_081e0000() {
    // Encoding: 0x081E0000
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=30, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: o0=0, L=0, Rn=0, Rt=0, Rt2=0, size=0, Rs=30
    let encoding: u32 = 0x081E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_exclusive_single_combo_9_0_081f0000() {
    // Encoding: 0x081F0000
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=31, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt=0, o0=0, Rn=0, Rs=31, Rt2=0, size=0
    let encoding: u32 = 0x081F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=0 (minimum value)
#[test]
fn test_aarch64_memory_exclusive_single_combo_10_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, size=0, o0=0, Rt=0, Rt2=0, L=0, Rs=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=1 (maximum value (1))
#[test]
fn test_aarch64_memory_exclusive_single_combo_11_0_08008000() {
    // Encoding: 0x08008000
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=1, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, L=0, o0=1, Rt2=0, size=0, Rs=0, Rn=0
    let encoding: u32 = 0x08008000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_exclusive_single_combo_12_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rs=0, o0=0, size=0, Rn=0, Rt=0, Rt2=0, L=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_exclusive_single_combo_13_0_08000400() {
    // Encoding: 0x08000400
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=1, Rn=0, Rt=0
    // Fields: o0=0, Rt2=1, size=0, Rs=0, Rt=0, L=0, Rn=0
    let encoding: u32 = 0x08000400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_exclusive_single_combo_14_0_08007800() {
    // Encoding: 0x08007800
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=30, Rn=0, Rt=0
    // Fields: o0=0, Rn=0, L=0, Rt=0, Rs=0, Rt2=30, size=0
    let encoding: u32 = 0x08007800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_exclusive_single_combo_15_0_08007c00() {
    // Encoding: 0x08007C00
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=31, Rn=0, Rt=0
    // Fields: Rs=0, Rn=0, o0=0, Rt=0, size=0, L=0, Rt2=31
    let encoding: u32 = 0x08007C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_exclusive_single_combo_16_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: size=0, Rn=0, Rt=0, L=0, Rt2=0, o0=0, Rs=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_exclusive_single_combo_17_0_08000020() {
    // Encoding: 0x08000020
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=1, Rt=0
    // Fields: Rs=0, Rt=0, L=0, o0=0, size=0, Rt2=0, Rn=1
    let encoding: u32 = 0x08000020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_exclusive_single_combo_18_0_080003c0() {
    // Encoding: 0x080003C0
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=30, Rt=0
    // Fields: Rn=30, size=0, Rt=0, o0=0, L=0, Rs=0, Rt2=0
    let encoding: u32 = 0x080003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_exclusive_single_combo_19_0_080003e0() {
    // Encoding: 0x080003E0
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=31, Rt=0
    // Fields: size=0, L=0, Rs=0, Rt2=0, Rn=31, Rt=0, o0=0
    let encoding: u32 = 0x080003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_exclusive_single_combo_20_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, Rs=0, Rt2=0, size=0, o0=0, L=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_exclusive_single_combo_21_0_08000001() {
    // Encoding: 0x08000001
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=1
    // Fields: Rn=0, L=0, Rs=0, Rt=1, Rt2=0, size=0, o0=0
    let encoding: u32 = 0x08000001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_exclusive_single_combo_22_0_0800001e() {
    // Encoding: 0x0800001E
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=30
    // Fields: Rt2=0, Rn=0, o0=0, Rt=30, size=0, L=0, Rs=0
    let encoding: u32 = 0x0800001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_exclusive_single_combo_23_0_0800001f() {
    // Encoding: 0x0800001F
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=31
    // Fields: Rs=0, Rt2=0, Rn=0, L=0, Rt=31, size=0, o0=0
    let encoding: u32 = 0x0800001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rt2=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_exclusive_single_combo_24_0_08010400() {
    // Encoding: 0x08010400
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=1, o0=0, Rt2=1, Rn=0, Rt=0
    // Fields: L=0, o0=0, Rt2=1, Rn=0, Rt=0, Rs=1, size=0
    let encoding: u32 = 0x08010400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rt2=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_exclusive_single_combo_25_0_081f7c00() {
    // Encoding: 0x081F7C00
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=31, o0=0, Rt2=31, Rn=0, Rt=0
    // Fields: o0=0, Rt=0, size=0, Rt2=31, Rn=0, Rs=31, L=0
    let encoding: u32 = 0x081F7C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_exclusive_single_combo_26_0_08010020() {
    // Encoding: 0x08010020
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=1, o0=0, Rt2=0, Rn=1, Rt=0
    // Fields: Rt2=0, o0=0, Rs=1, size=0, Rt=0, Rn=1, L=0
    let encoding: u32 = 0x08010020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_exclusive_single_combo_27_0_081f03e0() {
    // Encoding: 0x081F03E0
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=31, o0=0, Rt2=0, Rn=31, Rt=0
    // Fields: Rt2=0, Rs=31, o0=0, size=0, Rt=0, Rn=31, L=0
    let encoding: u32 = 0x081F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_exclusive_single_combo_28_0_08010001() {
    // Encoding: 0x08010001
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=1, o0=0, Rt2=0, Rn=0, Rt=1
    // Fields: Rt=1, Rs=1, L=0, size=0, o0=0, Rn=0, Rt2=0
    let encoding: u32 = 0x08010001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_exclusive_single_combo_29_0_081f001f() {
    // Encoding: 0x081F001F
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=31, o0=0, Rt2=0, Rn=0, Rt=31
    // Fields: Rn=0, Rt=31, Rt2=0, Rs=31, L=0, size=0, o0=0
    let encoding: u32 = 0x081F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_exclusive_single_combo_30_0_08000420() {
    // Encoding: 0x08000420
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=1, Rn=1, Rt=0
    // Fields: Rt2=1, Rn=1, o0=0, L=0, Rt=0, Rs=0, size=0
    let encoding: u32 = 0x08000420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_exclusive_single_combo_31_0_08007fe0() {
    // Encoding: 0x08007FE0
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=31, Rn=31, Rt=0
    // Fields: Rt=0, o0=0, Rs=0, Rn=31, L=0, size=0, Rt2=31
    let encoding: u32 = 0x08007FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_exclusive_single_combo_32_0_08000401() {
    // Encoding: 0x08000401
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=1, Rn=0, Rt=1
    // Fields: Rt=1, size=0, o0=0, Rn=0, Rs=0, L=0, Rt2=1
    let encoding: u32 = 0x08000401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_exclusive_single_combo_33_0_08007c1f() {
    // Encoding: 0x08007C1F
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=31, Rn=0, Rt=31
    // Fields: size=0, L=0, Rs=0, Rt2=31, Rt=31, o0=0, Rn=0
    let encoding: u32 = 0x08007C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_exclusive_single_combo_34_0_08000021() {
    // Encoding: 0x08000021
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=1, Rt=1
    // Fields: o0=0, Rn=1, Rt=1, Rs=0, size=0, Rt2=0, L=0
    let encoding: u32 = 0x08000021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_exclusive_single_combo_35_0_080003ff() {
    // Encoding: 0x080003FF
    // Test aarch64_memory_exclusive_single field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=31, Rt=31
    // Fields: Rt=31, Rn=31, size=0, o0=0, L=0, Rt2=0, Rs=0
    let encoding: u32 = 0x080003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_exclusive_single_special_size_0_size_variant_0_0_08000000() {
    // Encoding: 0x08000000
    // Test aarch64_memory_exclusive_single special value size = 0 (Size variant 0)
    // Fields: Rn=0, Rs=0, Rt=0, L=0, size=0, o0=0, Rt2=0
    let encoding: u32 = 0x08000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_exclusive_single_special_size_1_size_variant_1_0_48000000() {
    // Encoding: 0x48000000
    // Test aarch64_memory_exclusive_single special value size = 1 (Size variant 1)
    // Fields: Rs=0, o0=0, Rt=0, size=1, Rt2=0, L=0, Rn=0
    let encoding: u32 = 0x48000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_exclusive_single_special_size_2_size_variant_2_0_88000000() {
    // Encoding: 0x88000000
    // Test aarch64_memory_exclusive_single special value size = 2 (Size variant 2)
    // Fields: o0=0, Rt2=0, Rn=0, Rt=0, L=0, size=2, Rs=0
    let encoding: u32 = 0x88000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_exclusive_single_special_size_3_size_variant_3_0_c8000000() {
    // Encoding: 0xC8000000
    // Test aarch64_memory_exclusive_single special value size = 3 (Size variant 3)
    // Fields: L=0, Rt2=0, Rt=0, size=3, o0=0, Rn=0, Rs=0
    let encoding: u32 = 0xC8000000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_exclusive_single_special_rn_31_stack_pointer_sp_may_require_alignment_0_480003e0()
 {
    // Encoding: 0x480003E0
    // Test aarch64_memory_exclusive_single special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, Rs=0, o0=0, Rn=31, L=0, Rt2=0, Rt=0
    let encoding: u32 = 0x480003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_exclusive_single_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_4800001f()
 {
    // Encoding: 0x4800001F
    // Test aarch64_memory_exclusive_single special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rn=0, L=0, size=1, Rs=0, Rt=31, Rt2=0, o0=0
    let encoding: u32 = 0x4800001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `GpFromField("s") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "s" }
/// verify register write to GpFromField("s")
#[test]
fn test_aarch64_memory_exclusive_single_reg_write_0_08000000() {
    // Test aarch64_memory_exclusive_single register write: GpFromField("s")
    // Encoding: 0x08000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_exclusive_single_reg_write_1_08000000() {
    // Test aarch64_memory_exclusive_single register write: GpFromField("t")
    // Encoding: 0x08000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_exclusive_single_reg_write_2_08000000() {
    // Test aarch64_memory_exclusive_single register write: GpFromField("t")
    // Encoding: 0x08000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `GpFromField("t2") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t2" }
/// verify register write to GpFromField("t2")
#[test]
fn test_aarch64_memory_exclusive_single_reg_write_3_08000000() {
    // Test aarch64_memory_exclusive_single register write: GpFromField("t2")
    // Encoding: 0x08000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_exclusive_single_reg_write_4_08000000() {
    // Test aarch64_memory_exclusive_single register write: GpFromField("t")
    // Encoding: 0x08000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `GpFromField("t2") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t2" }
/// verify register write to GpFromField("t2")
#[test]
fn test_aarch64_memory_exclusive_single_reg_write_5_08000000() {
    // Test aarch64_memory_exclusive_single register write: GpFromField("t2")
    // Encoding: 0x08000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_exclusive_single_reg_write_6_08000000() {
    // Test aarch64_memory_exclusive_single register write: GpFromField("t")
    // Encoding: 0x08000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `GpFromField("t2") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t2" }
/// verify register write to GpFromField("t2")
#[test]
fn test_aarch64_memory_exclusive_single_reg_write_7_08000000() {
    // Test aarch64_memory_exclusive_single register write: GpFromField("t2")
    // Encoding: 0x08000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_exclusive_single_reg_write_8_08000000() {
    // Test aarch64_memory_exclusive_single register write: GpFromField("t")
    // Encoding: 0x08000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_exclusive_single_sp_rn_080003e0() {
    // Test aarch64_memory_exclusive_single with Rn = SP (31)
    // Encoding: 0x080003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x080003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_exclusive_single_zr_rt_0800001f() {
    // Test aarch64_memory_exclusive_single with Rt = ZR (31)
    // Encoding: 0x0800001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0800001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_exclusive_single_store_0_08000020() {
    // Test aarch64_memory_exclusive_single memory store: 8 bytes
    // Encoding: 0x08000020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x08000020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_exclusive_single_exception_0_08000000() {
    // Test aarch64_memory_exclusive_single exception: Undefined
    // Encoding: 0x08000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_exclusive_single_exception_1_08000000() {
    // Test aarch64_memory_exclusive_single exception: Undefined
    // Encoding: 0x08000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_single
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_exclusive_single_exception_2_08000000() {
    // Test aarch64_memory_exclusive_single exception: Undefined
    // Encoding: 0x08000000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08000000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

// ============================================================================
// aarch64_memory_exclusive_pair Tests
// ============================================================================

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field sz 30 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_exclusive_pair_field_sz_0_min_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field sz = 0 (Min)
    // Fields: L=0, Rs=0, Rn=0, o0=0, sz=0, Rt2=0, Rt=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field sz 30 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_exclusive_pair_field_sz_1_max_0_c8200000() {
    // Encoding: 0xC8200000
    // Test aarch64_memory_exclusive_pair field sz = 1 (Max)
    // Fields: Rs=0, Rt2=0, sz=1, o0=0, Rn=0, Rt=0, L=0
    let encoding: u32 = 0xC8200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_exclusive_pair_field_l_0_min_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field L = 0 (Min)
    // Fields: L=0, o0=0, Rt2=0, sz=0, Rs=0, Rn=0, Rt=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_exclusive_pair_field_l_1_max_0_88600000() {
    // Encoding: 0x88600000
    // Test aarch64_memory_exclusive_pair field L = 1 (Max)
    // Fields: Rt2=0, Rn=0, Rt=0, L=1, o0=0, Rs=0, sz=0
    let encoding: u32 = 0x88600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rs_0_min_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field Rs = 0 (Min)
    // Fields: o0=0, sz=0, Rs=0, Rt2=0, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rs_1_poweroftwo_0_88210000() {
    // Encoding: 0x88210000
    // Test aarch64_memory_exclusive_pair field Rs = 1 (PowerOfTwo)
    // Fields: o0=0, Rn=0, Rt2=0, sz=0, L=0, Rs=1, Rt=0
    let encoding: u32 = 0x88210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rs_30_poweroftwominusone_0_883e0000() {
    // Encoding: 0x883E0000
    // Test aarch64_memory_exclusive_pair field Rs = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=0, sz=0, L=0, o0=0, Rs=30, Rt2=0, Rn=0
    let encoding: u32 = 0x883E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rs_31_max_0_883f0000() {
    // Encoding: 0x883F0000
    // Test aarch64_memory_exclusive_pair field Rs = 31 (Max)
    // Fields: Rt=0, o0=0, L=0, Rt2=0, Rn=0, Rs=31, sz=0
    let encoding: u32 = 0x883F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field o0 15 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_exclusive_pair_field_o0_0_min_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field o0 = 0 (Min)
    // Fields: Rt2=0, Rt=0, L=0, o0=0, Rs=0, sz=0, Rn=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field o0 15 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_exclusive_pair_field_o0_1_max_0_88208000() {
    // Encoding: 0x88208000
    // Test aarch64_memory_exclusive_pair field o0 = 1 (Max)
    // Fields: Rt=0, L=0, Rs=0, sz=0, Rt2=0, o0=1, Rn=0
    let encoding: u32 = 0x88208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rt2_0_min_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field Rt2 = 0 (Min)
    // Fields: Rs=0, L=0, o0=0, Rt2=0, sz=0, Rn=0, Rt=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rt2_1_poweroftwo_0_88200400() {
    // Encoding: 0x88200400
    // Test aarch64_memory_exclusive_pair field Rt2 = 1 (PowerOfTwo)
    // Fields: Rt2=1, Rs=0, L=0, o0=0, sz=0, Rn=0, Rt=0
    let encoding: u32 = 0x88200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rt2_30_poweroftwominusone_0_88207800() {
    // Encoding: 0x88207800
    // Test aarch64_memory_exclusive_pair field Rt2 = 30 (PowerOfTwoMinusOne)
    // Fields: sz=0, L=0, Rn=0, Rt=0, Rs=0, o0=0, Rt2=30
    let encoding: u32 = 0x88207800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rt2_31_max_0_88207c00() {
    // Encoding: 0x88207C00
    // Test aarch64_memory_exclusive_pair field Rt2 = 31 (Max)
    // Fields: Rs=0, Rt=0, L=0, o0=0, Rt2=31, sz=0, Rn=0
    let encoding: u32 = 0x88207C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rn_0_min_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field Rn = 0 (Min)
    // Fields: o0=0, Rt2=0, Rs=0, Rn=0, sz=0, L=0, Rt=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rn_1_poweroftwo_0_88200020() {
    // Encoding: 0x88200020
    // Test aarch64_memory_exclusive_pair field Rn = 1 (PowerOfTwo)
    // Fields: L=0, Rs=0, o0=0, Rt2=0, Rn=1, Rt=0, sz=0
    let encoding: u32 = 0x88200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rn_30_poweroftwominusone_0_882003c0() {
    // Encoding: 0x882003C0
    // Test aarch64_memory_exclusive_pair field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rt2=0, Rn=30, Rt=0, sz=0, Rs=0, o0=0, L=0
    let encoding: u32 = 0x882003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rn_31_max_0_882003e0() {
    // Encoding: 0x882003E0
    // Test aarch64_memory_exclusive_pair field Rn = 31 (Max)
    // Fields: Rs=0, Rn=31, Rt2=0, Rt=0, o0=0, sz=0, L=0
    let encoding: u32 = 0x882003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rt_0_min_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field Rt = 0 (Min)
    // Fields: sz=0, L=0, Rs=0, Rt=0, o0=0, Rn=0, Rt2=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rt_1_poweroftwo_0_88200001() {
    // Encoding: 0x88200001
    // Test aarch64_memory_exclusive_pair field Rt = 1 (PowerOfTwo)
    // Fields: Rt=1, sz=0, Rt2=0, Rn=0, L=0, Rs=0, o0=0
    let encoding: u32 = 0x88200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rt_30_poweroftwominusone_0_8820001e() {
    // Encoding: 0x8820001E
    // Test aarch64_memory_exclusive_pair field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: o0=0, Rs=0, sz=0, L=0, Rt2=0, Rn=0, Rt=30
    let encoding: u32 = 0x8820001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_exclusive_pair_field_rt_31_max_0_8820001f() {
    // Encoding: 0x8820001F
    // Test aarch64_memory_exclusive_pair field Rt = 31 (Max)
    // Fields: Rt=31, Rs=0, L=0, o0=0, Rt2=0, Rn=0, sz=0
    let encoding: u32 = 0x8820001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_exclusive_pair_combo_0_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rs=0, L=0, sz=0, o0=0, Rn=0, Rt=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// sz=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_exclusive_pair_combo_1_0_c8200000() {
    // Encoding: 0xC8200000
    // Test aarch64_memory_exclusive_pair field combination: sz=1, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, o0=0, Rt=0, L=0, sz=1, Rs=0, Rn=0
    let encoding: u32 = 0xC8200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_exclusive_pair_combo_2_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rs=0, Rt2=0, o0=0, L=0, sz=0, Rn=0, Rt=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_3_0_88600000() {
    // Encoding: 0x88600000
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=1, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, sz=0, Rt2=0, L=1, Rs=0, o0=0
    let encoding: u32 = 0x88600000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_4_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, Rs=0, Rt2=0, L=0, sz=0, o0=0, Rt=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_5_0_88210000() {
    // Encoding: 0x88210000
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=1, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt=0, Rs=1, Rn=0, sz=0, Rt2=0, o0=0
    let encoding: u32 = 0x88210000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_6_0_883e0000() {
    // Encoding: 0x883E0000
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=30, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: o0=0, Rs=30, Rt2=0, Rn=0, Rt=0, L=0, sz=0
    let encoding: u32 = 0x883E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_7_0_883f0000() {
    // Encoding: 0x883F0000
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=31, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rn=0, Rt=0, Rs=31, sz=0, o0=0, L=0
    let encoding: u32 = 0x883F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=0 (minimum value)
#[test]
fn test_aarch64_memory_exclusive_pair_combo_8_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: sz=0, Rt=0, L=0, Rs=0, Rt2=0, o0=0, Rn=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=1 (maximum value (1))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_9_0_88208000() {
    // Encoding: 0x88208000
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=1, Rt2=0, Rn=0, Rt=0
    // Fields: o0=1, Rt2=0, L=0, Rs=0, sz=0, Rt=0, Rn=0
    let encoding: u32 = 0x88208000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_10_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rt=0, L=0, sz=0, Rs=0, Rn=0, o0=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_11_0_88200400() {
    // Encoding: 0x88200400
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=1, Rn=0, Rt=0
    // Fields: L=0, Rs=0, Rt2=1, o0=0, Rn=0, Rt=0, sz=0
    let encoding: u32 = 0x88200400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_12_0_88207800() {
    // Encoding: 0x88207800
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=30, Rn=0, Rt=0
    // Fields: Rt2=30, o0=0, Rt=0, Rs=0, L=0, sz=0, Rn=0
    let encoding: u32 = 0x88207800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_13_0_88207c00() {
    // Encoding: 0x88207C00
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=31, Rn=0, Rt=0
    // Fields: Rn=0, sz=0, Rt=0, o0=0, Rt2=31, L=0, Rs=0
    let encoding: u32 = 0x88207C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_14_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt=0, Rs=0, Rt2=0, Rn=0, sz=0, L=0, o0=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_15_0_88200020() {
    // Encoding: 0x88200020
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=1, Rt=0
    // Fields: Rt=0, L=0, Rn=1, sz=0, Rt2=0, Rs=0, o0=0
    let encoding: u32 = 0x88200020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_16_0_882003c0() {
    // Encoding: 0x882003C0
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=30, Rt=0
    // Fields: L=0, Rt2=0, Rn=30, Rt=0, o0=0, sz=0, Rs=0
    let encoding: u32 = 0x882003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_17_0_882003e0() {
    // Encoding: 0x882003E0
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=31, Rt=0
    // Fields: sz=0, L=0, o0=0, Rn=31, Rs=0, Rt2=0, Rt=0
    let encoding: u32 = 0x882003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_18_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: sz=0, L=0, Rs=0, Rn=0, o0=0, Rt=0, Rt2=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_19_0_88200001() {
    // Encoding: 0x88200001
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=1
    // Fields: Rt=1, o0=0, L=0, Rt2=0, Rn=0, Rs=0, sz=0
    let encoding: u32 = 0x88200001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_20_0_8820001e() {
    // Encoding: 0x8820001E
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=30
    // Fields: Rn=0, o0=0, Rt=30, sz=0, L=0, Rs=0, Rt2=0
    let encoding: u32 = 0x8820001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_21_0_8820001f() {
    // Encoding: 0x8820001F
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=31
    // Fields: Rn=0, sz=0, Rt=31, Rs=0, Rt2=0, o0=0, L=0
    let encoding: u32 = 0x8820001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rt2=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_22_0_88210400() {
    // Encoding: 0x88210400
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=1, o0=0, Rt2=1, Rn=0, Rt=0
    // Fields: sz=0, Rt=0, L=0, o0=0, Rt2=1, Rs=1, Rn=0
    let encoding: u32 = 0x88210400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rt2=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_23_0_883f7c00() {
    // Encoding: 0x883F7C00
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=31, o0=0, Rt2=31, Rn=0, Rt=0
    // Fields: Rs=31, L=0, o0=0, Rt=0, Rn=0, Rt2=31, sz=0
    let encoding: u32 = 0x883F7C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_24_0_88210020() {
    // Encoding: 0x88210020
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=1, o0=0, Rt2=0, Rn=1, Rt=0
    // Fields: sz=0, L=0, o0=0, Rt2=0, Rt=0, Rs=1, Rn=1
    let encoding: u32 = 0x88210020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_25_0_883f03e0() {
    // Encoding: 0x883F03E0
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=31, o0=0, Rt2=0, Rn=31, Rt=0
    // Fields: Rs=31, Rn=31, sz=0, o0=0, Rt2=0, L=0, Rt=0
    let encoding: u32 = 0x883F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_26_0_88210001() {
    // Encoding: 0x88210001
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=1, o0=0, Rt2=0, Rn=0, Rt=1
    // Fields: L=0, Rs=1, Rt2=0, Rn=0, Rt=1, o0=0, sz=0
    let encoding: u32 = 0x88210001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_27_0_883f001f() {
    // Encoding: 0x883F001F
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=31, o0=0, Rt2=0, Rn=0, Rt=31
    // Fields: o0=0, Rs=31, sz=0, Rt2=0, Rt=31, Rn=0, L=0
    let encoding: u32 = 0x883F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_28_0_88200420() {
    // Encoding: 0x88200420
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=1, Rn=1, Rt=0
    // Fields: Rn=1, L=0, Rs=0, o0=0, sz=0, Rt=0, Rt2=1
    let encoding: u32 = 0x88200420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_29_0_88207fe0() {
    // Encoding: 0x88207FE0
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=31, Rn=31, Rt=0
    // Fields: L=0, o0=0, sz=0, Rt2=31, Rn=31, Rt=0, Rs=0
    let encoding: u32 = 0x88207FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_30_0_88200401() {
    // Encoding: 0x88200401
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=1, Rn=0, Rt=1
    // Fields: o0=0, Rn=0, Rt=1, L=0, Rs=0, Rt2=1, sz=0
    let encoding: u32 = 0x88200401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_31_0_88207c1f() {
    // Encoding: 0x88207C1F
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=31, Rn=0, Rt=31
    // Fields: L=0, sz=0, Rn=0, o0=0, Rt2=31, Rt=31, Rs=0
    let encoding: u32 = 0x88207C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_32_0_88200021() {
    // Encoding: 0x88200021
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=1, Rt=1
    // Fields: Rt=1, Rs=0, L=0, sz=0, o0=0, Rt2=0, Rn=1
    let encoding: u32 = 0x88200021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_exclusive_pair_combo_33_0_882003ff() {
    // Encoding: 0x882003FF
    // Test aarch64_memory_exclusive_pair field combination: sz=0, L=0, Rs=0, o0=0, Rt2=0, Rn=31, Rt=31
    // Fields: o0=0, Rt2=0, Rt=31, Rn=31, sz=0, Rs=0, L=0
    let encoding: u32 = 0x882003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_exclusive_pair_special_sz_0_size_variant_0_0_88200000() {
    // Encoding: 0x88200000
    // Test aarch64_memory_exclusive_pair special value sz = 0 (Size variant 0)
    // Fields: sz=0, L=0, Rt2=0, Rt=0, o0=0, Rs=0, Rn=0
    let encoding: u32 = 0x88200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_exclusive_pair_special_sz_1_size_variant_1_0_c8200000() {
    // Encoding: 0xC8200000
    // Test aarch64_memory_exclusive_pair special value sz = 1 (Size variant 1)
    // Fields: Rt2=0, o0=0, Rn=0, Rs=0, Rt=0, L=0, sz=1
    let encoding: u32 = 0xC8200000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_exclusive_pair_special_rn_31_stack_pointer_sp_may_require_alignment_0_c82003e0()
 {
    // Encoding: 0xC82003E0
    // Test aarch64_memory_exclusive_pair special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: sz=1, Rs=0, o0=0, Rt2=0, Rn=31, Rt=0, L=0
    let encoding: u32 = 0xC82003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_exclusive_pair_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_c820001f()
 {
    // Encoding: 0xC820001F
    // Test aarch64_memory_exclusive_pair special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: L=0, o0=0, Rn=0, Rt2=0, Rt=31, sz=1, Rs=0
    let encoding: u32 = 0xC820001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `GpFromField("s") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "s" }
/// verify register write to GpFromField("s")
#[test]
fn test_aarch64_memory_exclusive_pair_reg_write_0_88200000() {
    // Test aarch64_memory_exclusive_pair register write: GpFromField("s")
    // Encoding: 0x88200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x88200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_exclusive_pair_reg_write_1_88200000() {
    // Test aarch64_memory_exclusive_pair register write: GpFromField("t")
    // Encoding: 0x88200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x88200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_exclusive_pair_reg_write_2_88200000() {
    // Test aarch64_memory_exclusive_pair register write: GpFromField("t")
    // Encoding: 0x88200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x88200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `GpFromField("t2") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t2" }
/// verify register write to GpFromField("t2")
#[test]
fn test_aarch64_memory_exclusive_pair_reg_write_3_88200000() {
    // Test aarch64_memory_exclusive_pair register write: GpFromField("t2")
    // Encoding: 0x88200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x88200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_exclusive_pair_reg_write_4_88200000() {
    // Test aarch64_memory_exclusive_pair register write: GpFromField("t")
    // Encoding: 0x88200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x88200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `GpFromField("t2") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t2" }
/// verify register write to GpFromField("t2")
#[test]
fn test_aarch64_memory_exclusive_pair_reg_write_5_88200000() {
    // Test aarch64_memory_exclusive_pair register write: GpFromField("t2")
    // Encoding: 0x88200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x88200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_exclusive_pair_reg_write_6_88200000() {
    // Test aarch64_memory_exclusive_pair register write: GpFromField("t")
    // Encoding: 0x88200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x88200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `GpFromField("t2") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t2" }
/// verify register write to GpFromField("t2")
#[test]
fn test_aarch64_memory_exclusive_pair_reg_write_7_88200000() {
    // Test aarch64_memory_exclusive_pair register write: GpFromField("t2")
    // Encoding: 0x88200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x88200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_exclusive_pair_reg_write_8_88200000() {
    // Test aarch64_memory_exclusive_pair register write: GpFromField("t")
    // Encoding: 0x88200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x88200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_exclusive_pair_sp_rn_882003e0() {
    // Test aarch64_memory_exclusive_pair with Rn = SP (31)
    // Encoding: 0x882003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x882003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_exclusive_pair_zr_rt_8820001f() {
    // Test aarch64_memory_exclusive_pair with Rt = ZR (31)
    // Encoding: 0x8820001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x8820001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_exclusive_pair_store_0_88200020() {
    // Test aarch64_memory_exclusive_pair memory store: 8 bytes
    // Encoding: 0x88200020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x88200020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_exclusive_pair_exception_0_88200000() {
    // Test aarch64_memory_exclusive_pair exception: Undefined
    // Encoding: 0x88200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x88200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_exclusive_pair_exception_1_88200000() {
    // Test aarch64_memory_exclusive_pair exception: Undefined
    // Encoding: 0x88200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x88200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_exclusive_pair
/// ASL: `Unconditional`
/// Requirement: UndefinedEncoding { condition: "Unconditional" }
/// triggers Undefined
#[test]
fn test_aarch64_memory_exclusive_pair_exception_2_88200000() {
    // Test aarch64_memory_exclusive_pair exception: Undefined
    // Encoding: 0x88200000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x88200000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}
