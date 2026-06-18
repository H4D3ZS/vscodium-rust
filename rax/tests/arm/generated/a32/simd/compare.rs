//! A32 simd compare tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_VCMP_A Tests
// ============================================================================

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_0_min_8c0_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Vm=0, size=0, M=0, D=0, Vd=0, E=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_1_poweroftwo_8c0_1eb40840() {
    // Encoding: 0x1EB40840
    // Test aarch32_VCMP_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, D=0, size=0, E=0, cond=1, M=0, Vm=0
    let encoding: u32 = 0x1EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_2_poweroftwo_8c0_2eb40840() {
    // Encoding: 0x2EB40840
    // Test aarch32_VCMP_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, size=0, cond=2, Vd=0, E=0, Vm=0, D=0
    let encoding: u32 = 0x2EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_3_poweroftwo_8c0_3eb40840() {
    // Encoding: 0x3EB40840
    // Test aarch32_VCMP_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, size=0, M=0, cond=3, Vm=0, E=0, D=0
    let encoding: u32 = 0x3EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_4_poweroftwo_8c0_4eb40840() {
    // Encoding: 0x4EB40840
    // Test aarch32_VCMP_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, cond=4, Vd=0, M=0, E=0, Vm=0, size=0
    let encoding: u32 = 0x4EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_5_poweroftwo_8c0_5eb40840() {
    // Encoding: 0x5EB40840
    // Test aarch32_VCMP_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: E=0, cond=5, M=0, Vm=0, Vd=0, D=0, size=0
    let encoding: u32 = 0x5EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_6_poweroftwo_8c0_6eb40840() {
    // Encoding: 0x6EB40840
    // Test aarch32_VCMP_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=0, cond=6, D=0, Vd=0, size=0, E=0, M=0
    let encoding: u32 = 0x6EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_7_poweroftwo_8c0_7eb40840() {
    // Encoding: 0x7EB40840
    // Test aarch32_VCMP_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, E=0, Vm=0, D=0, cond=7, size=0, M=0
    let encoding: u32 = 0x7EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_8_poweroftwo_8c0_8eb40840() {
    // Encoding: 0x8EB40840
    // Test aarch32_VCMP_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: E=0, D=0, Vm=0, Vd=0, cond=8, size=0, M=0
    let encoding: u32 = 0x8EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_9_poweroftwo_8c0_9eb40840() {
    // Encoding: 0x9EB40840
    // Test aarch32_VCMP_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, cond=9, M=0, size=0, Vd=0, Vm=0, E=0
    let encoding: u32 = 0x9EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_10_poweroftwo_8c0_aeb40840() {
    // Encoding: 0xAEB40840
    // Test aarch32_VCMP_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, size=0, Vd=0, E=0, M=0, Vm=0, cond=10
    let encoding: u32 = 0xAEB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_11_poweroftwo_8c0_beb40840() {
    // Encoding: 0xBEB40840
    // Test aarch32_VCMP_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, Vd=0, size=0, Vm=0, E=0, M=0, D=0
    let encoding: u32 = 0xBEB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_12_poweroftwo_8c0_ceb40840() {
    // Encoding: 0xCEB40840
    // Test aarch32_VCMP_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=0, E=0, size=0, M=0, cond=12, Vd=0, D=0
    let encoding: u32 = 0xCEB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_13_poweroftwo_8c0_deb40840() {
    // Encoding: 0xDEB40840
    // Test aarch32_VCMP_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, M=0, Vm=0, size=0, Vd=0, E=0, cond=13
    let encoding: u32 = 0xDEB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_14_poweroftwo_8c0_eeb40840() {
    // Encoding: 0xEEB40840
    // Test aarch32_VCMP_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=0, E=0, size=0, M=0, D=0, Vd=0, cond=14
    let encoding: u32 = 0xEEB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_vcmp_a1_a_field_cond_15_max_8c0_feb40840() {
    // Encoding: 0xFEB40840
    // Test aarch32_VCMP_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Vd=0, size=0, M=0, Vm=0, cond=15, E=0, D=0
    let encoding: u32 = 0xFEB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcmp_a1_a_field_d_0_min_8c0_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vd=0, size=0, M=0, Vm=0, E=0, cond=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcmp_a1_a_field_d_1_max_8c0_0ef40840() {
    // Encoding: 0x0EF40840
    // Test aarch32_VCMP_A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: Vm=0, cond=0, M=0, E=0, size=0, Vd=0, D=1
    let encoding: u32 = 0x0EF40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcmp_a1_a_field_vd_0_min_8c0_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, cond=0, size=0, Vd=0, D=0, E=0, M=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcmp_a1_a_field_vd_1_poweroftwo_8c0_0eb41840() {
    // Encoding: 0x0EB41840
    // Test aarch32_VCMP_A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=0, D=0, Vd=1, cond=0, size=0, E=0, M=0
    let encoding: u32 = 0x0EB41840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcmp_a1_a_field_size_0_min_8c0_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A field size = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vd=0, E=0, Vm=0, M=0, size=0, cond=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcmp_a1_a_field_size_1_poweroftwo_8c0_0eb40940() {
    // Encoding: 0x0EB40940
    // Test aarch32_VCMP_A1_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, D=0, E=0, M=0, size=1, cond=0, Vm=0
    let encoding: u32 = 0x0EB40940;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcmp_a1_a_field_size_2_poweroftwo_8c0_0eb40a40() {
    // Encoding: 0x0EB40A40
    // Test aarch32_VCMP_A1_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, D=0, E=0, M=0, Vm=0, size=2, Vd=0
    let encoding: u32 = 0x0EB40A40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcmp_a1_a_field_size_3_max_8c0_0eb40b40() {
    // Encoding: 0x0EB40B40
    // Test aarch32_VCMP_A1_A field size = 3 (Max)
    // ISET: A32
    // Fields: Vm=0, Vd=0, E=0, M=0, size=3, cond=0, D=0
    let encoding: u32 = 0x0EB40B40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field E 7 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcmp_a1_a_field_e_0_min_8c0_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A field E = 0 (Min)
    // ISET: A32
    // Fields: M=0, Vm=0, E=0, cond=0, D=0, Vd=0, size=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field E 7 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcmp_a1_a_field_e_1_max_8c0_0eb408c0() {
    // Encoding: 0x0EB408C0
    // Test aarch32_VCMP_A1_A field E = 1 (Max)
    // ISET: A32
    // Fields: Vd=0, E=1, size=0, M=0, D=0, cond=0, Vm=0
    let encoding: u32 = 0x0EB408C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcmp_a1_a_field_m_0_min_8c0_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: cond=0, size=0, Vm=0, D=0, Vd=0, E=0, M=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcmp_a1_a_field_m_1_max_8c0_0eb40860() {
    // Encoding: 0x0EB40860
    // Test aarch32_VCMP_A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: cond=0, D=0, Vd=0, size=0, E=0, M=1, Vm=0
    let encoding: u32 = 0x0EB40860;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcmp_a1_a_field_vm_0_min_8c0_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, cond=0, E=0, Vd=0, M=0, D=0, size=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcmp_a1_a_field_vm_1_poweroftwo_8c0_0eb40841() {
    // Encoding: 0x0EB40841
    // Test aarch32_VCMP_A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, E=0, M=0, cond=0, Vm=1, size=0, D=0
    let encoding: u32 = 0x0EB40841;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_vcmp_a1_a_combo_0_8c0_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A field combination: cond=0, D=0, Vd=0, size=0, E=0, M=0, Vm=0
    // ISET: A32
    // Fields: Vm=0, cond=0, D=0, size=0, Vd=0, E=0, M=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_0_condition_eq_2240_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Vd=0, D=0, size=0, cond=0, E=0, M=0, Vm=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_1_condition_ne_2240_1eb40840() {
    // Encoding: 0x1EB40840
    // Test aarch32_VCMP_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, Vm=0, M=0, D=0, Vd=0, E=0, size=0
    let encoding: u32 = 0x1EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_2_condition_cs_hs_2240_2eb40840() {
    // Encoding: 0x2EB40840
    // Test aarch32_VCMP_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: Vd=0, size=0, D=0, M=0, E=0, Vm=0, cond=2
    let encoding: u32 = 0x2EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_3_condition_cc_lo_2240_3eb40840() {
    // Encoding: 0x3EB40840
    // Test aarch32_VCMP_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: Vm=0, cond=3, Vd=0, size=0, D=0, E=0, M=0
    let encoding: u32 = 0x3EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_4_condition_mi_2240_4eb40840() {
    // Encoding: 0x4EB40840
    // Test aarch32_VCMP_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Vd=0, D=0, M=0, cond=4, size=0, Vm=0, E=0
    let encoding: u32 = 0x4EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_5_condition_pl_2240_5eb40840() {
    // Encoding: 0x5EB40840
    // Test aarch32_VCMP_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: M=0, Vm=0, size=0, D=0, E=0, Vd=0, cond=5
    let encoding: u32 = 0x5EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_6_condition_vs_2240_6eb40840() {
    // Encoding: 0x6EB40840
    // Test aarch32_VCMP_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, E=0, Vd=0, size=0, M=0, Vm=0, D=0
    let encoding: u32 = 0x6EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_7_condition_vc_2240_7eb40840() {
    // Encoding: 0x7EB40840
    // Test aarch32_VCMP_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: M=0, Vm=0, cond=7, Vd=0, D=0, E=0, size=0
    let encoding: u32 = 0x7EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_8_condition_hi_2240_8eb40840() {
    // Encoding: 0x8EB40840
    // Test aarch32_VCMP_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: E=0, M=0, Vm=0, size=0, D=0, Vd=0, cond=8
    let encoding: u32 = 0x8EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_9_condition_ls_2240_9eb40840() {
    // Encoding: 0x9EB40840
    // Test aarch32_VCMP_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Vm=0, M=0, Vd=0, D=0, cond=9, size=0, E=0
    let encoding: u32 = 0x9EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_10_condition_ge_2240_aeb40840() {
    // Encoding: 0xAEB40840
    // Test aarch32_VCMP_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: cond=10, D=0, size=0, M=0, E=0, Vm=0, Vd=0
    let encoding: u32 = 0xAEB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_11_condition_lt_2240_beb40840() {
    // Encoding: 0xBEB40840
    // Test aarch32_VCMP_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: E=0, Vd=0, M=0, D=0, cond=11, size=0, Vm=0
    let encoding: u32 = 0xBEB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_12_condition_gt_2240_ceb40840() {
    // Encoding: 0xCEB40840
    // Test aarch32_VCMP_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: size=0, Vm=0, Vd=0, D=0, cond=12, M=0, E=0
    let encoding: u32 = 0xCEB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_13_condition_le_2240_deb40840() {
    // Encoding: 0xDEB40840
    // Test aarch32_VCMP_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: size=0, Vm=0, cond=13, D=0, Vd=0, E=0, M=0
    let encoding: u32 = 0xDEB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_14_condition_al_2240_eeb40840() {
    // Encoding: 0xEEB40840
    // Test aarch32_VCMP_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: M=0, Vd=0, cond=14, size=0, E=0, Vm=0, D=0
    let encoding: u32 = 0xEEB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_vcmp_a1_a_special_cond_15_condition_nv_2240_feb40840() {
    // Encoding: 0xFEB40840
    // Test aarch32_VCMP_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: D=0, cond=15, size=0, M=0, E=0, Vd=0, Vm=0
    let encoding: u32 = 0xFEB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcmp_a1_a_special_size_0_size_variant_0_2240_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: D=0, M=0, E=0, size=0, Vd=0, cond=0, Vm=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcmp_a1_a_special_size_1_size_variant_1_2240_0eb40940() {
    // Encoding: 0x0EB40940
    // Test aarch32_VCMP_A1_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: E=0, Vm=0, M=0, Vd=0, cond=0, size=1, D=0
    let encoding: u32 = 0x0EB40940;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcmp_a1_a_special_size_2_size_variant_2_2240_0eb40a40() {
    // Encoding: 0x0EB40A40
    // Test aarch32_VCMP_A1_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: cond=0, E=0, D=0, M=0, Vm=0, Vd=0, size=2
    let encoding: u32 = 0x0EB40A40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcmp_a1_a_special_size_3_size_variant_3_2240_0eb40b40() {
    // Encoding: 0x0EB40B40
    // Test aarch32_VCMP_A1_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: cond=0, size=3, E=0, D=0, Vm=0, M=0, Vd=0
    let encoding: u32 = 0x0EB40B40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcmp_a1_a_invalid_0_8c0_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: A32
    // Fields: Vd=0, cond=0, size=0, E=0, D=0, M=0, Vm=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcmp_a1_a_invalid_1_8c0_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: cond=0, Vm=0, Vd=0, M=0, D=0, size=0, E=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "cond" }) } }, rhs: LitBits([true, true, true, false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"cond\" }) } }, rhs: LitBits([true, true, true, false]) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcmp_a1_a_invalid_2_8c0_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A invalid encoding: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "cond" }) } }, rhs: LitBits([true, true, true, false]) }
    // ISET: A32
    // Fields: Vm=0, D=0, cond=0, Vd=0, size=0, E=0, M=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VCMP_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcmp_a1_a_invalid_3_8c0_0eb40840() {
    // Encoding: 0x0EB40840
    // Test aarch32_VCMP_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: Vm=0, Vd=0, size=0, D=0, E=0, M=0, cond=0
    let encoding: u32 = 0x0EB40840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_0_min_8c0_0eb50840() {
    // Encoding: 0x0EB50840
    // Test aarch32_VCMP_A2_A field cond = 0 (Min)
    // ISET: A32
    // Fields: D=0, E=0, Vd=0, size=0, cond=0
    let encoding: u32 = 0x0EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_1_poweroftwo_8c0_1eb50840() {
    // Encoding: 0x1EB50840
    // Test aarch32_VCMP_A2_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vd=0, E=0, cond=1, size=0
    let encoding: u32 = 0x1EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_2_poweroftwo_8c0_2eb50840() {
    // Encoding: 0x2EB50840
    // Test aarch32_VCMP_A2_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, Vd=0, cond=2, D=0, E=0
    let encoding: u32 = 0x2EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_3_poweroftwo_8c0_3eb50840() {
    // Encoding: 0x3EB50840
    // Test aarch32_VCMP_A2_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, size=0, D=0, E=0, cond=3
    let encoding: u32 = 0x3EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_4_poweroftwo_8c0_4eb50840() {
    // Encoding: 0x4EB50840
    // Test aarch32_VCMP_A2_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, Vd=0, D=0, E=0, cond=4
    let encoding: u32 = 0x4EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_5_poweroftwo_8c0_5eb50840() {
    // Encoding: 0x5EB50840
    // Test aarch32_VCMP_A2_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: E=0, D=0, cond=5, Vd=0, size=0
    let encoding: u32 = 0x5EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_6_poweroftwo_8c0_6eb50840() {
    // Encoding: 0x6EB50840
    // Test aarch32_VCMP_A2_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: E=0, D=0, size=0, Vd=0, cond=6
    let encoding: u32 = 0x6EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_7_poweroftwo_8c0_7eb50840() {
    // Encoding: 0x7EB50840
    // Test aarch32_VCMP_A2_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vd=0, cond=7, size=0, E=0
    let encoding: u32 = 0x7EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_8_poweroftwo_8c0_8eb50840() {
    // Encoding: 0x8EB50840
    // Test aarch32_VCMP_A2_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, D=0, cond=8, E=0, Vd=0
    let encoding: u32 = 0x8EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_9_poweroftwo_8c0_9eb50840() {
    // Encoding: 0x9EB50840
    // Test aarch32_VCMP_A2_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, cond=9, D=0, E=0, Vd=0
    let encoding: u32 = 0x9EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_10_poweroftwo_8c0_aeb50840() {
    // Encoding: 0xAEB50840
    // Test aarch32_VCMP_A2_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, Vd=0, size=0, E=0, D=0
    let encoding: u32 = 0xAEB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_11_poweroftwo_8c0_beb50840() {
    // Encoding: 0xBEB50840
    // Test aarch32_VCMP_A2_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vd=0, size=0, cond=11, E=0
    let encoding: u32 = 0xBEB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_12_poweroftwo_8c0_ceb50840() {
    // Encoding: 0xCEB50840
    // Test aarch32_VCMP_A2_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, cond=12, D=0, Vd=0, E=0
    let encoding: u32 = 0xCEB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_13_poweroftwo_8c0_deb50840() {
    // Encoding: 0xDEB50840
    // Test aarch32_VCMP_A2_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, D=0, size=0, cond=13, E=0
    let encoding: u32 = 0xDEB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_14_poweroftwo_8c0_eeb50840() {
    // Encoding: 0xEEB50840
    // Test aarch32_VCMP_A2_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, E=0, Vd=0, cond=14, size=0
    let encoding: u32 = 0xEEB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_vcmp_a2_a_field_cond_15_max_8c0_feb50840() {
    // Encoding: 0xFEB50840
    // Test aarch32_VCMP_A2_A field cond = 15 (Max)
    // ISET: A32
    // Fields: cond=15, size=0, D=0, E=0, Vd=0
    let encoding: u32 = 0xFEB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcmp_a2_a_field_d_0_min_8c0_0eb50840() {
    // Encoding: 0x0EB50840
    // Test aarch32_VCMP_A2_A field D = 0 (Min)
    // ISET: A32
    // Fields: cond=0, D=0, Vd=0, size=0, E=0
    let encoding: u32 = 0x0EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcmp_a2_a_field_d_1_max_8c0_0ef50840() {
    // Encoding: 0x0EF50840
    // Test aarch32_VCMP_A2_A field D = 1 (Max)
    // ISET: A32
    // Fields: cond=0, size=0, D=1, E=0, Vd=0
    let encoding: u32 = 0x0EF50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcmp_a2_a_field_vd_0_min_8c0_0eb50840() {
    // Encoding: 0x0EB50840
    // Test aarch32_VCMP_A2_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: E=0, cond=0, D=0, size=0, Vd=0
    let encoding: u32 = 0x0EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcmp_a2_a_field_vd_1_poweroftwo_8c0_0eb51840() {
    // Encoding: 0x0EB51840
    // Test aarch32_VCMP_A2_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, E=0, D=0, size=0, Vd=1
    let encoding: u32 = 0x0EB51840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcmp_a2_a_field_size_0_min_8c0_0eb50840() {
    // Encoding: 0x0EB50840
    // Test aarch32_VCMP_A2_A field size = 0 (Min)
    // ISET: A32
    // Fields: cond=0, size=0, E=0, Vd=0, D=0
    let encoding: u32 = 0x0EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcmp_a2_a_field_size_1_poweroftwo_8c0_0eb50940() {
    // Encoding: 0x0EB50940
    // Test aarch32_VCMP_A2_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vd=0, size=1, cond=0, E=0
    let encoding: u32 = 0x0EB50940;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcmp_a2_a_field_size_2_poweroftwo_8c0_0eb50a40() {
    // Encoding: 0x0EB50A40
    // Test aarch32_VCMP_A2_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, cond=0, size=2, E=0, Vd=0
    let encoding: u32 = 0x0EB50A40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcmp_a2_a_field_size_3_max_8c0_0eb50b40() {
    // Encoding: 0x0EB50B40
    // Test aarch32_VCMP_A2_A field size = 3 (Max)
    // ISET: A32
    // Fields: D=0, Vd=0, cond=0, size=3, E=0
    let encoding: u32 = 0x0EB50B40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field E 7 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcmp_a2_a_field_e_0_min_8c0_0eb50840() {
    // Encoding: 0x0EB50840
    // Test aarch32_VCMP_A2_A field E = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, D=0, size=0, cond=0, E=0
    let encoding: u32 = 0x0EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field E 7 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcmp_a2_a_field_e_1_max_8c0_0eb508c0() {
    // Encoding: 0x0EB508C0
    // Test aarch32_VCMP_A2_A field E = 1 (Max)
    // ISET: A32
    // Fields: Vd=0, D=0, cond=0, size=0, E=1
    let encoding: u32 = 0x0EB508C0;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_vcmp_a2_a_combo_0_8c0_0eb50840() {
    // Encoding: 0x0EB50840
    // Test aarch32_VCMP_A2_A field combination: cond=0, D=0, Vd=0, size=0, E=0
    // ISET: A32
    // Fields: size=0, D=0, E=0, cond=0, Vd=0
    let encoding: u32 = 0x0EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_0_condition_eq_2240_0eb50840() {
    // Encoding: 0x0EB50840
    // Test aarch32_VCMP_A2_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: E=0, cond=0, D=0, Vd=0, size=0
    let encoding: u32 = 0x0EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_1_condition_ne_2240_1eb50840() {
    // Encoding: 0x1EB50840
    // Test aarch32_VCMP_A2_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: D=0, Vd=0, E=0, cond=1, size=0
    let encoding: u32 = 0x1EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_2_condition_cs_hs_2240_2eb50840() {
    // Encoding: 0x2EB50840
    // Test aarch32_VCMP_A2_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: D=0, Vd=0, cond=2, size=0, E=0
    let encoding: u32 = 0x2EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_3_condition_cc_lo_2240_3eb50840() {
    // Encoding: 0x3EB50840
    // Test aarch32_VCMP_A2_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, size=0, D=0, Vd=0, E=0
    let encoding: u32 = 0x3EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_4_condition_mi_2240_4eb50840() {
    // Encoding: 0x4EB50840
    // Test aarch32_VCMP_A2_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, D=0, Vd=0, E=0, size=0
    let encoding: u32 = 0x4EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_5_condition_pl_2240_5eb50840() {
    // Encoding: 0x5EB50840
    // Test aarch32_VCMP_A2_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: E=0, cond=5, D=0, size=0, Vd=0
    let encoding: u32 = 0x5EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_6_condition_vs_2240_6eb50840() {
    // Encoding: 0x6EB50840
    // Test aarch32_VCMP_A2_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Vd=0, E=0, D=0, cond=6, size=0
    let encoding: u32 = 0x6EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_7_condition_vc_2240_7eb50840() {
    // Encoding: 0x7EB50840
    // Test aarch32_VCMP_A2_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, size=0, D=0, Vd=0, E=0
    let encoding: u32 = 0x7EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_8_condition_hi_2240_8eb50840() {
    // Encoding: 0x8EB50840
    // Test aarch32_VCMP_A2_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: size=0, E=0, D=0, cond=8, Vd=0
    let encoding: u32 = 0x8EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_9_condition_ls_2240_9eb50840() {
    // Encoding: 0x9EB50840
    // Test aarch32_VCMP_A2_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, size=0, D=0, E=0, Vd=0
    let encoding: u32 = 0x9EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_10_condition_ge_2240_aeb50840() {
    // Encoding: 0xAEB50840
    // Test aarch32_VCMP_A2_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: E=0, D=0, cond=10, Vd=0, size=0
    let encoding: u32 = 0xAEB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_11_condition_lt_2240_beb50840() {
    // Encoding: 0xBEB50840
    // Test aarch32_VCMP_A2_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, size=0, E=0, Vd=0, D=0
    let encoding: u32 = 0xBEB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_12_condition_gt_2240_ceb50840() {
    // Encoding: 0xCEB50840
    // Test aarch32_VCMP_A2_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: D=0, Vd=0, E=0, cond=12, size=0
    let encoding: u32 = 0xCEB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_13_condition_le_2240_deb50840() {
    // Encoding: 0xDEB50840
    // Test aarch32_VCMP_A2_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: D=0, cond=13, size=0, E=0, Vd=0
    let encoding: u32 = 0xDEB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_14_condition_al_2240_eeb50840() {
    // Encoding: 0xEEB50840
    // Test aarch32_VCMP_A2_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: E=0, D=0, cond=14, Vd=0, size=0
    let encoding: u32 = 0xEEB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_vcmp_a2_a_special_cond_15_condition_nv_2240_feb50840() {
    // Encoding: 0xFEB50840
    // Test aarch32_VCMP_A2_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Vd=0, cond=15, D=0, size=0, E=0
    let encoding: u32 = 0xFEB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcmp_a2_a_special_size_0_size_variant_0_2240_0eb50840() {
    // Encoding: 0x0EB50840
    // Test aarch32_VCMP_A2_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: Vd=0, D=0, E=0, size=0, cond=0
    let encoding: u32 = 0x0EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcmp_a2_a_special_size_1_size_variant_1_2240_0eb50940() {
    // Encoding: 0x0EB50940
    // Test aarch32_VCMP_A2_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: size=1, cond=0, E=0, Vd=0, D=0
    let encoding: u32 = 0x0EB50940;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcmp_a2_a_special_size_2_size_variant_2_2240_0eb50a40() {
    // Encoding: 0x0EB50A40
    // Test aarch32_VCMP_A2_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: D=0, Vd=0, E=0, size=2, cond=0
    let encoding: u32 = 0x0EB50A40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcmp_a2_a_special_size_3_size_variant_3_2240_0eb50b40() {
    // Encoding: 0x0EB50B40
    // Test aarch32_VCMP_A2_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: cond=0, E=0, size=3, D=0, Vd=0
    let encoding: u32 = 0x0EB50B40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "A32 instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcmp_a2_a_invalid_0_8c0_0eb50840() {
    // Encoding: 0x0EB50840
    // Test aarch32_VCMP_A2_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: A32
    // Fields: Vd=0, D=0, cond=0, size=0, E=0
    let encoding: u32 = 0x0EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcmp_a2_a_invalid_1_8c0_0eb50840() {
    // Encoding: 0x0EB50840
    // Test aarch32_VCMP_A2_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: cond=0, Vd=0, D=0, E=0, size=0
    let encoding: u32 = 0x0EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "cond" }) } }, rhs: LitBits([true, true, true, false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"cond\" }) } }, rhs: LitBits([true, true, true, false]) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcmp_a2_a_invalid_2_8c0_0eb50840() {
    // Encoding: 0x0EB50840
    // Test aarch32_VCMP_A2_A invalid encoding: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "cond" }) } }, rhs: LitBits([true, true, true, false]) }
    // ISET: A32
    // Fields: D=0, size=0, cond=0, E=0, Vd=0
    let encoding: u32 = 0x0EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VCMP_A2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcmp_a2_a_invalid_3_8c0_0eb50840() {
    // Encoding: 0x0EB50840
    // Test aarch32_VCMP_A2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: D=0, size=0, E=0, cond=0, Vd=0
    let encoding: u32 = 0x0EB50840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcmp_t1_a_field_d_0_min_8c0_eeb40840() {
    // Thumb encoding (32): 0xEEB40840
    // Test aarch32_VCMP_T1_A field D = 0 (Min)
    // ISET: T32
    // Fields: Vm=0, Vd=0, size=0, D=0, E=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcmp_t1_a_field_d_1_max_8c0_eef40840() {
    // Thumb encoding (32): 0xEEF40840
    // Test aarch32_VCMP_T1_A field D = 1 (Max)
    // ISET: T32
    // Fields: M=0, Vd=0, E=0, size=0, Vm=0, D=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEF40840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcmp_t1_a_field_vd_0_min_8c0_eeb40840() {
    // Thumb encoding (32): 0xEEB40840
    // Test aarch32_VCMP_T1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: M=0, E=0, Vd=0, D=0, size=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcmp_t1_a_field_vd_1_poweroftwo_8c0_eeb41840() {
    // Thumb encoding (32): 0xEEB41840
    // Test aarch32_VCMP_T1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: size=0, M=0, D=0, E=0, Vm=0, Vd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB41840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcmp_t1_a_field_size_0_min_8c0_eeb40840() {
    // Thumb encoding (32): 0xEEB40840
    // Test aarch32_VCMP_T1_A field size = 0 (Min)
    // ISET: T32
    // Fields: M=0, Vm=0, Vd=0, D=0, size=0, E=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcmp_t1_a_field_size_1_poweroftwo_8c0_eeb40940() {
    // Thumb encoding (32): 0xEEB40940
    // Test aarch32_VCMP_T1_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=0, size=1, Vd=0, E=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40940;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcmp_t1_a_field_size_2_poweroftwo_8c0_eeb40a40() {
    // Thumb encoding (32): 0xEEB40A40
    // Test aarch32_VCMP_T1_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: E=0, D=0, size=2, M=0, Vm=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcmp_t1_a_field_size_3_max_8c0_eeb40b40() {
    // Thumb encoding (32): 0xEEB40B40
    // Test aarch32_VCMP_T1_A field size = 3 (Max)
    // ISET: T32
    // Fields: D=0, size=3, Vd=0, E=0, Vm=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field E 7 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcmp_t1_a_field_e_0_min_8c0_eeb40840() {
    // Thumb encoding (32): 0xEEB40840
    // Test aarch32_VCMP_T1_A field E = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, size=0, E=0, Vm=0, M=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field E 7 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcmp_t1_a_field_e_1_max_8c0_eeb408c0() {
    // Thumb encoding (32): 0xEEB408C0
    // Test aarch32_VCMP_T1_A field E = 1 (Max)
    // ISET: T32
    // Fields: size=0, E=1, Vd=0, D=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB408C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcmp_t1_a_field_m_0_min_8c0_eeb40840() {
    // Thumb encoding (32): 0xEEB40840
    // Test aarch32_VCMP_T1_A field M = 0 (Min)
    // ISET: T32
    // Fields: D=0, E=0, size=0, M=0, Vd=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcmp_t1_a_field_m_1_max_8c0_eeb40860() {
    // Thumb encoding (32): 0xEEB40860
    // Test aarch32_VCMP_T1_A field M = 1 (Max)
    // ISET: T32
    // Fields: E=0, M=1, size=0, Vm=0, D=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40860;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcmp_t1_a_field_vm_0_min_8c0_eeb40840() {
    // Thumb encoding (32): 0xEEB40840
    // Test aarch32_VCMP_T1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, E=0, Vm=0, D=0, size=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcmp_t1_a_field_vm_1_poweroftwo_8c0_eeb40841() {
    // Thumb encoding (32): 0xEEB40841
    // Test aarch32_VCMP_T1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, Vm=1, E=0, Vd=0, M=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40841;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcmp_t1_a_combo_0_8c0_eeb40840() {
    // Thumb encoding (32): 0xEEB40840
    // Test aarch32_VCMP_T1_A field combination: D=0, Vd=0, size=0, E=0, M=0, Vm=0
    // ISET: T32
    // Fields: D=0, size=0, Vm=0, E=0, Vd=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcmp_t1_a_special_size_0_size_variant_0_2240_eeb40840() {
    // Thumb encoding (32): 0xEEB40840
    // Test aarch32_VCMP_T1_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: Vd=0, M=0, D=0, size=0, E=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcmp_t1_a_special_size_1_size_variant_1_2240_eeb40940() {
    // Thumb encoding (32): 0xEEB40940
    // Test aarch32_VCMP_T1_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: Vd=0, size=1, D=0, M=0, Vm=0, E=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40940;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcmp_t1_a_special_size_2_size_variant_2_2240_eeb40a40() {
    // Thumb encoding (32): 0xEEB40A40
    // Test aarch32_VCMP_T1_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: D=0, Vd=0, size=2, M=0, Vm=0, E=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcmp_t1_a_special_size_3_size_variant_3_2240_eeb40b40() {
    // Thumb encoding (32): 0xEEB40B40
    // Test aarch32_VCMP_T1_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: size=3, Vd=0, D=0, E=0, Vm=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcmp_t1_a_invalid_0_8c0_eeb40840() {
    // Thumb encoding (32): 0xEEB40840
    // Test aarch32_VCMP_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: T32
    // Fields: size=0, E=0, M=0, D=0, Vm=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcmp_t1_a_invalid_1_8c0_eeb40840() {
    // Thumb encoding (32): 0xEEB40840
    // Test aarch32_VCMP_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: M=0, size=0, Vm=0, D=0, E=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcmp_t1_a_invalid_2_8c0_eeb40840() {
    // Thumb encoding (32): 0xEEB40840
    // Test aarch32_VCMP_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: Vd=0, E=0, M=0, Vm=0, size=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VCMP_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcmp_t1_a_invalid_3_8c0_eeb40840() {
    // Thumb encoding (32): 0xEEB40840
    // Test aarch32_VCMP_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: M=0, Vd=0, Vm=0, D=0, size=0, E=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB40840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcmp_t2_a_field_d_0_min_8c0_eeb50840() {
    // Thumb encoding (32): 0xEEB50840
    // Test aarch32_VCMP_T2_A field D = 0 (Min)
    // ISET: T32
    // Fields: size=0, D=0, E=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcmp_t2_a_field_d_1_max_8c0_eef50840() {
    // Thumb encoding (32): 0xEEF50840
    // Test aarch32_VCMP_T2_A field D = 1 (Max)
    // ISET: T32
    // Fields: size=0, E=0, D=1, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEF50840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcmp_t2_a_field_vd_0_min_8c0_eeb50840() {
    // Thumb encoding (32): 0xEEB50840
    // Test aarch32_VCMP_T2_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: size=0, Vd=0, E=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcmp_t2_a_field_vd_1_poweroftwo_8c0_eeb51840() {
    // Thumb encoding (32): 0xEEB51840
    // Test aarch32_VCMP_T2_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, E=0, Vd=1, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB51840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcmp_t2_a_field_size_0_min_8c0_eeb50840() {
    // Thumb encoding (32): 0xEEB50840
    // Test aarch32_VCMP_T2_A field size = 0 (Min)
    // ISET: T32
    // Fields: E=0, size=0, Vd=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcmp_t2_a_field_size_1_poweroftwo_8c0_eeb50940() {
    // Thumb encoding (32): 0xEEB50940
    // Test aarch32_VCMP_T2_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vd=0, D=0, size=1, E=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50940;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcmp_t2_a_field_size_2_poweroftwo_8c0_eeb50a40() {
    // Thumb encoding (32): 0xEEB50A40
    // Test aarch32_VCMP_T2_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, size=2, Vd=0, E=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcmp_t2_a_field_size_3_max_8c0_eeb50b40() {
    // Thumb encoding (32): 0xEEB50B40
    // Test aarch32_VCMP_T2_A field size = 3 (Max)
    // ISET: T32
    // Fields: D=0, size=3, Vd=0, E=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field E 7 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcmp_t2_a_field_e_0_min_8c0_eeb50840() {
    // Thumb encoding (32): 0xEEB50840
    // Test aarch32_VCMP_T2_A field E = 0 (Min)
    // ISET: T32
    // Fields: D=0, size=0, Vd=0, E=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field E 7 +: 1`
/// Requirement: FieldBoundary { field: "E", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcmp_t2_a_field_e_1_max_8c0_eeb508c0() {
    // Thumb encoding (32): 0xEEB508C0
    // Test aarch32_VCMP_T2_A field E = 1 (Max)
    // ISET: T32
    // Fields: size=0, D=0, E=1, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB508C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcmp_t2_a_combo_0_8c0_eeb50840() {
    // Thumb encoding (32): 0xEEB50840
    // Test aarch32_VCMP_T2_A field combination: D=0, Vd=0, size=0, E=0
    // ISET: T32
    // Fields: size=0, Vd=0, D=0, E=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcmp_t2_a_special_size_0_size_variant_0_2240_eeb50840() {
    // Thumb encoding (32): 0xEEB50840
    // Test aarch32_VCMP_T2_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: Vd=0, size=0, D=0, E=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcmp_t2_a_special_size_1_size_variant_1_2240_eeb50940() {
    // Thumb encoding (32): 0xEEB50940
    // Test aarch32_VCMP_T2_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: size=1, D=0, Vd=0, E=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50940;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcmp_t2_a_special_size_2_size_variant_2_2240_eeb50a40() {
    // Thumb encoding (32): 0xEEB50A40
    // Test aarch32_VCMP_T2_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: D=0, size=2, E=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcmp_t2_a_special_size_3_size_variant_3_2240_eeb50b40() {
    // Thumb encoding (32): 0xEEB50B40
    // Test aarch32_VCMP_T2_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: D=0, Vd=0, size=3, E=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcmp_t2_a_invalid_0_8c0_eeb50840() {
    // Thumb encoding (32): 0xEEB50840
    // Test aarch32_VCMP_T2_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: T32
    // Fields: E=0, Vd=0, size=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcmp_t2_a_invalid_1_8c0_eeb50840() {
    // Thumb encoding (32): 0xEEB50840
    // Test aarch32_VCMP_T2_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: D=0, size=0, Vd=0, E=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcmp_t2_a_invalid_2_8c0_eeb50840() {
    // Thumb encoding (32): 0xEEB50840
    // Test aarch32_VCMP_T2_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: E=0, D=0, size=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VCMP_T2_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcmp_t2_a_invalid_3_8c0_eeb50840() {
    // Thumb encoding (32): 0xEEB50840
    // Test aarch32_VCMP_T2_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: E=0, Vd=0, D=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB50840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}
