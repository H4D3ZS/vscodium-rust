//! A64 vector other tests.
//!
//! Auto-generated from ARM ASL specifications.
//! DO NOT EDIT MANUALLY.

#![allow(unused_imports)]
#![allow(dead_code)]

use crate::generated::test_helpers::*;

// ============================================================================
// aarch64_vector_logical Tests
// ============================================================================

/// Provenance: aarch64_vector_logical
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_logical_field_q_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field Q = 0 (Min)
    // Fields: h=0, c=0, cmode=0, b=0, Rd=0, a=0, d=0, Q=0, f=0, e=0, g=0, op=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_logical_field_q_1_max_400_4f000400() {
    // Encoding: 0x4F000400
    // Test aarch64_vector_logical field Q = 1 (Max)
    // Fields: cmode=0, g=0, d=0, Rd=0, f=0, h=0, Q=1, op=0, a=0, b=0, c=0, e=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field op 29 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_logical_field_op_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field op = 0 (Min)
    // Fields: op=0, c=0, e=0, g=0, d=0, a=0, b=0, Rd=0, cmode=0, Q=0, f=0, h=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field op 29 +: 1`
/// Requirement: FieldBoundary { field: "op", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_logical_field_op_1_max_400_2f000400() {
    // Encoding: 0x2F000400
    // Test aarch64_vector_logical field op = 1 (Max)
    // Fields: c=0, f=0, b=0, a=0, d=0, Q=0, g=0, cmode=0, op=1, e=0, h=0, Rd=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field a 18 +: 1`
/// Requirement: FieldBoundary { field: "a", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_logical_field_a_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field a = 0 (Min)
    // Fields: Rd=0, cmode=0, f=0, h=0, c=0, a=0, b=0, e=0, op=0, d=0, Q=0, g=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field a 18 +: 1`
/// Requirement: FieldBoundary { field: "a", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_logical_field_a_1_max_400_0f040400() {
    // Encoding: 0x0F040400
    // Test aarch64_vector_logical field a = 1 (Max)
    // Fields: op=0, c=0, Q=0, a=1, d=0, b=0, cmode=0, e=0, g=0, h=0, f=0, Rd=0
    let encoding: u32 = 0x0F040400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field b 17 +: 1`
/// Requirement: FieldBoundary { field: "b", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_logical_field_b_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field b = 0 (Min)
    // Fields: a=0, h=0, Q=0, op=0, Rd=0, c=0, g=0, b=0, f=0, cmode=0, d=0, e=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field b 17 +: 1`
/// Requirement: FieldBoundary { field: "b", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_logical_field_b_1_max_400_0f020400() {
    // Encoding: 0x0F020400
    // Test aarch64_vector_logical field b = 1 (Max)
    // Fields: d=0, c=0, b=1, g=0, cmode=0, a=0, f=0, e=0, h=0, Q=0, Rd=0, op=0
    let encoding: u32 = 0x0F020400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field c 16 +: 1`
/// Requirement: FieldBoundary { field: "c", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_logical_field_c_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field c = 0 (Min)
    // Fields: e=0, h=0, Q=0, f=0, b=0, g=0, op=0, Rd=0, a=0, cmode=0, c=0, d=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field c 16 +: 1`
/// Requirement: FieldBoundary { field: "c", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_logical_field_c_1_max_400_0f010400() {
    // Encoding: 0x0F010400
    // Test aarch64_vector_logical field c = 1 (Max)
    // Fields: g=0, h=0, cmode=0, a=0, d=0, op=0, e=0, b=0, c=1, Rd=0, f=0, Q=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field cmode 12 +: 4`
/// Requirement: FieldBoundary { field: "cmode", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_logical_field_cmode_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field cmode = 0 (Min)
    // Fields: f=0, Rd=0, b=0, cmode=0, h=0, Q=0, d=0, g=0, a=0, op=0, c=0, e=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field cmode 12 +: 4`
/// Requirement: FieldBoundary { field: "cmode", value: 1, boundary: PowerOfTwo }
/// value 1
#[test]
fn test_aarch64_vector_logical_field_cmode_1_poweroftwo_400_0f001400() {
    // Encoding: 0x0F001400
    // Test aarch64_vector_logical field cmode = 1 (PowerOfTwo)
    // Fields: cmode=1, c=0, b=0, Q=0, e=0, f=0, a=0, op=0, g=0, Rd=0, h=0, d=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field cmode 12 +: 4`
/// Requirement: FieldBoundary { field: "cmode", value: 7, boundary: PowerOfTwoMinusOne }
/// midpoint (7)
#[test]
fn test_aarch64_vector_logical_field_cmode_7_poweroftwominusone_400_0f007400() {
    // Encoding: 0x0F007400
    // Test aarch64_vector_logical field cmode = 7 (PowerOfTwoMinusOne)
    // Fields: Rd=0, Q=0, h=0, op=0, f=0, e=0, a=0, g=0, d=0, b=0, c=0, cmode=7
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

/// Provenance: aarch64_vector_logical
/// ASL: `field cmode 12 +: 4`
/// Requirement: FieldBoundary { field: "cmode", value: 15, boundary: Max }
/// maximum value (15)
#[test]
fn test_aarch64_vector_logical_field_cmode_15_max_400_0f00f400() {
    // Encoding: 0x0F00F400
    // Test aarch64_vector_logical field cmode = 15 (Max)
    // Fields: op=0, b=0, Q=0, a=0, cmode=15, e=0, f=0, g=0, h=0, c=0, Rd=0, d=0
    let encoding: u32 = 0x0F00F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field d 9 +: 1`
/// Requirement: FieldBoundary { field: "d", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_logical_field_d_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field d = 0 (Min)
    // Fields: Rd=0, d=0, Q=0, f=0, g=0, b=0, cmode=0, c=0, h=0, e=0, a=0, op=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field d 9 +: 1`
/// Requirement: FieldBoundary { field: "d", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_logical_field_d_1_max_400_0f000600() {
    // Encoding: 0x0F000600
    // Test aarch64_vector_logical field d = 1 (Max)
    // Fields: op=0, a=0, b=0, c=0, cmode=0, d=1, h=0, e=0, Rd=0, Q=0, g=0, f=0
    let encoding: u32 = 0x0F000600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field e 8 +: 1`
/// Requirement: FieldBoundary { field: "e", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_logical_field_e_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field e = 0 (Min)
    // Fields: h=0, Rd=0, d=0, op=0, Q=0, cmode=0, g=0, f=0, c=0, e=0, b=0, a=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field e 8 +: 1`
/// Requirement: FieldBoundary { field: "e", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_logical_field_e_1_max_400_0f000500() {
    // Encoding: 0x0F000500
    // Test aarch64_vector_logical field e = 1 (Max)
    // Fields: a=0, e=1, b=0, cmode=0, h=0, f=0, g=0, Rd=0, Q=0, d=0, c=0, op=0
    let encoding: u32 = 0x0F000500;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field f 7 +: 1`
/// Requirement: FieldBoundary { field: "f", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_logical_field_f_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field f = 0 (Min)
    // Fields: c=0, Q=0, g=0, Rd=0, cmode=0, b=0, d=0, a=0, op=0, e=0, f=0, h=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field f 7 +: 1`
/// Requirement: FieldBoundary { field: "f", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_logical_field_f_1_max_400_0f000480() {
    // Encoding: 0x0F000480
    // Test aarch64_vector_logical field f = 1 (Max)
    // Fields: g=0, cmode=0, e=0, f=1, Rd=0, d=0, op=0, a=0, h=0, b=0, c=0, Q=0
    let encoding: u32 = 0x0F000480;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field g 6 +: 1`
/// Requirement: FieldBoundary { field: "g", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_logical_field_g_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field g = 0 (Min)
    // Fields: Q=0, cmode=0, a=0, f=0, c=0, b=0, h=0, d=0, op=0, e=0, g=0, Rd=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field g 6 +: 1`
/// Requirement: FieldBoundary { field: "g", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_logical_field_g_1_max_400_0f000440() {
    // Encoding: 0x0F000440
    // Test aarch64_vector_logical field g = 1 (Max)
    // Fields: c=0, f=0, Rd=0, b=0, e=0, a=0, g=1, cmode=0, h=0, d=0, op=0, Q=0
    let encoding: u32 = 0x0F000440;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field h 5 +: 1`
/// Requirement: FieldBoundary { field: "h", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_logical_field_h_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field h = 0 (Min)
    // Fields: c=0, e=0, Rd=0, Q=0, d=0, op=0, a=0, f=0, h=0, g=0, b=0, cmode=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field h 5 +: 1`
/// Requirement: FieldBoundary { field: "h", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_logical_field_h_1_max_400_0f000420() {
    // Encoding: 0x0F000420
    // Test aarch64_vector_logical field h = 1 (Max)
    // Fields: d=0, f=0, h=1, op=0, a=0, Rd=0, b=0, cmode=0, c=0, Q=0, e=0, g=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_logical_field_rd_0_min_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field Rd = 0 (Min)
    // Fields: Q=0, a=0, b=0, d=0, e=0, f=0, h=0, cmode=0, g=0, op=0, c=0, Rd=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_logical_field_rd_1_poweroftwo_400_0f000401() {
    // Encoding: 0x0F000401
    // Test aarch64_vector_logical field Rd = 1 (PowerOfTwo)
    // Fields: a=0, h=0, Rd=1, cmode=0, g=0, b=0, Q=0, op=0, c=0, e=0, f=0, d=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_logical_field_rd_30_poweroftwominusone_400_0f00041e() {
    // Encoding: 0x0F00041E
    // Test aarch64_vector_logical field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: b=0, e=0, cmode=0, Q=0, op=0, g=0, f=0, d=0, h=0, a=0, Rd=30, c=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_logical_field_rd_31_max_400_0f00041f() {
    // Encoding: 0x0F00041F
    // Test aarch64_vector_logical field Rd = 31 (Max)
    // Fields: h=0, b=0, d=0, a=0, e=0, g=0, Rd=31, Q=0, c=0, op=0, cmode=0, f=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_logical_combo_0_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: a=0, c=0, e=0, Q=0, Rd=0, b=0, d=0, f=0, cmode=0, op=0, g=0, h=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_logical_combo_1_400_4f000400() {
    // Encoding: 0x4F000400
    // Test aarch64_vector_logical field combination: Q=1, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: b=0, Rd=0, g=0, h=0, Q=1, op=0, a=0, c=0, cmode=0, d=0, e=0, f=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=0 (minimum value)
#[test]
fn test_aarch64_vector_logical_combo_2_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: d=0, a=0, e=0, Rd=0, cmode=0, Q=0, op=0, g=0, h=0, c=0, b=0, f=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// op=1 (maximum value (1))
#[test]
fn test_aarch64_vector_logical_combo_3_400_2f000400() {
    // Encoding: 0x2F000400
    // Test aarch64_vector_logical field combination: Q=0, op=1, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: g=0, Rd=0, a=0, d=0, cmode=0, b=0, op=1, Q=0, f=0, c=0, e=0, h=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// a=0 (minimum value)
#[test]
fn test_aarch64_vector_logical_combo_4_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: c=0, g=0, Rd=0, b=0, d=0, op=0, e=0, f=0, Q=0, a=0, h=0, cmode=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// a=1 (maximum value (1))
#[test]
fn test_aarch64_vector_logical_combo_5_400_0f040400() {
    // Encoding: 0x0F040400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=1, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: a=1, f=0, b=0, e=0, g=0, h=0, Rd=0, c=0, Q=0, op=0, cmode=0, d=0
    let encoding: u32 = 0x0F040400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// b=0 (minimum value)
#[test]
fn test_aarch64_vector_logical_combo_6_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: d=0, e=0, f=0, a=0, Rd=0, Q=0, op=0, c=0, cmode=0, g=0, h=0, b=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// b=1 (maximum value (1))
#[test]
fn test_aarch64_vector_logical_combo_7_400_0f020400() {
    // Encoding: 0x0F020400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=1, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: op=0, c=0, d=0, f=0, b=1, g=0, h=0, Rd=0, Q=0, a=0, cmode=0, e=0
    let encoding: u32 = 0x0F020400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// c=0 (minimum value)
#[test]
fn test_aarch64_vector_logical_combo_8_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: a=0, f=0, g=0, Rd=0, h=0, op=0, e=0, cmode=0, Q=0, b=0, d=0, c=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// c=1 (maximum value (1))
#[test]
fn test_aarch64_vector_logical_combo_9_400_0f010400() {
    // Encoding: 0x0F010400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=1, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: op=0, Rd=0, a=0, e=0, h=0, Q=0, d=0, cmode=0, f=0, b=0, g=0, c=1
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cmode=0 (minimum value)
#[test]
fn test_aarch64_vector_logical_combo_10_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: b=0, Q=0, a=0, d=0, cmode=0, e=0, h=0, op=0, Rd=0, c=0, f=0, g=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cmode=1 (value 1)
#[test]
fn test_aarch64_vector_logical_combo_11_400_0f001400() {
    // Encoding: 0x0F001400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=1, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: g=0, h=0, Rd=0, a=0, b=0, c=0, cmode=1, d=0, op=0, e=0, f=0, Q=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cmode=7 (midpoint (7))
#[test]
fn test_aarch64_vector_logical_combo_12_400_0f007400() {
    // Encoding: 0x0F007400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=7, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: cmode=7, d=0, g=0, f=0, e=0, h=0, Rd=0, a=0, op=0, b=0, Q=0, c=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// cmode=15 (maximum value (15))
#[test]
fn test_aarch64_vector_logical_combo_13_400_0f00f400() {
    // Encoding: 0x0F00F400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=15, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: d=0, a=0, f=0, b=0, e=0, g=0, h=0, c=0, Rd=0, cmode=15, op=0, Q=0
    let encoding: u32 = 0x0F00F400;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// d=0 (minimum value)
#[test]
fn test_aarch64_vector_logical_combo_14_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: a=0, c=0, d=0, e=0, Q=0, h=0, f=0, Rd=0, b=0, op=0, cmode=0, g=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// d=1 (maximum value (1))
#[test]
fn test_aarch64_vector_logical_combo_15_400_0f000600() {
    // Encoding: 0x0F000600
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=1, e=0, f=0, g=0, h=0, Rd=0
    // Fields: d=1, b=0, Rd=0, Q=0, g=0, a=0, h=0, c=0, f=0, op=0, e=0, cmode=0
    let encoding: u32 = 0x0F000600;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// e=0 (minimum value)
#[test]
fn test_aarch64_vector_logical_combo_16_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: Rd=0, f=0, e=0, op=0, cmode=0, b=0, d=0, g=0, h=0, Q=0, a=0, c=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// e=1 (maximum value (1))
#[test]
fn test_aarch64_vector_logical_combo_17_400_0f000500() {
    // Encoding: 0x0F000500
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=1, f=0, g=0, h=0, Rd=0
    // Fields: Q=0, e=1, cmode=0, Rd=0, b=0, h=0, g=0, f=0, d=0, c=0, a=0, op=0
    let encoding: u32 = 0x0F000500;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// f=0 (minimum value)
#[test]
fn test_aarch64_vector_logical_combo_18_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: Q=0, Rd=0, h=0, a=0, op=0, g=0, c=0, cmode=0, d=0, b=0, e=0, f=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// f=1 (maximum value (1))
#[test]
fn test_aarch64_vector_logical_combo_19_400_0f000480() {
    // Encoding: 0x0F000480
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=1, g=0, h=0, Rd=0
    // Fields: c=0, b=0, g=0, d=0, a=0, e=0, h=0, f=1, op=0, Rd=0, cmode=0, Q=0
    let encoding: u32 = 0x0F000480;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// g=0 (minimum value)
#[test]
fn test_aarch64_vector_logical_combo_20_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: e=0, a=0, g=0, d=0, cmode=0, c=0, op=0, h=0, Rd=0, Q=0, f=0, b=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// g=1 (maximum value (1))
#[test]
fn test_aarch64_vector_logical_combo_21_400_0f000440() {
    // Encoding: 0x0F000440
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=1, h=0, Rd=0
    // Fields: op=0, Q=0, d=0, b=0, c=0, a=0, e=0, f=0, g=1, cmode=0, h=0, Rd=0
    let encoding: u32 = 0x0F000440;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 22`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// h=0 (minimum value)
#[test]
fn test_aarch64_vector_logical_combo_22_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: Rd=0, g=0, op=0, cmode=0, a=0, c=0, b=0, f=0, Q=0, d=0, e=0, h=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 23`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// h=1 (maximum value (1))
#[test]
fn test_aarch64_vector_logical_combo_23_400_0f000420() {
    // Encoding: 0x0F000420
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=1, Rd=0
    // Fields: op=0, c=0, a=0, cmode=0, f=0, g=0, d=0, b=0, Q=0, e=0, Rd=0, h=1
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 24`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_logical_combo_24_400_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: Rd=0, cmode=0, b=0, d=0, c=0, a=0, e=0, op=0, g=0, h=0, f=0, Q=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 25`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_logical_combo_25_400_0f000401() {
    // Encoding: 0x0F000401
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=1
    // Fields: op=0, b=0, h=0, Rd=1, f=0, c=0, Q=0, d=0, e=0, g=0, a=0, cmode=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 26`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_logical_combo_26_400_0f00041e() {
    // Encoding: 0x0F00041E
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=30
    // Fields: f=0, op=0, a=0, g=0, c=0, e=0, h=0, d=0, Q=0, cmode=0, Rd=30, b=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field combination 27`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_logical_combo_27_400_0f00041f() {
    // Encoding: 0x0F00041F
    // Test aarch64_vector_logical field combination: Q=0, op=0, a=0, b=0, c=0, cmode=0, d=0, e=0, f=0, g=0, h=0, Rd=31
    // Fields: g=0, Rd=31, Q=0, d=0, a=0, h=0, cmode=0, c=0, op=0, e=0, b=0, f=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_logical_special_q_0_size_variant_0_1024_0f000400() {
    // Encoding: 0x0F000400
    // Test aarch64_vector_logical special value Q = 0 (Size variant 0)
    // Fields: c=0, e=0, a=0, op=0, h=0, Q=0, b=0, g=0, f=0, cmode=0, Rd=0, d=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_logical_special_q_1_size_variant_1_1024_4f000400() {
    // Encoding: 0x4F000400
    // Test aarch64_vector_logical special value Q = 1 (Size variant 1)
    // Fields: Rd=0, op=0, cmode=0, d=0, e=0, a=0, c=0, b=0, Q=1, f=0, h=0, g=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_logical_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_1024_0f00041f()
 {
    // Encoding: 0x0F00041F
    // Test aarch64_vector_logical special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: c=0, cmode=0, a=0, g=0, b=0, h=0, f=0, e=0, Rd=31, op=0, d=0, Q=0
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

/// Provenance: aarch64_vector_logical
/// ASL: `SimdFromField("rd") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("rd")
#[test]
fn test_aarch64_vector_logical_reg_write_0_0f000400() {
    // Test aarch64_vector_logical register write: SimdFromField("rd")
    // Encoding: 0x0F000400
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F000400;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_logical
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_logical_zr_rd_0f00041f() {
    // Test aarch64_vector_logical with Rd = ZR (31)
    // Encoding: 0x0F00041F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00041F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}

// ============================================================================
// aarch64_vector_fp16_movi Tests
// ============================================================================

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 0, boundary: Min }
/// 8-bit / byte size
#[test]
fn test_aarch64_vector_fp16_movi_field_q_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field Q = 0 (Min)
    // Fields: c=0, a=0, g=0, d=0, h=0, Q=0, Rd=0, f=0, e=0, b=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field Q 30 +: 1`
/// Requirement: FieldBoundary { field: "Q", value: 1, boundary: Max }
/// 16-bit / halfword size
#[test]
fn test_aarch64_vector_fp16_movi_field_q_1_max_fc00_4f00fc00() {
    // Encoding: 0x4F00FC00
    // Test aarch64_vector_fp16_movi field Q = 1 (Max)
    // Fields: h=0, b=0, a=0, f=0, e=0, g=0, c=0, Q=1, d=0, Rd=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field a 18 +: 1`
/// Requirement: FieldBoundary { field: "a", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_fp16_movi_field_a_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field a = 0 (Min)
    // Fields: a=0, h=0, Rd=0, d=0, c=0, e=0, f=0, Q=0, g=0, b=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field a 18 +: 1`
/// Requirement: FieldBoundary { field: "a", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_fp16_movi_field_a_1_max_fc00_0f04fc00() {
    // Encoding: 0x0F04FC00
    // Test aarch64_vector_fp16_movi field a = 1 (Max)
    // Fields: g=0, Rd=0, d=0, f=0, a=1, c=0, Q=0, b=0, h=0, e=0
    let encoding: u32 = 0x0F04FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field b 17 +: 1`
/// Requirement: FieldBoundary { field: "b", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_fp16_movi_field_b_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field b = 0 (Min)
    // Fields: Rd=0, h=0, Q=0, c=0, g=0, a=0, f=0, d=0, e=0, b=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field b 17 +: 1`
/// Requirement: FieldBoundary { field: "b", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_fp16_movi_field_b_1_max_fc00_0f02fc00() {
    // Encoding: 0x0F02FC00
    // Test aarch64_vector_fp16_movi field b = 1 (Max)
    // Fields: a=0, h=0, d=0, g=0, c=0, b=1, Q=0, Rd=0, e=0, f=0
    let encoding: u32 = 0x0F02FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field c 16 +: 1`
/// Requirement: FieldBoundary { field: "c", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_fp16_movi_field_c_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field c = 0 (Min)
    // Fields: g=0, c=0, e=0, f=0, b=0, d=0, h=0, Rd=0, a=0, Q=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field c 16 +: 1`
/// Requirement: FieldBoundary { field: "c", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_fp16_movi_field_c_1_max_fc00_0f01fc00() {
    // Encoding: 0x0F01FC00
    // Test aarch64_vector_fp16_movi field c = 1 (Max)
    // Fields: e=0, a=0, c=1, b=0, f=0, Q=0, h=0, Rd=0, d=0, g=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field d 9 +: 1`
/// Requirement: FieldBoundary { field: "d", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_fp16_movi_field_d_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field d = 0 (Min)
    // Fields: c=0, a=0, e=0, h=0, f=0, g=0, Rd=0, Q=0, b=0, d=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field d 9 +: 1`
/// Requirement: FieldBoundary { field: "d", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_fp16_movi_field_d_1_max_fc00_0f00fe00() {
    // Encoding: 0x0F00FE00
    // Test aarch64_vector_fp16_movi field d = 1 (Max)
    // Fields: b=0, g=0, c=0, d=1, f=0, h=0, Rd=0, Q=0, a=0, e=0
    let encoding: u32 = 0x0F00FE00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field e 8 +: 1`
/// Requirement: FieldBoundary { field: "e", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_fp16_movi_field_e_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field e = 0 (Min)
    // Fields: Q=0, b=0, f=0, a=0, d=0, h=0, e=0, Rd=0, c=0, g=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field e 8 +: 1`
/// Requirement: FieldBoundary { field: "e", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_fp16_movi_field_e_1_max_fc00_0f00fd00() {
    // Encoding: 0x0F00FD00
    // Test aarch64_vector_fp16_movi field e = 1 (Max)
    // Fields: c=0, h=0, g=0, d=0, f=0, Q=0, b=0, Rd=0, e=1, a=0
    let encoding: u32 = 0x0F00FD00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field f 7 +: 1`
/// Requirement: FieldBoundary { field: "f", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_fp16_movi_field_f_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field f = 0 (Min)
    // Fields: b=0, e=0, a=0, f=0, h=0, c=0, d=0, Q=0, Rd=0, g=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field f 7 +: 1`
/// Requirement: FieldBoundary { field: "f", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_fp16_movi_field_f_1_max_fc00_0f00fc80() {
    // Encoding: 0x0F00FC80
    // Test aarch64_vector_fp16_movi field f = 1 (Max)
    // Fields: b=0, f=1, d=0, Q=0, a=0, e=0, h=0, c=0, Rd=0, g=0
    let encoding: u32 = 0x0F00FC80;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field g 6 +: 1`
/// Requirement: FieldBoundary { field: "g", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_fp16_movi_field_g_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field g = 0 (Min)
    // Fields: b=0, Q=0, d=0, g=0, Rd=0, f=0, a=0, h=0, e=0, c=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field g 6 +: 1`
/// Requirement: FieldBoundary { field: "g", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_fp16_movi_field_g_1_max_fc00_0f00fc40() {
    // Encoding: 0x0F00FC40
    // Test aarch64_vector_fp16_movi field g = 1 (Max)
    // Fields: g=1, f=0, Rd=0, c=0, a=0, h=0, e=0, Q=0, d=0, b=0
    let encoding: u32 = 0x0F00FC40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field h 5 +: 1`
/// Requirement: FieldBoundary { field: "h", value: 0, boundary: Min }
/// minimum value
#[test]
fn test_aarch64_vector_fp16_movi_field_h_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field h = 0 (Min)
    // Fields: b=0, e=0, h=0, f=0, Rd=0, g=0, a=0, c=0, Q=0, d=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field h 5 +: 1`
/// Requirement: FieldBoundary { field: "h", value: 1, boundary: Max }
/// maximum value (1)
#[test]
fn test_aarch64_vector_fp16_movi_field_h_1_max_fc00_0f00fc20() {
    // Encoding: 0x0F00FC20
    // Test aarch64_vector_fp16_movi field h = 1 (Max)
    // Fields: c=0, b=0, d=0, e=0, a=0, g=0, h=1, Q=0, Rd=0, f=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 0, boundary: Min }
/// register index 0 (first register)
#[test]
fn test_aarch64_vector_fp16_movi_field_rd_0_min_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field Rd = 0 (Min)
    // Fields: g=0, Q=0, h=0, Rd=0, b=0, c=0, a=0, d=0, e=0, f=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 1, boundary: PowerOfTwo }
/// register index 1 (second register)
#[test]
fn test_aarch64_vector_fp16_movi_field_rd_1_poweroftwo_fc00_0f00fc01() {
    // Encoding: 0x0F00FC01
    // Test aarch64_vector_fp16_movi field Rd = 1 (PowerOfTwo)
    // Fields: f=0, c=0, h=0, Rd=1, a=0, g=0, Q=0, e=0, d=0, b=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 30, boundary: PowerOfTwoMinusOne }
/// register index 30 (LR in some contexts)
#[test]
fn test_aarch64_vector_fp16_movi_field_rd_30_poweroftwominusone_fc00_0f00fc1e() {
    // Encoding: 0x0F00FC1E
    // Test aarch64_vector_fp16_movi field Rd = 30 (PowerOfTwoMinusOne)
    // Fields: a=0, Rd=30, Q=0, e=0, b=0, d=0, h=0, g=0, f=0, c=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field Rd 0 +: 5`
/// Requirement: FieldBoundary { field: "Rd", value: 31, boundary: Max }
/// register index 31 (ZR - zero register)
#[test]
fn test_aarch64_vector_fp16_movi_field_rd_31_max_fc00_0f00fc1f() {
    // Encoding: 0x0F00FC1F
    // Test aarch64_vector_fp16_movi field Rd = 31 (Max)
    // Fields: d=0, a=0, g=0, f=0, b=0, e=0, c=0, Rd=31, h=0, Q=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 0`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=0 (8-bit / byte size)
#[test]
fn test_aarch64_vector_fp16_movi_combo_0_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: g=0, b=0, f=0, Rd=0, h=0, Q=0, c=0, d=0, a=0, e=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 1`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Q=1 (16-bit / halfword size)
#[test]
fn test_aarch64_vector_fp16_movi_combo_1_fc00_4f00fc00() {
    // Encoding: 0x4F00FC00
    // Test aarch64_vector_fp16_movi field combination: Q=1, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: d=0, c=0, Rd=0, g=0, b=0, f=0, Q=1, h=0, e=0, a=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 2`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// a=0 (minimum value)
#[test]
fn test_aarch64_vector_fp16_movi_combo_2_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: c=0, g=0, d=0, e=0, b=0, a=0, f=0, h=0, Rd=0, Q=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 3`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// a=1 (maximum value (1))
#[test]
fn test_aarch64_vector_fp16_movi_combo_3_fc00_0f04fc00() {
    // Encoding: 0x0F04FC00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=1, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: Rd=0, a=1, h=0, c=0, Q=0, b=0, e=0, g=0, f=0, d=0
    let encoding: u32 = 0x0F04FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 4`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// b=0 (minimum value)
#[test]
fn test_aarch64_vector_fp16_movi_combo_4_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: c=0, f=0, e=0, h=0, a=0, g=0, Q=0, Rd=0, b=0, d=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 5`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// b=1 (maximum value (1))
#[test]
fn test_aarch64_vector_fp16_movi_combo_5_fc00_0f02fc00() {
    // Encoding: 0x0F02FC00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=1, c=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: f=0, e=0, h=0, a=0, c=0, b=1, d=0, Q=0, Rd=0, g=0
    let encoding: u32 = 0x0F02FC00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 6`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// c=0 (minimum value)
#[test]
fn test_aarch64_vector_fp16_movi_combo_6_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: a=0, h=0, b=0, g=0, e=0, d=0, f=0, Rd=0, Q=0, c=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 7`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// c=1 (maximum value (1))
#[test]
fn test_aarch64_vector_fp16_movi_combo_7_fc00_0f01fc00() {
    // Encoding: 0x0F01FC00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=1, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: d=0, e=0, Q=0, a=0, b=0, c=1, g=0, Rd=0, f=0, h=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 8`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// d=0 (minimum value)
#[test]
fn test_aarch64_vector_fp16_movi_combo_8_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: Rd=0, a=0, c=0, d=0, e=0, f=0, g=0, Q=0, h=0, b=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 9`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// d=1 (maximum value (1))
#[test]
fn test_aarch64_vector_fp16_movi_combo_9_fc00_0f00fe00() {
    // Encoding: 0x0F00FE00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=1, e=0, f=0, g=0, h=0, Rd=0
    // Fields: a=0, h=0, e=0, f=0, c=0, Q=0, b=0, Rd=0, d=1, g=0
    let encoding: u32 = 0x0F00FE00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 10`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// e=0 (minimum value)
#[test]
fn test_aarch64_vector_fp16_movi_combo_10_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: g=0, b=0, c=0, Rd=0, a=0, f=0, Q=0, d=0, e=0, h=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 11`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// e=1 (maximum value (1))
#[test]
fn test_aarch64_vector_fp16_movi_combo_11_fc00_0f00fd00() {
    // Encoding: 0x0F00FD00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=1, f=0, g=0, h=0, Rd=0
    // Fields: c=0, f=0, h=0, b=0, Rd=0, Q=0, d=0, g=0, e=1, a=0
    let encoding: u32 = 0x0F00FD00;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 12`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// f=0 (minimum value)
#[test]
fn test_aarch64_vector_fp16_movi_combo_12_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: e=0, g=0, h=0, a=0, Rd=0, d=0, b=0, Q=0, c=0, f=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 13`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// f=1 (maximum value (1))
#[test]
fn test_aarch64_vector_fp16_movi_combo_13_fc00_0f00fc80() {
    // Encoding: 0x0F00FC80
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=1, g=0, h=0, Rd=0
    // Fields: f=1, d=0, c=0, Q=0, b=0, g=0, h=0, e=0, a=0, Rd=0
    let encoding: u32 = 0x0F00FC80;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 14`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// g=0 (minimum value)
#[test]
fn test_aarch64_vector_fp16_movi_combo_14_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: h=0, Q=0, Rd=0, a=0, g=0, d=0, e=0, b=0, c=0, f=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 15`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// g=1 (maximum value (1))
#[test]
fn test_aarch64_vector_fp16_movi_combo_15_fc00_0f00fc40() {
    // Encoding: 0x0F00FC40
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=1, h=0, Rd=0
    // Fields: b=0, a=0, e=0, Q=0, d=0, c=0, f=0, g=1, h=0, Rd=0
    let encoding: u32 = 0x0F00FC40;
    let mut cpu = create_test_cpu();
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(
        exit,
        CpuExit::Continue,
        "instruction 0x{:08X} should execute successfully",
        encoding
    );
}

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 16`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// h=0 (minimum value)
#[test]
fn test_aarch64_vector_fp16_movi_combo_16_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: d=0, a=0, e=0, h=0, Rd=0, Q=0, g=0, f=0, b=0, c=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 17`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// h=1 (maximum value (1))
#[test]
fn test_aarch64_vector_fp16_movi_combo_17_fc00_0f00fc20() {
    // Encoding: 0x0F00FC20
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=1, Rd=0
    // Fields: c=0, Q=0, e=0, f=0, Rd=0, d=0, g=0, b=0, a=0, h=1
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 18`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=0 (register index 0 (first register))
#[test]
fn test_aarch64_vector_fp16_movi_combo_18_fc00_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=0
    // Fields: Q=0, d=0, h=0, c=0, e=0, f=0, a=0, Rd=0, g=0, b=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 19`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=1 (register index 1 (second register))
#[test]
fn test_aarch64_vector_fp16_movi_combo_19_fc00_0f00fc01() {
    // Encoding: 0x0F00FC01
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=1
    // Fields: b=0, g=0, f=0, Rd=1, c=0, e=0, a=0, Q=0, h=0, d=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 20`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=30 (register index 30 (LR in some contexts))
#[test]
fn test_aarch64_vector_fp16_movi_combo_20_fc00_0f00fc1e() {
    // Encoding: 0x0F00FC1E
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=30
    // Fields: b=0, g=0, Rd=30, c=0, d=0, f=0, a=0, e=0, Q=0, h=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field combination 21`
/// Requirement: FieldExtraction { field: "combination", bit_start: 0, bit_width: 32 }
/// Rd=31 (register index 31 (ZR - zero register))
#[test]
fn test_aarch64_vector_fp16_movi_combo_21_fc00_0f00fc1f() {
    // Encoding: 0x0F00FC1F
    // Test aarch64_vector_fp16_movi field combination: Q=0, a=0, b=0, c=0, d=0, e=0, f=0, g=0, h=0, Rd=31
    // Fields: h=0, d=0, e=0, b=0, c=0, Q=0, Rd=31, a=0, f=0, g=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field Q = 0 (Size variant 0)`
/// Requirement: FieldSpecial { field: "Q", value: 0, meaning: "Size variant 0" }
/// Size variant 0
#[test]
fn test_aarch64_vector_fp16_movi_special_q_0_size_variant_0_64512_0f00fc00() {
    // Encoding: 0x0F00FC00
    // Test aarch64_vector_fp16_movi special value Q = 0 (Size variant 0)
    // Fields: b=0, e=0, f=0, d=0, Q=0, a=0, Rd=0, h=0, c=0, g=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field Q = 1 (Size variant 1)`
/// Requirement: FieldSpecial { field: "Q", value: 1, meaning: "Size variant 1" }
/// Size variant 1
#[test]
fn test_aarch64_vector_fp16_movi_special_q_1_size_variant_1_64512_4f00fc00() {
    // Encoding: 0x4F00FC00
    // Test aarch64_vector_fp16_movi special value Q = 1 (Size variant 1)
    // Fields: Rd=0, Q=1, b=0, e=0, d=0, a=0, c=0, f=0, g=0, h=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `field Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)`
/// Requirement: FieldSpecial { field: "Rd", value: 31, meaning: "Zero register (XZR/WZR) - reads as 0, writes discarded" }
/// Zero register (XZR/WZR) - reads as 0, writes discarded
#[test]
fn test_aarch64_vector_fp16_movi_special_rd_31_zero_register_xzr_wzr_reads_as_0_writes_discarded_64512_0f00fc1f()
 {
    // Encoding: 0x0F00FC1F
    // Test aarch64_vector_fp16_movi special value Rd = 31 (Zero register (XZR/WZR) - reads as 0, writes discarded)
    // Fields: h=0, g=0, Rd=31, e=0, f=0, b=0, Q=0, d=0, c=0, a=0
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

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `SimdFromField("rd") write`
/// Requirement: RegisterWrite { reg_type: Simd128, dest_field: "unknown" }
/// verify register write to SimdFromField("rd")
#[test]
fn test_aarch64_vector_fp16_movi_reg_write_0_0f00fc00() {
    // Test aarch64_vector_fp16_movi register write: SimdFromField("rd")
    // Encoding: 0x0F00FC00
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00FC00;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
}

/// Provenance: aarch64_vector_fp16_movi
/// ASL: `Rd = 31 (ZR)`
/// Requirement: RegisterSpecial { reg: Zr, behavior: "reads as 0, writes discarded" }
/// zero register (Rd = 31)
#[test]
fn test_aarch64_vector_fp16_movi_zr_rd_0f00fc1f() {
    // Test aarch64_vector_fp16_movi with Rd = ZR (31)
    // Encoding: 0x0F00FC1F
    let mut cpu = create_test_cpu();
    let encoding: u32 = 0x0F00FC1F;
    write_insn(&mut cpu, 0, encoding);
    let exit = cpu.step().unwrap();
    assert_eq!(exit, CpuExit::Continue, "instruction should execute");
    assert_eq!(get_x(&cpu, 31), 0, "XZR should always be 0");
}
