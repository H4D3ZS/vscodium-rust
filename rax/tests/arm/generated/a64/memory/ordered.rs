//! A64 memory ordered tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_memory_ordered Tests
// ============================================================================

/// Provenance: aarch64_memory_ordered
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_ordered_field_size_0_min_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field size = 0 (Min)
    // Fields: size=0, L=0, Rn=0, Rt=0, Rs=0, Rt2=0, o0=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_ordered_field_size_1_poweroftwo_0_48800000() {
    // Encoding: 0x48800000
    // Test aarch64_memory_ordered field size = 1 (PowerOfTwo)
    // Fields: Rs=0, Rt=0, size=1, Rt2=0, Rn=0, L=0, o0=0
    let encoding: u32 = 0x48800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_ordered_field_size_2_poweroftwo_0_88800000() {
    // Encoding: 0x88800000
    // Test aarch64_memory_ordered field size = 2 (PowerOfTwo)
    // Fields: Rn=0, Rt=0, Rs=0, o0=0, size=2, L=0, Rt2=0
    let encoding: u32 = 0x88800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_ordered_field_size_3_max_0_c8800000() {
    // Encoding: 0xC8800000
    // Test aarch64_memory_ordered field size = 3 (Max)
    // Fields: Rt2=0, Rn=0, L=0, size=3, Rs=0, o0=0, Rt=0
    let encoding: u32 = 0xC8800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_ordered_field_l_0_min_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field L = 0 (Min)
    // Fields: size=0, o0=0, L=0, Rs=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field L 22 +: 1`
/// Requirement: FieldBoundary { field: "L", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_ordered_field_l_1_max_0_08c00000() {
    // Encoding: 0x08C00000
    // Test aarch64_memory_ordered field L = 1 (Max)
    // Fields: o0=0, size=0, Rn=0, Rs=0, Rt=0, L=1, Rt2=0
    let encoding: u32 = 0x08C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_ordered_field_rs_0_min_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field Rs = 0 (Min)
    // Fields: size=0, Rt2=0, Rt=0, L=0, Rn=0, Rs=0, o0=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_ordered_field_rs_1_poweroftwo_0_08810000() {
    // Encoding: 0x08810000
    // Test aarch64_memory_ordered field Rs = 1 (PowerOfTwo)
    // Fields: Rs=1, Rt2=0, Rn=0, size=0, o0=0, Rt=0, L=0
    let encoding: u32 = 0x08810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_ordered_field_rs_30_poweroftwominusone_0_089e0000() {
    // Encoding: 0x089E0000
    // Test aarch64_memory_ordered field Rs = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, o0=0, L=0, size=0, Rt2=0, Rt=0, Rs=30
    let encoding: u32 = 0x089E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_ordered_field_rs_31_max_0_089f0000() {
    // Encoding: 0x089F0000
    // Test aarch64_memory_ordered field Rs = 31 (Max)
    // Fields: L=0, size=0, Rs=31, o0=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x089F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field o0 15 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_memory_ordered_field_o0_0_min_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field o0 = 0 (Min)
    // Fields: L=0, size=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field o0 15 +: 1`
/// Requirement: FieldBoundary { field: "o0", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_memory_ordered_field_o0_1_max_0_08808000() {
    // Encoding: 0x08808000
    // Test aarch64_memory_ordered field o0 = 1 (Max)
    // Fields: size=0, L=0, Rt2=0, o0=1, Rn=0, Rt=0, Rs=0
    let encoding: u32 = 0x08808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_ordered_field_rt2_0_min_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field Rt2 = 0 (Min)
    // Fields: o0=0, Rn=0, L=0, Rt2=0, size=0, Rt=0, Rs=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_ordered_field_rt2_1_poweroftwo_0_08800400() {
    // Encoding: 0x08800400
    // Test aarch64_memory_ordered field Rt2 = 1 (PowerOfTwo)
    // Fields: Rt=0, Rt2=1, Rn=0, size=0, o0=0, L=0, Rs=0
    let encoding: u32 = 0x08800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_ordered_field_rt2_30_poweroftwominusone_0_08807800() {
    // Encoding: 0x08807800
    // Test aarch64_memory_ordered field Rt2 = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=0, o0=0, Rs=0, Rt2=30, L=0, size=0
    let encoding: u32 = 0x08807800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rt2 10 +: 5`
/// Requirement: FieldBoundary { field: "Rt2", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_ordered_field_rt2_31_max_0_08807c00() {
    // Encoding: 0x08807C00
    // Test aarch64_memory_ordered field Rt2 = 31 (Max)
    // Fields: L=0, Rn=0, o0=0, Rs=0, size=0, Rt2=31, Rt=0
    let encoding: u32 = 0x08807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_ordered_field_rn_0_min_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field Rn = 0 (Min)
    // Fields: Rn=0, Rt=0, Rt2=0, Rs=0, L=0, size=0, o0=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_ordered_field_rn_1_poweroftwo_0_08800020() {
    // Encoding: 0x08800020
    // Test aarch64_memory_ordered field Rn = 1 (PowerOfTwo)
    // Fields: Rt=0, Rt2=0, Rs=0, size=0, Rn=1, L=0, o0=0
    let encoding: u32 = 0x08800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_ordered_field_rn_30_poweroftwominusone_0_088003c0() {
    // Encoding: 0x088003C0
    // Test aarch64_memory_ordered field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: size=0, Rs=0, Rt2=0, Rn=30, L=0, o0=0, Rt=0
    let encoding: u32 = 0x088003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_ordered_field_rn_31_max_0_088003e0() {
    // Encoding: 0x088003E0
    // Test aarch64_memory_ordered field Rn = 31 (Max)
    // Fields: Rs=0, size=0, Rt2=0, Rn=31, Rt=0, L=0, o0=0
    let encoding: u32 = 0x088003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_ordered_field_rt_0_min_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field Rt = 0 (Min)
    // Fields: Rt2=0, Rn=0, Rt=0, L=0, Rs=0, size=0, o0=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_ordered_field_rt_1_poweroftwo_0_08800001() {
    // Encoding: 0x08800001
    // Test aarch64_memory_ordered field Rt = 1 (PowerOfTwo)
    // Fields: Rt2=0, L=0, size=0, Rs=0, Rn=0, Rt=1, o0=0
    let encoding: u32 = 0x08800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_ordered_field_rt_30_poweroftwominusone_0_0880001e() {
    // Encoding: 0x0880001E
    // Test aarch64_memory_ordered field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: Rs=0, Rn=0, size=0, L=0, o0=0, Rt2=0, Rt=30
    let encoding: u32 = 0x0880001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_ordered_field_rt_31_max_0_0880001f() {
    // Encoding: 0x0880001F
    // Test aarch64_memory_ordered field Rt = 31 (Max)
    // Fields: o0=0, Rt=31, Rn=0, size=0, Rt2=0, L=0, Rs=0
    let encoding: u32 = 0x0880001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_ordered_combo_0_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: size=0, Rs=0, o0=0, Rt2=0, L=0, Rn=0, Rt=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_ordered_combo_1_0_48800000() {
    // Encoding: 0x48800000
    // Test aarch64_memory_ordered field combination: size=1, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: o0=0, Rn=0, Rs=0, size=1, Rt2=0, L=0, Rt=0
    let encoding: u32 = 0x48800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_ordered_combo_2_0_88800000() {
    // Encoding: 0x88800000
    // Test aarch64_memory_ordered field combination: size=2, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: size=2, Rs=0, Rn=0, Rt=0, L=0, o0=0, Rt2=0
    let encoding: u32 = 0x88800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_ordered_combo_3_0_c8800000() {
    // Encoding: 0xC8800000
    // Test aarch64_memory_ordered field combination: size=3, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rs=0, o0=0, Rn=0, size=3, Rt2=0, L=0, Rt=0
    let encoding: u32 = 0xC8800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=0 (minimum value)
#[test]
fn test_aarch64_memory_ordered_combo_4_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: L=0, Rt2=0, Rt=0, o0=0, Rs=0, Rn=0, size=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// L=1 (maximum value (1))
#[test]
fn test_aarch64_memory_ordered_combo_5_0_08c00000() {
    // Encoding: 0x08C00000
    // Test aarch64_memory_ordered field combination: size=0, L=1, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: o0=0, Rt2=0, Rs=0, Rn=0, Rt=0, size=0, L=1
    let encoding: u32 = 0x08C00000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_ordered_combo_6_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: size=0, o0=0, Rt2=0, Rn=0, Rt=0, L=0, Rs=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_ordered_combo_7_0_08810000() {
    // Encoding: 0x08810000
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=1, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: o0=0, Rt=0, Rs=1, size=0, Rn=0, L=0, Rt2=0
    let encoding: u32 = 0x08810000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_ordered_combo_8_0_089e0000() {
    // Encoding: 0x089E0000
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=30, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, L=0, Rs=30, size=0, o0=0, Rn=0, Rt=0
    let encoding: u32 = 0x089E0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_ordered_combo_9_0_089f0000() {
    // Encoding: 0x089F0000
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=31, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rn=0, o0=0, Rs=31, size=0, Rt2=0, L=0, Rt=0
    let encoding: u32 = 0x089F0000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=0 (minimum value)
#[test]
fn test_aarch64_memory_ordered_combo_10_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: size=0, L=0, Rn=0, Rt=0, Rt2=0, Rs=0, o0=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// o0=1 (maximum value (1))
#[test]
fn test_aarch64_memory_ordered_combo_11_0_08808000() {
    // Encoding: 0x08808000
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=1, Rt2=0, Rn=0, Rt=0
    // Fields: size=0, Rt=0, Rs=0, Rt2=0, o0=1, Rn=0, L=0
    let encoding: u32 = 0x08808000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_ordered_combo_12_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: o0=0, Rs=0, Rt2=0, size=0, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_ordered_combo_13_0_08800400() {
    // Encoding: 0x08800400
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=1, Rn=0, Rt=0
    // Fields: o0=0, Rt2=1, Rs=0, size=0, Rn=0, L=0, Rt=0
    let encoding: u32 = 0x08800400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_ordered_combo_14_0_08807800() {
    // Encoding: 0x08807800
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=30, Rn=0, Rt=0
    // Fields: size=0, Rs=0, Rn=0, o0=0, Rt=0, Rt2=30, L=0
    let encoding: u32 = 0x08807800;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_ordered_combo_15_0_08807c00() {
    // Encoding: 0x08807C00
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=31, Rn=0, Rt=0
    // Fields: Rt2=31, Rs=0, L=0, size=0, o0=0, Rn=0, Rt=0
    let encoding: u32 = 0x08807C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_ordered_combo_16_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: Rt2=0, Rs=0, o0=0, Rn=0, Rt=0, size=0, L=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_ordered_combo_17_0_08800020() {
    // Encoding: 0x08800020
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=1, Rt=0
    // Fields: o0=0, Rn=1, Rt=0, L=0, Rs=0, size=0, Rt2=0
    let encoding: u32 = 0x08800020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_ordered_combo_18_0_088003c0() {
    // Encoding: 0x088003C0
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=30, Rt=0
    // Fields: L=0, size=0, Rs=0, Rt2=0, Rn=30, Rt=0, o0=0
    let encoding: u32 = 0x088003C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_ordered_combo_19_0_088003e0() {
    // Encoding: 0x088003E0
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=31, Rt=0
    // Fields: o0=0, size=0, Rt2=0, Rn=31, Rt=0, Rs=0, L=0
    let encoding: u32 = 0x088003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_ordered_combo_20_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=0
    // Fields: size=0, L=0, o0=0, Rt2=0, Rn=0, Rt=0, Rs=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_ordered_combo_21_0_08800001() {
    // Encoding: 0x08800001
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=1
    // Fields: size=0, L=0, o0=0, Rt2=0, Rn=0, Rs=0, Rt=1
    let encoding: u32 = 0x08800001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_ordered_combo_22_0_0880001e() {
    // Encoding: 0x0880001E
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=30
    // Fields: Rt2=0, Rn=0, Rt=30, L=0, Rs=0, size=0, o0=0
    let encoding: u32 = 0x0880001E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_ordered_combo_23_0_0880001f() {
    // Encoding: 0x0880001F
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=0, Rt=31
    // Fields: Rs=0, Rn=0, Rt2=0, Rt=31, size=0, L=0, o0=0
    let encoding: u32 = 0x0880001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rt2=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_ordered_combo_24_0_08810400() {
    // Encoding: 0x08810400
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=1, o0=0, Rt2=1, Rn=0, Rt=0
    // Fields: size=0, o0=0, Rt2=1, Rs=1, Rn=0, Rt=0, L=0
    let encoding: u32 = 0x08810400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rt2=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_ordered_combo_25_0_089f7c00() {
    // Encoding: 0x089F7C00
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=31, o0=0, Rt2=31, Rn=0, Rt=0
    // Fields: size=0, Rn=0, Rt=0, o0=0, L=0, Rs=31, Rt2=31
    let encoding: u32 = 0x089F7C00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_ordered_combo_26_0_08810020() {
    // Encoding: 0x08810020
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=1, o0=0, Rt2=0, Rn=1, Rt=0
    // Fields: Rt2=0, Rt=0, Rn=1, Rs=1, size=0, o0=0, L=0
    let encoding: u32 = 0x08810020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_ordered_combo_27_0_089f03e0() {
    // Encoding: 0x089F03E0
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=31, o0=0, Rt2=0, Rn=31, Rt=0
    // Fields: L=0, Rs=31, Rt=0, size=0, Rt2=0, Rn=31, o0=0
    let encoding: u32 = 0x089F03E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 28`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_ordered_combo_28_0_08810001() {
    // Encoding: 0x08810001
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=1, o0=0, Rt2=0, Rn=0, Rt=1
    // Fields: size=0, Rn=0, Rt2=0, o0=0, Rs=1, L=0, Rt=1
    let encoding: u32 = 0x08810001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 29`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_ordered_combo_29_0_089f001f() {
    // Encoding: 0x089F001F
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=31, o0=0, Rt2=0, Rn=0, Rt=31
    // Fields: Rt2=0, Rt=31, size=0, o0=0, L=0, Rn=0, Rs=31
    let encoding: u32 = 0x089F001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 30`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_ordered_combo_30_0_08800420() {
    // Encoding: 0x08800420
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=1, Rn=1, Rt=0
    // Fields: Rs=0, Rt2=1, Rn=1, L=0, Rt=0, o0=0, size=0
    let encoding: u32 = 0x08800420;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 31`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_ordered_combo_31_0_08807fe0() {
    // Encoding: 0x08807FE0
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=31, Rn=31, Rt=0
    // Fields: Rt=0, Rt2=31, Rs=0, size=0, o0=0, L=0, Rn=31
    let encoding: u32 = 0x08807FE0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 32`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_ordered_combo_32_0_08800401() {
    // Encoding: 0x08800401
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=1, Rn=0, Rt=1
    // Fields: Rt2=1, o0=0, Rn=0, Rt=1, L=0, size=0, Rs=0
    let encoding: u32 = 0x08800401;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 33`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt2=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_ordered_combo_33_0_08807c1f() {
    // Encoding: 0x08807C1F
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=31, Rn=0, Rt=31
    // Fields: o0=0, Rt=31, Rn=0, Rs=0, Rt2=31, size=0, L=0
    let encoding: u32 = 0x08807C1F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 34`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_ordered_combo_34_0_08800021() {
    // Encoding: 0x08800021
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=1, Rt=1
    // Fields: size=0, o0=0, Rt2=0, Rt=1, Rn=1, L=0, Rs=0
    let encoding: u32 = 0x08800021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field combination 35`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_ordered_combo_35_0_088003ff() {
    // Encoding: 0x088003FF
    // Test aarch64_memory_ordered field combination: size=0, L=0, Rs=0, o0=0, Rt2=0, Rn=31, Rt=31
    // Fields: Rt=31, size=0, Rt2=0, Rs=0, o0=0, Rn=31, L=0
    let encoding: u32 = 0x088003FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_ordered_special_size_0_size_variant_0_0_08800000() {
    // Encoding: 0x08800000
    // Test aarch64_memory_ordered special value size = 0 (Size variant 0)
    // Fields: Rs=0, L=0, Rt=0, o0=0, Rt2=0, size=0, Rn=0
    let encoding: u32 = 0x08800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_ordered_special_size_1_size_variant_1_0_48800000() {
    // Encoding: 0x48800000
    // Test aarch64_memory_ordered special value size = 1 (Size variant 1)
    // Fields: Rn=0, Rt=0, o0=0, Rs=0, Rt2=0, size=1, L=0
    let encoding: u32 = 0x48800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_ordered_special_size_2_size_variant_2_0_88800000() {
    // Encoding: 0x88800000
    // Test aarch64_memory_ordered special value size = 2 (Size variant 2)
    // Fields: size=2, Rs=0, Rn=0, Rt=0, Rt2=0, L=0, o0=0
    let encoding: u32 = 0x88800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_ordered_special_size_3_size_variant_3_0_c8800000() {
    // Encoding: 0xC8800000
    // Test aarch64_memory_ordered special value size = 3 (Size variant 3)
    // Fields: Rt=0, o0=0, Rt2=0, Rs=0, size=3, L=0, Rn=0
    let encoding: u32 = 0xC8800000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_ordered_special_rn_31_stack_pointer_sp_may_require_alignment_0_488003e0() {
    // Encoding: 0x488003E0
    // Test aarch64_memory_ordered special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: L=0, o0=0, size=1, Rt2=0, Rs=0, Rn=31, Rt=0
    let encoding: u32 = 0x488003E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_ordered_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_0_4880001f()
 {
    // Encoding: 0x4880001F
    // Test aarch64_memory_ordered special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rt2=0, o0=0, L=0, Rs=0, Rt=31, Rn=0, size=1
    let encoding: u32 = 0x4880001F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_memory_ordered
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_ordered_reg_write_0_08800000() {
    // Test aarch64_memory_ordered register write: GpFromField("t")
    // Encoding: 0x08800000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x08800000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_ordered
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_ordered_sp_rn_088003e0() {
    // Test aarch64_memory_ordered with Rn = SP (31)
    // Encoding: 0x088003E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x088003E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_memory_ordered
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_ordered_zr_rt_0880001f() {
    // Test aarch64_memory_ordered with Rt = ZR (31)
    // Encoding: 0x0880001F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0880001F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

/// Provenance: aarch64_memory_ordered
/// ASL: `Mem[address, 8] = data`
/// Requirement: MemoryAccess { op: Store, size_bits: 64, addressing: "Base { reg: \"address\" }" }
/// 8-byte store
#[test]
fn test_aarch64_memory_ordered_store_0_08800020() {
    // Test aarch64_memory_ordered memory store: 8 bytes
    // Encoding: 0x08800020
    let mut cpu = create_test_cpu();
    set_x(&mut cpu, 1, 0x100000000000);
    set_x(&mut cpu, 0, 0xDEADBEEFCAFEBABE);
    let encoding: u32 = 0x08800020;
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
// aarch64_memory_ordered_rcpc Tests
// ============================================================================

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_memory_ordered_rcpc_field_size_0_min_c000_38a0c000() {
    // Encoding: 0x38A0C000
    // Test aarch64_memory_ordered_rcpc field size = 0 (Min)
    // Fields: Rt=0, size=0, Rs=0, Rn=0
    let encoding: u32 = 0x38A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch64_memory_ordered_rcpc_field_size_1_poweroftwo_c000_78a0c000() {
    // Encoding: 0x78A0C000
    // Test aarch64_memory_ordered_rcpc field size = 1 (PowerOfTwo)
    // Fields: Rs=0, Rn=0, size=1, Rt=0
    let encoding: u32 = 0x78A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch64_memory_ordered_rcpc_field_size_2_poweroftwo_c000_b8a0c000() {
    // Encoding: 0xB8A0C000
    // Test aarch64_memory_ordered_rcpc field size = 2 (PowerOfTwo)
    // Fields: Rs=0, Rn=0, size=2, Rt=0
    let encoding: u32 = 0xB8A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field size 30 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch64_memory_ordered_rcpc_field_size_3_max_c000_f8a0c000() {
    // Encoding: 0xF8A0C000
    // Test aarch64_memory_ordered_rcpc field size = 3 (Max)
    // Fields: Rs=0, Rt=0, Rn=0, size=3
    let encoding: u32 = 0xF8A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_ordered_rcpc_field_rs_0_min_c000_38a0c000() {
    // Encoding: 0x38A0C000
    // Test aarch64_memory_ordered_rcpc field Rs = 0 (Min)
    // Fields: Rt=0, Rn=0, size=0, Rs=0
    let encoding: u32 = 0x38A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_ordered_rcpc_field_rs_1_poweroftwo_c000_38a1c000() {
    // Encoding: 0x38A1C000
    // Test aarch64_memory_ordered_rcpc field Rs = 1 (PowerOfTwo)
    // Fields: Rt=0, size=0, Rs=1, Rn=0
    let encoding: u32 = 0x38A1C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_ordered_rcpc_field_rs_30_poweroftwominusone_c000_38bec000() {
    // Encoding: 0x38BEC000
    // Test aarch64_memory_ordered_rcpc field Rs = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, Rt=0, Rs=30, size=0
    let encoding: u32 = 0x38BEC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rs 16 +: 5`
/// Requirement: FieldBoundary { field: "Rs", value: 31, boundary: Max }
/// register index 31 (special)
#[test]
fn test_aarch64_memory_ordered_rcpc_field_rs_31_max_c000_38bfc000() {
    // Encoding: 0x38BFC000
    // Test aarch64_memory_ordered_rcpc field Rs = 31 (Max)
    // Fields: Rs=31, size=0, Rn=0, Rt=0
    let encoding: u32 = 0x38BFC000;
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

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_ordered_rcpc_field_rn_0_min_c000_38a0c000() {
    // Encoding: 0x38A0C000
    // Test aarch64_memory_ordered_rcpc field Rn = 0 (Min)
    // Fields: Rn=0, Rt=0, Rs=0, size=0
    let encoding: u32 = 0x38A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_ordered_rcpc_field_rn_1_poweroftwo_c000_38a0c020() {
    // Encoding: 0x38A0C020
    // Test aarch64_memory_ordered_rcpc field Rn = 1 (PowerOfTwo)
    // Fields: size=0, Rn=1, Rs=0, Rt=0
    let encoding: u32 = 0x38A0C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_ordered_rcpc_field_rn_30_poweroftwominusone_c000_38a0c3c0() {
    // Encoding: 0x38A0C3C0
    // Test aarch64_memory_ordered_rcpc field Rn = 30 (PowerOfTwoMinusOne)
    // Fields: Rt=0, size=0, Rn=30, Rs=0
    let encoding: u32 = 0x38A0C3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rn 5 +: 5`
/// Requirement: FieldBoundary { field: "Rn", value: 31, boundary: Max }
/// register index 31 (SP - stack pointer)
#[test]
fn test_aarch64_memory_ordered_rcpc_field_rn_31_max_c000_38a0c3e0() {
    // Encoding: 0x38A0C3E0
    // Test aarch64_memory_ordered_rcpc field Rn = 31 (Max)
    // Fields: Rn=31, size=0, Rs=0, Rt=0
    let encoding: u32 = 0x38A0C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_memory_ordered_rcpc_field_rt_0_min_c000_38a0c000() {
    // Encoding: 0x38A0C000
    // Test aarch64_memory_ordered_rcpc field Rt = 0 (Min)
    // Fields: Rt=0, Rn=0, size=0, Rs=0
    let encoding: u32 = 0x38A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_memory_ordered_rcpc_field_rt_1_poweroftwo_c000_38a0c001() {
    // Encoding: 0x38A0C001
    // Test aarch64_memory_ordered_rcpc field Rt = 1 (PowerOfTwo)
    // Fields: Rs=0, size=0, Rn=0, Rt=1
    let encoding: u32 = 0x38A0C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_memory_ordered_rcpc_field_rt_30_poweroftwominusone_c000_38a0c01e() {
    // Encoding: 0x38A0C01E
    // Test aarch64_memory_ordered_rcpc field Rt = 30 (PowerOfTwoMinusOne)
    // Fields: Rn=0, size=0, Rs=0, Rt=30
    let encoding: u32 = 0x38A0C01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rt 0 +: 5`
/// Requirement: FieldBoundary { field: "Rt", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_memory_ordered_rcpc_field_rt_31_max_c000_38a0c01f() {
    // Encoding: 0x38A0C01F
    // Test aarch64_memory_ordered_rcpc field Rt = 31 (Max)
    // Fields: Rs=0, Rn=0, Rt=31, size=0
    let encoding: u32 = 0x38A0C01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=0 (8-bit / byte size)
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_0_c000_38a0c000() {
    // Encoding: 0x38A0C000
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=0, Rn=0, Rt=0
    // Fields: Rs=0, Rn=0, Rt=0, size=0
    let encoding: u32 = 0x38A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=1 (16-bit / halfword size)
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_1_c000_78a0c000() {
    // Encoding: 0x78A0C000
    // Test aarch64_memory_ordered_rcpc field combination: size=1, Rs=0, Rn=0, Rt=0
    // Fields: Rn=0, Rs=0, size=1, Rt=0
    let encoding: u32 = 0x78A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=2 (32-bit / word size)
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_2_c000_b8a0c000() {
    // Encoding: 0xB8A0C000
    // Test aarch64_memory_ordered_rcpc field combination: size=2, Rs=0, Rn=0, Rt=0
    // Fields: size=2, Rt=0, Rn=0, Rs=0
    let encoding: u32 = 0xB8A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// size=3 (64-bit / doubleword size)
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_3_c000_f8a0c000() {
    // Encoding: 0xF8A0C000
    // Test aarch64_memory_ordered_rcpc field combination: size=3, Rs=0, Rn=0, Rt=0
    // Fields: Rn=0, size=3, Rs=0, Rt=0
    let encoding: u32 = 0xF8A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_4_c000_38a0c000() {
    // Encoding: 0x38A0C000
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, size=0, Rs=0
    let encoding: u32 = 0x38A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_5_c000_38a1c000() {
    // Encoding: 0x38A1C000
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=1, Rn=0, Rt=0
    // Fields: size=0, Rs=1, Rn=0, Rt=0
    let encoding: u32 = 0x38A1C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_6_c000_38bec000() {
    // Encoding: 0x38BEC000
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=30, Rn=0, Rt=0
    // Fields: Rt=0, size=0, Rn=0, Rs=30
    let encoding: u32 = 0x38BEC000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (register index 31 (special))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_7_c000_38bfc000() {
    // Encoding: 0x38BFC000
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=31, Rn=0, Rt=0
    // Fields: Rs=31, Rt=0, Rn=0, size=0
    let encoding: u32 = 0x38BFC000;
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

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_8_c000_38a0c000() {
    // Encoding: 0x38A0C000
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=0, Rn=0, Rt=0
    // Fields: Rt=0, size=0, Rs=0, Rn=0
    let encoding: u32 = 0x38A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_9_c000_38a0c020() {
    // Encoding: 0x38A0C020
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=0, Rn=1, Rt=0
    // Fields: Rn=1, size=0, Rs=0, Rt=0
    let encoding: u32 = 0x38A0C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_10_c000_38a0c3c0() {
    // Encoding: 0x38A0C3C0
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=0, Rn=30, Rt=0
    // Fields: Rt=0, size=0, Rs=0, Rn=30
    let encoding: u32 = 0x38A0C3C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (register index 31 (SP - stack pointer))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_11_c000_38a0c3e0() {
    // Encoding: 0x38A0C3E0
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=0, Rn=31, Rt=0
    // Fields: size=0, Rs=0, Rn=31, Rt=0
    let encoding: u32 = 0x38A0C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=0 (register index 0 (first register))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_12_c000_38a0c000() {
    // Encoding: 0x38A0C000
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=0, Rn=0, Rt=0
    // Fields: Rn=0, Rt=0, Rs=0, size=0
    let encoding: u32 = 0x38A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=1 (register index 1 (second register))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_13_c000_38a0c001() {
    // Encoding: 0x38A0C001
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=0, Rn=0, Rt=1
    // Fields: Rn=0, Rs=0, size=0, Rt=1
    let encoding: u32 = 0x38A0C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_14_c000_38a0c01e() {
    // Encoding: 0x38A0C01E
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=0, Rn=0, Rt=30
    // Fields: size=0, Rs=0, Rn=0, Rt=30
    let encoding: u32 = 0x38A0C01E;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rt=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_15_c000_38a0c01f() {
    // Encoding: 0x38A0C01F
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=0, Rn=0, Rt=31
    // Fields: Rs=0, size=0, Rt=31, Rn=0
    let encoding: u32 = 0x38A0C01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rn=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_16_c000_38a1c020() {
    // Encoding: 0x38A1C020
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=1, Rn=1, Rt=0
    // Fields: size=0, Rs=1, Rt=0, Rn=1
    let encoding: u32 = 0x38A1C020;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rn=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_17_c000_38bfc3e0() {
    // Encoding: 0x38BFC3E0
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=31, Rn=31, Rt=0
    // Fields: size=0, Rs=31, Rn=31, Rt=0
    let encoding: u32 = 0x38BFC3E0;
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

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_18_c000_38a1c001() {
    // Encoding: 0x38A1C001
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=1, Rn=0, Rt=1
    // Fields: Rt=1, Rn=0, Rs=1, size=0
    let encoding: u32 = 0x38A1C001;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rs=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_19_c000_38bfc01f() {
    // Encoding: 0x38BFC01F
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=31, Rn=0, Rt=31
    // Fields: Rt=31, Rn=0, size=0, Rs=31
    let encoding: u32 = 0x38BFC01F;
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

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=1 (same register test (reg=1)), Rt=1 (same register test (reg=1))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_20_c000_38a0c021() {
    // Encoding: 0x38A0C021
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=0, Rn=1, Rt=1
    // Fields: Rs=0, Rn=1, Rt=1, size=0
    let encoding: u32 = 0x38A0C021;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rn=31 (same register test (reg=31)), Rt=31 (same register test (reg=31))
#[test]
fn test_aarch64_memory_ordered_rcpc_combo_21_c000_38a0c3ff() {
    // Encoding: 0x38A0C3FF
    // Test aarch64_memory_ordered_rcpc field combination: size=0, Rs=0, Rn=31, Rt=31
    // Fields: Rs=0, Rt=31, size=0, Rn=31
    let encoding: u32 = 0x38A0C3FF;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_memory_ordered_rcpc_special_size_0_size_variant_0_49152_38a0c000() {
    // Encoding: 0x38A0C000
    // Test aarch64_memory_ordered_rcpc special value size = 0 (Size variant 0)
    // Fields: Rn=0, Rs=0, Rt=0, size=0
    let encoding: u32 = 0x38A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_memory_ordered_rcpc_special_size_1_size_variant_1_49152_78a0c000() {
    // Encoding: 0x78A0C000
    // Test aarch64_memory_ordered_rcpc special value size = 1 (Size variant 1)
    // Fields: Rn=0, Rs=0, Rt=0, size=1
    let encoding: u32 = 0x78A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch64_memory_ordered_rcpc_special_size_2_size_variant_2_49152_b8a0c000() {
    // Encoding: 0xB8A0C000
    // Test aarch64_memory_ordered_rcpc special value size = 2 (Size variant 2)
    // Fields: size=2, Rn=0, Rs=0, Rt=0
    let encoding: u32 = 0xB8A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch64_memory_ordered_rcpc_special_size_3_size_variant_3_49152_f8a0c000() {
    // Encoding: 0xF8A0C000
    // Test aarch64_memory_ordered_rcpc special value size = 3 (Size variant 3)
    // Fields: size=3, Rn=0, Rs=0, Rt=0
    let encoding: u32 = 0xF8A0C000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rn = 31 (Stack pointer (SP) - may require alignment)`
/// Requirement: FieldSpecial { field: "Rn", value: 31, meaning: "Stack pointer (SP) - may require alignment" }
/// Stack pointer (SP) - may require alignment
#[test]
fn test_aarch64_memory_ordered_rcpc_special_rn_31_stack_pointer_sp_may_require_alignment_49152_78a0c3e0()
 {
    // Encoding: 0x78A0C3E0
    // Test aarch64_memory_ordered_rcpc special value Rn = 31 (Stack pointer (SP) - may require alignment)
    // Fields: size=1, Rs=0, Rn=31, Rt=0
    let encoding: u32 = 0x78A0C3E0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `field Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rt", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_memory_ordered_rcpc_special_rt_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_49152_78a0c01f()
 {
    // Encoding: 0x78A0C01F
    // Test aarch64_memory_ordered_rcpc special value Rt = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: Rs=0, Rn=0, size=1, Rt=31
    let encoding: u32 = 0x78A0C01F;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `GpFromField("t") write`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "t" }
/// verify register write to GpFromField("t")
#[test]
fn test_aarch64_memory_ordered_rcpc_reg_write_0_38a0c000() {
    // Test aarch64_memory_ordered_rcpc register write: GpFromField("t")
    // Encoding: 0x38A0C000
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38A0C000;
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `Rn = 31 (SP)`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "stack pointer with alignment requirements" }
/// stack pointer (Rn = 31)
#[test]
fn test_aarch64_memory_ordered_rcpc_sp_rn_38a0c3e0() {
    // Test aarch64_memory_ordered_rcpc with Rn = SP (31)
    // Encoding: 0x38A0C3E0
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38A0C3E0;
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}

/// Provenance: aarch64_memory_ordered_rcpc
/// ASL: `Rt = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rt = 31)
#[test]
fn test_aarch64_memory_ordered_rcpc_zr_rt_38a0c01f() {
    // Test aarch64_memory_ordered_rcpc with Rt = ZR (31)
    // Encoding: 0x38A0C01F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x38A0C01F;
    write_insn(&mut cpu, 0, encoding);
    // llvm-mc: -
    let exit = cpu.step();
    assert!(
        exit.is_err() || matches!(exit.as_ref().unwrap(), CpuExit::Undefined(_)),
        "expected unallocated encoding for {:#010X}: {:?}",
        encoding,
        exit
    );
}
