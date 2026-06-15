//! A32 simd convert tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers_32::*;

// ============================================================================
// aarch32_VCVTB_A Tests
// ============================================================================

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_0_min_ac0_0eb20a40() {
    // Encoding: 0x0EB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, sz=0, T=0, Vm=0, op=0, D=0, Vd=0, M=0
    let encoding: u32 = 0x0EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_1_poweroftwo_ac0_1eb20a40() {
    // Encoding: 0x1EB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=1, op=0, sz=0, M=0, T=0, Vd=0, Vm=0, D=0
    let encoding: u32 = 0x1EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_2_poweroftwo_ac0_2eb20a40() {
    // Encoding: 0x2EB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, M=0, T=0, cond=2, sz=0, op=0, Vd=0, Vm=0
    let encoding: u32 = 0x2EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_3_poweroftwo_ac0_3eb20a40() {
    // Encoding: 0x3EB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: op=0, Vd=0, sz=0, Vm=0, cond=3, T=0, D=0, M=0
    let encoding: u32 = 0x3EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_4_poweroftwo_ac0_4eb20a40() {
    // Encoding: 0x4EB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, D=0, T=0, sz=0, M=0, cond=4, op=0, Vm=0
    let encoding: u32 = 0x4EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_5_poweroftwo_ac0_5eb20a40() {
    // Encoding: 0x5EB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, T=0, Vm=0, sz=0, cond=5, D=0, op=0, Vd=0
    let encoding: u32 = 0x5EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_6_poweroftwo_ac0_6eb20a40() {
    // Encoding: 0x6EB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, sz=0, T=0, Vm=0, M=0, cond=6, D=0, op=0
    let encoding: u32 = 0x6EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_7_poweroftwo_ac0_7eb20a40() {
    // Encoding: 0x7EB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=7, Vd=0, M=0, sz=0, T=0, op=0, Vm=0, D=0
    let encoding: u32 = 0x7EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_8_poweroftwo_ac0_8eb20a40() {
    // Encoding: 0x8EB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, cond=8, op=0, Vd=0, Vm=0, D=0, T=0, sz=0
    let encoding: u32 = 0x8EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_9_poweroftwo_ac0_9eb20a40() {
    // Encoding: 0x9EB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=9, T=0, op=0, D=0, Vd=0, M=0, sz=0, Vm=0
    let encoding: u32 = 0x9EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_10_poweroftwo_ac0_aeb20a40() {
    // Encoding: 0xAEB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=0, cond=10, Vd=0, D=0, op=0, M=0, T=0, sz=0
    let encoding: u32 = 0xAEB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_11_poweroftwo_ac0_beb20a40() {
    // Encoding: 0xBEB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: sz=0, op=0, Vd=0, M=0, cond=11, T=0, Vm=0, D=0
    let encoding: u32 = 0xBEB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_12_poweroftwo_ac0_ceb20a40() {
    // Encoding: 0xCEB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, Vd=0, op=0, T=0, Vm=0, sz=0, M=0, D=0
    let encoding: u32 = 0xCEB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_13_poweroftwo_ac0_deb20a40() {
    // Encoding: 0xDEB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: T=0, D=0, op=0, M=0, sz=0, Vm=0, cond=13, Vd=0
    let encoding: u32 = 0xDEB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_14_poweroftwo_ac0_eeb20a40() {
    // Encoding: 0xEEB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: op=0, sz=0, Vm=0, Vd=0, D=0, T=0, cond=14, M=0
    let encoding: u32 = 0xEEB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_cond_15_max_ac0_feb20a40() {
    // Encoding: 0xFEB20A40
    // Test aarch32_VCVTB_T1A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: D=0, Vm=0, M=0, T=0, sz=0, op=0, Vd=0, cond=15
    let encoding: u32 = 0xFEB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_d_0_min_ac0_0eb20a40() {
    // Encoding: 0x0EB20A40
    // Test aarch32_VCVTB_T1A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: cond=0, M=0, T=0, Vd=0, Vm=0, op=0, D=0, sz=0
    let encoding: u32 = 0x0EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_d_1_max_ac0_0ef20a40() {
    // Encoding: 0x0EF20A40
    // Test aarch32_VCVTB_T1A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: M=0, cond=0, op=0, T=0, sz=0, Vm=0, D=1, Vd=0
    let encoding: u32 = 0x0EF20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field op 16 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_op_0_min_ac0_0eb20a40() {
    // Encoding: 0x0EB20A40
    // Test aarch32_VCVTB_T1A1_A field op = 0 (Min)
    // ISET: A32
    // Fields: T=0, cond=0, sz=0, D=0, Vm=0, M=0, op=0, Vd=0
    let encoding: u32 = 0x0EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field op 16 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_op_1_max_ac0_0eb30a40() {
    // Encoding: 0x0EB30A40
    // Test aarch32_VCVTB_T1A1_A field op = 1 (Max)
    // ISET: A32
    // Fields: op=1, D=0, T=0, Vm=0, Vd=0, sz=0, cond=0, M=0
    let encoding: u32 = 0x0EB30A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_vd_0_min_ac0_0eb20a40() {
    // Encoding: 0x0EB20A40
    // Test aarch32_VCVTB_T1A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: D=0, sz=0, Vm=0, T=0, Vd=0, op=0, M=0, cond=0
    let encoding: u32 = 0x0EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_vd_1_poweroftwo_ac0_0eb21a40() {
    // Encoding: 0x0EB21A40
    // Test aarch32_VCVTB_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Vd=1, M=0, sz=0, op=0, D=0, T=0, Vm=0
    let encoding: u32 = 0x0EB21A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field sz 8 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_sz_0_min_ac0_0eb20a40() {
    // Encoding: 0x0EB20A40
    // Test aarch32_VCVTB_T1A1_A field sz = 0 (Min)
    // ISET: A32
    // Fields: D=0, cond=0, Vm=0, M=0, sz=0, op=0, Vd=0, T=0
    let encoding: u32 = 0x0EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field sz 8 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_sz_1_max_ac0_0eb20b40() {
    // Encoding: 0x0EB20B40
    // Test aarch32_VCVTB_T1A1_A field sz = 1 (Max)
    // ISET: A32
    // Fields: Vm=0, Vd=0, T=0, op=0, D=0, sz=1, M=0, cond=0
    let encoding: u32 = 0x0EB20B40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field T 7 +: 1`
/// Requirement: FieldBoundary { field: "T", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_t_0_min_ac0_0eb20a40() {
    // Encoding: 0x0EB20A40
    // Test aarch32_VCVTB_T1A1_A field T = 0 (Min)
    // ISET: A32
    // Fields: sz=0, op=0, M=0, T=0, D=0, Vd=0, cond=0, Vm=0
    let encoding: u32 = 0x0EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field T 7 +: 1`
/// Requirement: FieldBoundary { field: "T", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_t_1_max_ac0_0eb20ac0() {
    // Encoding: 0x0EB20AC0
    // Test aarch32_VCVTB_T1A1_A field T = 1 (Max)
    // ISET: A32
    // Fields: op=0, T=1, cond=0, Vm=0, sz=0, D=0, Vd=0, M=0
    let encoding: u32 = 0x0EB20AC0;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_m_0_min_ac0_0eb20a40() {
    // Encoding: 0x0EB20A40
    // Test aarch32_VCVTB_T1A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: sz=0, D=0, Vm=0, T=0, M=0, op=0, cond=0, Vd=0
    let encoding: u32 = 0x0EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_m_1_max_ac0_0eb20a60() {
    // Encoding: 0x0EB20A60
    // Test aarch32_VCVTB_T1A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: Vd=0, sz=0, M=1, op=0, Vm=0, D=0, cond=0, T=0
    let encoding: u32 = 0x0EB20A60;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_vm_0_min_ac0_0eb20a40() {
    // Encoding: 0x0EB20A40
    // Test aarch32_VCVTB_T1A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: op=0, M=0, Vm=0, T=0, Vd=0, D=0, sz=0, cond=0
    let encoding: u32 = 0x0EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_vm_1_poweroftwo_ac0_0eb20a41() {
    // Encoding: 0x0EB20A41
    // Test aarch32_VCVTB_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, M=0, cond=0, op=0, Vm=1, T=0, sz=0, D=0
    let encoding: u32 = 0x0EB20A41;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_vcvtb_t1a1_a_combo_0_ac0_0eb20a40() {
    // Encoding: 0x0EB20A40
    // Test aarch32_VCVTB_T1A1_A field combination: cond=0, D=0, op=0, Vd=0, sz=0, T=0, M=0, Vm=0
    // ISET: A32
    // Fields: Vd=0, T=0, Vm=0, M=0, sz=0, D=0, cond=0, op=0
    let encoding: u32 = 0x0EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_0_condition_eq_2752_0eb20a40() {
    // Encoding: 0x0EB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Vd=0, Vm=0, sz=0, M=0, D=0, T=0, op=0, cond=0
    let encoding: u32 = 0x0EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_1_condition_ne_2752_1eb20a40() {
    // Encoding: 0x1EB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: D=0, T=0, M=0, op=0, Vm=0, sz=0, Vd=0, cond=1
    let encoding: u32 = 0x1EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_2_condition_cs_hs_2752_2eb20a40() {
    // Encoding: 0x2EB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: T=0, op=0, cond=2, Vd=0, D=0, M=0, sz=0, Vm=0
    let encoding: u32 = 0x2EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_3_condition_cc_lo_2752_3eb20a40() {
    // Encoding: 0x3EB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: D=0, sz=0, T=0, cond=3, M=0, Vm=0, Vd=0, op=0
    let encoding: u32 = 0x3EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_4_condition_mi_2752_4eb20a40() {
    // Encoding: 0x4EB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: cond=4, T=0, op=0, Vm=0, Vd=0, M=0, D=0, sz=0
    let encoding: u32 = 0x4EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_5_condition_pl_2752_5eb20a40() {
    // Encoding: 0x5EB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: Vd=0, T=0, D=0, op=0, Vm=0, sz=0, cond=5, M=0
    let encoding: u32 = 0x5EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_6_condition_vs_2752_6eb20a40() {
    // Encoding: 0x6EB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: cond=6, T=0, M=0, sz=0, op=0, D=0, Vm=0, Vd=0
    let encoding: u32 = 0x6EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_7_condition_vc_2752_7eb20a40() {
    // Encoding: 0x7EB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: op=0, Vm=0, M=0, D=0, Vd=0, cond=7, T=0, sz=0
    let encoding: u32 = 0x7EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_8_condition_hi_2752_8eb20a40() {
    // Encoding: 0x8EB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: sz=0, op=0, cond=8, Vm=0, M=0, Vd=0, D=0, T=0
    let encoding: u32 = 0x8EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_9_condition_ls_2752_9eb20a40() {
    // Encoding: 0x9EB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Vd=0, op=0, D=0, Vm=0, T=0, sz=0, cond=9, M=0
    let encoding: u32 = 0x9EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_10_condition_ge_2752_aeb20a40() {
    // Encoding: 0xAEB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: D=0, Vd=0, M=0, cond=10, sz=0, T=0, Vm=0, op=0
    let encoding: u32 = 0xAEB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_11_condition_lt_2752_beb20a40() {
    // Encoding: 0xBEB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: Vm=0, M=0, sz=0, T=0, D=0, cond=11, op=0, Vd=0
    let encoding: u32 = 0xBEB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_12_condition_gt_2752_ceb20a40() {
    // Encoding: 0xCEB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, Vm=0, sz=0, M=0, op=0, Vd=0, D=0, T=0
    let encoding: u32 = 0xCEB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_13_condition_le_2752_deb20a40() {
    // Encoding: 0xDEB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: M=0, D=0, sz=0, cond=13, Vd=0, Vm=0, T=0, op=0
    let encoding: u32 = 0xDEB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_14_condition_al_2752_eeb20a40() {
    // Encoding: 0xEEB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: D=0, T=0, cond=14, Vm=0, sz=0, M=0, op=0, Vd=0
    let encoding: u32 = 0xEEB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_cond_15_condition_nv_2752_feb20a40() {
    // Encoding: 0xFEB20A40
    // Test aarch32_VCVTB_T1A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: D=0, T=0, M=0, cond=15, Vd=0, sz=0, op=0, Vm=0
    let encoding: u32 = 0xFEB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_sz_0_size_variant_0_2752_0eb20a40() {
    // Encoding: 0x0EB20A40
    // Test aarch32_VCVTB_T1A1_A special value sz = 0 (Size variant 0)
    // ISET: A32
    // Fields: Vm=0, D=0, Vd=0, M=0, sz=0, op=0, T=0, cond=0
    let encoding: u32 = 0x0EB20A40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_sz_1_size_variant_1_2752_0eb20b40() {
    // Encoding: 0x0EB20B40
    // Test aarch32_VCVTB_T1A1_A special value sz = 1 (Size variant 1)
    // ISET: A32
    // Fields: op=0, Vd=0, cond=0, sz=1, Vm=0, T=0, M=0, D=0
    let encoding: u32 = 0x0EB20B40;
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

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_d_0_min_ac0_eeb20a40() {
    // Thumb encoding (32): 0xEEB20A40
    // Test aarch32_VCVTB_T1A1_A field D = 0 (Min)
    // ISET: T32
    // Fields: op=0, sz=0, T=0, M=0, Vm=0, D=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_d_1_max_ac0_eef20a40() {
    // Thumb encoding (32): 0xEEF20A40
    // Test aarch32_VCVTB_T1A1_A field D = 1 (Max)
    // ISET: T32
    // Fields: sz=0, op=0, Vm=0, T=0, D=1, M=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEF20A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field op 16 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_op_0_min_ac0_eeb20a40() {
    // Thumb encoding (32): 0xEEB20A40
    // Test aarch32_VCVTB_T1A1_A field op = 0 (Min)
    // ISET: T32
    // Fields: M=0, sz=0, Vd=0, D=0, op=0, T=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field op 16 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_op_1_max_ac0_eeb30a40() {
    // Thumb encoding (32): 0xEEB30A40
    // Test aarch32_VCVTB_T1A1_A field op = 1 (Max)
    // ISET: T32
    // Fields: Vm=0, Vd=0, D=0, M=0, T=0, op=1, sz=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB30A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_vd_0_min_ac0_eeb20a40() {
    // Thumb encoding (32): 0xEEB20A40
    // Test aarch32_VCVTB_T1A1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: D=0, op=0, sz=0, M=0, Vm=0, Vd=0, T=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_vd_1_poweroftwo_ac0_eeb21a40() {
    // Thumb encoding (32): 0xEEB21A40
    // Test aarch32_VCVTB_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=0, op=0, D=0, Vd=1, sz=0, T=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB21A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field sz 8 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_sz_0_min_ac0_eeb20a40() {
    // Thumb encoding (32): 0xEEB20A40
    // Test aarch32_VCVTB_T1A1_A field sz = 0 (Min)
    // ISET: T32
    // Fields: T=0, M=0, Vm=0, Vd=0, op=0, D=0, sz=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field sz 8 +: 1`
/// Requirement: FieldBoundary { field: "sz", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_sz_1_max_ac0_eeb20b40() {
    // Thumb encoding (32): 0xEEB20B40
    // Test aarch32_VCVTB_T1A1_A field sz = 1 (Max)
    // ISET: T32
    // Fields: op=0, Vd=0, T=0, M=0, D=0, sz=1, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field T 7 +: 1`
/// Requirement: FieldBoundary { field: "T", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_t_0_min_ac0_eeb20a40() {
    // Thumb encoding (32): 0xEEB20A40
    // Test aarch32_VCVTB_T1A1_A field T = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, M=0, Vm=0, D=0, sz=0, op=0, T=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field T 7 +: 1`
/// Requirement: FieldBoundary { field: "T", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_t_1_max_ac0_eeb20ac0() {
    // Thumb encoding (32): 0xEEB20AC0
    // Test aarch32_VCVTB_T1A1_A field T = 1 (Max)
    // ISET: T32
    // Fields: M=0, Vd=0, Vm=0, T=1, D=0, sz=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20AC0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_m_0_min_ac0_eeb20a40() {
    // Thumb encoding (32): 0xEEB20A40
    // Test aarch32_VCVTB_T1A1_A field M = 0 (Min)
    // ISET: T32
    // Fields: op=0, D=0, T=0, M=0, Vm=0, sz=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_m_1_max_ac0_eeb20a60() {
    // Thumb encoding (32): 0xEEB20A60
    // Test aarch32_VCVTB_T1A1_A field M = 1 (Max)
    // ISET: T32
    // Fields: Vm=0, Vd=0, D=0, sz=0, op=0, T=0, M=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20A60;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_vm_0_min_ac0_eeb20a40() {
    // Thumb encoding (32): 0xEEB20A40
    // Test aarch32_VCVTB_T1A1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, sz=0, T=0, M=0, Vm=0, op=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvtb_t1a1_a_field_vm_1_poweroftwo_ac0_eeb20a41() {
    // Thumb encoding (32): 0xEEB20A41
    // Test aarch32_VCVTB_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, op=0, T=0, M=0, Vm=1, sz=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20A41;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcvtb_t1a1_a_combo_0_ac0_eeb20a40() {
    // Thumb encoding (32): 0xEEB20A40
    // Test aarch32_VCVTB_T1A1_A field combination: D=0, op=0, Vd=0, sz=0, T=0, M=0, Vm=0
    // ISET: T32
    // Fields: Vm=0, D=0, Vd=0, sz=0, op=0, T=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field sz = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sz", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_sz_0_size_variant_0_2752_eeb20a40() {
    // Thumb encoding (32): 0xEEB20A40
    // Test aarch32_VCVTB_T1A1_A special value sz = 0 (Size variant 0)
    // ISET: T32
    // Fields: op=0, D=0, T=0, Vd=0, M=0, Vm=0, sz=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTB_T1A1_A
/// ASL: `field sz = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sz", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvtb_t1a1_a_special_sz_1_size_variant_1_2752_eeb20b40() {
    // Thumb encoding (32): 0xEEB20B40
    // Test aarch32_VCVTB_T1A1_A special value sz = 1 (Size variant 1)
    // ISET: T32
    // Fields: T=0, D=0, op=0, sz=1, Vd=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB20B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

// ============================================================================
// aarch32_VCVT_iv_A Tests
// ============================================================================

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_0_min_840_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vd=0, opc2=0, M=0, cond=0, size=0, Vm=0, op=0
    let encoding: u32 = 0x0EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_1_poweroftwo_840_1eb80840() {
    // Encoding: 0x1EB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, size=0, op=0, M=0, Vm=0, cond=1, Vd=0, opc2=0
    let encoding: u32 = 0x1EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_2_poweroftwo_840_2eb80840() {
    // Encoding: 0x2EB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, Vm=0, opc2=0, M=0, D=0, size=0, cond=2, op=0
    let encoding: u32 = 0x2EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_3_poweroftwo_840_3eb80840() {
    // Encoding: 0x3EB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, opc2=0, Vd=0, op=0, Vm=0, D=0, size=0, M=0
    let encoding: u32 = 0x3EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_4_poweroftwo_840_4eb80840() {
    // Encoding: 0x4EB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, D=0, size=0, op=0, opc2=0, Vm=0, cond=4, Vd=0
    let encoding: u32 = 0x4EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_5_poweroftwo_840_5eb80840() {
    // Encoding: 0x5EB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, Vm=0, M=0, opc2=0, op=0, size=0, cond=5, D=0
    let encoding: u32 = 0x5EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_6_poweroftwo_840_6eb80840() {
    // Encoding: 0x6EB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, size=0, opc2=0, Vd=0, D=0, M=0, Vm=0, op=0
    let encoding: u32 = 0x6EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_7_poweroftwo_840_7eb80840() {
    // Encoding: 0x7EB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: opc2=0, M=0, cond=7, Vm=0, Vd=0, D=0, size=0, op=0
    let encoding: u32 = 0x7EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_8_poweroftwo_840_8eb80840() {
    // Encoding: 0x8EB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, opc2=0, D=0, size=0, op=0, Vm=0, cond=8, M=0
    let encoding: u32 = 0x8EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_9_poweroftwo_840_9eb80840() {
    // Encoding: 0x9EB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: op=0, M=0, opc2=0, Vd=0, D=0, size=0, cond=9, Vm=0
    let encoding: u32 = 0x9EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_10_poweroftwo_840_aeb80840() {
    // Encoding: 0xAEB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=0, M=0, D=0, Vd=0, size=0, op=0, opc2=0, cond=10
    let encoding: u32 = 0xAEB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_11_poweroftwo_840_beb80840() {
    // Encoding: 0xBEB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=11, op=0, size=0, M=0, Vd=0, D=0, opc2=0, Vm=0
    let encoding: u32 = 0xBEB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_12_poweroftwo_840_ceb80840() {
    // Encoding: 0xCEB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=12, opc2=0, op=0, Vd=0, M=0, D=0, size=0, Vm=0
    let encoding: u32 = 0xCEB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_13_poweroftwo_840_deb80840() {
    // Encoding: 0xDEB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, size=0, opc2=0, op=0, Vm=0, cond=13, M=0, D=0
    let encoding: u32 = 0xDEB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_14_poweroftwo_840_eeb80840() {
    // Encoding: 0xEEB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: opc2=0, Vd=0, D=0, M=0, size=0, op=0, cond=14, Vm=0
    let encoding: u32 = 0xEEB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_cond_15_max_840_feb80840() {
    // Encoding: 0xFEB80840
    // Test aarch32_VCVT_iv_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: size=0, op=0, M=0, Vd=0, cond=15, D=0, Vm=0, opc2=0
    let encoding: u32 = 0xFEB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_d_0_min_840_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, M=0, opc2=0, D=0, size=0, op=0, cond=0, Vm=0
    let encoding: u32 = 0x0EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_d_1_max_840_0ef80840() {
    // Encoding: 0x0EF80840
    // Test aarch32_VCVT_iv_A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: Vm=0, size=0, opc2=0, op=0, M=0, cond=0, D=1, Vd=0
    let encoding: u32 = 0x0EF80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field opc2 16 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_opc2_0_min_840_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A field opc2 = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, Vd=0, op=0, cond=0, M=0, D=0, opc2=0, size=0
    let encoding: u32 = 0x0EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field opc2 16 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_opc2_1_poweroftwo_840_0eb90840() {
    // Encoding: 0x0EB90840
    // Test aarch32_VCVT_iv_A1_A field opc2 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, size=0, opc2=1, op=0, cond=0, Vd=0, D=0, Vm=0
    let encoding: u32 = 0x0EB90840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field opc2 16 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_opc2_7_max_840_0ebf0840() {
    // Encoding: 0x0EBF0840
    // Test aarch32_VCVT_iv_A1_A field opc2 = 7 (Max)
    // ISET: A32
    // Fields: cond=0, Vm=0, opc2=7, Vd=0, op=0, D=0, size=0, M=0
    let encoding: u32 = 0x0EBF0840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_vd_0_min_840_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: M=0, size=0, opc2=0, Vm=0, cond=0, D=0, Vd=0, op=0
    let encoding: u32 = 0x0EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_vd_1_poweroftwo_840_0eb81840() {
    // Encoding: 0x0EB81840
    // Test aarch32_VCVT_iv_A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: opc2=0, M=0, cond=0, D=0, size=0, op=0, Vm=0, Vd=1
    let encoding: u32 = 0x0EB81840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_size_0_min_840_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A field size = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Vd=0, Vm=0, size=0, D=0, opc2=0, op=0, M=0
    let encoding: u32 = 0x0EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_size_1_poweroftwo_840_0eb80940() {
    // Encoding: 0x0EB80940
    // Test aarch32_VCVT_iv_A1_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, D=0, cond=0, opc2=0, M=0, op=0, Vm=0, size=1
    let encoding: u32 = 0x0EB80940;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_size_2_poweroftwo_840_0eb80a40() {
    // Encoding: 0x0EB80A40
    // Test aarch32_VCVT_iv_A1_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, size=2, opc2=0, cond=0, Vd=0, D=0, op=0, Vm=0
    let encoding: u32 = 0x0EB80A40;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_size_3_max_840_0eb80b40() {
    // Encoding: 0x0EB80B40
    // Test aarch32_VCVT_iv_A1_A field size = 3 (Max)
    // ISET: A32
    // Fields: size=3, M=0, Vm=0, op=0, opc2=0, D=0, cond=0, Vd=0
    let encoding: u32 = 0x0EB80B40;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field op 7 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_op_0_min_840_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A field op = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, op=0, opc2=0, cond=0, D=0, Vm=0, size=0, M=0
    let encoding: u32 = 0x0EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field op 7 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_op_1_max_840_0eb808c0() {
    // Encoding: 0x0EB808C0
    // Test aarch32_VCVT_iv_A1_A field op = 1 (Max)
    // ISET: A32
    // Fields: Vm=0, cond=0, Vd=0, op=1, size=0, M=0, opc2=0, D=0
    let encoding: u32 = 0x0EB808C0;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_m_0_min_840_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, M=0, op=0, cond=0, D=0, size=0, opc2=0, Vm=0
    let encoding: u32 = 0x0EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_m_1_max_840_0eb80860() {
    // Encoding: 0x0EB80860
    // Test aarch32_VCVT_iv_A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: D=0, size=0, op=0, Vm=0, Vd=0, cond=0, M=1, opc2=0
    let encoding: u32 = 0x0EB80860;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_vm_0_min_840_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: opc2=0, size=0, M=0, Vd=0, Vm=0, D=0, op=0, cond=0
    let encoding: u32 = 0x0EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_iv_a1_a_field_vm_1_poweroftwo_840_0eb80841() {
    // Encoding: 0x0EB80841
    // Test aarch32_VCVT_iv_A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, Vd=0, D=0, M=0, opc2=0, size=0, Vm=1, op=0
    let encoding: u32 = 0x0EB80841;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_vcvt_iv_a1_a_combo_0_840_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A field combination: cond=0, D=0, opc2=0, Vd=0, size=0, op=0, M=0, Vm=0
    // ISET: A32
    // Fields: op=0, Vm=0, M=0, size=0, cond=0, opc2=0, D=0, Vd=0
    let encoding: u32 = 0x0EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_0_condition_eq_2112_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: D=0, Vm=0, M=0, opc2=0, op=0, Vd=0, size=0, cond=0
    let encoding: u32 = 0x0EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_1_condition_ne_2112_1eb80840() {
    // Encoding: 0x1EB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, opc2=0, Vd=0, size=0, op=0, Vm=0, D=0, M=0
    let encoding: u32 = 0x1EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_2_condition_cs_hs_2112_2eb80840() {
    // Encoding: 0x2EB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: op=0, Vd=0, cond=2, D=0, opc2=0, Vm=0, M=0, size=0
    let encoding: u32 = 0x2EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_3_condition_cc_lo_2112_3eb80840() {
    // Encoding: 0x3EB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: size=0, Vd=0, opc2=0, Vm=0, op=0, cond=3, M=0, D=0
    let encoding: u32 = 0x3EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_4_condition_mi_2112_4eb80840() {
    // Encoding: 0x4EB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: opc2=0, op=0, cond=4, Vd=0, size=0, M=0, D=0, Vm=0
    let encoding: u32 = 0x4EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_5_condition_pl_2112_5eb80840() {
    // Encoding: 0x5EB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: size=0, cond=5, Vm=0, D=0, M=0, Vd=0, op=0, opc2=0
    let encoding: u32 = 0x5EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_6_condition_vs_2112_6eb80840() {
    // Encoding: 0x6EB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: opc2=0, D=0, cond=6, Vd=0, size=0, op=0, Vm=0, M=0
    let encoding: u32 = 0x6EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_7_condition_vc_2112_7eb80840() {
    // Encoding: 0x7EB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: op=0, M=0, size=0, cond=7, opc2=0, D=0, Vd=0, Vm=0
    let encoding: u32 = 0x7EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_8_condition_hi_2112_8eb80840() {
    // Encoding: 0x8EB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: Vd=0, opc2=0, size=0, op=0, M=0, D=0, cond=8, Vm=0
    let encoding: u32 = 0x8EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_9_condition_ls_2112_9eb80840() {
    // Encoding: 0x9EB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: cond=9, opc2=0, op=0, M=0, Vm=0, Vd=0, size=0, D=0
    let encoding: u32 = 0x9EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_10_condition_ge_2112_aeb80840() {
    // Encoding: 0xAEB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: D=0, size=0, Vd=0, Vm=0, M=0, opc2=0, cond=10, op=0
    let encoding: u32 = 0xAEB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_11_condition_lt_2112_beb80840() {
    // Encoding: 0xBEB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, opc2=0, Vd=0, size=0, op=0, M=0, D=0, Vm=0
    let encoding: u32 = 0xBEB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_12_condition_gt_2112_ceb80840() {
    // Encoding: 0xCEB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Vm=0, M=0, D=0, cond=12, op=0, size=0, Vd=0, opc2=0
    let encoding: u32 = 0xCEB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_13_condition_le_2112_deb80840() {
    // Encoding: 0xDEB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: opc2=0, Vm=0, cond=13, size=0, D=0, op=0, Vd=0, M=0
    let encoding: u32 = 0xDEB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_14_condition_al_2112_eeb80840() {
    // Encoding: 0xEEB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: size=0, M=0, cond=14, op=0, D=0, opc2=0, Vd=0, Vm=0
    let encoding: u32 = 0xEEB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_cond_15_condition_nv_2112_feb80840() {
    // Encoding: 0xFEB80840
    // Test aarch32_VCVT_iv_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: Vd=0, D=0, Vm=0, M=0, op=0, size=0, cond=15, opc2=0
    let encoding: u32 = 0xFEB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_size_0_size_variant_0_2112_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: cond=0, op=0, M=0, Vd=0, opc2=0, D=0, size=0, Vm=0
    let encoding: u32 = 0x0EB80840;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_size_1_size_variant_1_2112_0eb80940() {
    // Encoding: 0x0EB80940
    // Test aarch32_VCVT_iv_A1_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: op=0, Vm=0, opc2=0, Vd=0, D=0, cond=0, size=1, M=0
    let encoding: u32 = 0x0EB80940;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_size_2_size_variant_2_2112_0eb80a40() {
    // Encoding: 0x0EB80A40
    // Test aarch32_VCVT_iv_A1_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: size=2, D=0, Vd=0, Vm=0, M=0, opc2=0, op=0, cond=0
    let encoding: u32 = 0x0EB80A40;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvt_iv_a1_a_special_size_3_size_variant_3_2112_0eb80b40() {
    // Encoding: 0x0EB80B40
    // Test aarch32_VCVT_iv_A1_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: cond=0, D=0, Vd=0, size=3, M=0, Vm=0, op=0, opc2=0
    let encoding: u32 = 0x0EB80B40;
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

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_iv_a1_a_invalid_0_840_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: A32
    // Fields: op=0, opc2=0, cond=0, D=0, Vm=0, Vd=0, size=0, M=0
    let encoding: u32 = 0x0EB80840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_iv_a1_a_invalid_1_840_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: op=0, Vm=0, Vd=0, M=0, opc2=0, size=0, D=0, cond=0
    let encoding: u32 = 0x0EB80840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "cond" }) } }, rhs: LitBits([true, true, true, false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"cond\" }) } }, rhs: LitBits([true, true, true, false]) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_iv_a1_a_invalid_2_840_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A invalid encoding: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "cond" }) } }, rhs: LitBits([true, true, true, false]) }
    // ISET: A32
    // Fields: Vm=0, size=0, cond=0, Vd=0, M=0, opc2=0, D=0, op=0
    let encoding: u32 = 0x0EB80840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVT_iv_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_iv_a1_a_invalid_3_840_0eb80840() {
    // Encoding: 0x0EB80840
    // Test aarch32_VCVT_iv_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: opc2=0, cond=0, D=0, Vd=0, size=0, Vm=0, op=0, M=0
    let encoding: u32 = 0x0EB80840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_d_0_min_840_eeb80840() {
    // Thumb encoding (32): 0xEEB80840
    // Test aarch32_VCVT_iv_T1_A field D = 0 (Min)
    // ISET: T32
    // Fields: D=0, opc2=0, Vd=0, M=0, size=0, op=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_d_1_max_840_eef80840() {
    // Thumb encoding (32): 0xEEF80840
    // Test aarch32_VCVT_iv_T1_A field D = 1 (Max)
    // ISET: T32
    // Fields: D=1, Vd=0, opc2=0, size=0, op=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEF80840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field opc2 16 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_opc2_0_min_840_eeb80840() {
    // Thumb encoding (32): 0xEEB80840
    // Test aarch32_VCVT_iv_T1_A field opc2 = 0 (Min)
    // ISET: T32
    // Fields: opc2=0, M=0, Vm=0, op=0, D=0, Vd=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field opc2 16 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_opc2_1_poweroftwo_840_eeb90840() {
    // Thumb encoding (32): 0xEEB90840
    // Test aarch32_VCVT_iv_T1_A field opc2 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=0, size=0, op=0, D=0, opc2=1, Vd=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB90840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field opc2 16 +: 3`
/// Requirement: FieldBoundary { field: "opc2", value: 7, boundary: Max }
/// maximum value (7)
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_opc2_7_max_840_eebf0840() {
    // Thumb encoding (32): 0xEEBF0840
    // Test aarch32_VCVT_iv_T1_A field opc2 = 7 (Max)
    // ISET: T32
    // Fields: D=0, op=0, Vd=0, opc2=7, size=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBF0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_vd_0_min_840_eeb80840() {
    // Thumb encoding (32): 0xEEB80840
    // Test aarch32_VCVT_iv_T1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: opc2=0, op=0, Vm=0, size=0, D=0, Vd=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_vd_1_poweroftwo_840_eeb81840() {
    // Thumb encoding (32): 0xEEB81840
    // Test aarch32_VCVT_iv_T1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=0, D=0, Vd=1, M=0, op=0, size=0, opc2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB81840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_size_0_min_840_eeb80840() {
    // Thumb encoding (32): 0xEEB80840
    // Test aarch32_VCVT_iv_T1_A field size = 0 (Min)
    // ISET: T32
    // Fields: D=0, opc2=0, M=0, Vd=0, size=0, op=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_size_1_poweroftwo_840_eeb80940() {
    // Thumb encoding (32): 0xEEB80940
    // Test aarch32_VCVT_iv_T1_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: opc2=0, size=1, Vd=0, D=0, op=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80940;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_size_2_poweroftwo_840_eeb80a40() {
    // Thumb encoding (32): 0xEEB80A40
    // Test aarch32_VCVT_iv_T1_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: op=0, D=0, Vm=0, opc2=0, size=2, Vd=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_size_3_max_840_eeb80b40() {
    // Thumb encoding (32): 0xEEB80B40
    // Test aarch32_VCVT_iv_T1_A field size = 3 (Max)
    // ISET: T32
    // Fields: M=0, size=3, Vm=0, Vd=0, opc2=0, op=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field op 7 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_op_0_min_840_eeb80840() {
    // Thumb encoding (32): 0xEEB80840
    // Test aarch32_VCVT_iv_T1_A field op = 0 (Min)
    // ISET: T32
    // Fields: D=0, op=0, size=0, opc2=0, Vd=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field op 7 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_op_1_max_840_eeb808c0() {
    // Thumb encoding (32): 0xEEB808C0
    // Test aarch32_VCVT_iv_T1_A field op = 1 (Max)
    // ISET: T32
    // Fields: D=0, opc2=0, op=1, M=0, Vd=0, size=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB808C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_m_0_min_840_eeb80840() {
    // Thumb encoding (32): 0xEEB80840
    // Test aarch32_VCVT_iv_T1_A field M = 0 (Min)
    // ISET: T32
    // Fields: size=0, D=0, M=0, Vm=0, opc2=0, op=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_m_1_max_840_eeb80860() {
    // Thumb encoding (32): 0xEEB80860
    // Test aarch32_VCVT_iv_T1_A field M = 1 (Max)
    // ISET: T32
    // Fields: Vm=0, op=0, Vd=0, size=0, D=0, opc2=0, M=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80860;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_vm_0_min_840_eeb80840() {
    // Thumb encoding (32): 0xEEB80840
    // Test aarch32_VCVT_iv_T1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: op=0, size=0, opc2=0, D=0, Vd=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_iv_t1_a_field_vm_1_poweroftwo_840_eeb80841() {
    // Thumb encoding (32): 0xEEB80841
    // Test aarch32_VCVT_iv_T1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=1, opc2=0, D=0, Vd=0, M=0, op=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80841;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcvt_iv_t1_a_combo_0_840_eeb80840() {
    // Thumb encoding (32): 0xEEB80840
    // Test aarch32_VCVT_iv_T1_A field combination: D=0, opc2=0, Vd=0, size=0, op=0, M=0, Vm=0
    // ISET: T32
    // Fields: D=0, Vm=0, size=0, op=0, M=0, opc2=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_iv_t1_a_special_size_0_size_variant_0_2112_eeb80840() {
    // Thumb encoding (32): 0xEEB80840
    // Test aarch32_VCVT_iv_T1_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: opc2=0, op=0, Vm=0, Vd=0, D=0, size=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_iv_t1_a_special_size_1_size_variant_1_2112_eeb80940() {
    // Thumb encoding (32): 0xEEB80940
    // Test aarch32_VCVT_iv_T1_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: size=1, Vd=0, M=0, D=0, opc2=0, Vm=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80940;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvt_iv_t1_a_special_size_2_size_variant_2_2112_eeb80a40() {
    // Thumb encoding (32): 0xEEB80A40
    // Test aarch32_VCVT_iv_T1_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: Vd=0, D=0, Vm=0, size=2, opc2=0, op=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvt_iv_t1_a_special_size_3_size_variant_3_2112_eeb80b40() {
    // Thumb encoding (32): 0xEEB80B40
    // Test aarch32_VCVT_iv_T1_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: size=3, Vm=0, Vd=0, M=0, op=0, opc2=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_iv_t1_a_invalid_0_840_eeb80840() {
    // Thumb encoding (32): 0xEEB80840
    // Test aarch32_VCVT_iv_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: T32
    // Fields: op=0, M=0, Vd=0, Vm=0, size=0, D=0, opc2=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_iv_t1_a_invalid_1_840_eeb80840() {
    // Thumb encoding (32): 0xEEB80840
    // Test aarch32_VCVT_iv_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: Vd=0, Vm=0, op=0, D=0, opc2=0, size=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_iv_t1_a_invalid_2_840_eeb80840() {
    // Thumb encoding (32): 0xEEB80840
    // Test aarch32_VCVT_iv_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: op=0, Vd=0, Vm=0, opc2=0, size=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVT_iv_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_iv_t1_a_invalid_3_840_eeb80840() {
    // Thumb encoding (32): 0xEEB80840
    // Test aarch32_VCVT_iv_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: size=0, Vm=0, opc2=0, op=0, Vd=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB80840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_VCVTA_vfp_A Tests
// ============================================================================

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_d_0_min_840_febc0840() {
    // Encoding: 0xFEBC0840
    // Test aarch32_VCVTA_vfp_A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, RM=0, op=0, Vd=0, size=0, M=0, D=0
    let encoding: u32 = 0xFEBC0840;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_d_1_max_840_fefc0840() {
    // Encoding: 0xFEFC0840
    // Test aarch32_VCVTA_vfp_A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: RM=0, D=1, size=0, Vd=0, op=0, M=0, Vm=0
    let encoding: u32 = 0xFEFC0840;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field RM 16 +: 2`
/// Requirement: FieldBoundary { field: "RM", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_rm_0_min_840_febc0840() {
    // Encoding: 0xFEBC0840
    // Test aarch32_VCVTA_vfp_A1_A field RM = 0 (Min)
    // ISET: A32
    // Fields: RM=0, D=0, Vd=0, op=0, M=0, Vm=0, size=0
    let encoding: u32 = 0xFEBC0840;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field RM 16 +: 2`
/// Requirement: FieldBoundary { field: "RM", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_rm_1_poweroftwo_840_febd0840() {
    // Encoding: 0xFEBD0840
    // Test aarch32_VCVTA_vfp_A1_A field RM = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=0, RM=1, D=0, size=0, M=0, Vd=0, op=0
    let encoding: u32 = 0xFEBD0840;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field RM 16 +: 2`
/// Requirement: FieldBoundary { field: "RM", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_rm_3_max_840_febf0840() {
    // Encoding: 0xFEBF0840
    // Test aarch32_VCVTA_vfp_A1_A field RM = 3 (Max)
    // ISET: A32
    // Fields: D=0, size=0, op=0, Vm=0, Vd=0, M=0, RM=3
    let encoding: u32 = 0xFEBF0840;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_vd_0_min_840_febc0840() {
    // Encoding: 0xFEBC0840
    // Test aarch32_VCVTA_vfp_A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vd=0, op=0, Vm=0, RM=0, size=0, M=0
    let encoding: u32 = 0xFEBC0840;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_vd_1_poweroftwo_840_febc1840() {
    // Encoding: 0xFEBC1840
    // Test aarch32_VCVTA_vfp_A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=1, D=0, size=0, RM=0, M=0, Vm=0, op=0
    let encoding: u32 = 0xFEBC1840;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_size_0_min_840_febc0840() {
    // Encoding: 0xFEBC0840
    // Test aarch32_VCVTA_vfp_A1_A field size = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, M=0, Vd=0, RM=0, D=0, size=0, op=0
    let encoding: u32 = 0xFEBC0840;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_size_1_poweroftwo_840_febc0940() {
    // Encoding: 0xFEBC0940
    // Test aarch32_VCVTA_vfp_A1_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, Vm=0, D=0, op=0, RM=0, Vd=0, size=1
    let encoding: u32 = 0xFEBC0940;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_size_2_poweroftwo_840_febc0a40() {
    // Encoding: 0xFEBC0A40
    // Test aarch32_VCVTA_vfp_A1_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: RM=0, M=0, Vd=0, Vm=0, size=2, op=0, D=0
    let encoding: u32 = 0xFEBC0A40;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_size_3_max_840_febc0b40() {
    // Encoding: 0xFEBC0B40
    // Test aarch32_VCVTA_vfp_A1_A field size = 3 (Max)
    // ISET: A32
    // Fields: Vm=0, Vd=0, size=3, D=0, op=0, RM=0, M=0
    let encoding: u32 = 0xFEBC0B40;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field op 7 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_op_0_min_840_febc0840() {
    // Encoding: 0xFEBC0840
    // Test aarch32_VCVTA_vfp_A1_A field op = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, D=0, op=0, M=0, RM=0, size=0, Vm=0
    let encoding: u32 = 0xFEBC0840;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field op 7 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_op_1_max_840_febc08c0() {
    // Encoding: 0xFEBC08C0
    // Test aarch32_VCVTA_vfp_A1_A field op = 1 (Max)
    // ISET: A32
    // Fields: M=0, Vm=0, RM=0, Vd=0, D=0, size=0, op=1
    let encoding: u32 = 0xFEBC08C0;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_m_0_min_840_febc0840() {
    // Encoding: 0xFEBC0840
    // Test aarch32_VCVTA_vfp_A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: M=0, Vm=0, RM=0, size=0, D=0, Vd=0, op=0
    let encoding: u32 = 0xFEBC0840;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_m_1_max_840_febc0860() {
    // Encoding: 0xFEBC0860
    // Test aarch32_VCVTA_vfp_A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: size=0, RM=0, D=0, op=0, M=1, Vm=0, Vd=0
    let encoding: u32 = 0xFEBC0860;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_vm_0_min_840_febc0840() {
    // Encoding: 0xFEBC0840
    // Test aarch32_VCVTA_vfp_A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: size=0, M=0, Vm=0, D=0, RM=0, Vd=0, op=0
    let encoding: u32 = 0xFEBC0840;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvta_vfp_a1_a_field_vm_1_poweroftwo_840_febc0841() {
    // Encoding: 0xFEBC0841
    // Test aarch32_VCVTA_vfp_A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, D=0, RM=0, M=0, op=0, Vd=0, Vm=1
    let encoding: u32 = 0xFEBC0841;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcvta_vfp_a1_a_combo_0_840_febc0840() {
    // Encoding: 0xFEBC0840
    // Test aarch32_VCVTA_vfp_A1_A field combination: D=0, RM=0, Vd=0, size=0, op=0, M=0, Vm=0
    // ISET: A32
    // Fields: D=0, RM=0, Vd=0, M=0, Vm=0, op=0, size=0
    let encoding: u32 = 0xFEBC0840;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvta_vfp_a1_a_special_size_0_size_variant_0_2112_febc0840() {
    // Encoding: 0xFEBC0840
    // Test aarch32_VCVTA_vfp_A1_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: M=0, D=0, Vm=0, RM=0, Vd=0, size=0, op=0
    let encoding: u32 = 0xFEBC0840;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvta_vfp_a1_a_special_size_1_size_variant_1_2112_febc0940() {
    // Encoding: 0xFEBC0940
    // Test aarch32_VCVTA_vfp_A1_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: RM=0, Vm=0, op=0, D=0, Vd=0, M=0, size=1
    let encoding: u32 = 0xFEBC0940;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvta_vfp_a1_a_special_size_2_size_variant_2_2112_febc0a40() {
    // Encoding: 0xFEBC0A40
    // Test aarch32_VCVTA_vfp_A1_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: Vd=0, size=2, D=0, op=0, M=0, Vm=0, RM=0
    let encoding: u32 = 0xFEBC0A40;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvta_vfp_a1_a_special_size_3_size_variant_3_2112_febc0b40() {
    // Encoding: 0xFEBC0B40
    // Test aarch32_VCVTA_vfp_A1_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: RM=0, D=0, Vd=0, size=3, op=0, M=0, Vm=0
    let encoding: u32 = 0xFEBC0B40;
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

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvta_vfp_a1_a_invalid_0_840_febc0840() {
    // Encoding: 0xFEBC0840
    // Test aarch32_VCVTA_vfp_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: A32
    // Fields: D=0, M=0, RM=0, Vm=0, size=0, Vd=0, op=0
    let encoding: u32 = 0xFEBC0840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVTA_vfp_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvta_vfp_a1_a_invalid_1_840_febc0840() {
    // Encoding: 0xFEBC0840
    // Test aarch32_VCVTA_vfp_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: op=0, Vd=0, Vm=0, RM=0, size=0, M=0, D=0
    let encoding: u32 = 0xFEBC0840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_d_0_min_840_febc0840() {
    // Thumb encoding (32): 0xFEBC0840
    // Test aarch32_VCVTA_vfp_T1_A field D = 0 (Min)
    // ISET: T32
    // Fields: D=0, Vd=0, RM=0, size=0, op=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_d_1_max_840_fefc0840() {
    // Thumb encoding (32): 0xFEFC0840
    // Test aarch32_VCVTA_vfp_T1_A field D = 1 (Max)
    // ISET: T32
    // Fields: RM=0, Vm=0, op=0, size=0, D=1, Vd=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEFC0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field RM 16 +: 2`
/// Requirement: FieldBoundary { field: "RM", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_rm_0_min_840_febc0840() {
    // Thumb encoding (32): 0xFEBC0840
    // Test aarch32_VCVTA_vfp_T1_A field RM = 0 (Min)
    // ISET: T32
    // Fields: M=0, RM=0, size=0, op=0, D=0, Vd=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field RM 16 +: 2`
/// Requirement: FieldBoundary { field: "RM", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_rm_1_poweroftwo_840_febd0840() {
    // Thumb encoding (32): 0xFEBD0840
    // Test aarch32_VCVTA_vfp_T1_A field RM = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: op=0, RM=1, Vm=0, M=0, D=0, Vd=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBD0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field RM 16 +: 2`
/// Requirement: FieldBoundary { field: "RM", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_rm_3_max_840_febf0840() {
    // Thumb encoding (32): 0xFEBF0840
    // Test aarch32_VCVTA_vfp_T1_A field RM = 3 (Max)
    // ISET: T32
    // Fields: Vd=0, op=0, RM=3, M=0, size=0, Vm=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBF0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_vd_0_min_840_febc0840() {
    // Thumb encoding (32): 0xFEBC0840
    // Test aarch32_VCVTA_vfp_T1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: RM=0, Vd=0, size=0, D=0, op=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_vd_1_poweroftwo_840_febc1840() {
    // Thumb encoding (32): 0xFEBC1840
    // Test aarch32_VCVTA_vfp_T1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: M=0, RM=0, op=0, Vm=0, D=0, Vd=1, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC1840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_size_0_min_840_febc0840() {
    // Thumb encoding (32): 0xFEBC0840
    // Test aarch32_VCVTA_vfp_T1_A field size = 0 (Min)
    // ISET: T32
    // Fields: M=0, size=0, Vm=0, D=0, RM=0, Vd=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_size_1_poweroftwo_840_febc0940() {
    // Thumb encoding (32): 0xFEBC0940
    // Test aarch32_VCVTA_vfp_T1_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: size=1, op=0, M=0, Vm=0, D=0, RM=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0940;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_size_2_poweroftwo_840_febc0a40() {
    // Thumb encoding (32): 0xFEBC0A40
    // Test aarch32_VCVTA_vfp_T1_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: RM=0, Vd=0, size=2, M=0, D=0, op=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_size_3_max_840_febc0b40() {
    // Thumb encoding (32): 0xFEBC0B40
    // Test aarch32_VCVTA_vfp_T1_A field size = 3 (Max)
    // ISET: T32
    // Fields: size=3, op=0, Vd=0, D=0, RM=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field op 7 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_op_0_min_840_febc0840() {
    // Thumb encoding (32): 0xFEBC0840
    // Test aarch32_VCVTA_vfp_T1_A field op = 0 (Min)
    // ISET: T32
    // Fields: RM=0, op=0, M=0, Vm=0, Vd=0, size=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field op 7 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_op_1_max_840_febc08c0() {
    // Thumb encoding (32): 0xFEBC08C0
    // Test aarch32_VCVTA_vfp_T1_A field op = 1 (Max)
    // ISET: T32
    // Fields: Vm=0, D=0, M=0, size=0, RM=0, Vd=0, op=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC08C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_m_0_min_840_febc0840() {
    // Thumb encoding (32): 0xFEBC0840
    // Test aarch32_VCVTA_vfp_T1_A field M = 0 (Min)
    // ISET: T32
    // Fields: RM=0, M=0, Vd=0, size=0, Vm=0, D=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_m_1_max_840_febc0860() {
    // Thumb encoding (32): 0xFEBC0860
    // Test aarch32_VCVTA_vfp_T1_A field M = 1 (Max)
    // ISET: T32
    // Fields: RM=0, Vm=0, D=0, size=0, Vd=0, op=0, M=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0860;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_vm_0_min_840_febc0840() {
    // Thumb encoding (32): 0xFEBC0840
    // Test aarch32_VCVTA_vfp_T1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: RM=0, size=0, op=0, D=0, Vd=0, Vm=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvta_vfp_t1_a_field_vm_1_poweroftwo_840_febc0841() {
    // Thumb encoding (32): 0xFEBC0841
    // Test aarch32_VCVTA_vfp_T1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: M=0, D=0, size=0, op=0, Vd=0, Vm=1, RM=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0841;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcvta_vfp_t1_a_combo_0_840_febc0840() {
    // Thumb encoding (32): 0xFEBC0840
    // Test aarch32_VCVTA_vfp_T1_A field combination: D=0, RM=0, Vd=0, size=0, op=0, M=0, Vm=0
    // ISET: T32
    // Fields: op=0, Vm=0, M=0, D=0, Vd=0, RM=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvta_vfp_t1_a_special_size_0_size_variant_0_2112_febc0840() {
    // Thumb encoding (32): 0xFEBC0840
    // Test aarch32_VCVTA_vfp_T1_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: D=0, RM=0, Vm=0, op=0, Vd=0, M=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvta_vfp_t1_a_special_size_1_size_variant_1_2112_febc0940() {
    // Thumb encoding (32): 0xFEBC0940
    // Test aarch32_VCVTA_vfp_T1_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: Vm=0, Vd=0, op=0, RM=0, D=0, size=1, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0940;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvta_vfp_t1_a_special_size_2_size_variant_2_2112_febc0a40() {
    // Thumb encoding (32): 0xFEBC0A40
    // Test aarch32_VCVTA_vfp_T1_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: D=0, M=0, Vd=0, RM=0, op=0, Vm=0, size=2
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvta_vfp_t1_a_special_size_3_size_variant_3_2112_febc0b40() {
    // Thumb encoding (32): 0xFEBC0B40
    // Test aarch32_VCVTA_vfp_T1_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: RM=0, Vm=0, op=0, M=0, D=0, Vd=0, size=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }`
/// Requirement: UndefinedEncoding { condition: "Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvta_vfp_t1_a_invalid_0_840_febc0840() {
    // Thumb encoding (32): 0xFEBC0840
    // Test aarch32_VCVTA_vfp_T1_A invalid encoding: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }
    // ISET: T32
    // Fields: RM=0, op=0, M=0, Vm=0, Vd=0, size=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvta_vfp_t1_a_invalid_1_840_febc0840() {
    // Thumb encoding (32): 0xFEBC0840
    // Test aarch32_VCVTA_vfp_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: M=0, Vm=0, op=0, size=0, Vd=0, D=0, RM=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvta_vfp_t1_a_invalid_2_840_febc0840() {
    // Thumb encoding (32): 0xFEBC0840
    // Test aarch32_VCVTA_vfp_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: T32
    // Fields: M=0, Vm=0, Vd=0, size=0, RM=0, D=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVTA_vfp_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvta_vfp_t1_a_invalid_3_840_febc0840() {
    // Thumb encoding (32): 0xFEBC0840
    // Test aarch32_VCVTA_vfp_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: D=0, size=0, op=0, M=0, Vm=0, Vd=0, RM=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFEBC0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

// ============================================================================
// aarch32_VCVTA_asimd_A Tests
// ============================================================================

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_d_0_min_200_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: op=0, RM=0, D=0, Vd=0, M=0, size=0, Q=0, Vm=0
    let encoding: u32 = 0xF3B30000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_d_1_max_200_f3f30000() {
    // Encoding: 0xF3F30000
    // Test aarch32_VCVTA_asimd_A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: D=1, op=0, Vd=0, Q=0, RM=0, M=0, size=0, Vm=0
    let encoding: u32 = 0xF3F30000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_size_0_min_200_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A field size = 0 (Min)
    // ISET: A32
    // Fields: op=0, Vd=0, M=0, Q=0, Vm=0, RM=0, size=0, D=0
    let encoding: u32 = 0xF3B30000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_size_1_poweroftwo_200_f3b70000() {
    // Encoding: 0xF3B70000
    // Test aarch32_VCVTA_asimd_A1_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=1, op=0, M=0, D=0, Q=0, Vm=0, Vd=0, RM=0
    let encoding: u32 = 0xF3B70000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_size_2_poweroftwo_200_f3bb0000() {
    // Encoding: 0xF3BB0000
    // Test aarch32_VCVTA_asimd_A1_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: op=0, Vd=0, Q=0, size=2, D=0, RM=0, M=0, Vm=0
    let encoding: u32 = 0xF3BB0000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_size_3_max_200_f3bf0000() {
    // Encoding: 0xF3BF0000
    // Test aarch32_VCVTA_asimd_A1_A field size = 3 (Max)
    // ISET: A32
    // Fields: RM=0, Vd=0, M=0, D=0, op=0, size=3, Vm=0, Q=0
    let encoding: u32 = 0xF3BF0000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_vd_0_min_200_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, M=0, RM=0, Q=0, Vm=0, op=0, size=0, D=0
    let encoding: u32 = 0xF3B30000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_vd_1_poweroftwo_200_f3b31000() {
    // Encoding: 0xF3B31000
    // Test aarch32_VCVTA_asimd_A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: op=0, D=0, Vm=0, M=0, Vd=1, RM=0, size=0, Q=0
    let encoding: u32 = 0xF3B31000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field RM 8 +: 2`
/// Requirement: FieldBoundary { field: "RM", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_rm_0_min_200_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A field RM = 0 (Min)
    // ISET: A32
    // Fields: D=0, M=0, Vm=0, RM=0, Vd=0, op=0, Q=0, size=0
    let encoding: u32 = 0xF3B30000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field RM 8 +: 2`
/// Requirement: FieldBoundary { field: "RM", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_rm_1_poweroftwo_200_f3b30100() {
    // Encoding: 0xF3B30100
    // Test aarch32_VCVTA_asimd_A1_A field RM = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, op=0, M=0, D=0, Vm=0, Q=0, RM=1, Vd=0
    let encoding: u32 = 0xF3B30100;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field RM 8 +: 2`
/// Requirement: FieldBoundary { field: "RM", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_rm_3_max_200_f3b30300() {
    // Encoding: 0xF3B30300
    // Test aarch32_VCVTA_asimd_A1_A field RM = 3 (Max)
    // ISET: A32
    // Fields: op=0, size=0, M=0, Vm=0, RM=3, Vd=0, D=0, Q=0
    let encoding: u32 = 0xF3B30300;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field op 7 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_op_0_min_200_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A field op = 0 (Min)
    // ISET: A32
    // Fields: size=0, Vm=0, RM=0, Vd=0, op=0, M=0, D=0, Q=0
    let encoding: u32 = 0xF3B30000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field op 7 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_op_1_max_200_f3b30080() {
    // Encoding: 0xF3B30080
    // Test aarch32_VCVTA_asimd_A1_A field op = 1 (Max)
    // ISET: A32
    // Fields: size=0, Q=0, M=0, op=1, Vm=0, Vd=0, RM=0, D=0
    let encoding: u32 = 0xF3B30080;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_q_0_min_200_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A field Q = 0 (Min)
    // ISET: A32
    // Fields: Q=0, Vd=0, RM=0, size=0, D=0, M=0, op=0, Vm=0
    let encoding: u32 = 0xF3B30000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_q_1_max_200_f3b30040() {
    // Encoding: 0xF3B30040
    // Test aarch32_VCVTA_asimd_A1_A field Q = 1 (Max)
    // ISET: A32
    // Fields: M=0, D=0, size=0, Vm=0, Vd=0, op=0, RM=0, Q=1
    let encoding: u32 = 0xF3B30040;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_m_0_min_200_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, Q=0, size=0, Vm=0, M=0, D=0, RM=0, op=0
    let encoding: u32 = 0xF3B30000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_m_1_max_200_f3b30020() {
    // Encoding: 0xF3B30020
    // Test aarch32_VCVTA_asimd_A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: M=1, D=0, op=0, Vd=0, Q=0, size=0, Vm=0, RM=0
    let encoding: u32 = 0xF3B30020;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_vm_0_min_200_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: Q=0, size=0, op=0, Vm=0, D=0, M=0, Vd=0, RM=0
    let encoding: u32 = 0xF3B30000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvta_asimd_a1_a_field_vm_1_poweroftwo_200_f3b30001() {
    // Encoding: 0xF3B30001
    // Test aarch32_VCVTA_asimd_A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Q=0, op=0, RM=0, Vd=0, Vm=1, D=0, M=0, size=0
    let encoding: u32 = 0xF3B30001;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_combo_0_200_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A field combination: D=0, size=0, Vd=0, RM=0, op=0, Q=0, M=0, Vm=0
    // ISET: A32
    // Fields: size=0, Q=0, RM=0, Vd=0, op=0, D=0, Vm=0, M=0
    let encoding: u32 = 0xF3B30000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvta_asimd_a1_a_special_size_0_size_variant_0_512_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: D=0, op=0, RM=0, Q=0, Vd=0, size=0, M=0, Vm=0
    let encoding: u32 = 0xF3B30000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvta_asimd_a1_a_special_size_1_size_variant_1_512_f3b70000() {
    // Encoding: 0xF3B70000
    // Test aarch32_VCVTA_asimd_A1_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: op=0, M=0, RM=0, D=0, Q=0, Vd=0, Vm=0, size=1
    let encoding: u32 = 0xF3B70000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvta_asimd_a1_a_special_size_2_size_variant_2_512_f3bb0000() {
    // Encoding: 0xF3BB0000
    // Test aarch32_VCVTA_asimd_A1_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: op=0, Q=0, Vm=0, M=0, D=0, Vd=0, RM=0, size=2
    let encoding: u32 = 0xF3BB0000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvta_asimd_a1_a_special_size_3_size_variant_3_512_f3bf0000() {
    // Encoding: 0xF3BF0000
    // Test aarch32_VCVTA_asimd_A1_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: D=0, RM=0, Q=0, M=0, Vm=0, op=0, Vd=0, size=3
    let encoding: u32 = 0xF3BF0000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvta_asimd_a1_a_special_q_0_size_variant_0_512_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A special value Q = 0 (Size variant 0)
    // ISET: A32
    // Fields: Q=0, RM=0, Vd=0, Vm=0, op=0, M=0, D=0, size=0
    let encoding: u32 = 0xF3B30000;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvta_asimd_a1_a_special_q_1_size_variant_1_512_f3b30040() {
    // Encoding: 0xF3B30040
    // Test aarch32_VCVTA_asimd_A1_A special value Q = 1 (Size variant 1)
    // ISET: A32
    // Fields: size=0, Vm=0, RM=0, Vd=0, Q=1, op=0, M=0, D=0
    let encoding: u32 = 0xF3B30040;
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

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvta_asimd_a1_a_invalid_0_200_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: A32
    // Fields: Vd=0, size=0, Vm=0, Q=0, RM=0, op=0, D=0, M=0
    let encoding: u32 = 0xF3B30000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvta_asimd_a1_a_invalid_1_200_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: op=0, Vd=0, D=0, RM=0, M=0, Q=0, Vm=0, size=0
    let encoding: u32 = 0xF3B30000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `Binary { op: Or, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }, rhs: InSet { expr: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), elements: [Single(LitBits([false, false])), Single(LitBits([true, true]))] } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Or, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } }, rhs: InSet { expr: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), elements: [Single(LitBits([false, false])), Single(LitBits([true, true]))] } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvta_asimd_a1_a_invalid_2_200_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A invalid encoding: Binary { op: Or, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }, rhs: InSet { expr: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), elements: [Single(LitBits([false, false])), Single(LitBits([true, true]))] } }
    // ISET: A32
    // Fields: M=0, Vm=0, D=0, size=0, op=0, Q=0, Vd=0, RM=0
    let encoding: u32 = 0xF3B30000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvta_asimd_a1_a_invalid_3_200_f3b30000() {
    // Encoding: 0xF3B30000
    // Test aarch32_VCVTA_asimd_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: Vm=0, Vd=0, M=0, size=0, D=0, op=0, RM=0, Q=0
    let encoding: u32 = 0xF3B30000;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_d_0_min_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A field D = 0 (Min)
    // ISET: T32
    // Fields: size=0, Q=0, op=0, Vd=0, M=0, Vm=0, RM=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_d_1_max_200_fff30000() {
    // Thumb encoding (32): 0xFFF30000
    // Test aarch32_VCVTA_asimd_T1_A field D = 1 (Max)
    // ISET: T32
    // Fields: D=1, size=0, RM=0, M=0, Vd=0, Vm=0, Q=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFF30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_size_0_min_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A field size = 0 (Min)
    // ISET: T32
    // Fields: D=0, size=0, Q=0, M=0, op=0, Vm=0, Vd=0, RM=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_size_1_poweroftwo_200_ffb70000() {
    // Thumb encoding (32): 0xFFB70000
    // Test aarch32_VCVTA_asimd_T1_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=0, RM=0, Q=0, op=0, size=1, M=0, D=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB70000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_size_2_poweroftwo_200_ffbb0000() {
    // Thumb encoding (32): 0xFFBB0000
    // Test aarch32_VCVTA_asimd_T1_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: size=2, op=0, Vm=0, M=0, D=0, Q=0, Vd=0, RM=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFBB0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_size_3_max_200_ffbf0000() {
    // Thumb encoding (32): 0xFFBF0000
    // Test aarch32_VCVTA_asimd_T1_A field size = 3 (Max)
    // ISET: T32
    // Fields: RM=0, op=0, D=0, Vd=0, M=0, Q=0, size=3, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFBF0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_vd_0_min_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, op=0, RM=0, Q=0, Vm=0, size=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_vd_1_poweroftwo_200_ffb31000() {
    // Thumb encoding (32): 0xFFB31000
    // Test aarch32_VCVTA_asimd_T1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: op=0, Q=0, D=0, Vm=0, Vd=1, M=0, size=0, RM=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB31000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field RM 8 +: 2`
/// Requirement: FieldBoundary { field: "RM", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_rm_0_min_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A field RM = 0 (Min)
    // ISET: T32
    // Fields: D=0, Vm=0, size=0, op=0, Q=0, Vd=0, RM=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field RM 8 +: 2`
/// Requirement: FieldBoundary { field: "RM", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_rm_1_poweroftwo_200_ffb30100() {
    // Thumb encoding (32): 0xFFB30100
    // Test aarch32_VCVTA_asimd_T1_A field RM = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: RM=1, D=0, M=0, Vm=0, size=0, Vd=0, op=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30100;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field RM 8 +: 2`
/// Requirement: FieldBoundary { field: "RM", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_rm_3_max_200_ffb30300() {
    // Thumb encoding (32): 0xFFB30300
    // Test aarch32_VCVTA_asimd_T1_A field RM = 3 (Max)
    // ISET: T32
    // Fields: RM=3, M=0, op=0, D=0, size=0, Vd=0, Vm=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30300;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field op 7 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_op_0_min_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A field op = 0 (Min)
    // ISET: T32
    // Fields: M=0, D=0, Q=0, op=0, Vd=0, Vm=0, RM=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field op 7 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_op_1_max_200_ffb30080() {
    // Thumb encoding (32): 0xFFB30080
    // Test aarch32_VCVTA_asimd_T1_A field op = 1 (Max)
    // ISET: T32
    // Fields: Vm=0, D=0, M=0, RM=0, size=0, Q=0, op=1, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30080;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_q_0_min_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A field Q = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, size=0, M=0, D=0, RM=0, op=0, Vm=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_q_1_max_200_ffb30040() {
    // Thumb encoding (32): 0xFFB30040
    // Test aarch32_VCVTA_asimd_T1_A field Q = 1 (Max)
    // ISET: T32
    // Fields: RM=0, D=0, op=0, Vm=0, M=0, Q=1, size=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_m_0_min_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A field M = 0 (Min)
    // ISET: T32
    // Fields: size=0, Vm=0, RM=0, D=0, Vd=0, M=0, op=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_m_1_max_200_ffb30020() {
    // Thumb encoding (32): 0xFFB30020
    // Test aarch32_VCVTA_asimd_T1_A field M = 1 (Max)
    // ISET: T32
    // Fields: RM=0, Vm=0, op=0, Q=0, Vd=0, D=0, size=0, M=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30020;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_vm_0_min_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, Q=0, RM=0, Vm=0, size=0, op=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvta_asimd_t1_a_field_vm_1_poweroftwo_200_ffb30001() {
    // Thumb encoding (32): 0xFFB30001
    // Test aarch32_VCVTA_asimd_T1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=1, RM=0, M=0, Vd=0, op=0, size=0, Q=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30001;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcvta_asimd_t1_a_combo_0_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A field combination: D=0, size=0, Vd=0, RM=0, op=0, Q=0, M=0, Vm=0
    // ISET: T32
    // Fields: size=0, M=0, op=0, D=0, RM=0, Q=0, Vd=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvta_asimd_t1_a_special_size_0_size_variant_0_512_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: Vd=0, RM=0, size=0, op=0, M=0, Vm=0, Q=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvta_asimd_t1_a_special_size_1_size_variant_1_512_ffb70000() {
    // Thumb encoding (32): 0xFFB70000
    // Test aarch32_VCVTA_asimd_T1_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: op=0, Vd=0, size=1, Q=0, RM=0, Vm=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB70000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvta_asimd_t1_a_special_size_2_size_variant_2_512_ffbb0000() {
    // Thumb encoding (32): 0xFFBB0000
    // Test aarch32_VCVTA_asimd_T1_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: RM=0, op=0, M=0, Vd=0, size=2, Q=0, D=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFBB0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvta_asimd_t1_a_special_size_3_size_variant_3_512_ffbf0000() {
    // Thumb encoding (32): 0xFFBF0000
    // Test aarch32_VCVTA_asimd_T1_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: Vm=0, op=0, D=0, RM=0, Vd=0, Q=0, size=3, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFBF0000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvta_asimd_t1_a_special_q_0_size_variant_0_512_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A special value Q = 0 (Size variant 0)
    // ISET: T32
    // Fields: RM=0, Q=0, Vd=0, D=0, size=0, op=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvta_asimd_t1_a_special_q_1_size_variant_1_512_ffb30040() {
    // Thumb encoding (32): 0xFFB30040
    // Test aarch32_VCVTA_asimd_T1_A special value Q = 1 (Size variant 1)
    // ISET: T32
    // Fields: RM=0, D=0, size=0, Vd=0, op=0, M=0, Vm=0, Q=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30040;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }`
/// Requirement: UndefinedEncoding { condition: "Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvta_asimd_t1_a_invalid_0_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A invalid encoding: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] }
    // ISET: T32
    // Fields: size=0, Q=0, Vd=0, op=0, RM=0, M=0, D=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvta_asimd_t1_a_invalid_1_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: RM=0, size=0, M=0, Vm=0, D=0, Q=0, op=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvta_asimd_t1_a_invalid_2_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: T32
    // Fields: M=0, size=0, RM=0, Vd=0, Q=0, op=0, Vm=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvta_asimd_t1_a_invalid_3_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: RM=0, size=0, op=0, Vd=0, Q=0, M=0, Vm=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `Binary { op: Or, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }, rhs: InSet { expr: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), elements: [Single(LitBits([false, false])), Single(LitBits([true, true]))] } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Or, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } }, rhs: InSet { expr: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), elements: [Single(LitBits([false, false])), Single(LitBits([true, true]))] } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvta_asimd_t1_a_invalid_4_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A invalid encoding: Binary { op: Or, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }, rhs: InSet { expr: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), elements: [Single(LitBits([false, false])), Single(LitBits([true, true]))] } }
    // ISET: T32
    // Fields: Vm=0, D=0, size=0, Vd=0, M=0, Q=0, op=0, RM=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVTA_asimd_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvta_asimd_t1_a_invalid_5_200_ffb30000() {
    // Thumb encoding (32): 0xFFB30000
    // Test aarch32_VCVTA_asimd_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: size=0, op=0, D=0, M=0, Vm=0, RM=0, Vd=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOVZ X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// lower 16 bits (32)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_movz_oracle_32_0_f3b34680() {
    // Test MOVZ 32-bit: lower 16 bits (oracle)
    // Encoding: 0xF3B34680
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B34680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1234, "W0 should be 0x00001234");
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOVZ X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// lower 16 bits (64)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_movz_oracle_64_0_f3b34680() {
    // Test MOVZ 64-bit: lower 16 bits (oracle)
    // Encoding: 0xF3B34680
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B34680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1234, "X0 should be 0x0000000000001234");
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOVZ X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shifted 16 bits (32)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_movz_oracle_32_1_f3b77ba0() {
    // Test MOVZ 32-bit: shifted 16 bits (oracle)
    // Encoding: 0xF3B77BA0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B77BA0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xABCD0000, "W0 should be 0xABCD0000");
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOVZ X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 16 bits (64)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_movz_oracle_64_1_f3b77ba0() {
    // Test MOVZ 64-bit: shifted 16 bits (oracle)
    // Encoding: 0xF3B77BA0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B77BA0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xABCD0000,
        "X0 should be 0x00000000ABCD0000"
    );
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOVZ X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm16 (32)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_movz_oracle_32_2_f3bfffe0() {
    // Test MOVZ 32-bit: max imm16 (oracle)
    // Encoding: 0xF3BFFFE0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3BFFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "W0 should be 0x0000FFFF");
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOVZ X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm16 (64)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_movz_oracle_64_2_f3bfffe0() {
    // Test MOVZ 64-bit: max imm16 (oracle)
    // Encoding: 0xF3BFFFE0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3BFFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "X0 should be 0x000000000000FFFF");
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOVZ X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero imm16 (32)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_movz_oracle_32_3_f3b30200() {
    // Test MOVZ 32-bit: zero imm16 (oracle)
    // Encoding: 0xF3B30200
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B30200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOVZ X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero imm16 (64)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_movz_oracle_64_3_f3b30200() {
    // Test MOVZ 64-bit: zero imm16 (oracle)
    // Encoding: 0xF3B30200
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B30200;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOVZ X0, #0x5678, LSL #32`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 32 bits (64)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_movz_oracle_64_4_f3fbcf00() {
    // Test MOVZ 64-bit: shifted 32 bits (oracle)
    // Encoding: 0xF3FBCF00
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3FBCF00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000567800000000");
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOVZ X0, #0xDEAD, LSL #48`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 48 bits (64)
#[test]
fn test_aarch32_vcvta_asimd_a1_a_movz_oracle_64_5_f3fbd7a0() {
    // Test MOVZ 64-bit: shifted 48 bits (oracle)
    // Encoding: 0xF3FBD7A0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3FBD7A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xDEAD000000000000");
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOV R0, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate
#[test]
fn test_aarch32_vcvta_asimd_a1_a_a32_mov_imm_0_f3a0000a() {
    // Test A32 MOV: small immediate (oracle)
    // Encoding: 0xF3A0000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A0000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA, "R0 should be 0x0000000A");
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOV R0, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8
#[test]
fn test_aarch32_vcvta_asimd_a1_a_a32_mov_imm_1_f3a000ff() {
    // Test A32 MOV: max imm8 (oracle)
    // Encoding: 0xF3A000FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A000FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOV R0, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2
#[test]
fn test_aarch32_vcvta_asimd_a1_a_a32_mov_imm_2_f3a00180() {
    // Test A32 MOV: rotated by 2 (oracle)
    // Encoding: 0xF3A00180
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A00180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x20, "R0 should be 0x00000020");
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOV R0, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8
#[test]
fn test_aarch32_vcvta_asimd_a1_a_a32_mov_imm_3_f3a0040f() {
    // Test A32 MOV: rotated by 8 (oracle)
    // Encoding: 0xF3A0040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A0040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xF000000, "R0 should be 0x0F000000");
}

/// Provenance: aarch32_VCVTA_asimd_A1_A
/// ASL: `MOV R0, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate
#[test]
fn test_aarch32_vcvta_asimd_a1_a_a32_mov_imm_4_f3a00000() {
    // Test A32 MOV: zero immediate (oracle)
    // Encoding: 0xF3A00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

// ============================================================================
// aarch32_VCVT_xs_A Tests
// ============================================================================

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field U 24 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_u_0_min_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A field U = 0 (Min)
    // ISET: A32
    // Fields: imm6=0, Vd=0, Vm=0, U=0, Q=0, op=0, M=0, D=0
    let encoding: u32 = 0xF2800C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field U 24 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_u_1_max_c10_f3800c10() {
    // Encoding: 0xF3800C10
    // Test aarch32_VCVT_xs_A1_A field U = 1 (Max)
    // ISET: A32
    // Fields: imm6=0, Q=0, M=0, U=1, D=0, op=0, Vd=0, Vm=0
    let encoding: u32 = 0xF3800C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_d_0_min_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vm=0, Vd=0, Q=0, M=0, U=0, op=0, imm6=0
    let encoding: u32 = 0xF2800C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_d_1_max_c10_f2c00c10() {
    // Encoding: 0xF2C00C10
    // Test aarch32_VCVT_xs_A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: U=0, Q=0, Vm=0, Vd=0, op=0, M=0, imm6=0, D=1
    let encoding: u32 = 0xF2C00C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_imm6_0_zero_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A field imm6 = 0 (Zero)
    // ISET: A32
    // Fields: Q=0, op=0, imm6=0, Vd=0, M=0, D=0, Vm=0, U=0
    let encoding: u32 = 0xF2800C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_imm6_1_poweroftwo_c10_f2810c10() {
    // Encoding: 0xF2810C10
    // Test aarch32_VCVT_xs_A1_A field imm6 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vm=0, op=0, U=0, Vd=0, imm6=1, Q=0, M=0
    let encoding: u32 = 0xF2810C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_imm6_3_poweroftwominusone_c10_f2830c10() {
    // Encoding: 0xF2830C10
    // Test aarch32_VCVT_xs_A1_A field imm6 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: U=0, op=0, D=0, Q=0, imm6=3, Vd=0, M=0, Vm=0
    let encoding: u32 = 0xF2830C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_imm6_4_poweroftwo_c10_f2840c10() {
    // Encoding: 0xF2840C10
    // Test aarch32_VCVT_xs_A1_A field imm6 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=0, Vd=0, M=0, U=0, op=0, imm6=4, Q=0, D=0
    let encoding: u32 = 0xF2840C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_imm6_7_poweroftwominusone_c10_f2870c10() {
    // Encoding: 0xF2870C10
    // Test aarch32_VCVT_xs_A1_A field imm6 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: op=0, Vm=0, M=0, imm6=7, U=0, Q=0, D=0, Vd=0
    let encoding: u32 = 0xF2870C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_imm6_8_poweroftwo_c10_f2880c10() {
    // Encoding: 0xF2880C10
    // Test aarch32_VCVT_xs_A1_A field imm6 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: U=0, op=0, Q=0, D=0, M=0, Vm=0, imm6=8, Vd=0
    let encoding: u32 = 0xF2880C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_imm6_15_poweroftwominusone_c10_f28f0c10() {
    // Encoding: 0xF28F0C10
    // Test aarch32_VCVT_xs_A1_A field imm6 = 15 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: imm6=15, op=0, D=0, U=0, Q=0, M=0, Vm=0, Vd=0
    let encoding: u32 = 0xF28F0C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_imm6_16_poweroftwo_c10_f2900c10() {
    // Encoding: 0xF2900C10
    // Test aarch32_VCVT_xs_A1_A field imm6 = 16 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vm=0, Q=0, Vd=0, U=0, imm6=16, op=0, M=0
    let encoding: u32 = 0xF2900C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_imm6_31_poweroftwominusone_c10_f29f0c10() {
    // Encoding: 0xF29F0C10
    // Test aarch32_VCVT_xs_A1_A field imm6 = 31 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: U=0, M=0, Q=0, D=0, Vd=0, imm6=31, op=0, Vm=0
    let encoding: u32 = 0xF29F0C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_imm6_32_poweroftwo_c10_f2a00c10() {
    // Encoding: 0xF2A00C10
    // Test aarch32_VCVT_xs_A1_A field imm6 = 32 (PowerOfTwo)
    // ISET: A32
    // Fields: op=0, Vm=0, D=0, U=0, Vd=0, M=0, imm6=32, Q=0
    let encoding: u32 = 0xF2A00C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_imm6_63_max_c10_f2bf0c10() {
    // Encoding: 0xF2BF0C10
    // Test aarch32_VCVT_xs_A1_A field imm6 = 63 (Max)
    // ISET: A32
    // Fields: Q=0, D=0, Vm=0, imm6=63, U=0, M=0, Vd=0, op=0
    let encoding: u32 = 0xF2BF0C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_vd_0_min_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: imm6=0, U=0, D=0, Q=0, op=0, M=0, Vd=0, Vm=0
    let encoding: u32 = 0xF2800C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_vd_1_poweroftwo_c10_f2801c10() {
    // Encoding: 0xF2801C10
    // Test aarch32_VCVT_xs_A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, U=0, Vd=1, imm6=0, op=0, Q=0, Vm=0, M=0
    let encoding: u32 = 0xF2801C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field op 8 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_op_0_min_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A field op = 0 (Min)
    // ISET: A32
    // Fields: U=0, M=0, Vm=0, Q=0, op=0, Vd=0, D=0, imm6=0
    let encoding: u32 = 0xF2800C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field op 8 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_op_1_poweroftwo_c10_f2800d10() {
    // Encoding: 0xF2800D10
    // Test aarch32_VCVT_xs_A1_A field op = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Q=0, Vm=0, op=1, imm6=0, M=0, D=0, Vd=0, U=0
    let encoding: u32 = 0xF2800D10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field op 8 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_op_3_max_c10_f2800f10() {
    // Encoding: 0xF2800F10
    // Test aarch32_VCVT_xs_A1_A field op = 3 (Max)
    // ISET: A32
    // Fields: Q=0, Vm=0, op=3, Vd=0, D=0, imm6=0, U=0, M=0
    let encoding: u32 = 0xF2800F10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_q_0_min_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A field Q = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, U=0, Vm=0, M=0, imm6=0, D=0, op=0, Q=0
    let encoding: u32 = 0xF2800C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_q_1_max_c10_f2800c50() {
    // Encoding: 0xF2800C50
    // Test aarch32_VCVT_xs_A1_A field Q = 1 (Max)
    // ISET: A32
    // Fields: D=0, Vd=0, Q=1, M=0, op=0, imm6=0, U=0, Vm=0
    let encoding: u32 = 0xF2800C50;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_m_0_min_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: op=0, D=0, M=0, imm6=0, Vd=0, Q=0, U=0, Vm=0
    let encoding: u32 = 0xF2800C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_m_1_max_c10_f2800c30() {
    // Encoding: 0xF2800C30
    // Test aarch32_VCVT_xs_A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: U=0, Q=0, M=1, imm6=0, D=0, op=0, Vm=0, Vd=0
    let encoding: u32 = 0xF2800C30;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_vm_0_min_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: imm6=0, op=0, Q=0, M=0, U=0, D=0, Vd=0, Vm=0
    let encoding: u32 = 0xF2800C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_xs_a1_a_field_vm_1_poweroftwo_c10_f2800c11() {
    // Encoding: 0xF2800C11
    // Test aarch32_VCVT_xs_A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: op=0, M=0, Vm=1, U=0, D=0, imm6=0, Vd=0, Q=0
    let encoding: u32 = 0xF2800C11;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch32_vcvt_xs_a1_a_combo_0_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A field combination: U=0, D=0, imm6=0, Vd=0, op=0, Q=0, M=0, Vm=0
    // ISET: A32
    // Fields: M=0, imm6=0, Vd=0, D=0, op=0, Q=0, U=0, Vm=0
    let encoding: u32 = 0xF2800C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_xs_a1_a_special_q_0_size_variant_0_3088_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A special value Q = 0 (Size variant 0)
    // ISET: A32
    // Fields: D=0, Vd=0, Vm=0, Q=0, U=0, imm6=0, op=0, M=0
    let encoding: u32 = 0xF2800C10;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_xs_a1_a_special_q_1_size_variant_1_3088_f2800c50() {
    // Encoding: 0xF2800C50
    // Test aarch32_VCVT_xs_A1_A special value Q = 1 (Size variant 1)
    // ISET: A32
    // Fields: U=0, D=0, Vd=0, Q=1, op=0, Vm=0, M=0, imm6=0
    let encoding: u32 = 0xF2800C50;
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

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), indices: [Single(LitInt(1))] }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"op\" }), indices: [Single(LitInt(1))] }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_a1_a_invalid_0_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A invalid encoding: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), indices: [Single(LitInt(1))] }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }
    // ISET: A32
    // Fields: D=0, Vm=0, M=0, U=0, imm6=0, Q=0, op=0, Vd=0
    let encoding: u32 = 0xF2800C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_a1_a_invalid_1_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: imm6=0, op=0, M=0, D=0, Vd=0, Vm=0, U=0, Q=0
    let encoding: u32 = 0xF2800C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), indices: [Single(LitInt(1))] }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "imm6" }) } }, rhs: LitMask([One, Zero, Either, Either, Either, Either]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"op\" }), indices: [Single(LitInt(1))] }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"imm6\" }) } }, rhs: LitMask([One, Zero, Either, Either, Either, Either]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_a1_a_invalid_2_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), indices: [Single(LitInt(1))] }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "imm6" }) } }, rhs: LitMask([One, Zero, Either, Either, Either, Either]) }
    // ISET: A32
    // Fields: Vm=0, M=0, Q=0, D=0, imm6=0, Vd=0, U=0, op=0
    let encoding: u32 = 0xF2800C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_a1_a_invalid_3_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: imm6=0, D=0, op=0, Q=0, Vm=0, U=0, M=0, Vd=0
    let encoding: u32 = 0xF2800C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "imm6" }), rhs: LitMask([Zero, Either, Either, Either, Either, Either]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"imm6\" }), rhs: LitMask([Zero, Either, Either, Either, Either, Either]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_a1_a_invalid_4_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "imm6" }), rhs: LitMask([Zero, Either, Either, Either, Either, Either]) }
    // ISET: A32
    // Fields: U=0, imm6=0, Vd=0, Vm=0, M=0, op=0, Q=0, D=0
    let encoding: u32 = 0xF2800C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_a1_a_invalid_5_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: D=0, U=0, imm6=0, op=0, M=0, Vm=0, Vd=0, Q=0
    let encoding: u32 = 0xF2800C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_a1_a_invalid_6_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: A32
    // Fields: M=0, Vd=0, Vm=0, D=0, imm6=0, Q=0, U=0, op=0
    let encoding: u32 = 0xF2800C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_a1_a_invalid_7_c10_f2800c10() {
    // Encoding: 0xF2800C10
    // Test aarch32_VCVT_xs_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: Vm=0, M=0, U=0, Vd=0, Q=0, D=0, op=0, imm6=0
    let encoding: u32 = 0xF2800C10;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field U 28 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_u_0_min_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A field U = 0 (Min)
    // ISET: T32
    // Fields: op=0, Q=0, Vd=0, D=0, U=0, imm6=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field U 28 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_u_1_max_c10_ff800c10() {
    // Thumb encoding (32): 0xFF800C10
    // Test aarch32_VCVT_xs_T1_A field U = 1 (Max)
    // ISET: T32
    // Fields: M=0, imm6=0, Vm=0, U=1, Q=0, D=0, op=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_d_0_min_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A field D = 0 (Min)
    // ISET: T32
    // Fields: imm6=0, Vd=0, D=0, Q=0, M=0, op=0, U=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_d_1_max_c10_efc00c10() {
    // Thumb encoding (32): 0xEFC00C10
    // Test aarch32_VCVT_xs_T1_A field D = 1 (Max)
    // ISET: T32
    // Fields: Q=0, Vm=0, D=1, M=0, U=0, op=0, imm6=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFC00C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_imm6_0_zero_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A field imm6 = 0 (Zero)
    // ISET: T32
    // Fields: U=0, D=0, Q=0, imm6=0, op=0, M=0, Vm=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_imm6_1_poweroftwo_c10_ef810c10() {
    // Thumb encoding (32): 0xEF810C10
    // Test aarch32_VCVT_xs_T1_A field imm6 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vd=0, Q=0, op=0, D=0, imm6=1, M=0, Vm=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF810C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_imm6_3_poweroftwominusone_c10_ef830c10() {
    // Thumb encoding (32): 0xEF830C10
    // Test aarch32_VCVT_xs_T1_A field imm6 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Vd=0, Q=0, M=0, Vm=0, imm6=3, op=0, D=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF830C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_imm6_4_poweroftwo_c10_ef840c10() {
    // Thumb encoding (32): 0xEF840C10
    // Test aarch32_VCVT_xs_T1_A field imm6 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: op=0, Vd=0, Vm=0, imm6=4, U=0, D=0, Q=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF840C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 7, boundary: PowerOfTwoMinusOne }
/// 2^3 - 1 = 7
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_imm6_7_poweroftwominusone_c10_ef870c10() {
    // Thumb encoding (32): 0xEF870C10
    // Test aarch32_VCVT_xs_T1_A field imm6 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Q=0, U=0, op=0, M=0, Vd=0, Vm=0, D=0, imm6=7
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF870C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_imm6_8_poweroftwo_c10_ef880c10() {
    // Thumb encoding (32): 0xEF880C10
    // Test aarch32_VCVT_xs_T1_A field imm6 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=0, op=0, U=0, M=0, Q=0, Vd=0, D=0, imm6=8
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF880C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 15, boundary: PowerOfTwoMinusOne }
/// 2^4 - 1 = 15
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_imm6_15_poweroftwominusone_c10_ef8f0c10() {
    // Thumb encoding (32): 0xEF8F0C10
    // Test aarch32_VCVT_xs_T1_A field imm6 = 15 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: imm6=15, Vd=0, op=0, Q=0, U=0, Vm=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF8F0C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 16, boundary: PowerOfTwo }
/// power of 2 (2^4 = 16)
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_imm6_16_poweroftwo_c10_ef900c10() {
    // Thumb encoding (32): 0xEF900C10
    // Test aarch32_VCVT_xs_T1_A field imm6 = 16 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, Vd=0, imm6=16, Q=0, op=0, Vm=0, M=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF900C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 31, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (31)
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_imm6_31_poweroftwominusone_c10_ef9f0c10() {
    // Thumb encoding (32): 0xEF9F0C10
    // Test aarch32_VCVT_xs_T1_A field imm6 = 31 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: Vm=0, Q=0, D=0, U=0, op=0, Vd=0, M=0, imm6=31
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF9F0C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 32, boundary: PowerOfTwo }
/// power of 2 (2^5 = 32)
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_imm6_32_poweroftwo_c10_efa00c10() {
    // Thumb encoding (32): 0xEFA00C10
    // Test aarch32_VCVT_xs_T1_A field imm6 = 32 (PowerOfTwo)
    // ISET: T32
    // Fields: U=0, Q=0, imm6=32, op=0, Vm=0, D=0, M=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFA00C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field imm6 16 +: 6`
/// Requirement: FieldBoundary { field: "imm6", value: 63, boundary: Max }
/// maximum immediate (63)
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_imm6_63_max_c10_efbf0c10() {
    // Thumb encoding (32): 0xEFBF0C10
    // Test aarch32_VCVT_xs_T1_A field imm6 = 63 (Max)
    // ISET: T32
    // Fields: Vd=0, Q=0, M=0, D=0, imm6=63, op=0, U=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEFBF0C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_vd_0_min_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: op=0, Vm=0, M=0, U=0, imm6=0, Vd=0, Q=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_vd_1_poweroftwo_c10_ef801c10() {
    // Thumb encoding (32): 0xEF801C10
    // Test aarch32_VCVT_xs_T1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, U=0, imm6=0, Vm=0, Q=0, M=0, Vd=1, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF801C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field op 8 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_op_0_min_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A field op = 0 (Min)
    // ISET: T32
    // Fields: Vm=0, M=0, imm6=0, D=0, Q=0, op=0, U=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field op 8 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_op_1_poweroftwo_c10_ef800d10() {
    // Thumb encoding (32): 0xEF800D10
    // Test aarch32_VCVT_xs_T1_A field op = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, imm6=0, Vm=0, op=1, Q=0, U=0, M=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800D10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field op 8 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_op_3_max_c10_ef800f10() {
    // Thumb encoding (32): 0xEF800F10
    // Test aarch32_VCVT_xs_T1_A field op = 3 (Max)
    // ISET: T32
    // Fields: M=0, imm6=0, Vm=0, U=0, Q=0, D=0, op=3, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800F10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_q_0_min_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A field Q = 0 (Min)
    // ISET: T32
    // Fields: imm6=0, D=0, Q=0, Vd=0, op=0, Vm=0, M=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_q_1_max_c10_ef800c50() {
    // Thumb encoding (32): 0xEF800C50
    // Test aarch32_VCVT_xs_T1_A field Q = 1 (Max)
    // ISET: T32
    // Fields: M=0, U=0, Q=1, Vm=0, D=0, imm6=0, op=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C50;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_m_0_min_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A field M = 0 (Min)
    // ISET: T32
    // Fields: op=0, U=0, M=0, Vm=0, Vd=0, imm6=0, D=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_m_1_max_c10_ef800c30() {
    // Thumb encoding (32): 0xEF800C30
    // Test aarch32_VCVT_xs_T1_A field M = 1 (Max)
    // ISET: T32
    // Fields: imm6=0, op=0, U=0, Q=0, M=1, Vd=0, Vm=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_vm_0_min_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: M=0, Vd=0, imm6=0, D=0, Q=0, op=0, U=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_xs_t1_a_field_vm_1_poweroftwo_c10_ef800c11() {
    // Thumb encoding (32): 0xEF800C11
    // Test aarch32_VCVT_xs_T1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, U=0, Vm=1, Vd=0, imm6=0, Q=0, M=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C11;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// U=0 (minimum value)
#[test]
fn test_aarch32_vcvt_xs_t1_a_combo_0_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A field combination: U=0, D=0, imm6=0, Vd=0, op=0, Q=0, M=0, Vm=0
    // ISET: T32
    // Fields: M=0, U=0, op=0, D=0, Q=0, Vd=0, Vm=0, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_xs_t1_a_special_q_0_size_variant_0_3088_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A special value Q = 0 (Size variant 0)
    // ISET: T32
    // Fields: D=0, Vm=0, op=0, Q=0, imm6=0, U=0, Vd=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_xs_t1_a_special_q_1_size_variant_1_3088_ef800c50() {
    // Thumb encoding (32): 0xEF800C50
    // Test aarch32_VCVT_xs_T1_A special value Q = 1 (Size variant 1)
    // ISET: T32
    // Fields: D=0, imm6=0, op=0, Vm=0, U=0, Q=1, M=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C50;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), indices: [Single(LitInt(1))] }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"op\" }), indices: [Single(LitInt(1))] }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_t1_a_invalid_0_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A invalid encoding: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), indices: [Single(LitInt(1))] }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }
    // ISET: T32
    // Fields: Vm=0, imm6=0, op=0, D=0, U=0, M=0, Vd=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_t1_a_invalid_1_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: D=0, Vd=0, op=0, Q=0, Vm=0, M=0, U=0, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), indices: [Single(LitInt(1))] }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "imm6" }) } }, rhs: LitMask([One, Zero, Either, Either, Either, Either]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"op\" }), indices: [Single(LitInt(1))] }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"imm6\" }) } }, rhs: LitMask([One, Zero, Either, Either, Either, Either]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_t1_a_invalid_2_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "op" }), indices: [Single(LitInt(1))] }, rhs: Binary { op: And, lhs: LitBits([false]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "imm6" }) } }, rhs: LitMask([One, Zero, Either, Either, Either, Either]) }
    // ISET: T32
    // Fields: op=0, M=0, U=0, Vm=0, imm6=0, D=0, Q=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_t1_a_invalid_3_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: D=0, U=0, op=0, Vd=0, Vm=0, Q=0, imm6=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "imm6" }), rhs: LitMask([Zero, Either, Either, Either, Either, Either]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"imm6\" }), rhs: LitMask([Zero, Either, Either, Either, Either, Either]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_t1_a_invalid_4_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "imm6" }), rhs: LitMask([Zero, Either, Either, Either, Either, Either]) }
    // ISET: T32
    // Fields: D=0, op=0, Vd=0, Q=0, imm6=0, M=0, Vm=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_t1_a_invalid_5_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: Vd=0, D=0, Q=0, U=0, op=0, M=0, Vm=0, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_t1_a_invalid_6_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: T32
    // Fields: U=0, Q=0, M=0, imm6=0, D=0, Vm=0, op=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_xs_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xs_t1_a_invalid_7_c10_ef800c10() {
    // Thumb encoding (32): 0xEF800C10
    // Test aarch32_VCVT_xs_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: U=0, Vd=0, M=0, op=0, Vm=0, D=0, Q=0, imm6=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEF800C10;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_32_0_f2802c30() {
    // Test ADD 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 16), 0x59, "X16 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_64_0_f2802c30() {
    // Test ADD 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 16), 0x59, "X16 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero operands (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_32_1_f2800c30() {
    // Test ADD 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero operands (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_64_1_f2800c30() {
    // Test ADD 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small values (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_32_2_f2800c30() {
    // Test ADD 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFE,
        "X16 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_64_2_f2800c30() {
    // Test ADD 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFE,
        "X16 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_32_3_f2bffc30() {
    // Test ADD 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFF001,
        "X16 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_64_3_f2bffc30() {
    // Test ADD 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFF001,
        "X16 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_32_4_f2fffc30() {
    // Test ADD 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFF001000,
        "X16 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_64_4_f2fffc30() {
    // Test ADD 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFF001000,
        "X16 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max u64 operand (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_32_5_f2800c30() {
    // Test ADD 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max u64 operand (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_64_5_f2800c30() {
    // Test ADD 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0xFFFFFFFFFFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_32_6_f2800c30() {
    // Test ADD 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_64_6_f2800c30() {
    // Test ADD 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_32_7_f2800c30() {
    // Test ADD 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_64_7_f2800c30() {
    // Test ADD 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x7FFFFFFFFFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_32_8_f2800c30() {
    // Test ADD 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0x7FFFFFFC,
        "X16 should be 0x000000007FFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_64_8_f2800c30() {
    // Test ADD 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0x7FFFFFFC,
        "X16 should be 0x000000007FFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_32_9_f2800c30() {
    // Test ADD 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_64_9_f2800c30() {
    // Test ADD 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0xFFFFFFFFFFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_32_10_f2800c30() {
    // Test ADD 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_64_10_f2800c30() {
    // Test ADD 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_32_rd31_sp_f2802c3f() {
    // Test ADD 32-bit with Rd=31 (SP)
    // Encoding: 0xF2802C3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_add_oracle_64_rd31_sp_f2802c3f() {
    // Test ADD 64-bit with Rd=31 (SP)
    // Encoding: 0xF2802C3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_32_0_f2802c30() {
    // Test ADDS 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 16), 0x59, "X16 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_64_0_f2802c30() {
    // Test ADDS 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 16), 0x59, "X16 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_32_1_f2800c30() {
    // Test ADDS 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_64_1_f2800c30() {
    // Test ADDS 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_32_2_f2800c30() {
    // Test ADDS 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFE,
        "X16 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_64_2_f2800c30() {
    // Test ADDS 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFE,
        "X16 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_32_3_f2bffc30() {
    // Test ADDS 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFF001,
        "X16 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_64_3_f2bffc30() {
    // Test ADDS 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFF001,
        "X16 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_32_4_f2fffc30() {
    // Test ADDS 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFF001000,
        "X16 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_64_4_f2fffc30() {
    // Test ADDS 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFF001000,
        "X16 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_32_5_f2800c30() {
    // Test ADDS 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_64_5_f2800c30() {
    // Test ADDS 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0xFFFFFFFFFFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_32_6_f2800c30() {
    // Test ADDS 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_64_6_f2800c30() {
    // Test ADDS 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_32_7_f2800c30() {
    // Test ADDS 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_64_7_f2800c30() {
    // Test ADDS 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x7FFFFFFFFFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_32_8_f2800c30() {
    // Test ADDS 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0x7FFFFFFC,
        "X16 should be 0x000000007FFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_64_8_f2800c30() {
    // Test ADDS 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0x7FFFFFFC,
        "X16 should be 0x000000007FFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_32_9_f2800c30() {
    // Test ADDS 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_64_9_f2800c30() {
    // Test ADDS 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0xFFFFFFFFFFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_32_10_f2800c30() {
    // Test ADDS 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_64_10_f2800c30() {
    // Test ADDS 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_32_rd31_zr_f2802c3f() {
    // Test ADDS 32-bit with Rd=31 (ZR)
    // Encoding: 0xF2802C3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADDS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_adds_oracle_64_rd31_zr_f2802c3f() {
    // Test ADDS 64-bit with Rd=31 (ZR)
    // Encoding: 0xF2802C3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_32_0_f2802c30() {
    // Test SUB 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 16), 0x59, "X16 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #10`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_64_0_f2802c30() {
    // Test SUB 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 16), 0x59, "X16 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero operands (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_32_1_f2800c30() {
    // Test SUB 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero operands (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_64_1_f2800c30() {
    // Test SUB 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small values (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_32_2_f2800c30() {
    // Test SUB 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFE,
        "X16 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// small values (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_64_2_f2800c30() {
    // Test SUB 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFE,
        "X16 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_32_3_f2bffc30() {
    // Test SUB 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFF001,
        "X16 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #4095`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_64_3_f2bffc30() {
    // Test SUB 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFF001,
        "X16 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_32_4_f2fffc30() {
    // Test SUB 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFF001000,
        "X16 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #4095, LSL #12`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_64_4_f2fffc30() {
    // Test SUB 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFF001000,
        "X16 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max u64 operand (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_32_5_f2800c30() {
    // Test SUB 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max u64 operand (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_64_5_f2800c30() {
    // Test SUB 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0xFFFFFFFFFFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_32_6_f2800c30() {
    // Test SUB 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_64_6_f2800c30() {
    // Test SUB 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_32_7_f2800c30() {
    // Test SUB 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_64_7_f2800c30() {
    // Test SUB 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x7FFFFFFFFFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_32_8_f2800c30() {
    // Test SUB 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0x7FFFFFFC,
        "X16 should be 0x000000007FFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_64_8_f2800c30() {
    // Test SUB 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0x7FFFFFFC,
        "X16 should be 0x000000007FFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_32_9_f2800c30() {
    // Test SUB 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_64_9_f2800c30() {
    // Test SUB 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0xFFFFFFFFFFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_32_10_f2800c30() {
    // Test SUB 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB X0, X1, #1`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_64_10_f2800c30() {
    // Test SUB 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_32_rd31_sp_f2802c3f() {
    // Test SUB 32-bit with Rd=31 (SP)
    // Encoding: 0xF2802C3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUB SP, X1, #10`
/// Requirement: RegisterSpecial { reg: Sp, behavior: "writes to stack pointer" }
/// SP destination (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_sub_oracle_64_rd31_sp_f2802c3f() {
    // Test SUB 64-bit with Rd=31 (SP)
    // Encoding: 0xF2802C3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_32_0_f2802c30() {
    // Test SUBS 32-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 16), 0x59, "X16 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #10`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// simple addition/subtraction (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_64_0_f2802c30() {
    // Test SUBS 64-bit: simple addition/subtraction (with oracle verification)
    // Encoding: 0xF2802C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 16), 0x59, "X16 should be 0x0000000000000059");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_32_1_f2800c30() {
    // Test SUBS 32-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #0`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero operands (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_64_1_f2800c30() {
    // Test SUBS 64-bit: zero operands (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_32_2_f2800c30() {
    // Test SUBS 32-bit: small values (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFE,
        "X16 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// small values (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_64_2_f2800c30() {
    // Test SUBS 64-bit: small values (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x1);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFE,
        "X16 should be 0xFFFFFFFFFFFFFFFE"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_32_3_f2bffc30() {
    // Test SUBS 32-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFF001,
        "X16 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #4095`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 unshifted (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_64_3_f2bffc30() {
    // Test SUBS 64-bit: max imm12 unshifted (with oracle verification)
    // Encoding: 0xF2BFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2BFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFF001,
        "X16 should be 0xFFFFFFFFFFFFF001"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_32_4_f2fffc30() {
    // Test SUBS 32-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFF001000,
        "X16 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #4095, LSL #12`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max imm12 shifted (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_64_4_f2fffc30() {
    // Test SUBS 64-bit: max imm12 shifted (with oracle verification)
    // Encoding: 0xF2FFFC30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2FFFC30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFF001000,
        "X16 should be 0xFFFFFFFFFF001000"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_32_5_f2800c30() {
    // Test SUBS 32-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// max u64 operand (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_64_5_f2800c30() {
    // Test SUBS 64-bit: max u64 operand (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0xFFFFFFFFFFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_32_6_f2800c30() {
    // Test SUBS 32-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// zero result (for sub 1-1) (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_64_6_f2800c30() {
    // Test SUBS 64-bit: zero result (for sub 1-1) (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFD,
        "X16 should be 0xFFFFFFFFFFFFFFFD"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, false, "C flag should be false");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_32_7_f2800c30() {
    // Test SUBS 32-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 64-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_64_7_f2800c30() {
    // Test SUBS 64-bit: signed overflow boundary 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x7FFFFFFFFFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_32_8_f2800c30() {
    // Test SUBS 32-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0x7FFFFFFC,
        "X16 should be 0x000000007FFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// signed overflow boundary 32-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_64_8_f2800c30() {
    // Test SUBS 64-bit: signed overflow boundary 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0x7FFFFFFC,
        "X16 should be 0x000000007FFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_32_9_f2800c30() {
    // Test SUBS 32-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 64-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_64_9_f2800c30() {
    // Test SUBS 64-bit: unsigned overflow 64-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0xFFFFFFFFFFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, true, "N flag should be true");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_32_10_f2800c30() {
    // Test SUBS 32-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS X0, X1, #1`
/// Requirement: FlagComputation { flag: N, scenario: NonZeroResult }
/// unsigned overflow 32-bit (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_64_10_f2800c30() {
    // Test SUBS 64-bit: unsigned overflow 32-bit (with oracle verification)
    // Encoding: 0xF2800C30
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2800C30;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 16),
        0xFFFFFFFC,
        "X16 should be 0x00000000FFFFFFFC"
    );
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (32)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_32_rd31_zr_f2802c3f() {
    // Test SUBS 32-bit with Rd=31 (ZR)
    // Encoding: 0xF2802C3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `SUBS ZR, X1, #10`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "result discarded, flags set" }
/// ZR destination (64)
#[test]
fn test_aarch32_vcvt_xs_a1_a_subs_oracle_64_rd31_zr_f2802c3f() {
    // Test SUBS 64-bit with Rd=31 (ZR)
    // Encoding: 0xF2802C3F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2802C3F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(cpu.get_pstate().n, false, "N flag should be false");
    assert_eq!(cpu.get_pstate().z, false, "Z flag should be false");
    assert_eq!(cpu.get_pstate().c, true, "C flag should be true");
    assert_eq!(cpu.get_pstate().v, false, "V flag should be false");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_0_0_f281000a() {
    // Test A32 ADD: small immediate (oracle)
    // Encoding: 0xF281000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF281000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA, "R0 should be 0x0000000A");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x00000064)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_0_64_f281000a() {
    // Test A32 ADD: small immediate (oracle)
    // Encoding: 0xF281000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF281000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x6E, "R0 should be 0x0000006E");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_0_7fffffff_f281000a() {
    // Test A32 ADD: small immediate (oracle)
    // Encoding: 0xF281000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF281000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000009, "R0 should be 0x80000009");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0x80000000)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_0_80000000_f281000a() {
    // Test A32 ADD: small immediate (oracle)
    // Encoding: 0xF281000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x80000000);
    let encoding: u32 = 0xF281000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x8000000A, "R0 should be 0x8000000A");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_0_ffffffff_f281000a() {
    // Test A32 ADD: small immediate (oracle)
    // Encoding: 0xF281000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF281000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x9, "R0 should be 0x00000009");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x00000000)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_1_0_f28100ff() {
    // Test A32 ADD: max imm8 (oracle)
    // Encoding: 0xF28100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF28100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x00000064)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_1_64_f28100ff() {
    // Test A32 ADD: max imm8 (oracle)
    // Encoding: 0xF28100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF28100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x163, "R0 should be 0x00000163");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_1_7fffffff_f28100ff() {
    // Test A32 ADD: max imm8 (oracle)
    // Encoding: 0xF28100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF28100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x800000FE, "R0 should be 0x800000FE");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0x80000000)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_1_80000000_f28100ff() {
    // Test A32 ADD: max imm8 (oracle)
    // Encoding: 0xF28100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x80000000);
    let encoding: u32 = 0xF28100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x800000FF, "R0 should be 0x800000FF");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_1_ffffffff_f28100ff() {
    // Test A32 ADD: max imm8 (oracle)
    // Encoding: 0xF28100FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF28100FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFE, "R0 should be 0x000000FE");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x00000000)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_2_0_f2810180() {
    // Test A32 ADD: rotated by 2 (oracle)
    // Encoding: 0xF2810180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2810180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x20, "R0 should be 0x00000020");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x00000064)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_2_64_f2810180() {
    // Test A32 ADD: rotated by 2 (oracle)
    // Encoding: 0xF2810180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2810180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x84, "R0 should be 0x00000084");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_2_7fffffff_f2810180() {
    // Test A32 ADD: rotated by 2 (oracle)
    // Encoding: 0xF2810180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2810180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x8000001F, "R0 should be 0x8000001F");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0x80000000)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_2_80000000_f2810180() {
    // Test A32 ADD: rotated by 2 (oracle)
    // Encoding: 0xF2810180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x80000000);
    let encoding: u32 = 0xF2810180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000020, "R0 should be 0x80000020");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_2_ffffffff_f2810180() {
    // Test A32 ADD: rotated by 2 (oracle)
    // Encoding: 0xF2810180
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2810180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1F, "R0 should be 0x0000001F");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x00000000)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_3_0_f281040f() {
    // Test A32 ADD: rotated by 8 (oracle)
    // Encoding: 0xF281040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF281040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xF000000, "R0 should be 0x0F000000");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x00000064)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_3_64_f281040f() {
    // Test A32 ADD: rotated by 8 (oracle)
    // Encoding: 0xF281040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF281040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xF000064, "R0 should be 0x0F000064");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_3_7fffffff_f281040f() {
    // Test A32 ADD: rotated by 8 (oracle)
    // Encoding: 0xF281040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF281040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x8EFFFFFF, "R0 should be 0x8EFFFFFF");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0x80000000)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_3_80000000_f281040f() {
    // Test A32 ADD: rotated by 8 (oracle)
    // Encoding: 0xF281040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x80000000);
    let encoding: u32 = 0xF281040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x8F000000, "R0 should be 0x8F000000");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8 (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_3_ffffffff_f281040f() {
    // Test A32 ADD: rotated by 8 (oracle)
    // Encoding: 0xF281040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF281040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xEFFFFFF, "R0 should be 0x0EFFFFFF");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x00000000)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_4_0_f2810000() {
    // Test A32 ADD: zero immediate (oracle)
    // Encoding: 0xF2810000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x0);
    let encoding: u32 = 0xF2810000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x00000064)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_4_64_f2810000() {
    // Test A32 ADD: zero immediate (oracle)
    // Encoding: 0xF2810000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x64);
    let encoding: u32 = 0xF2810000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x64, "R0 should be 0x00000064");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x7FFFFFFF)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_4_7fffffff_f2810000() {
    // Test A32 ADD: zero immediate (oracle)
    // Encoding: 0xF2810000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x7FFFFFFF);
    let encoding: u32 = 0xF2810000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x7FFFFFFF, "R0 should be 0x7FFFFFFF");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0x80000000)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_4_80000000_f2810000() {
    // Test A32 ADD: zero immediate (oracle)
    // Encoding: 0xF2810000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0x80000000);
    let encoding: u32 = 0xF2810000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x80000000, "R0 should be 0x80000000");
}

/// Provenance: aarch32_VCVT_xs_A1_A
/// ASL: `ADD R0, R1, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate (Rn=0xFFFFFFFF)
#[test]
fn test_aarch32_vcvt_xs_a1_a_a32_add_sub_imm_4_ffffffff_f2810000() {
    // Test A32 ADD: zero immediate (oracle)
    // Encoding: 0xF2810000
    // ISET: A32
    let mut cpu = create_test_cpu();
    set_w(&mut cpu, 1, 0xFFFFFFFF);
    let encoding: u32 = 0xF2810000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFFFFFF, "R0 should be 0xFFFFFFFF");
}

// ============================================================================
// aarch32_VCVT_xv_A Tests
// ============================================================================

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_0_min_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: D=0, sf=0, i=0, sx=0, U=0, op=0, Vd=0, imm4=0, cond=0
    let encoding: u32 = 0x0EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_1_poweroftwo_840_1eba0840() {
    // Encoding: 0x1EBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, op=0, imm4=0, i=0, Vd=0, U=0, sx=0, sf=0, cond=1
    let encoding: u32 = 0x1EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_2_poweroftwo_840_2eba0840() {
    // Encoding: 0x2EBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, cond=2, U=0, sx=0, Vd=0, op=0, i=0, D=0, sf=0
    let encoding: u32 = 0x2EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_3_poweroftwo_840_3eba0840() {
    // Encoding: 0x3EBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, cond=3, sx=0, op=0, Vd=0, i=0, imm4=0, U=0, sf=0
    let encoding: u32 = 0x3EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_4_poweroftwo_840_4eba0840() {
    // Encoding: 0x4EBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, i=0, op=0, cond=4, Vd=0, U=0, sx=0, sf=0, D=0
    let encoding: u32 = 0x4EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_5_poweroftwo_840_5eba0840() {
    // Encoding: 0x5EBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: imm4=0, op=0, D=0, Vd=0, sf=0, U=0, i=0, cond=5, sx=0
    let encoding: u32 = 0x5EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_6_poweroftwo_840_6eba0840() {
    // Encoding: 0x6EBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=6, op=0, sf=0, imm4=0, Vd=0, D=0, U=0, i=0, sx=0
    let encoding: u32 = 0x6EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_7_poweroftwo_840_7eba0840() {
    // Encoding: 0x7EBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: op=0, imm4=0, sx=0, i=0, D=0, Vd=0, U=0, cond=7, sf=0
    let encoding: u32 = 0x7EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_8_poweroftwo_840_8eba0840() {
    // Encoding: 0x8EBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: op=0, Vd=0, sx=0, D=0, imm4=0, sf=0, cond=8, U=0, i=0
    let encoding: u32 = 0x8EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_9_poweroftwo_840_9eba0840() {
    // Encoding: 0x9EBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: sx=0, imm4=0, i=0, sf=0, Vd=0, op=0, cond=9, D=0, U=0
    let encoding: u32 = 0x9EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_10_poweroftwo_840_aeba0840() {
    // Encoding: 0xAEBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, U=0, D=0, i=0, imm4=0, sf=0, Vd=0, op=0, sx=0
    let encoding: u32 = 0xAEBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_11_poweroftwo_840_beba0840() {
    // Encoding: 0xBEBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: sf=0, i=0, imm4=0, sx=0, D=0, op=0, Vd=0, cond=11, U=0
    let encoding: u32 = 0xBEBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_12_poweroftwo_840_ceba0840() {
    // Encoding: 0xCEBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: op=0, U=0, Vd=0, sx=0, D=0, imm4=0, cond=12, sf=0, i=0
    let encoding: u32 = 0xCEBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_13_poweroftwo_840_deba0840() {
    // Encoding: 0xDEBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: sf=0, op=0, U=0, imm4=0, i=0, D=0, cond=13, sx=0, Vd=0
    let encoding: u32 = 0xDEBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_14_poweroftwo_840_eeba0840() {
    // Encoding: 0xEEBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: op=0, i=0, D=0, cond=14, sx=0, imm4=0, U=0, sf=0, Vd=0
    let encoding: u32 = 0xEEBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_cond_15_max_840_feba0840() {
    // Encoding: 0xFEBA0840
    // Test aarch32_VCVT_xv_A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: sx=0, sf=0, cond=15, U=0, Vd=0, imm4=0, D=0, i=0, op=0
    let encoding: u32 = 0xFEBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_d_0_min_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, op=0, i=0, D=0, imm4=0, cond=0, sf=0, U=0, sx=0
    let encoding: u32 = 0x0EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_d_1_max_840_0efa0840() {
    // Encoding: 0x0EFA0840
    // Test aarch32_VCVT_xv_A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: U=0, imm4=0, sx=0, i=0, Vd=0, cond=0, D=1, sf=0, op=0
    let encoding: u32 = 0x0EFA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field op 18 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_op_0_min_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A field op = 0 (Min)
    // ISET: A32
    // Fields: sf=0, sx=0, i=0, imm4=0, op=0, D=0, Vd=0, cond=0, U=0
    let encoding: u32 = 0x0EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field op 18 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_op_1_max_840_0ebe0840() {
    // Encoding: 0x0EBE0840
    // Test aarch32_VCVT_xv_A1_A field op = 1 (Max)
    // ISET: A32
    // Fields: op=1, cond=0, D=0, i=0, U=0, sx=0, imm4=0, Vd=0, sf=0
    let encoding: u32 = 0x0EBE0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field U 16 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_u_0_min_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A field U = 0 (Min)
    // ISET: A32
    // Fields: U=0, cond=0, op=0, sx=0, D=0, sf=0, i=0, Vd=0, imm4=0
    let encoding: u32 = 0x0EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field U 16 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_u_1_max_840_0ebb0840() {
    // Encoding: 0x0EBB0840
    // Test aarch32_VCVT_xv_A1_A field U = 1 (Max)
    // ISET: A32
    // Fields: Vd=0, imm4=0, U=1, sf=0, op=0, cond=0, sx=0, D=0, i=0
    let encoding: u32 = 0x0EBB0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_vd_0_min_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: i=0, cond=0, op=0, Vd=0, U=0, sf=0, D=0, sx=0, imm4=0
    let encoding: u32 = 0x0EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_vd_1_poweroftwo_840_0eba1840() {
    // Encoding: 0x0EBA1840
    // Test aarch32_VCVT_xv_A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=1, U=0, imm4=0, D=0, sx=0, cond=0, sf=0, op=0, i=0
    let encoding: u32 = 0x0EBA1840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field sf 8 +: 2`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_sf_0_min_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A field sf = 0 (Min)
    // ISET: A32
    // Fields: sx=0, cond=0, op=0, Vd=0, D=0, sf=0, imm4=0, U=0, i=0
    let encoding: u32 = 0x0EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field sf 8 +: 2`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_sf_1_poweroftwo_840_0eba0940() {
    // Encoding: 0x0EBA0940
    // Test aarch32_VCVT_xv_A1_A field sf = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: op=0, D=0, sf=1, U=0, i=0, cond=0, sx=0, Vd=0, imm4=0
    let encoding: u32 = 0x0EBA0940;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field sf 8 +: 2`
/// Requirement: FieldBoundary { field: "sf", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_sf_2_poweroftwo_840_0eba0a40() {
    // Encoding: 0x0EBA0A40
    // Test aarch32_VCVT_xv_A1_A field sf = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, op=0, Vd=0, D=0, i=0, imm4=0, U=0, sx=0, sf=2
    let encoding: u32 = 0x0EBA0A40;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field sf 8 +: 2`
/// Requirement: FieldBoundary { field: "sf", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_sf_3_max_840_0eba0b40() {
    // Encoding: 0x0EBA0B40
    // Test aarch32_VCVT_xv_A1_A field sf = 3 (Max)
    // ISET: A32
    // Fields: i=0, imm4=0, U=0, D=0, Vd=0, sx=0, op=0, sf=3, cond=0
    let encoding: u32 = 0x0EBA0B40;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field sx 7 +: 1`
/// Requirement: FieldBoundary { field: "sx", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_sx_0_min_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A field sx = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, op=0, sx=0, i=0, U=0, imm4=0, cond=0, D=0, sf=0
    let encoding: u32 = 0x0EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field sx 7 +: 1`
/// Requirement: FieldBoundary { field: "sx", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_sx_1_max_840_0eba08c0() {
    // Encoding: 0x0EBA08C0
    // Test aarch32_VCVT_xv_A1_A field sx = 1 (Max)
    // ISET: A32
    // Fields: i=0, sf=0, sx=1, op=0, U=0, imm4=0, D=0, Vd=0, cond=0
    let encoding: u32 = 0x0EBA08C0;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field i 5 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_i_0_min_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A field i = 0 (Min)
    // ISET: A32
    // Fields: op=0, D=0, sf=0, cond=0, sx=0, i=0, imm4=0, U=0, Vd=0
    let encoding: u32 = 0x0EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field i 5 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_i_1_max_840_0eba0860() {
    // Encoding: 0x0EBA0860
    // Test aarch32_VCVT_xv_A1_A field i = 1 (Max)
    // ISET: A32
    // Fields: D=0, imm4=0, U=0, sx=0, i=1, Vd=0, sf=0, op=0, cond=0
    let encoding: u32 = 0x0EBA0860;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_imm4_0_zero_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A field imm4 = 0 (Zero)
    // ISET: A32
    // Fields: D=0, U=0, cond=0, op=0, Vd=0, sf=0, sx=0, imm4=0, i=0
    let encoding: u32 = 0x0EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_imm4_1_poweroftwo_840_0eba0841() {
    // Encoding: 0x0EBA0841
    // Test aarch32_VCVT_xv_A1_A field imm4 = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: sf=0, Vd=0, D=0, op=0, imm4=1, cond=0, U=0, i=0, sx=0
    let encoding: u32 = 0x0EBA0841;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_imm4_3_poweroftwominusone_840_0eba0843() {
    // Encoding: 0x0EBA0843
    // Test aarch32_VCVT_xv_A1_A field imm4 = 3 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, D=0, op=0, sf=0, imm4=3, Vd=0, U=0, sx=0, i=0
    let encoding: u32 = 0x0EBA0843;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_imm4_4_poweroftwo_840_0eba0844() {
    // Encoding: 0x0EBA0844
    // Test aarch32_VCVT_xv_A1_A field imm4 = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: sf=0, i=0, U=0, cond=0, Vd=0, imm4=4, op=0, D=0, sx=0
    let encoding: u32 = 0x0EBA0844;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_imm4_7_poweroftwominusone_840_0eba0847() {
    // Encoding: 0x0EBA0847
    // Test aarch32_VCVT_xv_A1_A field imm4 = 7 (PowerOfTwoMinusOne)
    // ISET: A32
    // Fields: cond=0, imm4=7, op=0, D=0, Vd=0, sx=0, sf=0, i=0, U=0
    let encoding: u32 = 0x0EBA0847;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_imm4_8_poweroftwo_840_0eba0848() {
    // Encoding: 0x0EBA0848
    // Test aarch32_VCVT_xv_A1_A field imm4 = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, D=0, op=0, cond=0, sx=0, imm4=8, sf=0, U=0, i=0
    let encoding: u32 = 0x0EBA0848;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch32_vcvt_xv_a1_a_field_imm4_15_max_840_0eba084f() {
    // Encoding: 0x0EBA084F
    // Test aarch32_VCVT_xv_A1_A field imm4 = 15 (Max)
    // ISET: A32
    // Fields: sx=0, cond=0, Vd=0, U=0, op=0, sf=0, D=0, i=0, imm4=15
    let encoding: u32 = 0x0EBA084F;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_vcvt_xv_a1_a_combo_0_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A field combination: cond=0, D=0, op=0, U=0, Vd=0, sf=0, sx=0, i=0, imm4=0
    // ISET: A32
    // Fields: sf=0, Vd=0, U=0, D=0, imm4=0, sx=0, i=0, op=0, cond=0
    let encoding: u32 = 0x0EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_0_condition_eq_2112_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: op=0, D=0, i=0, Vd=0, imm4=0, U=0, sx=0, sf=0, cond=0
    let encoding: u32 = 0x0EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_1_condition_ne_2112_1eba0840() {
    // Encoding: 0x1EBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: cond=1, D=0, Vd=0, sx=0, sf=0, op=0, imm4=0, i=0, U=0
    let encoding: u32 = 0x1EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_2_condition_cs_hs_2112_2eba0840() {
    // Encoding: 0x2EBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: sx=0, i=0, sf=0, Vd=0, D=0, imm4=0, cond=2, op=0, U=0
    let encoding: u32 = 0x2EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_3_condition_cc_lo_2112_3eba0840() {
    // Encoding: 0x3EBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, D=0, Vd=0, sf=0, imm4=0, U=0, sx=0, i=0, op=0
    let encoding: u32 = 0x3EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_4_condition_mi_2112_4eba0840() {
    // Encoding: 0x4EBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: sf=0, cond=4, sx=0, i=0, imm4=0, Vd=0, op=0, D=0, U=0
    let encoding: u32 = 0x4EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_5_condition_pl_2112_5eba0840() {
    // Encoding: 0x5EBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: imm4=0, Vd=0, i=0, D=0, op=0, cond=5, sx=0, U=0, sf=0
    let encoding: u32 = 0x5EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_6_condition_vs_2112_6eba0840() {
    // Encoding: 0x6EBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: D=0, U=0, sf=0, cond=6, imm4=0, Vd=0, sx=0, i=0, op=0
    let encoding: u32 = 0x6EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_7_condition_vc_2112_7eba0840() {
    // Encoding: 0x7EBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: op=0, D=0, i=0, U=0, Vd=0, sf=0, imm4=0, sx=0, cond=7
    let encoding: u32 = 0x7EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_8_condition_hi_2112_8eba0840() {
    // Encoding: 0x8EBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: cond=8, sx=0, D=0, i=0, U=0, op=0, Vd=0, sf=0, imm4=0
    let encoding: u32 = 0x8EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_9_condition_ls_2112_9eba0840() {
    // Encoding: 0x9EBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Vd=0, sf=0, U=0, sx=0, cond=9, i=0, op=0, imm4=0, D=0
    let encoding: u32 = 0x9EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_10_condition_ge_2112_aeba0840() {
    // Encoding: 0xAEBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: imm4=0, D=0, U=0, sf=0, cond=10, i=0, op=0, Vd=0, sx=0
    let encoding: u32 = 0xAEBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_11_condition_lt_2112_beba0840() {
    // Encoding: 0xBEBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: sf=0, U=0, cond=11, imm4=0, op=0, Vd=0, sx=0, i=0, D=0
    let encoding: u32 = 0xBEBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_12_condition_gt_2112_ceba0840() {
    // Encoding: 0xCEBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: cond=12, op=0, sx=0, D=0, Vd=0, sf=0, U=0, i=0, imm4=0
    let encoding: u32 = 0xCEBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_13_condition_le_2112_deba0840() {
    // Encoding: 0xDEBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: sx=0, op=0, Vd=0, imm4=0, D=0, i=0, cond=13, sf=0, U=0
    let encoding: u32 = 0xDEBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_14_condition_al_2112_eeba0840() {
    // Encoding: 0xEEBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: imm4=0, D=0, op=0, U=0, sx=0, sf=0, i=0, Vd=0, cond=14
    let encoding: u32 = 0xEEBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_cond_15_condition_nv_2112_feba0840() {
    // Encoding: 0xFEBA0840
    // Test aarch32_VCVT_xv_A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: sx=0, sf=0, imm4=0, Vd=0, D=0, U=0, i=0, cond=15, op=0
    let encoding: u32 = 0xFEBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_sf_0_size_variant_0_2112_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A special value sf = 0 (Size variant 0)
    // ISET: A32
    // Fields: sx=0, U=0, D=0, i=0, Vd=0, cond=0, op=0, sf=0, imm4=0
    let encoding: u32 = 0x0EBA0840;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_sf_1_size_variant_1_2112_0eba0940() {
    // Encoding: 0x0EBA0940
    // Test aarch32_VCVT_xv_A1_A special value sf = 1 (Size variant 1)
    // ISET: A32
    // Fields: imm4=0, op=0, Vd=0, sf=1, D=0, cond=0, sx=0, U=0, i=0
    let encoding: u32 = 0x0EBA0940;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field sf = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "sf", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_sf_2_size_variant_2_2112_0eba0a40() {
    // Encoding: 0x0EBA0A40
    // Test aarch32_VCVT_xv_A1_A special value sf = 2 (Size variant 2)
    // ISET: A32
    // Fields: Vd=0, cond=0, i=0, U=0, sx=0, imm4=0, op=0, sf=2, D=0
    let encoding: u32 = 0x0EBA0A40;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `field sf = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "sf", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvt_xv_a1_a_special_sf_3_size_variant_3_2112_0eba0b40() {
    // Encoding: 0x0EBA0B40
    // Test aarch32_VCVT_xv_A1_A special value sf = 3 (Size variant 3)
    // ISET: A32
    // Fields: Vd=0, sf=3, D=0, cond=0, i=0, sx=0, op=0, U=0, imm4=0
    let encoding: u32 = 0x0EBA0B40;
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

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sf" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sf" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"sf\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"sf\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xv_a1_a_invalid_0_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sf" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sf" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: A32
    // Fields: D=0, U=0, imm4=0, sf=0, op=0, Vd=0, cond=0, sx=0, i=0
    let encoding: u32 = 0x0EBA0840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xv_a1_a_invalid_1_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: i=0, imm4=0, U=0, Vd=0, sf=0, cond=0, sx=0, op=0, D=0
    let encoding: u32 = 0x0EBA0840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sf" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "cond" }) } }, rhs: LitBits([true, true, true, false]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"sf\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: \"cond\" }) } }, rhs: LitBits([true, true, true, false]) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_xv_a1_a_invalid_2_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A invalid encoding: Binary { op: Ne, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sf" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Var(QualifiedIdentifier { qualifier: Any, name: "cond" }) } }, rhs: LitBits([true, true, true, false]) }
    // ISET: A32
    // Fields: cond=0, sx=0, imm4=0, Vd=0, sf=0, D=0, op=0, i=0, U=0
    let encoding: u32 = 0x0EBA0840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_xv_a1_a_invalid_3_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: imm4=0, op=0, U=0, Vd=0, sx=0, D=0, sf=0, i=0, cond=0
    let encoding: u32 = 0x0EBA0840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `Binary { op: Lt, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "frac_bits" }), rhs: LitInt(0) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Lt, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"frac_bits\" }), rhs: LitInt(0) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_xv_a1_a_invalid_4_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A invalid encoding: Binary { op: Lt, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "frac_bits" }), rhs: LitInt(0) }
    // ISET: A32
    // Fields: cond=0, op=0, Vd=0, i=0, imm4=0, sf=0, sx=0, D=0, U=0
    let encoding: u32 = 0x0EBA0840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVT_xv_A1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_xv_a1_a_invalid_5_840_0eba0840() {
    // Encoding: 0x0EBA0840
    // Test aarch32_VCVT_xv_A1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: A32
    // Fields: sx=0, cond=0, imm4=0, sf=0, Vd=0, D=0, op=0, i=0, U=0
    let encoding: u32 = 0x0EBA0840;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    // UNPREDICTABLE - behavior is implementation-defined
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_d_0_min_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A field D = 0 (Min)
    // ISET: T32
    // Fields: U=0, sx=0, sf=0, Vd=0, op=0, i=0, imm4=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_d_1_max_840_eefa0840() {
    // Thumb encoding (32): 0xEEFA0840
    // Test aarch32_VCVT_xv_T1_A field D = 1 (Max)
    // ISET: T32
    // Fields: sx=0, imm4=0, D=1, sf=0, i=0, U=0, Vd=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEFA0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field op 18 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_op_0_min_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A field op = 0 (Min)
    // ISET: T32
    // Fields: sf=0, sx=0, D=0, op=0, Vd=0, i=0, imm4=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field op 18 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_op_1_max_840_eebe0840() {
    // Thumb encoding (32): 0xEEBE0840
    // Test aarch32_VCVT_xv_T1_A field op = 1 (Max)
    // ISET: T32
    // Fields: op=1, Vd=0, sf=0, sx=0, i=0, imm4=0, D=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBE0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field U 16 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_u_0_min_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A field U = 0 (Min)
    // ISET: T32
    // Fields: Vd=0, U=0, op=0, imm4=0, i=0, sf=0, D=0, sx=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field U 16 +: 1`
/// Requirement: FieldBoundary { field: "U", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_u_1_max_840_eebb0840() {
    // Thumb encoding (32): 0xEEBB0840
    // Test aarch32_VCVT_xv_T1_A field U = 1 (Max)
    // ISET: T32
    // Fields: U=1, sf=0, sx=0, op=0, Vd=0, imm4=0, D=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBB0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_vd_0_min_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: sf=0, D=0, U=0, i=0, op=0, sx=0, imm4=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_vd_1_poweroftwo_840_eeba1840() {
    // Thumb encoding (32): 0xEEBA1840
    // Test aarch32_VCVT_xv_T1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: sx=0, i=0, Vd=1, sf=0, D=0, imm4=0, U=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA1840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field sf 8 +: 2`
/// Requirement: FieldBoundary { field: "sf", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_sf_0_min_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A field sf = 0 (Min)
    // ISET: T32
    // Fields: sx=0, D=0, Vd=0, U=0, sf=0, imm4=0, i=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field sf 8 +: 2`
/// Requirement: FieldBoundary { field: "sf", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_sf_1_poweroftwo_840_eeba0940() {
    // Thumb encoding (32): 0xEEBA0940
    // Test aarch32_VCVT_xv_T1_A field sf = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: U=0, D=0, Vd=0, imm4=0, sx=0, i=0, op=0, sf=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0940;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field sf 8 +: 2`
/// Requirement: FieldBoundary { field: "sf", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_sf_2_poweroftwo_840_eeba0a40() {
    // Thumb encoding (32): 0xEEBA0A40
    // Test aarch32_VCVT_xv_T1_A field sf = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: imm4=0, D=0, Vd=0, op=0, U=0, sf=2, sx=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field sf 8 +: 2`
/// Requirement: FieldBoundary { field: "sf", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_sf_3_max_840_eeba0b40() {
    // Thumb encoding (32): 0xEEBA0B40
    // Test aarch32_VCVT_xv_T1_A field sf = 3 (Max)
    // ISET: T32
    // Fields: Vd=0, imm4=0, D=0, U=0, sf=3, i=0, op=0, sx=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field sx 7 +: 1`
/// Requirement: FieldBoundary { field: "sx", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_sx_0_min_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A field sx = 0 (Min)
    // ISET: T32
    // Fields: D=0, op=0, U=0, Vd=0, sf=0, imm4=0, sx=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field sx 7 +: 1`
/// Requirement: FieldBoundary { field: "sx", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_sx_1_max_840_eeba08c0() {
    // Thumb encoding (32): 0xEEBA08C0
    // Test aarch32_VCVT_xv_T1_A field sx = 1 (Max)
    // ISET: T32
    // Fields: sf=0, U=0, D=0, sx=1, i=0, Vd=0, imm4=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA08C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field i 5 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_i_0_min_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A field i = 0 (Min)
    // ISET: T32
    // Fields: i=0, D=0, sf=0, Vd=0, U=0, sx=0, op=0, imm4=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field i 5 +: 1`
/// Requirement: FieldBoundary { field: "i", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_i_1_max_840_eeba0860() {
    // Thumb encoding (32): 0xEEBA0860
    // Test aarch32_VCVT_xv_T1_A field i = 1 (Max)
    // ISET: T32
    // Fields: op=0, U=0, sf=0, imm4=0, D=0, Vd=0, i=1, sx=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0860;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 0, boundary: Zero }
/// immediate value 0
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_imm4_0_zero_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A field imm4 = 0 (Zero)
    // ISET: T32
    // Fields: D=0, sf=0, sx=0, U=0, imm4=0, op=0, Vd=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 1, boundary: PowerOfTwo }
/// immediate value 1
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_imm4_1_poweroftwo_840_eeba0841() {
    // Thumb encoding (32): 0xEEBA0841
    // Test aarch32_VCVT_xv_T1_A field imm4 = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vd=0, D=0, sx=0, op=0, sf=0, i=0, imm4=1, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0841;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 3, boundary: PowerOfTwoMinusOne }
/// 2^2 - 1 = 3
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_imm4_3_poweroftwominusone_840_eeba0843() {
    // Thumb encoding (32): 0xEEBA0843
    // Test aarch32_VCVT_xv_T1_A field imm4 = 3 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: sx=0, Vd=0, op=0, D=0, i=0, imm4=3, sf=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0843;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 4, boundary: PowerOfTwo }
/// power of 2 (2^2 = 4)
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_imm4_4_poweroftwo_840_eeba0844() {
    // Thumb encoding (32): 0xEEBA0844
    // Test aarch32_VCVT_xv_T1_A field imm4 = 4 (PowerOfTwo)
    // ISET: T32
    // Fields: op=0, i=0, imm4=4, sf=0, D=0, Vd=0, U=0, sx=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0844;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 7, boundary: PowerOfTwoMinusOne }
/// immediate midpoint (7)
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_imm4_7_poweroftwominusone_840_eeba0847() {
    // Thumb encoding (32): 0xEEBA0847
    // Test aarch32_VCVT_xv_T1_A field imm4 = 7 (PowerOfTwoMinusOne)
    // ISET: T32
    // Fields: op=0, sf=0, U=0, Vd=0, i=0, D=0, imm4=7, sx=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0847;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 8, boundary: PowerOfTwo }
/// power of 2 (2^3 = 8)
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_imm4_8_poweroftwo_840_eeba0848() {
    // Thumb encoding (32): 0xEEBA0848
    // Test aarch32_VCVT_xv_T1_A field imm4 = 8 (PowerOfTwo)
    // ISET: T32
    // Fields: i=0, imm4=8, D=0, sx=0, op=0, sf=0, U=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0848;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field imm4 0 +: 4`
/// Requirement: FieldBoundary { field: "imm4", value: 15, boundary: Max }
/// maximum immediate (15)
#[test]
fn test_aarch32_vcvt_xv_t1_a_field_imm4_15_max_840_eeba084f() {
    // Thumb encoding (32): 0xEEBA084F
    // Test aarch32_VCVT_xv_T1_A field imm4 = 15 (Max)
    // ISET: T32
    // Fields: sf=0, Vd=0, U=0, op=0, sx=0, i=0, D=0, imm4=15
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA084F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcvt_xv_t1_a_combo_0_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A field combination: D=0, op=0, U=0, Vd=0, sf=0, sx=0, i=0, imm4=0
    // ISET: T32
    // Fields: D=0, Vd=0, i=0, sf=0, op=0, imm4=0, U=0, sx=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field sf = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "sf", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_xv_t1_a_special_sf_0_size_variant_0_2112_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A special value sf = 0 (Size variant 0)
    // ISET: T32
    // Fields: D=0, op=0, sf=0, imm4=0, i=0, sx=0, U=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field sf = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "sf", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_xv_t1_a_special_sf_1_size_variant_1_2112_eeba0940() {
    // Thumb encoding (32): 0xEEBA0940
    // Test aarch32_VCVT_xv_T1_A special value sf = 1 (Size variant 1)
    // ISET: T32
    // Fields: D=0, Vd=0, i=0, op=0, sf=1, imm4=0, sx=0, U=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0940;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field sf = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "sf", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvt_xv_t1_a_special_sf_2_size_variant_2_2112_eeba0a40() {
    // Thumb encoding (32): 0xEEBA0A40
    // Test aarch32_VCVT_xv_T1_A special value sf = 2 (Size variant 2)
    // ISET: T32
    // Fields: Vd=0, imm4=0, U=0, sx=0, i=0, D=0, op=0, sf=2
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0A40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `field sf = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "sf", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvt_xv_t1_a_special_sf_3_size_variant_3_2112_eeba0b40() {
    // Thumb encoding (32): 0xEEBA0B40
    // Test aarch32_VCVT_xv_T1_A special value sf = 3 (Size variant 3)
    // ISET: T32
    // Fields: op=0, D=0, Vd=0, U=0, imm4=0, sf=3, sx=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0B40;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sf" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sf" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"sf\" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"sf\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xv_t1_a_invalid_0_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sf" }), rhs: Binary { op: Or, lhs: LitBits([false, false]), rhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sf" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } } } }
    // ISET: T32
    // Fields: Vd=0, imm4=0, sx=0, i=0, sf=0, op=0, U=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_xv_t1_a_invalid_1_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: sf=0, sx=0, D=0, Vd=0, U=0, i=0, imm4=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sf" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"sf\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_xv_t1_a_invalid_2_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "sf" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: D=0, op=0, Vd=0, sf=0, sx=0, imm4=0, U=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_xv_t1_a_invalid_3_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: U=0, op=0, sf=0, D=0, i=0, Vd=0, imm4=0, sx=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `Binary { op: Lt, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "frac_bits" }), rhs: LitInt(0) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Lt, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"frac_bits\" }), rhs: LitInt(0) }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_xv_t1_a_invalid_4_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A invalid encoding: Binary { op: Lt, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "frac_bits" }), rhs: LitInt(0) }
    // ISET: T32
    // Fields: U=0, sf=0, sx=0, D=0, Vd=0, imm4=0, op=0, i=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVT_xv_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_xv_t1_a_invalid_5_840_eeba0840() {
    // Thumb encoding (32): 0xEEBA0840
    // Test aarch32_VCVT_xv_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: sx=0, D=0, sf=0, i=0, Vd=0, imm4=0, U=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEBA0840;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

// ============================================================================
// aarch32_VCVT_is_A Tests
// ============================================================================

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_is_a1_a_field_d_0_min_600_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: size=0, M=0, D=0, op=0, Vm=0, Vd=0, Q=0
    let encoding: u32 = 0xF3B30600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_is_a1_a_field_d_1_max_600_f3f30600() {
    // Encoding: 0xF3F30600
    // Test aarch32_VCVT_is_A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: size=0, D=1, Q=0, Vd=0, M=0, op=0, Vm=0
    let encoding: u32 = 0xF3F30600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_is_a1_a_field_size_0_min_600_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A field size = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, Q=0, size=0, M=0, op=0, Vm=0, D=0
    let encoding: u32 = 0xF3B30600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_is_a1_a_field_size_1_poweroftwo_600_f3b70600() {
    // Encoding: 0xF3B70600
    // Test aarch32_VCVT_is_A1_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, size=1, Vd=0, M=0, Vm=0, Q=0, op=0
    let encoding: u32 = 0xF3B70600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvt_is_a1_a_field_size_2_poweroftwo_600_f3bb0600() {
    // Encoding: 0xF3BB0600
    // Test aarch32_VCVT_is_A1_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: Q=0, op=0, D=0, M=0, Vm=0, Vd=0, size=2
    let encoding: u32 = 0xF3BB0600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvt_is_a1_a_field_size_3_max_600_f3bf0600() {
    // Encoding: 0xF3BF0600
    // Test aarch32_VCVT_is_A1_A field size = 3 (Max)
    // ISET: A32
    // Fields: op=0, D=0, Q=0, Vd=0, size=3, Vm=0, M=0
    let encoding: u32 = 0xF3BF0600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_is_a1_a_field_vd_0_min_600_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: M=0, D=0, Vd=0, Vm=0, size=0, op=0, Q=0
    let encoding: u32 = 0xF3B30600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_is_a1_a_field_vd_1_poweroftwo_600_f3b31600() {
    // Encoding: 0xF3B31600
    // Test aarch32_VCVT_is_A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vm=0, op=0, size=0, Q=0, M=0, Vd=1
    let encoding: u32 = 0xF3B31600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field op 7 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_is_a1_a_field_op_0_min_600_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A field op = 0 (Min)
    // ISET: A32
    // Fields: D=0, M=0, Vd=0, Vm=0, op=0, Q=0, size=0
    let encoding: u32 = 0xF3B30600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field op 7 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_vcvt_is_a1_a_field_op_1_poweroftwo_600_f3b30680() {
    // Encoding: 0xF3B30680
    // Test aarch32_VCVT_is_A1_A field op = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vm=0, Q=0, size=0, op=1, M=0, Vd=0
    let encoding: u32 = 0xF3B30680;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field op 7 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_vcvt_is_a1_a_field_op_3_max_600_f3b30780() {
    // Encoding: 0xF3B30780
    // Test aarch32_VCVT_is_A1_A field op = 3 (Max)
    // ISET: A32
    // Fields: Vm=0, size=0, Vd=0, D=0, op=3, Q=0, M=0
    let encoding: u32 = 0xF3B30780;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_is_a1_a_field_q_0_min_600_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A field Q = 0 (Min)
    // ISET: A32
    // Fields: size=0, D=0, Q=0, M=0, Vm=0, Vd=0, op=0
    let encoding: u32 = 0xF3B30600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_is_a1_a_field_q_1_max_600_f3b30640() {
    // Encoding: 0xF3B30640
    // Test aarch32_VCVT_is_A1_A field Q = 1 (Max)
    // ISET: A32
    // Fields: Q=1, M=0, Vm=0, D=0, size=0, Vd=0, op=0
    let encoding: u32 = 0xF3B30640;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_is_a1_a_field_m_0_min_600_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: D=0, size=0, Vd=0, Q=0, Vm=0, M=0, op=0
    let encoding: u32 = 0xF3B30600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_is_a1_a_field_m_1_max_600_f3b30620() {
    // Encoding: 0xF3B30620
    // Test aarch32_VCVT_is_A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: op=0, size=0, Q=0, M=1, D=0, Vm=0, Vd=0
    let encoding: u32 = 0xF3B30620;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_is_a1_a_field_vm_0_min_600_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vd=0, size=0, Q=0, M=0, Vm=0, op=0
    let encoding: u32 = 0xF3B30600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_is_a1_a_field_vm_1_poweroftwo_600_f3b30601() {
    // Encoding: 0xF3B30601
    // Test aarch32_VCVT_is_A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: Vm=1, Q=0, D=0, M=0, Vd=0, size=0, op=0
    let encoding: u32 = 0xF3B30601;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcvt_is_a1_a_combo_0_600_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A field combination: D=0, size=0, Vd=0, op=0, Q=0, M=0, Vm=0
    // ISET: A32
    // Fields: D=0, size=0, Vd=0, op=0, Q=0, M=0, Vm=0
    let encoding: u32 = 0xF3B30600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_is_a1_a_special_size_0_size_variant_0_1536_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: D=0, Vd=0, Vm=0, Q=0, M=0, size=0, op=0
    let encoding: u32 = 0xF3B30600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_is_a1_a_special_size_1_size_variant_1_1536_f3b70600() {
    // Encoding: 0xF3B70600
    // Test aarch32_VCVT_is_A1_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: M=0, op=0, Vm=0, size=1, Vd=0, D=0, Q=0
    let encoding: u32 = 0xF3B70600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvt_is_a1_a_special_size_2_size_variant_2_1536_f3bb0600() {
    // Encoding: 0xF3BB0600
    // Test aarch32_VCVT_is_A1_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: Vm=0, Vd=0, Q=0, size=2, D=0, op=0, M=0
    let encoding: u32 = 0xF3BB0600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvt_is_a1_a_special_size_3_size_variant_3_1536_f3bf0600() {
    // Encoding: 0xF3BF0600
    // Test aarch32_VCVT_is_A1_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: D=0, op=0, M=0, Q=0, Vm=0, Vd=0, size=3
    let encoding: u32 = 0xF3BF0600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_is_a1_a_special_q_0_size_variant_0_1536_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A special value Q = 0 (Size variant 0)
    // ISET: A32
    // Fields: op=0, size=0, Q=0, M=0, D=0, Vm=0, Vd=0
    let encoding: u32 = 0xF3B30600;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_is_a1_a_special_q_1_size_variant_1_1536_f3b30640() {
    // Encoding: 0xF3B30640
    // Test aarch32_VCVT_is_A1_A special value Q = 1 (Size variant 1)
    // ISET: A32
    // Fields: op=0, M=0, Vm=0, D=0, Vd=0, Q=1, size=0
    let encoding: u32 = 0xF3B30640;
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

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_is_a1_a_invalid_0_600_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: A32
    // Fields: Vd=0, D=0, Q=0, M=0, Vm=0, op=0, size=0
    let encoding: u32 = 0xF3B30600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_is_a1_a_invalid_1_600_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: size=0, Q=0, M=0, Vm=0, op=0, D=0, Vd=0
    let encoding: u32 = 0xF3B30600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `Binary { op: Or, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }, rhs: InSet { expr: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), elements: [Single(LitBits([false, false])), Single(LitBits([true, true]))] } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Or, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } }, rhs: InSet { expr: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), elements: [Single(LitBits([false, false])), Single(LitBits([true, true]))] } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_is_a1_a_invalid_2_600_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A invalid encoding: Binary { op: Or, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }, rhs: InSet { expr: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), elements: [Single(LitBits([false, false])), Single(LitBits([true, true]))] } }
    // ISET: A32
    // Fields: Vd=0, op=0, M=0, D=0, Vm=0, Q=0, size=0
    let encoding: u32 = 0xF3B30600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_is_a1_a_invalid_3_600_f3b30600() {
    // Encoding: 0xF3B30600
    // Test aarch32_VCVT_is_A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: Vm=0, op=0, Vd=0, D=0, size=0, Q=0, M=0
    let encoding: u32 = 0xF3B30600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_is_t1_a_field_d_0_min_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A field D = 0 (Min)
    // ISET: T32
    // Fields: size=0, Vd=0, D=0, op=0, Q=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_is_t1_a_field_d_1_max_600_fff30600() {
    // Thumb encoding (32): 0xFFF30600
    // Test aarch32_VCVT_is_T1_A field D = 1 (Max)
    // ISET: T32
    // Fields: op=0, D=1, M=0, Vm=0, size=0, Q=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFF30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_is_t1_a_field_size_0_min_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A field size = 0 (Min)
    // ISET: T32
    // Fields: D=0, Vd=0, op=0, size=0, Q=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_is_t1_a_field_size_1_poweroftwo_600_ffb70600() {
    // Thumb encoding (32): 0xFFB70600
    // Test aarch32_VCVT_is_T1_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, Vm=0, op=0, size=1, Vd=0, M=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB70600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvt_is_t1_a_field_size_2_poweroftwo_600_ffbb0600() {
    // Thumb encoding (32): 0xFFBB0600
    // Test aarch32_VCVT_is_T1_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: size=2, Vd=0, M=0, Vm=0, D=0, op=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFBB0600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvt_is_t1_a_field_size_3_max_600_ffbf0600() {
    // Thumb encoding (32): 0xFFBF0600
    // Test aarch32_VCVT_is_T1_A field size = 3 (Max)
    // ISET: T32
    // Fields: op=0, Q=0, size=3, D=0, M=0, Vm=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFBF0600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_is_t1_a_field_vd_0_min_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: op=0, Vd=0, Q=0, size=0, M=0, Vm=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_is_t1_a_field_vd_1_poweroftwo_600_ffb31600() {
    // Thumb encoding (32): 0xFFB31600
    // Test aarch32_VCVT_is_T1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, size=0, Vd=1, Q=0, M=0, op=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB31600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field op 7 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_is_t1_a_field_op_0_min_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A field op = 0 (Min)
    // ISET: T32
    // Fields: M=0, Vm=0, op=0, size=0, D=0, Q=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field op 7 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch32_vcvt_is_t1_a_field_op_1_poweroftwo_600_ffb30680() {
    // Thumb encoding (32): 0xFFB30680
    // Test aarch32_VCVT_is_T1_A field op = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vd=0, op=1, M=0, size=0, Q=0, D=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field op 7 +: 2`
/// Requirement: FieldBoundary { field: "op", value: 3, boundary: Max }
/// maximum value (3)
#[test]
fn test_aarch32_vcvt_is_t1_a_field_op_3_max_600_ffb30780() {
    // Thumb encoding (32): 0xFFB30780
    // Test aarch32_VCVT_is_T1_A field op = 3 (Max)
    // ISET: T32
    // Fields: Vm=0, Vd=0, Q=0, size=0, D=0, op=3, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30780;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_is_t1_a_field_q_0_min_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A field Q = 0 (Min)
    // ISET: T32
    // Fields: op=0, size=0, D=0, Vd=0, M=0, Vm=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field Q 6 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_is_t1_a_field_q_1_max_600_ffb30640() {
    // Thumb encoding (32): 0xFFB30640
    // Test aarch32_VCVT_is_T1_A field Q = 1 (Max)
    // ISET: T32
    // Fields: M=0, D=0, Vm=0, op=0, size=0, Vd=0, Q=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30640;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_is_t1_a_field_m_0_min_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A field M = 0 (Min)
    // ISET: T32
    // Fields: op=0, Vd=0, M=0, D=0, Q=0, size=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_is_t1_a_field_m_1_max_600_ffb30620() {
    // Thumb encoding (32): 0xFFB30620
    // Test aarch32_VCVT_is_T1_A field M = 1 (Max)
    // ISET: T32
    // Fields: Vm=0, size=0, Vd=0, D=0, op=0, Q=0, M=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_is_t1_a_field_vm_0_min_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: op=0, size=0, M=0, Vm=0, D=0, Vd=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_is_t1_a_field_vm_1_poweroftwo_600_ffb30601() {
    // Thumb encoding (32): 0xFFB30601
    // Test aarch32_VCVT_is_T1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: M=0, Vd=0, D=0, op=0, Vm=1, Q=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30601;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcvt_is_t1_a_combo_0_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A field combination: D=0, size=0, Vd=0, op=0, Q=0, M=0, Vm=0
    // ISET: T32
    // Fields: Q=0, M=0, Vm=0, size=0, op=0, Vd=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_is_t1_a_special_size_0_size_variant_0_1536_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: op=0, Q=0, M=0, Vm=0, Vd=0, D=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_is_t1_a_special_size_1_size_variant_1_1536_ffb70600() {
    // Thumb encoding (32): 0xFFB70600
    // Test aarch32_VCVT_is_T1_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: Q=0, size=1, Vm=0, D=0, op=0, Vd=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB70600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvt_is_t1_a_special_size_2_size_variant_2_1536_ffbb0600() {
    // Thumb encoding (32): 0xFFBB0600
    // Test aarch32_VCVT_is_T1_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: M=0, Q=0, Vm=0, size=2, D=0, Vd=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFBB0600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvt_is_t1_a_special_size_3_size_variant_3_1536_ffbf0600() {
    // Thumb encoding (32): 0xFFBF0600
    // Test aarch32_VCVT_is_T1_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: op=0, Q=0, M=0, D=0, Vd=0, Vm=0, size=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFBF0600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_is_t1_a_special_q_0_size_variant_0_1536_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A special value Q = 0 (Size variant 0)
    // ISET: T32
    // Fields: D=0, op=0, size=0, Q=0, M=0, Vd=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_is_t1_a_special_q_1_size_variant_1_1536_ffb30640() {
    // Thumb encoding (32): 0xFFB30640
    // Test aarch32_VCVT_is_T1_A special value Q = 1 (Size variant 1)
    // ISET: T32
    // Fields: M=0, Vm=0, D=0, Q=1, size=0, Vd=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30640;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"Q\" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_is_t1_a_invalid_0_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "Q" }), rhs: Binary { op: And, lhs: LitBits([true]), rhs: Binary { op: Eq, lhs: Binary { op: Eq, lhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] }, rhs: Binary { op: Or, lhs: LitBits([true]), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } } }, rhs: LitBits([true]) } } }
    // ISET: T32
    // Fields: Vd=0, Vm=0, Q=0, D=0, M=0, size=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_is_t1_a_invalid_1_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: D=0, op=0, Q=0, M=0, Vd=0, Vm=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `Binary { op: Or, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }, rhs: InSet { expr: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), elements: [Single(LitBits([false, false])), Single(LitBits([true, true]))] } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Or, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: \"HaveFP16Ext\" }, args: [] } } } }, rhs: InSet { expr: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), elements: [Single(LitBits([false, false])), Single(LitBits([true, true]))] } }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_is_t1_a_invalid_2_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A invalid encoding: Binary { op: Or, lhs: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Unary { op: Not, operand: Call { name: QualifiedIdentifier { qualifier: Any, name: "HaveFP16Ext" }, args: [] } } } }, rhs: InSet { expr: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), elements: [Single(LitBits([false, false])), Single(LitBits([true, true]))] } }
    // ISET: T32
    // Fields: Q=0, D=0, op=0, M=0, size=0, Vd=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_is_t1_a_invalid_3_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: M=0, size=0, Vm=0, D=0, Vd=0, op=0, Q=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: \"InITBlock\" }, args: [] } } }" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_is_t1_a_invalid_4_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A invalid encoding: Binary { op: Eq, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: Binary { op: And, lhs: LitBits([false, true]), rhs: Call { name: QualifiedIdentifier { qualifier: Any, name: "InITBlock" }, args: [] } } }
    // ISET: T32
    // Fields: Q=0, D=0, M=0, size=0, Vd=0, Vm=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVT_is_T1_A
/// ASL: `Unconditional UNPREDICTABLE`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNPREDICTABLE" }
/// triggers Unpredictable
#[test]
fn test_aarch32_vcvt_is_t1_a_invalid_5_600_ffb30600() {
    // Thumb encoding (32): 0xFFB30600
    // Test aarch32_VCVT_is_T1_A invalid encoding: Unconditional UNPREDICTABLE
    // ISET: T32
    // Fields: Vd=0, size=0, D=0, op=0, Q=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB30600;
    write_insn(&mut cpu, 0, encoding);
    let _ = cpu.step();
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOVZ X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// lower 16 bits (32)
#[test]
fn test_aarch32_vcvt_is_a1_a_movz_oracle_32_0_f3b34680() {
    // Test MOVZ 32-bit: lower 16 bits (oracle)
    // Encoding: 0xF3B34680
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B34680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1234, "W0 should be 0x00001234");
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOVZ X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// lower 16 bits (64)
#[test]
fn test_aarch32_vcvt_is_a1_a_movz_oracle_64_0_f3b34680() {
    // Test MOVZ 64-bit: lower 16 bits (oracle)
    // Encoding: 0xF3B34680
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B34680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1234, "X0 should be 0x0000000000001234");
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOVZ X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shifted 16 bits (32)
#[test]
fn test_aarch32_vcvt_is_a1_a_movz_oracle_32_1_f3b77fa0() {
    // Test MOVZ 32-bit: shifted 16 bits (oracle)
    // Encoding: 0xF3B77FA0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B77FA0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xABCD0000, "W0 should be 0xABCD0000");
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOVZ X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 16 bits (64)
#[test]
fn test_aarch32_vcvt_is_a1_a_movz_oracle_64_1_f3b77fa0() {
    // Test MOVZ 64-bit: shifted 16 bits (oracle)
    // Encoding: 0xF3B77FA0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B77FA0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xABCD0000,
        "X0 should be 0x00000000ABCD0000"
    );
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOVZ X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm16 (32)
#[test]
fn test_aarch32_vcvt_is_a1_a_movz_oracle_32_2_f3bfffe0() {
    // Test MOVZ 32-bit: max imm16 (oracle)
    // Encoding: 0xF3BFFFE0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3BFFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "W0 should be 0x0000FFFF");
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOVZ X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm16 (64)
#[test]
fn test_aarch32_vcvt_is_a1_a_movz_oracle_64_2_f3bfffe0() {
    // Test MOVZ 64-bit: max imm16 (oracle)
    // Encoding: 0xF3BFFFE0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3BFFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "X0 should be 0x000000000000FFFF");
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOVZ X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero imm16 (32)
#[test]
fn test_aarch32_vcvt_is_a1_a_movz_oracle_32_3_f3b30600() {
    // Test MOVZ 32-bit: zero imm16 (oracle)
    // Encoding: 0xF3B30600
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOVZ X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero imm16 (64)
#[test]
fn test_aarch32_vcvt_is_a1_a_movz_oracle_64_3_f3b30600() {
    // Test MOVZ 64-bit: zero imm16 (oracle)
    // Encoding: 0xF3B30600
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B30600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOVZ X0, #0x5678, LSL #32`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 32 bits (64)
#[test]
fn test_aarch32_vcvt_is_a1_a_movz_oracle_64_4_f3fbcf00() {
    // Test MOVZ 64-bit: shifted 32 bits (oracle)
    // Encoding: 0xF3FBCF00
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3FBCF00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000567800000000");
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOVZ X0, #0xDEAD, LSL #48`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 48 bits (64)
#[test]
fn test_aarch32_vcvt_is_a1_a_movz_oracle_64_5_f3fbd7a0() {
    // Test MOVZ 64-bit: shifted 48 bits (oracle)
    // Encoding: 0xF3FBD7A0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3FBD7A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xDEAD000000000000");
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOV R0, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate
#[test]
fn test_aarch32_vcvt_is_a1_a_a32_mov_imm_0_f3a0000a() {
    // Test A32 MOV: small immediate (oracle)
    // Encoding: 0xF3A0000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A0000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA, "R0 should be 0x0000000A");
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOV R0, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8
#[test]
fn test_aarch32_vcvt_is_a1_a_a32_mov_imm_1_f3a000ff() {
    // Test A32 MOV: max imm8 (oracle)
    // Encoding: 0xF3A000FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A000FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOV R0, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2
#[test]
fn test_aarch32_vcvt_is_a1_a_a32_mov_imm_2_f3a00180() {
    // Test A32 MOV: rotated by 2 (oracle)
    // Encoding: 0xF3A00180
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A00180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x20, "R0 should be 0x00000020");
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOV R0, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8
#[test]
fn test_aarch32_vcvt_is_a1_a_a32_mov_imm_3_f3a0040f() {
    // Test A32 MOV: rotated by 8 (oracle)
    // Encoding: 0xF3A0040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A0040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xF000000, "R0 should be 0x0F000000");
}

/// Provenance: aarch32_VCVT_is_A1_A
/// ASL: `MOV R0, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate
#[test]
fn test_aarch32_vcvt_is_a1_a_a32_mov_imm_4_f3a00000() {
    // Test A32 MOV: zero immediate (oracle)
    // Encoding: 0xF3A00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}

// ============================================================================
// aarch32_VCVT_ds_A Tests
// ============================================================================

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 0, boundary: Min }
/// condition EQ (equal)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_0_min_ac0_0eb708c0() {
    // Encoding: 0x0EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 0 (Min)
    // ISET: A32
    // Fields: cond=0, Vm=0, D=0, Vd=0, size=0, M=0
    let encoding: u32 = 0x0EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 1, boundary: PowerOfTwo }
/// condition NE (not equal)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_1_poweroftwo_ac0_1eb708c0() {
    // Encoding: 0x1EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, D=0, Vd=0, M=0, Vm=0, cond=1
    let encoding: u32 = 0x1EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 2, boundary: PowerOfTwo }
/// condition CS/HS (carry set)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_2_poweroftwo_ac0_2eb708c0() {
    // Encoding: 0x2EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, cond=2, Vm=0, M=0, Vd=0, D=0
    let encoding: u32 = 0x2EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 3, boundary: PowerOfTwo }
/// condition CC/LO (carry clear)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_3_poweroftwo_ac0_3eb708c0() {
    // Encoding: 0x3EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 3 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=3, Vm=0, size=0, D=0, Vd=0, M=0
    let encoding: u32 = 0x3EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 4, boundary: PowerOfTwo }
/// condition MI (minus/negative)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_4_poweroftwo_ac0_4eb708c0() {
    // Encoding: 0x4EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 4 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, D=0, M=0, size=0, Vm=0, cond=4
    let encoding: u32 = 0x4EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 5, boundary: PowerOfTwo }
/// condition PL (plus/positive)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_5_poweroftwo_ac0_5eb708c0() {
    // Encoding: 0x5EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 5 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, size=0, D=0, Vm=0, cond=5, M=0
    let encoding: u32 = 0x5EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 6, boundary: PowerOfTwo }
/// condition VS (overflow set)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_6_poweroftwo_ac0_6eb708c0() {
    // Encoding: 0x6EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 6 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, size=0, cond=6, D=0, Vm=0, M=0
    let encoding: u32 = 0x6EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 7, boundary: PowerOfTwo }
/// condition VC (overflow clear)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_7_poweroftwo_ac0_7eb708c0() {
    // Encoding: 0x7EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 7 (PowerOfTwo)
    // ISET: A32
    // Fields: Vd=0, cond=7, D=0, M=0, Vm=0, size=0
    let encoding: u32 = 0x7EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 8, boundary: PowerOfTwo }
/// condition HI (unsigned higher)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_8_poweroftwo_ac0_8eb708c0() {
    // Encoding: 0x8EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 8 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, Vd=0, M=0, Vm=0, cond=8, D=0
    let encoding: u32 = 0x8EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 9, boundary: PowerOfTwo }
/// condition LS (unsigned lower or same)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_9_poweroftwo_ac0_9eb708c0() {
    // Encoding: 0x9EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 9 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, Vd=0, Vm=0, M=0, D=0, cond=9
    let encoding: u32 = 0x9EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 10, boundary: PowerOfTwo }
/// condition GE (signed >=)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_10_poweroftwo_ac0_aeb708c0() {
    // Encoding: 0xAEB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 10 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=10, D=0, Vd=0, Vm=0, M=0, size=0
    let encoding: u32 = 0xAEB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 11, boundary: PowerOfTwo }
/// condition LT (signed <)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_11_poweroftwo_ac0_beb708c0() {
    // Encoding: 0xBEB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 11 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, M=0, Vd=0, size=0, Vm=0, cond=11
    let encoding: u32 = 0xBEB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 12, boundary: PowerOfTwo }
/// condition GT (signed >)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_12_poweroftwo_ac0_ceb708c0() {
    // Encoding: 0xCEB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 12 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, D=0, Vd=0, cond=12, Vm=0, M=0
    let encoding: u32 = 0xCEB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 13, boundary: PowerOfTwo }
/// condition LE (signed <=)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_13_poweroftwo_ac0_deb708c0() {
    // Encoding: 0xDEB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 13 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, cond=13, D=0, Vd=0, size=0, Vm=0
    let encoding: u32 = 0xDEB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 14, boundary: PowerOfTwo }
/// condition AL (always)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_14_poweroftwo_ac0_eeb708c0() {
    // Encoding: 0xEEB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 14 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=14, Vd=0, M=0, Vm=0, D=0, size=0
    let encoding: u32 = 0xEEB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond 28 +: 4`
/// Requirement: FieldBoundary { field: "cond", value: 15, boundary: Max }
/// condition NV (never, reserved)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_cond_15_max_ac0_feb708c0() {
    // Encoding: 0xFEB708C0
    // Test aarch32_VCVT_ds_T1A1_A field cond = 15 (Max)
    // ISET: A32
    // Fields: Vm=0, D=0, M=0, cond=15, Vd=0, size=0
    let encoding: u32 = 0xFEB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_d_0_min_ac0_0eb708c0() {
    // Encoding: 0x0EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: D=0, cond=0, size=0, Vm=0, M=0, Vd=0
    let encoding: u32 = 0x0EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_d_1_max_ac0_0ef708c0() {
    // Encoding: 0x0EF708C0
    // Test aarch32_VCVT_ds_T1A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: M=0, cond=0, Vd=0, size=0, Vm=0, D=1
    let encoding: u32 = 0x0EF708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_vd_0_min_ac0_0eb708c0() {
    // Encoding: 0x0EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vd=0, Vm=0, size=0, M=0, cond=0
    let encoding: u32 = 0x0EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_vd_1_poweroftwo_ac0_0eb718c0() {
    // Encoding: 0x0EB718C0
    // Test aarch32_VCVT_ds_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, size=0, cond=0, M=0, Vm=0, Vd=1
    let encoding: u32 = 0x0EB718C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_size_0_min_ac0_0eb708c0() {
    // Encoding: 0x0EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field size = 0 (Min)
    // ISET: A32
    // Fields: D=0, Vd=0, size=0, cond=0, M=0, Vm=0
    let encoding: u32 = 0x0EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_size_1_poweroftwo_ac0_0eb709c0() {
    // Encoding: 0x0EB709C0
    // Test aarch32_VCVT_ds_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, cond=0, D=0, Vm=0, Vd=0, size=1
    let encoding: u32 = 0x0EB709C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_size_2_poweroftwo_ac0_0eb70ac0() {
    // Encoding: 0x0EB70AC0
    // Test aarch32_VCVT_ds_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: D=0, Vd=0, size=2, M=0, Vm=0, cond=0
    let encoding: u32 = 0x0EB70AC0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_size_3_max_ac0_0eb70bc0() {
    // Encoding: 0x0EB70BC0
    // Test aarch32_VCVT_ds_T1A1_A field size = 3 (Max)
    // ISET: A32
    // Fields: D=0, size=3, M=0, Vm=0, Vd=0, cond=0
    let encoding: u32 = 0x0EB70BC0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_m_0_min_ac0_0eb708c0() {
    // Encoding: 0x0EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: Vm=0, cond=0, size=0, M=0, Vd=0, D=0
    let encoding: u32 = 0x0EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_m_1_max_ac0_0eb708e0() {
    // Encoding: 0x0EB708E0
    // Test aarch32_VCVT_ds_T1A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: D=0, size=0, Vd=0, cond=0, M=1, Vm=0
    let encoding: u32 = 0x0EB708E0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_vm_0_min_ac0_0eb708c0() {
    // Encoding: 0x0EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: cond=0, size=0, D=0, M=0, Vm=0, Vd=0
    let encoding: u32 = 0x0EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_vm_1_poweroftwo_ac0_0eb708c1() {
    // Encoding: 0x0EB708C1
    // Test aarch32_VCVT_ds_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: cond=0, size=0, Vm=1, Vd=0, D=0, M=0
    let encoding: u32 = 0x0EB708C1;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cond=0 (condition EQ (equal))
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_combo_0_ac0_0eb708c0() {
    // Encoding: 0x0EB708C0
    // Test aarch32_VCVT_ds_T1A1_A field combination: cond=0, D=0, Vd=0, size=0, M=0, Vm=0
    // ISET: A32
    // Fields: cond=0, D=0, Vm=0, size=0, Vd=0, M=0
    let encoding: u32 = 0x0EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 0 (Condition EQ)`
/// Requirement: FieldSpecial { field: "cond", value: 0, meaning: "Condition EQ" }
/// Condition EQ
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_0_condition_eq_2752_0eb708c0() {
    // Encoding: 0x0EB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 0 (Condition EQ)
    // ISET: A32
    // Fields: Vm=0, Vd=0, M=0, D=0, size=0, cond=0
    let encoding: u32 = 0x0EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 1 (Condition NE)`
/// Requirement: FieldSpecial { field: "cond", value: 1, meaning: "Condition NE" }
/// Condition NE
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_1_condition_ne_2752_1eb708c0() {
    // Encoding: 0x1EB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 1 (Condition NE)
    // ISET: A32
    // Fields: D=0, Vd=0, cond=1, Vm=0, size=0, M=0
    let encoding: u32 = 0x1EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 2 (Condition CS/HS)`
/// Requirement: FieldSpecial { field: "cond", value: 2, meaning: "Condition CS/HS" }
/// Condition CS/HS
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_2_condition_cs_hs_2752_2eb708c0() {
    // Encoding: 0x2EB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 2 (Condition CS/HS)
    // ISET: A32
    // Fields: size=0, Vd=0, cond=2, M=0, D=0, Vm=0
    let encoding: u32 = 0x2EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 3 (Condition CC/LO)`
/// Requirement: FieldSpecial { field: "cond", value: 3, meaning: "Condition CC/LO" }
/// Condition CC/LO
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_3_condition_cc_lo_2752_3eb708c0() {
    // Encoding: 0x3EB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 3 (Condition CC/LO)
    // ISET: A32
    // Fields: cond=3, D=0, Vm=0, size=0, Vd=0, M=0
    let encoding: u32 = 0x3EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 4 (Condition MI)`
/// Requirement: FieldSpecial { field: "cond", value: 4, meaning: "Condition MI" }
/// Condition MI
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_4_condition_mi_2752_4eb708c0() {
    // Encoding: 0x4EB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 4 (Condition MI)
    // ISET: A32
    // Fields: Vd=0, cond=4, size=0, Vm=0, M=0, D=0
    let encoding: u32 = 0x4EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 5 (Condition PL)`
/// Requirement: FieldSpecial { field: "cond", value: 5, meaning: "Condition PL" }
/// Condition PL
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_5_condition_pl_2752_5eb708c0() {
    // Encoding: 0x5EB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 5 (Condition PL)
    // ISET: A32
    // Fields: D=0, cond=5, Vd=0, size=0, M=0, Vm=0
    let encoding: u32 = 0x5EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 6 (Condition VS)`
/// Requirement: FieldSpecial { field: "cond", value: 6, meaning: "Condition VS" }
/// Condition VS
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_6_condition_vs_2752_6eb708c0() {
    // Encoding: 0x6EB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 6 (Condition VS)
    // ISET: A32
    // Fields: Vm=0, cond=6, size=0, D=0, Vd=0, M=0
    let encoding: u32 = 0x6EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 7 (Condition VC)`
/// Requirement: FieldSpecial { field: "cond", value: 7, meaning: "Condition VC" }
/// Condition VC
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_7_condition_vc_2752_7eb708c0() {
    // Encoding: 0x7EB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 7 (Condition VC)
    // ISET: A32
    // Fields: cond=7, Vd=0, D=0, size=0, Vm=0, M=0
    let encoding: u32 = 0x7EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 8 (Condition HI)`
/// Requirement: FieldSpecial { field: "cond", value: 8, meaning: "Condition HI" }
/// Condition HI
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_8_condition_hi_2752_8eb708c0() {
    // Encoding: 0x8EB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 8 (Condition HI)
    // ISET: A32
    // Fields: size=0, M=0, Vm=0, D=0, cond=8, Vd=0
    let encoding: u32 = 0x8EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 9 (Condition LS)`
/// Requirement: FieldSpecial { field: "cond", value: 9, meaning: "Condition LS" }
/// Condition LS
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_9_condition_ls_2752_9eb708c0() {
    // Encoding: 0x9EB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 9 (Condition LS)
    // ISET: A32
    // Fields: Vd=0, size=0, cond=9, M=0, D=0, Vm=0
    let encoding: u32 = 0x9EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 10 (Condition GE)`
/// Requirement: FieldSpecial { field: "cond", value: 10, meaning: "Condition GE" }
/// Condition GE
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_10_condition_ge_2752_aeb708c0() {
    // Encoding: 0xAEB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 10 (Condition GE)
    // ISET: A32
    // Fields: M=0, Vd=0, cond=10, size=0, D=0, Vm=0
    let encoding: u32 = 0xAEB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 11 (Condition LT)`
/// Requirement: FieldSpecial { field: "cond", value: 11, meaning: "Condition LT" }
/// Condition LT
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_11_condition_lt_2752_beb708c0() {
    // Encoding: 0xBEB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 11 (Condition LT)
    // ISET: A32
    // Fields: cond=11, size=0, M=0, Vd=0, Vm=0, D=0
    let encoding: u32 = 0xBEB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 12 (Condition GT)`
/// Requirement: FieldSpecial { field: "cond", value: 12, meaning: "Condition GT" }
/// Condition GT
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_12_condition_gt_2752_ceb708c0() {
    // Encoding: 0xCEB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 12 (Condition GT)
    // ISET: A32
    // Fields: Vd=0, cond=12, D=0, Vm=0, size=0, M=0
    let encoding: u32 = 0xCEB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 13 (Condition LE)`
/// Requirement: FieldSpecial { field: "cond", value: 13, meaning: "Condition LE" }
/// Condition LE
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_13_condition_le_2752_deb708c0() {
    // Encoding: 0xDEB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 13 (Condition LE)
    // ISET: A32
    // Fields: cond=13, M=0, D=0, Vd=0, size=0, Vm=0
    let encoding: u32 = 0xDEB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 14 (Condition AL)`
/// Requirement: FieldSpecial { field: "cond", value: 14, meaning: "Condition AL" }
/// Condition AL
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_14_condition_al_2752_eeb708c0() {
    // Encoding: 0xEEB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 14 (Condition AL)
    // ISET: A32
    // Fields: size=0, D=0, Vd=0, Vm=0, M=0, cond=14
    let encoding: u32 = 0xEEB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field cond = 15 (Condition NV)`
/// Requirement: FieldSpecial { field: "cond", value: 15, meaning: "Condition NV" }
/// Condition NV
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_cond_15_condition_nv_2752_feb708c0() {
    // Encoding: 0xFEB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value cond = 15 (Condition NV)
    // ISET: A32
    // Fields: D=0, Vd=0, cond=15, size=0, M=0, Vm=0
    let encoding: u32 = 0xFEB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_size_0_size_variant_0_2752_0eb708c0() {
    // Encoding: 0x0EB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: cond=0, Vd=0, size=0, Vm=0, M=0, D=0
    let encoding: u32 = 0x0EB708C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_size_1_size_variant_1_2752_0eb709c0() {
    // Encoding: 0x0EB709C0
    // Test aarch32_VCVT_ds_T1A1_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: cond=0, M=0, Vd=0, size=1, Vm=0, D=0
    let encoding: u32 = 0x0EB709C0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_size_2_size_variant_2_2752_0eb70ac0() {
    // Encoding: 0x0EB70AC0
    // Test aarch32_VCVT_ds_T1A1_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: Vd=0, Vm=0, size=2, M=0, cond=0, D=0
    let encoding: u32 = 0x0EB70AC0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_size_3_size_variant_3_2752_0eb70bc0() {
    // Encoding: 0x0EB70BC0
    // Test aarch32_VCVT_ds_T1A1_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: D=0, size=3, Vm=0, Vd=0, M=0, cond=0
    let encoding: u32 = 0x0EB70BC0;
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

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_d_0_min_ac0_eeb708c0() {
    // Thumb encoding (32): 0xEEB708C0
    // Test aarch32_VCVT_ds_T1A1_A field D = 0 (Min)
    // ISET: T32
    // Fields: Vm=0, Vd=0, size=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB708C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_d_1_max_ac0_eef708c0() {
    // Thumb encoding (32): 0xEEF708C0
    // Test aarch32_VCVT_ds_T1A1_A field D = 1 (Max)
    // ISET: T32
    // Fields: Vd=0, size=0, D=1, Vm=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEF708C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_vd_0_min_ac0_eeb708c0() {
    // Thumb encoding (32): 0xEEB708C0
    // Test aarch32_VCVT_ds_T1A1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: size=0, D=0, M=0, Vd=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB708C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_vd_1_poweroftwo_ac0_eeb718c0() {
    // Thumb encoding (32): 0xEEB718C0
    // Test aarch32_VCVT_ds_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vd=1, size=0, D=0, Vm=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB718C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_size_0_min_ac0_eeb708c0() {
    // Thumb encoding (32): 0xEEB708C0
    // Test aarch32_VCVT_ds_T1A1_A field size = 0 (Min)
    // ISET: T32
    // Fields: M=0, Vd=0, size=0, D=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB708C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_size_1_poweroftwo_ac0_eeb709c0() {
    // Thumb encoding (32): 0xEEB709C0
    // Test aarch32_VCVT_ds_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, Vm=0, M=0, size=1, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB709C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_size_2_poweroftwo_ac0_eeb70ac0() {
    // Thumb encoding (32): 0xEEB70AC0
    // Test aarch32_VCVT_ds_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: Vm=0, Vd=0, M=0, size=2, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB70AC0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size 8 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_size_3_max_ac0_eeb70bc0() {
    // Thumb encoding (32): 0xEEB70BC0
    // Test aarch32_VCVT_ds_T1A1_A field size = 3 (Max)
    // ISET: T32
    // Fields: Vm=0, D=0, size=3, M=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB70BC0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_m_0_min_ac0_eeb708c0() {
    // Thumb encoding (32): 0xEEB708C0
    // Test aarch32_VCVT_ds_T1A1_A field M = 0 (Min)
    // ISET: T32
    // Fields: M=0, D=0, Vm=0, size=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB708C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_m_1_max_ac0_eeb708e0() {
    // Thumb encoding (32): 0xEEB708E0
    // Test aarch32_VCVT_ds_T1A1_A field M = 1 (Max)
    // ISET: T32
    // Fields: Vm=0, D=0, size=0, Vd=0, M=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB708E0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_vm_0_min_ac0_eeb708c0() {
    // Thumb encoding (32): 0xEEB708C0
    // Test aarch32_VCVT_ds_T1A1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: size=0, M=0, Vm=0, Vd=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB708C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_field_vm_1_poweroftwo_ac0_eeb708c1() {
    // Thumb encoding (32): 0xEEB708C1
    // Test aarch32_VCVT_ds_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: Vd=0, D=0, size=0, Vm=1, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB708C1;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_combo_0_ac0_eeb708c0() {
    // Thumb encoding (32): 0xEEB708C0
    // Test aarch32_VCVT_ds_T1A1_A field combination: D=0, Vd=0, size=0, M=0, Vm=0
    // ISET: T32
    // Fields: Vd=0, M=0, size=0, D=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB708C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_size_0_size_variant_0_2752_eeb708c0() {
    // Thumb encoding (32): 0xEEB708C0
    // Test aarch32_VCVT_ds_T1A1_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: Vd=0, size=0, D=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB708C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_size_1_size_variant_1_2752_eeb709c0() {
    // Thumb encoding (32): 0xEEB709C0
    // Test aarch32_VCVT_ds_T1A1_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: Vd=0, size=1, Vm=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB709C0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_size_2_size_variant_2_2752_eeb70ac0() {
    // Thumb encoding (32): 0xEEB70AC0
    // Test aarch32_VCVT_ds_T1A1_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: Vd=0, D=0, Vm=0, M=0, size=2
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB70AC0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_ds_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvt_ds_t1a1_a_special_size_3_size_variant_3_2752_eeb70bc0() {
    // Thumb encoding (32): 0xEEB70BC0
    // Test aarch32_VCVT_ds_T1A1_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: Vd=0, D=0, M=0, Vm=0, size=3
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xEEB70BC0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

// ============================================================================
// aarch32_VCVT_hs_A Tests
// ============================================================================

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_d_0_min_600_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A field D = 0 (Min)
    // ISET: A32
    // Fields: op=0, Vm=0, M=0, D=0, size=0, Vd=0
    let encoding: u32 = 0xF3B20600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_d_1_max_600_f3f20600() {
    // Encoding: 0xF3F20600
    // Test aarch32_VCVT_hs_T1A1_A field D = 1 (Max)
    // ISET: A32
    // Fields: op=0, Vd=0, Vm=0, size=0, M=0, D=1
    let encoding: u32 = 0xF3F20600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_size_0_min_600_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A field size = 0 (Min)
    // ISET: A32
    // Fields: size=0, op=0, D=0, M=0, Vm=0, Vd=0
    let encoding: u32 = 0xF3B20600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_size_1_poweroftwo_600_f3b60600() {
    // Encoding: 0xF3B60600
    // Test aarch32_VCVT_hs_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, Vd=0, size=1, op=0, D=0, Vm=0
    let encoding: u32 = 0xF3B60600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_size_2_poweroftwo_600_f3ba0600() {
    // Encoding: 0xF3BA0600
    // Test aarch32_VCVT_hs_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: A32
    // Fields: op=0, Vm=0, M=0, D=0, size=2, Vd=0
    let encoding: u32 = 0xF3BA0600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_size_3_max_600_f3be0600() {
    // Encoding: 0xF3BE0600
    // Test aarch32_VCVT_hs_T1A1_A field size = 3 (Max)
    // ISET: A32
    // Fields: op=0, size=3, Vd=0, Vm=0, D=0, M=0
    let encoding: u32 = 0xF3BE0600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_vd_0_min_600_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A field Vd = 0 (Min)
    // ISET: A32
    // Fields: size=0, op=0, Vm=0, M=0, D=0, Vd=0
    let encoding: u32 = 0xF3B20600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_vd_1_poweroftwo_600_f3b21600() {
    // Encoding: 0xF3B21600
    // Test aarch32_VCVT_hs_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: M=0, Vm=0, op=0, D=0, Vd=1, size=0
    let encoding: u32 = 0xF3B21600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field op 8 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_op_0_min_600_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A field op = 0 (Min)
    // ISET: A32
    // Fields: size=0, Vd=0, D=0, M=0, Vm=0, op=0
    let encoding: u32 = 0xF3B20600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field op 8 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_op_1_max_600_f3b20700() {
    // Encoding: 0xF3B20700
    // Test aarch32_VCVT_hs_T1A1_A field op = 1 (Max)
    // ISET: A32
    // Fields: M=0, Vd=0, size=0, op=1, Vm=0, D=0
    let encoding: u32 = 0xF3B20700;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_m_0_min_600_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A field M = 0 (Min)
    // ISET: A32
    // Fields: Vd=0, op=0, size=0, D=0, Vm=0, M=0
    let encoding: u32 = 0xF3B20600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_m_1_max_600_f3b20620() {
    // Encoding: 0xF3B20620
    // Test aarch32_VCVT_hs_T1A1_A field M = 1 (Max)
    // ISET: A32
    // Fields: D=0, M=1, size=0, op=0, Vm=0, Vd=0
    let encoding: u32 = 0xF3B20620;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_vm_0_min_600_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A field Vm = 0 (Min)
    // ISET: A32
    // Fields: D=0, size=0, op=0, Vd=0, M=0, Vm=0
    let encoding: u32 = 0xF3B20600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_vm_1_poweroftwo_600_f3b20601() {
    // Encoding: 0xF3B20601
    // Test aarch32_VCVT_hs_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: A32
    // Fields: size=0, M=0, Vd=0, Vm=1, D=0, op=0
    let encoding: u32 = 0xF3B20601;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_combo_0_600_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A field combination: D=0, size=0, Vd=0, op=0, M=0, Vm=0
    // ISET: A32
    // Fields: Vd=0, size=0, op=0, M=0, D=0, Vm=0
    let encoding: u32 = 0xF3B20600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_special_size_0_size_variant_0_1536_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A special value size = 0 (Size variant 0)
    // ISET: A32
    // Fields: M=0, Vm=0, D=0, Vd=0, size=0, op=0
    let encoding: u32 = 0xF3B20600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_special_size_1_size_variant_1_1536_f3b60600() {
    // Encoding: 0xF3B60600
    // Test aarch32_VCVT_hs_T1A1_A special value size = 1 (Size variant 1)
    // ISET: A32
    // Fields: op=0, D=0, size=1, Vd=0, M=0, Vm=0
    let encoding: u32 = 0xF3B60600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_special_size_2_size_variant_2_1536_f3ba0600() {
    // Encoding: 0xF3BA0600
    // Test aarch32_VCVT_hs_T1A1_A special value size = 2 (Size variant 2)
    // ISET: A32
    // Fields: op=0, Vm=0, D=0, M=0, Vd=0, size=2
    let encoding: u32 = 0xF3BA0600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_special_size_3_size_variant_3_1536_f3be0600() {
    // Encoding: 0xF3BE0600
    // Test aarch32_VCVT_hs_T1A1_A special value size = 3 (Size variant 3)
    // ISET: A32
    // Fields: op=0, Vm=0, size=3, Vd=0, D=0, M=0
    let encoding: u32 = 0xF3BE0600;
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

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `Binary { op: Ne, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: LitBits([false, true]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Ne, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: LitBits([false, true]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_invalid_0_600_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A invalid encoding: Binary { op: Ne, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: LitBits([false, true]) }
    // ISET: A32
    // Fields: M=0, D=0, op=0, Vm=0, Vd=0, size=0
    let encoding: u32 = 0xF3B20600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_invalid_1_600_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: Vd=0, M=0, D=0, Vm=0, size=0, op=0
    let encoding: u32 = 0xF3B20600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: And, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "half_to_single" }), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] } }, rhs: LitBits([true]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: And, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"half_to_single\" }), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] } }, rhs: LitBits([true]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_invalid_2_600_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: And, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "half_to_single" }), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] } }, rhs: LitBits([true]) }
    // ISET: A32
    // Fields: Vm=0, Vd=0, op=0, D=0, size=0, M=0
    let encoding: u32 = 0xF3B20600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_invalid_3_600_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: Vm=0, D=0, Vd=0, size=0, op=0, M=0
    let encoding: u32 = 0xF3B20600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: And, lhs: Unary { op: Not, operand: Var(QualifiedIdentifier { qualifier: Any, name: "half_to_single" }) }, rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } }, rhs: LitBits([true]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: And, lhs: Unary { op: Not, operand: Var(QualifiedIdentifier { qualifier: Any, name: \"half_to_single\" }) }, rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } }, rhs: LitBits([true]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_invalid_4_600_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: And, lhs: Unary { op: Not, operand: Var(QualifiedIdentifier { qualifier: Any, name: "half_to_single" }) }, rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } }, rhs: LitBits([true]) }
    // ISET: A32
    // Fields: Vd=0, op=0, size=0, D=0, Vm=0, M=0
    let encoding: u32 = 0xF3B20600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_invalid_5_600_f3b20600() {
    // Encoding: 0xF3B20600
    // Test aarch32_VCVT_hs_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: A32
    // Fields: Vm=0, D=0, Vd=0, size=0, M=0, op=0
    let encoding: u32 = 0xF3B20600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for A32 encoding 0x{:08X}",
        encoding
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_d_0_min_600_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A field D = 0 (Min)
    // ISET: T32
    // Fields: D=0, M=0, op=0, size=0, Vd=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field D 22 +: 1`
/// Requirement: FieldBoundary { field: "D", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_d_1_max_600_fff20600() {
    // Thumb encoding (32): 0xFFF20600
    // Test aarch32_VCVT_hs_T1A1_A field D = 1 (Max)
    // ISET: T32
    // Fields: Vd=0, M=0, op=0, size=0, Vm=0, D=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFF20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_size_0_min_600_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A field size = 0 (Min)
    // ISET: T32
    // Fields: Vm=0, M=0, op=0, D=0, size=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 1, boundary: PowerOfTwo }
/// 16-bit / halfword size
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_size_1_poweroftwo_600_ffb60600() {
    // Thumb encoding (32): 0xFFB60600
    // Test aarch32_VCVT_hs_T1A1_A field size = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: op=0, M=0, Vd=0, D=0, Vm=0, size=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB60600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 2, boundary: PowerOfTwo }
/// 32-bit / word size
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_size_2_poweroftwo_600_ffba0600() {
    // Thumb encoding (32): 0xFFBA0600
    // Test aarch32_VCVT_hs_T1A1_A field size = 2 (PowerOfTwo)
    // ISET: T32
    // Fields: size=2, Vm=0, D=0, M=0, Vd=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFBA0600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size 18 +: 2`
/// Requirement: FieldBoundary { field: "size", value: 3, boundary: Max }
/// 64-bit / doubleword size
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_size_3_max_600_ffbe0600() {
    // Thumb encoding (32): 0xFFBE0600
    // Test aarch32_VCVT_hs_T1A1_A field size = 3 (Max)
    // ISET: T32
    // Fields: Vd=0, size=3, op=0, M=0, D=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFBE0600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_vd_0_min_600_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A field Vd = 0 (Min)
    // ISET: T32
    // Fields: size=0, Vd=0, D=0, Vm=0, op=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field Vd 12 +: 4`
/// Requirement: FieldBoundary { field: "Vd", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_vd_1_poweroftwo_600_ffb21600() {
    // Thumb encoding (32): 0xFFB21600
    // Test aarch32_VCVT_hs_T1A1_A field Vd = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: size=0, Vm=0, M=0, op=0, D=0, Vd=1
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB21600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field op 8 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_op_0_min_600_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A field op = 0 (Min)
    // ISET: T32
    // Fields: M=0, D=0, Vm=0, op=0, size=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field op 8 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_op_1_max_600_ffb20700() {
    // Thumb encoding (32): 0xFFB20700
    // Test aarch32_VCVT_hs_T1A1_A field op = 1 (Max)
    // ISET: T32
    // Fields: Vd=0, D=0, size=0, op=1, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20700;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_m_0_min_600_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A field M = 0 (Min)
    // ISET: T32
    // Fields: M=0, Vd=0, op=0, size=0, D=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field M 5 +: 1`
/// Requirement: FieldBoundary { field: "M", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_m_1_max_600_ffb20620() {
    // Thumb encoding (32): 0xFFB20620
    // Test aarch32_VCVT_hs_T1A1_A field M = 1 (Max)
    // ISET: T32
    // Fields: op=0, size=0, D=0, Vd=0, M=1, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20620;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 0, boundary: Min }
/// SIMD register V0
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_vm_0_min_600_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A field Vm = 0 (Min)
    // ISET: T32
    // Fields: M=0, Vm=0, Vd=0, D=0, size=0, op=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field Vm 0 +: 4`
/// Requirement: FieldBoundary { field: "Vm", value: 1, boundary: PowerOfTwo }
/// SIMD register V1
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_field_vm_1_poweroftwo_600_ffb20601() {
    // Thumb encoding (32): 0xFFB20601
    // Test aarch32_VCVT_hs_T1A1_A field Vm = 1 (PowerOfTwo)
    // ISET: T32
    // Fields: D=0, M=0, Vm=1, op=0, size=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20601;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// D=0 (minimum value)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_combo_0_600_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A field combination: D=0, size=0, Vd=0, op=0, M=0, Vm=0
    // ISET: T32
    // Fields: size=0, op=0, M=0, Vm=0, Vd=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "size", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_special_size_0_size_variant_0_1536_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A special value size = 0 (Size variant 0)
    // ISET: T32
    // Fields: D=0, Vd=0, size=0, op=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "size", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_special_size_1_size_variant_1_1536_ffb60600() {
    // Thumb encoding (32): 0xFFB60600
    // Test aarch32_VCVT_hs_T1A1_A special value size = 1 (Size variant 1)
    // ISET: T32
    // Fields: op=0, D=0, size=1, Vm=0, Vd=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB60600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size = 2 (Size variant 2)`
/// Requirement: FieldSpecial { field: "size", value: 2, meaning: "Size variant 2" }
/// Size variant 2
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_special_size_2_size_variant_2_1536_ffba0600() {
    // Thumb encoding (32): 0xFFBA0600
    // Test aarch32_VCVT_hs_T1A1_A special value size = 2 (Size variant 2)
    // ISET: T32
    // Fields: size=2, Vd=0, op=0, M=0, Vm=0, D=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFBA0600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `field size = 3 (Size variant 3)`
/// Requirement: FieldSpecial { field: "size", value: 3, meaning: "Size variant 3" }
/// Size variant 3
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_special_size_3_size_variant_3_1536_ffbe0600() {
    // Thumb encoding (32): 0xFFBE0600
    // Test aarch32_VCVT_hs_T1A1_A special value size = 3 (Size variant 3)
    // ISET: T32
    // Fields: D=0, Vd=0, Vm=0, op=0, size=3, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFBE0600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "T32 instruction should execute successfully"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `Binary { op: Ne, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: LitBits([false, true]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Ne, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"size\" }), rhs: LitBits([false, true]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_invalid_0_600_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A invalid encoding: Binary { op: Ne, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "size" }), rhs: LitBits([false, true]) }
    // ISET: T32
    // Fields: op=0, Vm=0, D=0, M=0, size=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_invalid_1_600_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: op=0, size=0, Vm=0, Vd=0, D=0, M=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: And, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "half_to_single" }), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] } }, rhs: LitBits([true]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: And, lhs: Var(QualifiedIdentifier { qualifier: Any, name: \"half_to_single\" }), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vd\" }), indices: [Single(LitInt(0))] } }, rhs: LitBits([true]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_invalid_2_600_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: And, lhs: Var(QualifiedIdentifier { qualifier: Any, name: "half_to_single" }), rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vd" }), indices: [Single(LitInt(0))] } }, rhs: LitBits([true]) }
    // ISET: T32
    // Fields: M=0, Vm=0, size=0, D=0, op=0, Vd=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_invalid_3_600_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: op=0, D=0, Vm=0, Vd=0, M=0, size=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `Binary { op: Eq, lhs: Binary { op: And, lhs: Unary { op: Not, operand: Var(QualifiedIdentifier { qualifier: Any, name: "half_to_single" }) }, rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } }, rhs: LitBits([true]) }`
/// Requirement: UndefinedEncoding { condition: "Binary { op: Eq, lhs: Binary { op: And, lhs: Unary { op: Not, operand: Var(QualifiedIdentifier { qualifier: Any, name: \"half_to_single\" }) }, rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: \"Vm\" }), indices: [Single(LitInt(0))] } }, rhs: LitBits([true]) }" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_invalid_4_600_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A invalid encoding: Binary { op: Eq, lhs: Binary { op: And, lhs: Unary { op: Not, operand: Var(QualifiedIdentifier { qualifier: Any, name: "half_to_single" }) }, rhs: Index { base: Var(QualifiedIdentifier { qualifier: Any, name: "Vm" }), indices: [Single(LitInt(0))] } }, rhs: LitBits([true]) }
    // ISET: T32
    // Fields: D=0, Vd=0, size=0, op=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `Unconditional UNDEFINED`
/// Requirement: UndefinedEncoding { condition: "Unconditional UNDEFINED" }
/// triggers Undefined
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_invalid_5_600_ffb20600() {
    // Thumb encoding (32): 0xFFB20600
    // Test aarch32_VCVT_hs_T1A1_A invalid encoding: Unconditional UNDEFINED
    // ISET: T32
    // Fields: D=0, size=0, Vd=0, op=0, M=0, Vm=0
    let mut cpu = create_thumb_cpu();
    let encoding: u32 = 0xFFB20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step();
    assert!(
        exit.is_err() || !matches!(exit.unwrap(), CpuExit::Continue),
        "expected UNDEFINED for T32 encoding"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOVZ X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// lower 16 bits (32)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_movz_oracle_32_0_f3b24680() {
    // Test MOVZ 32-bit: lower 16 bits (oracle)
    // Encoding: 0xF3B24680
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B24680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1234, "W0 should be 0x00001234");
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOVZ X0, #0x1234, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// lower 16 bits (64)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_movz_oracle_64_0_f3b24680() {
    // Test MOVZ 64-bit: lower 16 bits (oracle)
    // Encoding: 0xF3B24680
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B24680;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x1234, "X0 should be 0x0000000000001234");
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOVZ X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// shifted 16 bits (32)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_movz_oracle_32_1_f3b77fa0() {
    // Test MOVZ 32-bit: shifted 16 bits (oracle)
    // Encoding: 0xF3B77FA0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B77FA0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xABCD0000, "W0 should be 0xABCD0000");
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOVZ X0, #0xABCD, LSL #16`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 16 bits (64)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_movz_oracle_64_1_f3b77fa0() {
    // Test MOVZ 64-bit: shifted 16 bits (oracle)
    // Encoding: 0xF3B77FA0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B77FA0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(
        get_w(&cpu, 0),
        0xABCD0000,
        "X0 should be 0x00000000ABCD0000"
    );
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOVZ X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm16 (32)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_movz_oracle_32_2_f3bfffe0() {
    // Test MOVZ 32-bit: max imm16 (oracle)
    // Encoding: 0xF3BFFFE0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3BFFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "W0 should be 0x0000FFFF");
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOVZ X0, #0xFFFF, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// max imm16 (64)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_movz_oracle_64_2_f3bfffe0() {
    // Test MOVZ 64-bit: max imm16 (oracle)
    // Encoding: 0xF3BFFFE0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3BFFFE0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFFFF, "X0 should be 0x000000000000FFFF");
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOVZ X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero imm16 (32)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_movz_oracle_32_3_f3b20600() {
    // Test MOVZ 32-bit: zero imm16 (oracle)
    // Encoding: 0xF3B20600
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "W0 should be 0x00000000");
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOVZ X0, #0x0000, LSL #0`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// zero imm16 (64)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_movz_oracle_64_3_f3b20600() {
    // Test MOVZ 64-bit: zero imm16 (oracle)
    // Encoding: 0xF3B20600
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3B20600;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000000000000000");
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOVZ X0, #0x5678, LSL #32`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 32 bits (64)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_movz_oracle_64_4_f3facf00() {
    // Test MOVZ 64-bit: shifted 32 bits (oracle)
    // Encoding: 0xF3FACF00
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3FACF00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0x0000567800000000");
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOVZ X0, #0xDEAD, LSL #48`
/// Requirement: RegisterWrite { reg_type: Gp64, dest_field: "Rd" }
/// shifted 48 bits (64)
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_movz_oracle_64_5_f3fbd7a0() {
    // Test MOVZ 64-bit: shifted 48 bits (oracle)
    // Encoding: 0xF3FBD7A0
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3FBD7A0;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "X0 should be 0xDEAD000000000000");
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOV R0, #10`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// small immediate
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_a32_mov_imm_0_f3a0000a() {
    // Test A32 MOV: small immediate (oracle)
    // Encoding: 0xF3A0000A
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A0000A;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xA, "R0 should be 0x0000000A");
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOV R0, #255`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// max imm8
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_a32_mov_imm_1_f3a000ff() {
    // Test A32 MOV: max imm8 (oracle)
    // Encoding: 0xF3A000FF
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A000FF;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xFF, "R0 should be 0x000000FF");
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOV R0, #32`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 2
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_a32_mov_imm_2_f3a00180() {
    // Test A32 MOV: rotated by 2 (oracle)
    // Encoding: 0xF3A00180
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A00180;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x20, "R0 should be 0x00000020");
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOV R0, #251658240`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// rotated by 8
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_a32_mov_imm_3_f3a0040f() {
    // Test A32 MOV: rotated by 8 (oracle)
    // Encoding: 0xF3A0040F
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A0040F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0xF000000, "R0 should be 0x0F000000");
}

/// Provenance: aarch32_VCVT_hs_T1A1_A
/// ASL: `MOV R0, #0`
/// Requirement: RegisterWrite { reg_type: Gp32, dest_field: "Rd" }
/// zero immediate
#[test]
fn test_aarch32_vcvt_hs_t1a1_a_a32_mov_imm_4_f3a00000() {
    // Test A32 MOV: zero immediate (oracle)
    // Encoding: 0xF3A00000
    // ISET: A32
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0xF3A00000;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_w(&cpu, 0), 0x0, "R0 should be 0x00000000");
}
